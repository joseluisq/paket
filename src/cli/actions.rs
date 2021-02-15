use std::fs;

use crate::git::Git;
use crate::helpers::Command;
use crate::paket::Paket;
use crate::pkg::fmt::PkgNameFmt;
use crate::result::{Context, Result};

/// Define actions for every `Paket` command.
pub struct Actions<'a> {
    pkt: &'a Paket,
    git: Git,
}

impl<'a> Actions<'a> {
    /// Create a new `Action` instance based on `Paket` object.
    pub fn new(pkt: &'a Paket) -> Result<Self> {
        let git = Git::new(pkt.paths.paket_dir.clone())?;
        Ok(Self { pkt, git })
    }

    /// Command action to install a new package and invoke a `paket_install` Fish shell event.
    pub fn install(&self, pkg_name: &str, git_provider: &str) -> Result {
        let pkgfmt = PkgNameFmt::from(&pkg_name)?;
        let pkg_name = &pkgfmt.get_short_name();
        let pkg_tag = Some(pkgfmt.pkg_tag.as_ref());
        let branch_tag = pkg_tag.unwrap_or("");

        // Check for a local package (directory path) or a remote one
        let pkg_dir = if let Some(pkg_path) = pkgfmt.get_pkg_path() {
            println!(
                "Installing package from directory `{}`...",
                pkg_path.display()
            );
            pkg_path
        } else {
            println!("Installing package `{}@{}`...", &pkg_name, branch_tag);

            if self.pkt.pkg_exists(pkg_name) {
                bail!(
                    "package `{}` is already installed. Try to use the `up` command to upgrade it.",
                    pkg_name
                );
            }

            // Clone the remote repository
            self.git.clone(pkg_name, pkg_tag, git_provider)?;

            let pkg_dir = self.git.base_dir.join(&pkg_name);
            if !self.pkt.pkg_exists(pkg_name) {
                bail!("package `{}` was not cloned with success.", pkg_name);
            }
            pkg_dir
        };

        // Process Fish shell package structure
        self.pkt
            .read_pkg_dir(&pkg_dir, &pkg_name, |src_path, dest_path| {
                fs::copy(src_path, dest_path)?;
                Ok(())
            })?;

        // Dispatch the Fish shell `paket_install` event
        let out = Command::new("fish", None)
            .arg("-c")
            .arg("emit paket_install")
            .execute()?;

        if !out.is_empty() {
            print!("{}", out);
        }

        println!("Package was installed successfully.");
        println!("Now just reload your current Fish shell session.");

        Ok(())
    }

    /// Command action to update an existing package
    pub fn update(&mut self, pkg_name: &str) -> Result {
        let pkgfmt = PkgNameFmt::from(&pkg_name)?;
        let pkg_name = &pkgfmt.get_short_name();
        let pkg_tag = Some(pkgfmt.pkg_tag.as_ref());
        let branch_tag = pkg_tag.unwrap_or("");

        // Check for a local package (directory path) or a remote one
        let pkg_dir = if let Some(pkg_path) = pkgfmt.get_pkg_path() {
            println!(
                "Updating package from directory `{}`...",
                pkg_path.display()
            );
            pkg_path
        } else {
            println!("Updating package `{}@{}`...", &pkg_name, branch_tag);

            if !self.pkt.pkg_exists(pkg_name) {
                bail!(
                "package `{}` is not installed. Try to use the `add` command to install it first.",
                pkg_name
                )
            }

            // Fetch remote repository references and checkout
            self.git.fetch(pkg_name, pkg_tag)?;
            self.git.checkout(pkg_name, Some("FETCH_HEAD"))?;

            self.git
                .base_dir
                .join(&pkg_name)
                .canonicalize()
                .with_context(|| format!("package `{}` was not updated properly.", pkg_name))?
        };

        // Process Fish shell package structure
        self.pkt
            .read_pkg_dir(&pkg_dir, &pkg_name, |src_path, dest_path| {
                fs::copy(src_path, dest_path)?;
                Ok(())
            })?;

        // Dispatch the Fish shell `paket_update` event
        let out = Command::new("fish", None)
            .arg("-c")
            .arg("emit paket_update")
            .execute()?;

        if !out.is_empty() {
            print!("{}", out);
        }

        println!("Package was updated successfully.");
        println!("Now just reload your current Fish shell session.");

        Ok(())
    }

    /// Command action to remove an existing package and invoke a `paket_uninstall` Fish shell event.
    pub fn remove(&self, pkg_name: &str) -> Result {
        let pkgfmt = PkgNameFmt::from(&pkg_name)?;
        let pkg_name = &pkgfmt.get_short_name();
        let pkg_path = pkgfmt.get_pkg_path();
        let is_pkg_path = pkg_path.is_some();

        // Check for a local package (directory path) or a remote one
        let pkg_dir = if is_pkg_path {
            let pkg_path = pkg_path.unwrap_or_default();
            println!(
                "Removing installed package using as reference directory `{}`...",
                pkg_path.display()
            );
            pkg_path
        } else {
            println!("Removing package `{}`...", &pkg_name);

            // Process Fish shell package structure
            let pkg_dir = self.git.base_dir.join(&pkg_name);
            if !self.pkt.pkg_exists(pkg_name) {
                bail!(
                    "package `{}` is not installed or was already removed.",
                    pkg_name
                );
            }

            pkg_dir.canonicalize()?
        };

        // Dispatch the Fish shell `paket_uninstall` event
        let out = Command::new("fish", None)
            .arg("-c")
            .arg("emit paket_uninstall")
            .execute()?;

        if !out.is_empty() {
            print!("{}", out);
        }

        self.pkt.read_pkg_dir(&pkg_dir, &pkg_name, |_, dest_path| {
            if dest_path.exists() {
                fs::remove_file(dest_path)?;
            }
            Ok(())
        })?;

        if !is_pkg_path {
            // TODO: Prevent unnecessary clones for same versions (using cache)
            // https://github.com/joseluisq/paket/issues/5

            // NOTE: For now we just remove the "cached" Git repository package too and
            // only for remote-installed ones
            fs::remove_dir_all(pkg_dir)?;
        }

        println!("Package was removed successfully.");
        println!("Now just reload your current Fish shell session.");

        Ok(())
    }
}
