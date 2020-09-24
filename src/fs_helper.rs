use std::path::{Path, PathBuf};

pub fn contains_any_suffixes<P: AsRef<Path>>(path: P, list: &Vec<&str>) -> bool
where
    PathBuf: From<P>,
{
    let path = PathBuf::from(path);
    list.iter().any(|x| path.ends_with(x))
}
