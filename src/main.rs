use sss::{render_template_file_to_string, RecursiveWalker};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).unwrap();

    let walker = RecursiveWalker::new(path)?.extension_filter("tmpl");

    for path in walker {
        println!("{}", render_template_file_to_string(path)?);
    }

    Ok(())
}
