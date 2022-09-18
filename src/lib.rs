mod template;
pub use template::{preprocess, Template};

mod walk;
pub use walk::RecursiveWalker;

#[macro_export]
macro_rules! env_with_default {
    ($env:expr, $default:expr) => {
        std::env::var_os($env).unwrap_or_else(|| $default.into())
    };
}
