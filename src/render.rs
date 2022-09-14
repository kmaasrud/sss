use crate::preprocess;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn render<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut file = BufReader::new(File::open(path)?);

    let mut cmd = Command::new("/bin/sh").stdin(Stdio::piped()).spawn()?;

    let stdin = cmd.stdin.as_mut().unwrap();

    preprocess(&mut file, stdin)?;

    Ok(String::from_utf8(cmd.wait_with_output()?.stdout)?)
}
