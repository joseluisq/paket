use serde::Deserialize;
use std::path::Path;

use crate::helpers::file;
use crate::result::Result;

pub mod conf;

pub use conf::*;

pub fn read_pkg_file(path: &Path) -> Result<toml::Value> {
    let toml = file::read(path)?;

    let first_error = match toml.parse() {
        Ok(ret) => return Ok(ret),
        Err(e) => e,
    };

    let mut second_parser = toml::de::Deserializer::new(&toml);
    second_parser.set_require_newline_after_table(false);
    if let Ok(ret) = toml::Value::deserialize(&mut second_parser) {
        let msg = format!(
            "\
TOML file found which contains invalid syntax and will soon not parse
at `{}`.
The TOML spec requires newlines after table definitions (e.g., `[a] b = 1` is
invalid), but this file has a table header which does not have a newline after
it. A newline needs to be added and this warning will soon become a hard error
in the future.",
            path.display()
        );
        println!("{}", &msg);
        return Ok(ret);
    }

    let first_error = anyhow::Error::from(first_error);
    Err(first_error.context("could not parse input as TOML"))
}
