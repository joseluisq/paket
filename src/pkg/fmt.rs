use crate::result::Result;

/// Defaines the package name format based on a fomatted package name string.
pub struct PkgNameFmt {
    /// Contain the package user name.
    pub user_name: String,
    /// Contain the package name.
    pub pkg_name: String,
    /// Contain the package version name (Git branch or tag).
    pub pkg_tag: String,
}

impl<'a> PkgNameFmt {
    /// Return a `PkgNameFmt` instance but making sure that current package name format is valid.
    /// Format: username/package_name@(tag_name|branch_name)
    pub fn from(pkg_name: &'a str) -> Result<Self> {
        if pkg_name.is_empty() {
            bail!("provide a package name.");
        }

        let pkg_parts: Vec<&str> = pkg_name.splitn(2, '/').collect();
        if pkg_parts.len() < 2 {
            bail!(
                "provide a valid package format. E.g username/package_name@(tag_name|branch_name)"
            );
        }

        let username = pkg_parts[0].trim().to_string();
        let pkg_name_parts: Vec<&str> = pkg_parts[1].splitn(2, '@').collect();
        if username.is_empty() || pkg_name_parts.is_empty() {
            bail!("provide a valid user and package name format. E.g username/package_name");
        }

        let pkg_name = pkg_name_parts[0].trim().to_string();
        if pkg_name.is_empty() {
            bail!("provide a valid package name value. E.g username/package_name");
        }

        let mut pkg_tag = "master".to_string();
        if pkg_name_parts.len() == 2 && !pkg_name_parts[1].is_empty() {
            pkg_tag = pkg_name_parts[1].trim().to_string();
        }

        Ok(Self {
            user_name: username,
            pkg_name,
            pkg_tag,
        })
    }

    /// Return the user and package name concatenated. E.g `username/package_name`.
    pub fn get_short_name(&self) -> String {
        [&self.user_name, "/", &self.pkg_name].concat()
    }
}
