use sss::{RecursiveWalker, Template};
use std::{error::Error, fs::File, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let walker = RecursiveWalker::new(sss::src_dir())?;

    let tmpl_dir = PathBuf::from(sss::tmpl_dir());
    let dst_dir = PathBuf::from(sss::dst_dir());

    std::fs::create_dir_all(&dst_dir)?;

    for path in walker {
        let tmpl_path = tmpl_dir
            .components()
            .chain(path.components().skip(1))
            .collect::<PathBuf>()
            .with_extension("tmpl");

        let mut tmpl = match tmpl_path.is_file() {
            true => Template::new(&tmpl_path)?,
            false => Template::new(tmpl_dir.join("default.tmpl"))?,
        };

        tmpl.env("SRC", &path);
        tmpl.env("CONTENT", std::fs::read_to_string(&path)?);

        let dst_path = dst_dir
            .components()
            .chain(path.components().skip(1))
            .collect::<PathBuf>()
            .with_extension("html");

        tmpl.render(&mut File::create(dst_path)?)?;
    }

    Ok(())
}
