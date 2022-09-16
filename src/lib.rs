mod template;
pub use template::{preprocess, Template};

mod walk;
pub use walk::RecursiveWalker;

pub fn src_dir() -> std::ffi::OsString {
    std::env::var_os("SSS_SRC").unwrap_or_else(|| String::from("src").into())
}

pub fn dst_dir() -> std::ffi::OsString {
    std::env::var_os("SSS_DST").unwrap_or_else(|| String::from("dst").into())
}

pub fn tmpl_dir() -> std::ffi::OsString {
    std::env::var_os("SSS_TMPL").unwrap_or_else(|| String::from("tmpl").into())
}
