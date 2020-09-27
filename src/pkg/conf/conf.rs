use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TomlDependency {
    version: Option<String>,
    path: Option<String>,
    git: Option<String>,
    branch: Option<String>,
    tag: Option<String>,
    rev: Option<String>,
    package: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TomlManifest {
    package: Option<Box<TomlPackage>>,
    dependencies: Option<BTreeMap<String, TomlDependency>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TomlPackage {
    name: String,
    version: semver::Version,
    authors: Option<Vec<String>>,
    include: Option<Vec<String>>,

    // Package metadata
    description: Option<String>,
    keywords: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    license: Option<String>,
    repository: Option<String>,
}
