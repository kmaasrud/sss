use std::error::Error;
use std::ffi::OsStr;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};

enum WalkNode {
    Dir(ReadDir),
    File(PathBuf),
}

pub struct RecursiveWalker {
    stack: Vec<WalkNode>,
    extension_filter: Option<String>,
}

impl RecursiveWalker {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let stack = vec![WalkNode::Dir(path.as_ref().read_dir()?)];

        Ok(Self {
            stack,
            extension_filter: None,
        })
    }

    #[must_use]
    pub fn extension_filter(self, filter: impl Into<String>) -> Self {
        Self {
            extension_filter: Some(filter.into()),
            ..self
        }
    }
}

impl Iterator for RecursiveWalker {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                Some(WalkNode::Dir(iter)) => {
                    iter.filter_map(|e| e.ok()).for_each(|e| {
                        let path = e.path();
                        if path.is_dir() {
                            self.stack.push(WalkNode::Dir(path.read_dir().unwrap()));
                        } else {
                            self.stack.push(WalkNode::File(path));
                        }
                    });
                }
                Some(WalkNode::File(path)) => match (&self.extension_filter, path.extension()) {
                    (None, _) => return Some(path),
                    (Some(ext_filter), Some(ext)) if OsStr::new(ext_filter) == ext => {
                        return Some(path)
                    }
                    _ => {}
                },
                None => return None,
            }
        }
    }
}
