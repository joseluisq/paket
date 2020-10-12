// NOTE: Several functions here were borrowed from Cargo
// https://github.com/rust-lang/cargo/blob/master/src/cargo/util/toml/mod.rs
use std::fs;
use std::path::{Path, PathBuf};

use crate::result::{Context, Result};

/// Check if a path matches a given list of suffixes.
pub fn has_path_any_suffixes<P: AsRef<Path>>(path: P, suffixes: &[&str]) -> bool
where
    PathBuf: From<P>,
{
    let path = PathBuf::from(path);
    suffixes.iter().any(|x| path.ends_with(x))
}

/// Read an UTF-8 file from a specific path.
pub fn read(path: &Path) -> Result<String> {
    match String::from_utf8(read_bytes(path)?) {
        Ok(s) => Ok(s),
        Err(_) => bail!("path at `{}` was not valid utf-8", path.display()),
    }
}

/// Read the entire contents of a file into a bytes vector.
pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("failed to read `{}`", path.display()))
}

pub fn stringify(dst: &mut String, path: &serde_ignored::Path<'_>) {
    use serde_ignored::Path;

    match *path {
        Path::Root => {}
        Path::Seq { parent, index } => {
            stringify(dst, parent);
            if !dst.is_empty() {
                dst.push('.');
            }
            dst.push_str(&index.to_string());
        }
        Path::Map { parent, ref key } => {
            stringify(dst, parent);
            if !dst.is_empty() {
                dst.push('.');
            }
            dst.push_str(key);
        }
        Path::Some { parent }
        | Path::NewtypeVariant { parent }
        | Path::NewtypeStruct { parent } => stringify(dst, parent),
    }
}
