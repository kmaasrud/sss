use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct Template {
    file: BufReader<File>,
    envs: HashMap<OsString, OsString>,
}

impl Template {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            file: BufReader::new(File::open(path)?),
            envs: HashMap::new(),
        })
    }

    pub fn env<K: Into<OsString>, V: Into<OsString>>(&mut self, k: K, v: V) {
        self.envs.insert(k.into(), v.into());
    }

    pub fn envs<I, K, V>(&mut self, envs: I)
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<OsString>,
        V: Into<OsString>,
    {
        for (k, v) in envs.into_iter() {
            self.env(k, v);
        }
    }

    pub fn render<W: Write + Send>(mut self, w: &mut W) -> Result<(), Box<dyn Error>> {
        if !self.file.fill_buf().map(|b| !b.is_empty())? {
            self.file.rewind()?;
        }

        let mut cmd = Command::new("/bin/sh")
            .envs(self.envs)
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

                preprocess(&mut self.file, &mut stdin).unwrap();
                drop(stdin);
            });
        }

        cmd.wait()?;

        Ok(())
    }
}

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
