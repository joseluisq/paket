use serde::Deserialize;
use std::collections::BTreeMap;

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
