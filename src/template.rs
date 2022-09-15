use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn preprocess<R: BufRead, W: Write>(r: &mut R, w: &mut W) -> Result<(), Box<dyn Error>> {
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

pub fn render<R: BufRead + Send, W: Write + Send>(
    r: &mut R,
    w: &mut W,
) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("/bin/sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = cmd.stdin.take().unwrap();
        let stdout = cmd.stdout.take().unwrap();

        std::thread::scope(|s| {
            s.spawn(|| {
                stdout.bytes().flatten().for_each(|byte| {
                    w.write_all(&[byte]).unwrap();
                })
            });

            preprocess(r, &mut stdin).unwrap();
            drop(stdin);
        });
    }

    cmd.wait()?;

    Ok(())
}

pub fn render_template_file<P: AsRef<Path>, W: Write + Send>(
    path: P,
    w: &mut W,
) -> Result<(), Box<dyn Error>> {
    let mut file = BufReader::new(File::open(path)?);
    render(&mut file, w)
}

pub fn render_template_file_to_string<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut bytes = Vec::new();
    render_template_file(path, &mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}
