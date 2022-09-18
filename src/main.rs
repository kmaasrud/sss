use sss::{env_with_default, RecursiveWalker, Template};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let src_dir = PathBuf::from(env_with_default!("SSS_SRC", "src"));
    let tmpl_dir = PathBuf::from(env_with_default!("SSS_TMPL", "tmpl"));
    let dst_dir = PathBuf::from(env_with_default!("SSS_DST", "dst"));

    std::fs::create_dir_all(&dst_dir)?;

    let paths: Vec<PathBuf> = RecursiveWalker::new(&src_dir)?.collect();
    for path in paths.iter() {
        let tmpl_path = exchange_dirs(path, &src_dir, &tmpl_dir).with_extension("html");

        let mut tmpl = match tmpl_path.is_file() {
            true => Template::new(&tmpl_path)?,
            false => Template::new(tmpl_dir.join("default.html"))?,
        };

        tmpl.env("src", path);
        tmpl.env("content", std::fs::read_to_string(path)?);
        tmpl.env(
            "srcs",
            paths.iter().fold(String::new(), |acc, x| {
                format!("{acc} {}", x.to_string_lossy())
            }),
        );

        let dst_path = exchange_dirs(path, &src_dir, &dst_dir).with_extension("html");

        tmpl.render(&mut File::create(dst_path)?)?;
    }

    Ok(())
}

fn exchange_dirs<A, B, C>(path: A, from: B, to: C) -> PathBuf
where
    A: AsRef<Path>,
    B: AsRef<Path>,
    C: AsRef<Path>,
{
    to.as_ref()
        .components()
        .chain(
            path.as_ref()
                .components()
                .skip(from.as_ref().components().count()),
        )
        .collect()
}
