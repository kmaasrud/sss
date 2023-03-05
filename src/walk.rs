use std::fs::ReadDir;
use std::io;
use std::path::{Path, PathBuf};

enum WalkNode {
    Dir(ReadDir),
    File(PathBuf),
}

pub struct Walker {
    stack: Vec<WalkNode>,
}

impl Walker {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut stack = Vec::new();
        let path = path.as_ref();

        if path.is_dir() {
            match path.read_dir() {
                Ok(read_dir) => stack.push(WalkNode::Dir(read_dir)),
                Err(e) => return Err(e),
            }
        } else {
            stack.push(WalkNode::File(path.into()));
        }

        Ok(Self { stack })
    }
}

impl Iterator for Walker {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                Some(WalkNode::Dir(iter)) => {
                    iter.filter_map(|e| e.ok()).for_each(|e| {
                        let path = e.path();
                        if path.is_dir() {
                            if let Ok(read_dir) = path.read_dir() {
                                self.stack.push(WalkNode::Dir(read_dir));
                            }
                        } else {
                            self.stack.push(WalkNode::File(path));
                        }
                    });
                }
                Some(WalkNode::File(path)) => return Some(path),
                None => return None,
            }
        }
    }
}
