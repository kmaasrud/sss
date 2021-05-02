mod slide;
mod parse;

use slide::Slide;

fn main() {
    let slides = parse::parse().unwrap();
    for slide in slides.iter() {
        println!("{}\n", slide.text); 
    }
}
