mod slide;

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("test").unwrap();
    for line in io::BufReader::new(file).lines() {
        if let Ok(s) = line {
            if s.trim().is_empty() {
                println!("Is empty line");
            }
        }
    }
}
