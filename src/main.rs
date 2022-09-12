#![feature(iter_intersperse)]

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio};

fn preprocess<R: Read, W: Write>(r: &mut R, w: &mut W) {
    let mut buf = String::new();
    r.read_to_string(&mut buf).unwrap();

    write!(w, "echo \"").unwrap();

    let mut echo = true;
    buf.split("\n#!\n")
        .intersperse_with(|| {
            echo = !echo;
            match echo {
                true => "\necho \"",
                false => "\"\n",
            }
        })
        .for_each(|s| write!(w, "{}", s).unwrap());

    if echo {
        write!(w, "\"").unwrap();
    }
}

fn main() {
    let mut file = File::open(std::env::args().nth(1).unwrap()).unwrap();

    let mut cmd = Command::new("/bin/sh")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = cmd.stdin.as_mut().unwrap();

    preprocess(&mut file, stdin);

    let out = String::from_utf8(cmd.wait_with_output().unwrap().stdout).unwrap();
    println!("{}", out);
}
