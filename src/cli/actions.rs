use std::fs;

use crate::fs_helper;
use crate::git::Git;
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

        if self.pk.pkg_exists(pkg_name) {
            bail!(
                "package `{}` was already installed. Try to use the `up` command to upgrade it.",
                pkg_name
            );
        }

        self.git.clone(pkg_name, pkg_tag)?;

        // TODO: process Fish shell package structure
        let mut pkg_dir = self.git.base_dir.clone();
        pkg_dir.push(&pkg_name);

        if !self.pk.pkg_exists(pkg_name) {
            bail!("package `{}` was not cloned with success.", pkg_name);
        }

        // Copy `configuration snippets` -> conf.d/*.fish
        // Copy `completions` -> completions/*.fish
        // Copy `functions` -> functions/*.fish
        let mut snippets = self.pk.fish_dir.clone();
        snippets.push("conf.d");
        let snippets = snippets.canonicalize()?;

        let mut completions = self.pk.fish_dir.clone();
        completions.push("completions");
        let completions = completions.canonicalize()?;

        let mut functions = self.pk.fish_dir.clone();
        functions.push("functions");
        let functions = functions.canonicalize()?;

        let mut stack_paths = vec![pkg_dir];
        let path_suffixes = vec!["conf.d", "completions", "functions"];

        while let Some(working_path) = stack_paths.pop() {
            println!("working path: {:?}", &working_path);

            for entry in fs::read_dir(working_path)? {
                let path = entry?.path();

                if path.is_dir() {
                    // Check for valid allowed directories
                    if fs_helper::contains_any_suffixes(&path, &path_suffixes) {
                        stack_paths.push(path);
                    }
                    continue;
                }

                if let Some(parent) = path.parent() {
                    // Check for files with allowed directory parents
                    if !fs_helper::contains_any_suffixes(&parent, &path_suffixes) {
                        continue;
                    }

                    let mut fish_dir = &snippets;
                    if parent.ends_with("completions") {
                        fish_dir = &completions;
                    }
                    if parent.ends_with("functions") {
                        fish_dir = &functions;
                    }

                    // TODO: copy the .fish files to their corresponding dirs
                    match path.file_name() {
                        Some(filename) => {
                            let dest_path = fish_dir.join(filename);
                            println!("  copy: {:?} -> {:?}", &path, &dest_path);
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

    /// Command action to update an existing package
    pub fn update(&mut self, pkg_name: &str) -> Result {
        let pkgv = PkgValidator::new(&pkg_name)?;
        let pkg_name = &pkgv.get_user_pkg_name();
        let pkg_tag = Some(pkgv.pkg_tag.as_ref());

        if !self.pk.pkg_exists(pkg_name) {
            bail!(
                "package `{}` was not installed. Try to use the `add` command to install it first.",
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
