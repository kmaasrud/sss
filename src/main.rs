#![feature(iter_intersperse)]
use sss::{env_with_default, RecursiveWalker, Template};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let src_dir = PathBuf::from(env_with_default!("SSS_SRC", "src"));
    let tmpl_dir = PathBuf::from(env_with_default!("SSS_TMPL", "tmpl"));
    let dst_dir = PathBuf::from(env_with_default!("SSS_DST", "dst"));

    std::fs::create_dir_all(&dst_dir)?;

    let src_paths: Vec<PathBuf> = RecursiveWalker::new(&src_dir)?.collect();
    let srcs = src_paths.iter().fold(String::new(), |acc, x| {
        format!("{acc}\n{}", x.to_string_lossy())
    });

    let mut tmpl_paths: Vec<PathBuf> = RecursiveWalker::new(&tmpl_dir)?.collect();

    let i = tmpl_paths
        .iter()
        .position(|x| *x == tmpl_dir.join("default.html"))
        .unwrap();
    tmpl_paths.swap_remove(i);

    for src in src_paths.iter() {
        let tmpl_path = exchange_dirs(src, &src_dir, &tmpl_dir).with_extension("html");

        let mut tmpl = match tmpl_paths.iter().position(|x| *x == tmpl_path) {
            Some(i) => {
                tmpl_paths.swap_remove(i);
                Template::new(&tmpl_path)?
            }
            None => Template::new(tmpl_dir.join("default.html"))?,
        };

        tmpl.env("src", src);
        tmpl.env("srcs", &srcs);
        tmpl.env("path_to_root", path_to_root(src, &src_dir));
        tmpl.env("src_dir", &src_dir);
        tmpl.env("tmpl_dir", &tmpl_dir);
        tmpl.env("dst_dir", &dst_dir);

        let dst_path = exchange_dirs(src, &src_dir, &dst_dir).with_extension("html");

        if let Some(parent) = dst_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        tmpl.render(&mut File::create(dst_path)?)?;
    }

    for tmpl_path in tmpl_paths.iter() {
        let mut tmpl = Template::new(tmpl_path)?;
        tmpl.env("srcs", &srcs);
        tmpl.env("path_to_root", path_to_root(tmpl_path, &tmpl_dir));
        tmpl.env("src_dir", &src_dir);
        tmpl.env("tmpl_dir", &tmpl_dir);
        tmpl.env("dst_dir", &dst_dir);

        let dst_path = exchange_dirs(tmpl_path, &tmpl_dir, &dst_dir);
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

fn path_to_root<A: AsRef<Path>, B: AsRef<Path>>(path: A, root: B) -> String {
    path.as_ref()
        .components()
        .skip(root.as_ref().components().count() + 1)
        .map(|_| "..")
        .intersperse("/")
        .collect()
}
