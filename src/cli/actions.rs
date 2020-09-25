use std::fs;

use crate::git::Git;
use crate::helpers::{file as file_helper, Command};
use crate::paket::Paket;
use crate::pkg::validator::PkgValidator;
use crate::result::Result;

pub struct Actions<'a> {
    pk: &'a Paket,
    git: Git,
}

impl<'a> Actions<'a> {
    pub fn new(pk: &'a Paket) -> Result<Self> {
        let git = Git::new(pk.paket_dir.clone())?;
        Ok(Self { pk, git })
    }

    /// Command action to install a new package
    pub fn install(self, pkg_name: &str) -> Result {
        let pkgv = PkgValidator::new(&pkg_name)?;
        let pkg_name = &pkgv.get_user_pkg_name();
        let pkg_tag = Some(pkgv.pkg_tag.as_ref());

        let branch_tag = pkg_tag.unwrap_or("");
        println!("installing package `{}@{}`", &pkg_name, branch_tag);

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

        // Copy `configuration snippets` -> conf.d/*.fish
        // Copy `completions` -> completions/*.fish
        // Copy `functions` -> functions/*.fish
        let snippets = self.pk.fish_dir.join("conf.d").canonicalize()?;
        let completions = self.pk.fish_dir.join("completions").canonicalize()?;
        let functions = self.pk.fish_dir.join("functions").canonicalize()?;

        let mut stack_paths = vec![pkg_dir];
        let path_suffixes = vec!["conf.d", "completions", "functions"];

        // TODO: Detect and read the `paket.toml` file

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

                    let mut fish_dir = &snippets;
                    if parent.ends_with("completions") {
                        fish_dir = &completions;
                    }
                    if parent.ends_with("functions") {
                        fish_dir = &functions;
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

        // TODO: Invoke Fish shell events
        let cwd = std::env::current_dir()?;
        let mut cmd = Command::new("fish", &cwd);
        cmd.arg("-v");
        let out = cmd.execute()?;

        println!("{}", out);

        println!("package was installed successfully");

        Ok(())
    }

    /// Command action to update an existing package
    pub fn update(&mut self, pkg_name: &str) -> Result {
        let pkgv = PkgValidator::new(&pkg_name)?;
        let pkg_name = &pkgv.get_user_pkg_name();
        let pkg_tag = Some(pkgv.pkg_tag.as_ref());

        if !self.pk.pkg_exists(pkg_name) {
            bail!(
                "package `{}` is not installed. Try to use the `add` command to install it first.",
                pkg_name
            );
        }

        self.git.fetch(pkg_name, pkg_tag)?;
        self.git.checkout(pkg_name, Some("FETCH_HEAD"))?;

        Ok(())
    }

    /// Command action to remove an existing package
    pub fn remove(&self, pkg_name: &str) -> Result {
        println!("Remove: pkg {:?}", pkg_name);

        Ok(())
    }
}
