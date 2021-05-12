use std::{
    io::{BufRead, BufReader, Lines, Error},
    fs::File,
    iter::Iterator,
    path::PathBuf,
};

pub struct Presentation {
    filepath: PathBuf,
    lines: Lines<BufReader<File>>,
}

impl Presentation {
    pub fn new(path: &str) -> Result<Self, Error> {
        let file = File::open(path)?;
        Ok(Presentation{ filepath: PathBuf::from(path), lines: BufReader::new(file).lines() })
    }
}

impl Iterator for Presentation {
    type Item = Slide;

    fn next(&mut self) -> Option<Self::Item> {
        let mut slide = Slide::new();
        loop {
            match self.lines.next() {
                Some(line) => {
                    if let Ok(s) = line {
                        match (slide.linecount, s.trim().is_empty()) {
                            // Empty line and no previously begun slide. Just continue
                            (0, true) => continue,

                            // Non-empty line and previously begun slide. Push the line
                            (_, false) => slide.push_line(&s),

                            // Empty line and previously begun slide. Append the slide to the collection and
                            // create new one.
                            (_, true) => {
                                return Some(slide)
                            },
                        }
                    }
                }
                None => {
                    if slide.linecount != 0 {
                        return Some(slide)
                    } else {
                        return None
                    }
                },
            }
        }        
    }
}

pub struct Slide {
    pub text: String,
    pub linecount: usize,
}

impl Slide {
    pub fn new() -> Self {
        Slide { text: "".to_string(), linecount: 0, }
    }

    pub fn push_line(&mut self, text: &str) {
        if self.linecount != 0 { self.text.push('\n'); }
        self.text.push_str(text);
        self.linecount += 1;
    }
}
