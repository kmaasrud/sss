use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::Iterator,
    path::Path,
};

pub struct Presentation {
    reader: BufReader<File>,
}

impl Presentation {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(path)?;
        Ok(Presentation {
            reader: BufReader::new(file),
        })
    }
}

impl Iterator for Presentation {
    type Item = Slide;

    fn next(&mut self) -> Option<Self::Item> {
        let mut slide = Slide::new();
        let mut buf = String::new();

        while !matches!(self.reader.read_line(&mut buf), Ok(0)) {
            match (buf.trim().is_empty(), slide.text.is_empty()) {
                (true, false) => return Some(slide),
                (false, _) => slide.push(&buf),
                _ => {}
            };

            buf.clear();
        }

        if slide.text.is_empty() {
            None
        } else {
            Some(slide)
        }
    }
}

pub struct Slide {
    pub text: String,
}

impl Slide {
    pub fn new() -> Self {
        Slide {
            text: "".to_string(),
        }
    }

    pub fn push(&mut self, text: &str) {
        self.text.push_str(text);
    }
}
