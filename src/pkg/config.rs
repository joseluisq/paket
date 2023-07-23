use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::helpers::file;
use crate::result::{Context, Result};

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
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TomlEvents {
    pub after_install: Option<String>,
    pub after_update: Option<String>,
    pub before_uninstall: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TomlManifest {
    pub package: Option<Box<TomlPackage>>,
    pub dependencies: Option<BTreeMap<String, TomlDependency>>,
    pub events: Option<Box<TomlEvents>>,
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

    // Events
    pub events: Option<TomlEvents>,
    // TODO: Dependencies
}

pub fn read_pkg_file(path: &Path) -> Result<TomlManifest> {
    // Validate TOML file extension
    let ext = path.extension();
    if ext.is_none() || ext.unwrap().is_empty() || ext.unwrap().ne("toml") {
        bail!("configuration file should be in toml format. E.g `config.toml`");
    }

    // TODO: validate minimal TOML file structure needed
    let toml = read_toml_file(path).with_context(|| "error reading toml configuration file")?;
    let mut unused = BTreeSet::new();
    let manifest: TomlManifest = serde_ignored::deserialize(toml, |path| {
        let mut key = String::new();
        file::stringify(&mut key, &path);
        unused.insert(key);
    })
    .with_context(|| "error during toml configuration file deserialization")?;

    for key in unused {
        println!("Warning: unused configuration manifest key \"{key}\" or unsupported");
    }

    Ok(manifest)
}

/// Read and parse a TOML file from an specific path.
fn read_toml_file(path: &Path) -> Result<toml::Value> {
    let toml_str = file::read(path).with_context(|| {
        format!(
            "error trying to deserialize toml configuration file at \"{}\"",
            path.display()
        )
    })?;
    toml_str
        .parse()
        .map_err(|e| anyhow::Error::from(e).context("could not parse input as TOML"))
}
