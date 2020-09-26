use std::fs;

use crate::git::Git;
use crate::helpers::{file as file_helper, Command};
use crate::paket::Paket;
use crate::pkg::validator::PkgValidator;
use crate::result::{Context, Result};

pub struct Actions<'a> {
    pk: &'a Paket,
    git: Git,
}

impl<'a> Actions<'a> {
    pub fn new(pk: &'a Paket) -> Result<Self> {
        let git = Git::new(pk.paths.paket_dir.clone())?;
        Ok(Self { pk, git })
    }

    // Copy a Fish package source files to their corresponding directories
    pub fn copy_fish_pkg_source_files(&self, pkg_dir: &std::path::PathBuf) -> Result {
        // Copy `configuration snippets` -> conf.d/*.fish
        // Copy `completions` -> completions/*.fish
        // Copy `functions` -> functions/*.fish
        let pkg_dir = pkg_dir.clone();
        let snippets_dir = &self.pk.paths.fish_snippets_dir;
        let completions_dir = &self.pk.paths.fish_completions_dir;
        let functions_dir = &self.pk.paths.fish_functions_dir;

        let mut stack_paths = vec![pkg_dir];
        let path_suffixes = vec!["conf.d", "completions", "functions"];

        // TODO: Detect and read the `paket.toml` file

        // TODO: Move file copy process out to a new source file
        while let Some(working_path) = stack_paths.pop() {
            for entry in fs::read_dir(working_path)? {
                let path = entry?.path();

                if path.is_dir() {
                    // Check for valid allowed directories
                    if file_helper::has_path_any_suffixes(&path, &path_suffixes) {
                        stack_paths.push(path);
                    }
                    continue;
                }

                // Check for files with allowed directory parents
                if let Some(parent) = path.parent() {
                    if !file_helper::has_path_any_suffixes(&parent, &path_suffixes) {
                        continue;
                    }

                    let mut fish_dir = &snippets_dir;
                    if parent.ends_with("completions") {
                        fish_dir = &completions_dir;
                    }
                    if parent.ends_with("functions") {
                        fish_dir = &functions_dir;
                    }

                    // copy the .fish files to their corresponding dirs
                    match path.file_name() {
                        Some(filename) => {
                            let is_fish_file = match filename.to_str() {
                                Some(f) => f.ends_with(".fish"),
                                _ => false,
                            };

                            if is_fish_file {
                                let dest_path = fish_dir.join(filename);
                                fs::copy(&path, &dest_path)?;
                            }
                        }
                        None => {
                            bail!("failed to get file name for path {:?}", path);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Command action to install a new package and invoke a `paket_install` Fish shell event.
    pub fn install(&self, pkg_name: &str) -> Result {
        let pkgv = PkgValidator::new(&pkg_name)?;
        let pkg_name = &pkgv.get_user_pkg_name();
        let pkg_tag = Some(pkgv.pkg_tag.as_ref());

        let branch_tag = pkg_tag.unwrap_or("");
        println!("Installing package `{}@{}`...", &pkg_name, branch_tag);

        if self.pk.pkg_exists(pkg_name) {
            bail!(
                "package `{}` is already installed. Try to use the `up` command to upgrade it.",
                pkg_name
            );
        }

        self.git.clone(pkg_name, pkg_tag)?;

        // Process Fish shell package structure
        let pkg_dir = self.git.base_dir.join(&pkg_name);
        if !self.pk.pkg_exists(pkg_name) {
            bail!("package `{}` was not cloned with success.", pkg_name);
        }

        self.copy_fish_pkg_source_files(&pkg_dir)?;
        // TODO: make sure of copy additional files based on `paket.toml`

        // Dispatch the Fish shell `paket_install` event
        let cwd = std::env::current_dir()?;
        let out = Command::new("fish", &cwd)
            .arg("-c")
            .arg("emit paket_install")
            .execute()?;

        if !out.is_empty() {
            println!("{}", out);
        }

        println!("Package was installed successfully.");
        println!("Now reload your current Fish shell session or try:");
        println!("source ~/.config/fish/config.fish");

        Ok(())
    }

    /// Command action to update an existing package
    pub fn update(&mut self, pkg_name: &str) -> Result {
        let pkgv = PkgValidator::new(&pkg_name)?;
        let pkg_name = &pkgv.get_user_pkg_name();
        let pkg_tag = Some(pkgv.pkg_tag.as_ref());

        let branch_tag = pkg_tag.unwrap_or("");
        println!("Updating package `{}@{}`...", &pkg_name, branch_tag);

        // TODO: make sure to remove installed source files tracking the current version files first
        // TODO: make sure of remove additional files based on `paket.toml`

        if !self.pk.pkg_exists(pkg_name) {
            bail!(
                "package `{}` is not installed. Try to use the `add` command to install it first.",
                pkg_name
            );
        }

        self.git.fetch(pkg_name, pkg_tag)?;
        self.git.checkout(pkg_name, Some("FETCH_HEAD"))?;

        // Process Fish shell package structure
        let pkg_dir = self
            .git
            .base_dir
            .join(&pkg_name)
            .canonicalize()
            .with_context(|| format!("package `{}` was not updated properly.", pkg_name))?;

        self.copy_fish_pkg_source_files(&pkg_dir)?;

        println!("Package was updated successfully.");
        println!("Now reload your current Fish shell session or try:");
        println!("source ~/.config/fish/config.fish");

        Ok(())
    }

    /// Command action to remove an existing package and invoke a `paket_uninstall` Fish shell event.
    pub fn remove(&self, pkg_name: &str) -> Result {
        // TODO: Remove a package
        println!("Remove: pkg {:?}", pkg_name);

        // Dispatch the Fish shell `paket_uninstall` event
        let cwd = std::env::current_dir()?;
        let out = Command::new("fish", &cwd)
            .arg("-c")
            .arg("emit paket_uninstall")
            .execute()?;

        if !out.is_empty() {
            println!("{}", out);
        }

        Ok(())
    }
}
