use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

use crate::helpers::file;
use crate::result::Result;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TomlDependency {
    pub version: Option<String>,
    pub path: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    pub package: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TomlManifest {
    pub package: Option<Box<TomlPackage>>,
    pub dependencies: Option<BTreeMap<String, TomlDependency>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TomlPackage {
    pub name: String,
    pub version: semver::Version,
    pub authors: Option<Vec<String>>,
    pub include: Option<Vec<String>>,

    // Package metadata
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub license: Option<String>,
    pub repository: Option<String>,
}

pub fn read_pkg_file(path: &Path) -> Result<toml::Value> {
    let toml_str = file::read(path)?;

    let first_error = match toml_str.parse() {
        Ok(ret) => return Ok(ret),
        Err(e) => e,
    };

    let mut second_parser = toml::de::Deserializer::new(&toml_str);
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
