use std::fs::File;
use std::io::{self, BufRead};

use crate::Slide;

pub fn parse() -> Result<Vec<Slide>, io::Error> {
    let mut slide = Slide::new();
    let mut slides = Vec::new();

    let file = File::open("test")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(s) = line {
            match (slide.linecount, s.trim().is_empty()) {
                // Empty line and no previously begun slide. Just continue
                (0, true) => continue,

                // Non-empty line and previously begun slide. Push the line
                (_, false) => slide.push_line(&s),

                // Empty line and previously begun slide. Append the slide to the collection and
                // create new one.
                (_, true) => {
                    slides.push(slide);
                    slide = Slide::new();
                },
            }
        }
    }
    // Get the last slide
    slides.push(slide);

    Ok(slides)
}
