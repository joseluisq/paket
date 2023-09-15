use std::fs;

use crate::git::Git;
use crate::paket::{Paket, PaketEvents};
use crate::pkg::fmt::PkgNameFmt;
use crate::result::{Context, Result};

/// Define actions for every `Paket` command.
pub struct Actions<'a> {
    paket: &'a Paket,
    git: Git,
}

impl<'a> Actions<'a> {
    /// Create a new `Action` instance based on `Paket` object.
    pub fn new(paket: &'a Paket) -> Result<Self> {
        let git = Git::new(paket.paths.paket_dir.clone())?;
        Ok(Self { paket, git })
    }

    /// Command action to install a new package and invoke a `paket_install` Fish shell event.
    pub fn install(&mut self, pkg_name: &str, git_provider: &str) -> Result {
        let pkg_fmt = PkgNameFmt::from(pkg_name)?;
        let pkg_name = &pkg_fmt.get_short_name();
        let pkg_tag = pkg_fmt.pkg_tag.as_str().trim();
        let pkg_tag = if pkg_tag.is_empty() {
            None
        } else {
            Some(pkg_tag)
        };
        let branch_tag = pkg_tag.unwrap_or("");
        let mut is_pkg_local = false;

        // Check for a local package (directory path) or a remote one
        let pkg_dir = if let Some(pkg_path) = pkg_fmt.get_pkg_path() {
            is_pkg_local = true;

            // Check if package dir path is a valid Git repository
            self.git
                .check_valid_repo(&pkg_path)
                .with_context(|| "provided package directory is not a valid Git repository.")?;

            println!(
                "Installing package from directory `{}`...",
                pkg_path.display()
            );
            pkg_path
        } else {
            println!("Installing package `{}@{}`...", &pkg_name, branch_tag);

            if self.paket.pkg_exists(pkg_name) {
                bail!(
                    "package `{}` is already installed. Try to use the `up` command to upgrade it.",
                    pkg_name
                );
            }

            // Clone the remote repository
            self.git.clone(pkg_name, pkg_tag, git_provider)?;

            let pkg_dir = self.git.base_dir.join(pkg_name);
            if !self.paket.pkg_exists(pkg_name) {
                bail!("package `{}` was not cloned with success.", pkg_name);
            }
            pkg_dir
        };

        // Process Fish shell package structure and read the Packet manifest
        let manifest =
            self.paket
                .read_pkg_dir_with_manifest(&pkg_dir, &pkg_fmt.pkg_name, is_pkg_local)?;

        if let Some(toml_pkg) = manifest.package {
            // Copy all corresponding package files to Fish shell directories
            self.paket
                .scan_pkg_dir(pkg_dir, &toml_pkg.include, |src, dest| {
                    fs::copy(src, dest)?;
                    Ok(())
                })?;

            // Emit an `after_install` Fish shell event if there is an associated Paket event
            if let Some(toml_events) = manifest.events {
                self.paket
                    .emit_event(&toml_pkg.name, &toml_events, PaketEvents::AfterInstall)?;
            }
        } else {
            bail!("`paket.toml` file could not be parsed correctly.")
        };

        println!("Package was installed successfully.");
        println!("Now just reload your current Fish shell session.");

        Ok(())
    }

    /// Command action to update an existing package
    pub fn update(&mut self, pkg_name: &str) -> Result {
        let pkg_fmt = PkgNameFmt::from(pkg_name)?;
        let pkg_name = &pkg_fmt.get_short_name();
        let pkg_tag = pkg_fmt.pkg_tag.as_str().trim();
        let pkg_tag = if pkg_tag.is_empty() {
            None
        } else {
            Some(pkg_tag)
        };
        let branch_tag = pkg_tag.unwrap_or("");
        let mut is_pkg_local = false;

        // Check for a local package (directory path) or a remote one
        let pkg_dir = if let Some(pkg_path) = pkg_fmt.get_pkg_path() {
            is_pkg_local = true;
            println!(
                "Updating package from directory `{}`...",
                pkg_path.display()
            );
            pkg_path
        } else {
            println!("Updating package `{}@{}`...", &pkg_name, branch_tag);

            if !self.paket.pkg_exists(pkg_name) {
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
                .join(pkg_name)
                .canonicalize()
                .with_context(|| format!("package `{}` was not updated properly.", pkg_name))?
        };

        // Process Fish shell package structure and read the Packet manifest
        let manifest =
            self.paket
                .read_pkg_dir_with_manifest(&pkg_dir, &pkg_fmt.pkg_name, is_pkg_local)?;

        if let Some(toml_pkg) = manifest.package {
            // Copy all corresponding package files to Fish shell directories
            self.paket
                .scan_pkg_dir(pkg_dir, &toml_pkg.include, |src, dest| {
                    fs::copy(src, dest)?;
                    Ok(())
                })?;

            // Emit an `after_update` Fish shell event if there is an associated Paket event
            if let Some(toml_events) = manifest.events {
                self.paket
                    .emit_event(&toml_pkg.name, &toml_events, PaketEvents::AfterUpdate)?;
            }
        } else {
            bail!("`paket.toml` file could not be parsed correctly.")
        };

        println!("Package was updated successfully.");
        println!("Now just reload your current Fish shell session.");

        Ok(())
    }

    /// Command action to remove an existing package and invoke a `paket_uninstall` Fish shell event.
    pub fn remove(&mut self, pkg_name: &str) -> Result {
        let pkg_fmt = PkgNameFmt::from(pkg_name)?;
        let pkg_name = &pkg_fmt.get_short_name();
        let pkg_path = pkg_fmt.get_pkg_path();
        let is_pkg_path = pkg_path.is_some();

        // Check for a local package (directory path) or a remote one
        let pkg_dir = if is_pkg_path {
            let pkg_path = pkg_path.unwrap_or_default();
            println!(
                "Uninstalling package using directory `{}` as reference...",
                pkg_path.display()
            );
            pkg_path
        } else {
            println!("Uninstalling package `{}`...", &pkg_name);

            // Process Fish shell package structure
            let pkg_dir = self.git.base_dir.join(pkg_name);
            if !self.paket.pkg_exists(pkg_name) {
                bail!(
                    "package `{}` is not installed or was already removed.",
                    pkg_name
                );
            }

            pkg_dir.canonicalize()?
        };

        // Process Fish shell package structure and read the Packet manifest
        let manifest =
            self.paket
                .read_pkg_dir_with_manifest(&pkg_dir, &pkg_fmt.pkg_name, is_pkg_path)?;

        if let Some(toml_pkg) = manifest.package {
            // Emit an `after_update` Fish shell event if there is an associated Paket event
            if let Some(toml_events) = manifest.events {
                self.paket.emit_event(
                    &toml_pkg.name,
                    &toml_events,
                    PaketEvents::BeforeUninstall,
                )?;
            }

            // Remove all corresponding package files from Fish shell directories
            self.paket
                .scan_pkg_dir(pkg_dir.clone(), &toml_pkg.include, |_, dest| {
                    if dest.exists() {
                        fs::remove_file(dest)?;
                    }
                    Ok(())
                })?;
        } else {
            bail!("`paket.toml` file could not be parsed correctly.")
        };

        if !is_pkg_path {
            // TODO: Prevent unnecessary clones for same versions (using cache)
            // https://github.com/joseluisq/paket/issues/5

            // NOTE: For now we just remove the "cached" Git repository package too and
            // only for remote-installed ones
            fs::remove_dir_all(pkg_dir)?;
        }

        println!("Package was uninstalled successfully.");
        println!("Now just reload your current Fish shell session.");

        Ok(())
    }
}
