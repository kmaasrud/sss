use sss::{RecursiveWalker, Template};
use std::{error::Error, fs::File, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    for path in RecursiveWalker::new("content")?.extension_filter("md") {
        let mut out_path = PathBuf::from("out");
        let mut tmpl_path = PathBuf::from("tmpl");
        for component in path.components().skip(1) {
            out_path.push(component);
            tmpl_path.push(component);
        }
        out_path.set_extension("html");
        tmpl_path.set_extension("tmpl");

        let mut tmpl = Template::new(&tmpl_path)?;
        tmpl.env("SRC", &path);
        tmpl.env("CONTENT", std::fs::read_to_string(&path)?);

        tmpl.render(&mut File::create(out_path)?)?;
    }

    Ok(())
}
