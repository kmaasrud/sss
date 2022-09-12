#![feature(iter_intersperse)]

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn preprocess<R: BufRead, W: Write>(r: &mut R, w: &mut W) -> Result<(), Box<dyn Error>> {
    write!(w, "echo \"")?;

    let mut echo = true;
    for line in r.lines().flatten() {
        match line.trim() {
            "#!" => {
                echo = !echo;
                match echo {
                    true => write!(w, "\necho \""),
                    false => writeln!(w, "\""),
                }
            }
            s => writeln!(w, "{}", s),
        }?;
    }

    if echo {
        write!(w, "\"")?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = BufReader::new(File::open(std::env::args().nth(1).unwrap())?);

    let mut cmd = Command::new("/bin/sh").stdin(Stdio::piped()).spawn()?;

    let stdin = cmd.stdin.as_mut().unwrap();

    preprocess(&mut file, stdin)?;

    let out = String::from_utf8(cmd.wait_with_output()?.stdout)?;
    println!("{}", out);

    Ok(())
}
