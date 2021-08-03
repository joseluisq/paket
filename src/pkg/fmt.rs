use std::path::PathBuf;

use crate::result::{Context, Result};

/// Defaines the package name format based on a fomatted package name string.
pub struct PkgNameFmt {
    /// Contain the package user name.
    pub user_name: String,
    /// Contain the package name.
    pub pkg_name: String,
    /// Contain the package version name (Git branch or tag).
    pub pkg_tag: String,

    pkg_path: Option<PathBuf>,
}

impl<'a> PkgNameFmt {
    /// Return a `PkgNameFmt` instance but making sure that current package name format is valid.
    /// Format: username/package_name@(tag_name|branch_name)
    pub fn from(pkg_name: &'a str) -> Result<Self> {
        if pkg_name.is_empty() {
            bail!("provide a package name or a local Git package directory path.");
        }

        // Default Git tag for package repository
        let mut pkg_tag = "master";

        // Check if current `pkg_name` is an Git based package path directory
        let pkg_path = std::path::Path::new(pkg_name);
        if pkg_path.is_dir() {
            let pkg_path = pkg_path
                .canonicalize()
                .with_context(|| "Package path directory doesn't exist or inaccessible.")?;

            // We take the dirname as package name
            let pkg_name = match pkg_path.iter().last() {
                Some(v) => v.to_str().unwrap().into(),
                None => bail!(
                    "directory name for path \"{}\" was not determined",
                    pkg_path.display(),
                ),
            };

            return Ok(Self {
                user_name: String::new(),
                pkg_name,
                pkg_tag: pkg_tag.into(),
                pkg_path: Some(pkg_path),
            });
        }

        let pkg_parts: Vec<&str> = pkg_name.splitn(2, '/').collect();
        if pkg_parts.len() < 2 {
            bail!(
                "provide a valid package format. E.g username/package_name@(tag_name|branch_name)"
            );
        }

        let username = pkg_parts[0].trim();
        let pkg_name_parts: Vec<&str> = pkg_parts[1].splitn(2, '@').collect();
        if username.is_empty() || pkg_name_parts.is_empty() {
            //  TODO: This message below is a workaround since either the `pacakge/name` format as well as
            // a package dir path can have a "path-like" structure (name with slashes),
            // however we could approach this differently in the future
            bail!("provided package has not a valid `username/package_name` format or if it was a package path directory it doesn't exist or is inaccessible.");
        }

        let pkg_name = pkg_name_parts[0].trim();
        if pkg_name.is_empty() {
            bail!("provide a valid package name value. E.g username/package_name");
        }

        if pkg_name_parts.len() == 2 && !pkg_name_parts[1].is_empty() {
            pkg_tag = pkg_name_parts[1].trim();
        }

        Ok(Self {
            user_name: username.into(),
            pkg_name: pkg_name.into(),
            pkg_tag: pkg_tag.into(),
            pkg_path: None,
        })
    }

    /// Return if the current package is a valid Git-based package directory path.
    pub fn get_pkg_path(&self) -> Option<PathBuf> {
        self.pkg_path.clone()
    }

    /// Return the user and package name concatenated. E.g `username/package_name`.
    pub fn get_short_name(&self) -> String {
        [&self.user_name, "/", &self.pkg_name].concat()
    }
}
