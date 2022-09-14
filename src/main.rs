use sss::RecursiveWalker;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).unwrap();

    let walker = RecursiveWalker::new(path)?.extension_filter("md");

    for path in walker {
        println!("{:?}", path);
    }

    Ok(())
}
