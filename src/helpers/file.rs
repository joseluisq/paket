use std::path::{Path, PathBuf};

pub fn has_any_suffixes<P: AsRef<Path>>(path: P, suffixes: &[&str]) -> bool
where
    PathBuf: From<P>,
{
    let path = PathBuf::from(path);
    suffixes.iter().any(|x| path.ends_with(x))
}
