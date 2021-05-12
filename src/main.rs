mod slide;

use slide::*;

fn main() {
    let slides = match Presentation::new("test") {
        Ok(slides) => slides,
        Err(_) => panic!("Could not find a file"),
    };
    for slide in slides {
        println!("{}\n---", slide.text); 
    }
}
