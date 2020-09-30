use std::fs;
use std::path::{Path, PathBuf};

use crate::result::{Context, Result};

pub fn has_path_any_suffixes<P: AsRef<Path>>(path: P, suffixes: &[&str]) -> bool
where
    PathBuf: From<P>,
{
    let path = PathBuf::from(path);
    suffixes.iter().any(|x| path.ends_with(x))
}

pub fn read(path: &Path) -> Result<String> {
    match String::from_utf8(read_bytes(path)?) {
        Ok(s) => Ok(s),
        Err(_) => bail!("path at `{}` was not valid utf-8", path.display()),
    }
}

pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("failed to read `{}`", path.display()))
}
