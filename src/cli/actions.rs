use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use crate::git::Git;
use crate::helpers::{file as helper_file, Command};
use crate::paket::Paket;
use crate::pkg::{conf, fmt::PkgNameFmt};
use crate::result::{Context, Result};

/// Define actions for every `Paket` command.
pub struct Actions<'a> {
    pk: &'a Paket,
    git: Git,
}

impl<'a> Actions<'a> {
    /// Create a new `Action` instance based on `Paket` object.
    pub fn new(pk: &'a Paket) -> Result<Self> {
        let git = Git::new(pk.paths.paket_dir.clone())?;
        Ok(Self { pk, git })
    }

    /// Read a Fish package directory and call a function passing per every
    /// file path read along with its equivalent destination path.
    pub fn read_pkg_dir<F>(&self, pkg_dir: &PathBuf, pkg_name: &str, mut func: F) -> Result
    where
        F: FnMut(&PathBuf, &PathBuf) -> Result,
    {
        let pkg_dir = pkg_dir.clone();
        let pkg_toml_path = pkg_dir.join("paket.toml").canonicalize().with_context(|| {
            format!(
                "`paket.toml` file was not found on package `{}` or is not accessible.",
                pkg_name
            )
        })?;

        // DEV: enable this just for testing during development
        // let pkg_toml_path = PathBuf::from("./src/pkg/conf/paket.toml")
        //     .canonicalize()
        //     .with_context(|| {
        //         format!(
        //             "`paket.toml` file was not found on package `{}` or is not accessible.",
        //             pkg_name
        //         )
        //     })?;

        // Detect and read the `paket.toml` file
        let toml = conf::read_pkg_file(&pkg_toml_path)?;
        let mut unused = BTreeSet::new();
        let manifest: conf::TomlManifest = serde_ignored::deserialize(toml, |path| {
            let mut key = String::new();
            helper_file::stringify(&mut key, &path);
            unused.insert(key);
        })?;

        for key in unused {
            println!("unused manifest key: {}", key);
        }

        // Read `package` toml section
        let toml_pkg = if manifest.package.is_some() {
            manifest.package.unwrap()
        } else {
            bail!("`paket.toml` file is empty or can not be read.")
        };

        // Read `include` toml property of `package` section
        let _ = &toml_pkg.include.unwrap_or(vec![]);
        // TODO: support Git glob-like file's reading

        // `configuration snippets` -> conf.d/*.fish
        // `completions` -> completions/*.fish
        // `functions` -> functions/*.fish
        let snippets_dir = &self.pk.paths.fish_snippets_dir;
        let completions_dir = &self.pk.paths.fish_completions_dir;
        let functions_dir = &self.pk.paths.fish_functions_dir;

        let mut stack_paths = vec![pkg_dir];
        let path_suffixes = vec!["conf.d", "completions", "functions"];

        while let Some(working_path) = stack_paths.pop() {
            for entry in fs::read_dir(working_path)? {
                let path = entry?.path();

                if path.is_dir() {
                    // Check for valid allowed directories
                    if helper_file::has_path_any_suffixes(&path, &path_suffixes) {
                        stack_paths.push(path);
                    }
                    continue;
                }

                // Check for files with allowed directory parents
                if let Some(parent) = path.parent() {
                    if !helper_file::has_path_any_suffixes(&parent, &path_suffixes) {
                        continue;
                    }

                    let mut fish_dir = snippets_dir;
                    if parent.ends_with("completions") {
                        fish_dir = &completions_dir;
                    }
                    if parent.ends_with("functions") {
                        fish_dir = &functions_dir;
                    }

                    // call callback function with a source and destination paths
                    match path.file_name() {
                        Some(filename) => {
                            let is_fish_file = match filename.to_str() {
                                Some(f) => f.ends_with(".fish"),
                                _ => false,
                            };

                            if is_fish_file {
                                let dest_path = fish_dir.join(filename);

                                func(&path, &dest_path)?;
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
        let pkgfmt = PkgNameFmt::from(&pkg_name)?;
        let pkg_name = &pkgfmt.get_short_name();
        let pkg_tag = Some(pkgfmt.pkg_tag.as_ref());

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

        self.read_pkg_dir(&pkg_dir, &pkg_name, |src_path, dest_path| {
            fs::copy(src_path, dest_path)?;
            Ok(())
        })?;
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
        let pkgfmt = PkgNameFmt::from(&pkg_name)?;
        let pkg_name = &pkgfmt.get_short_name();
        let pkg_tag = Some(pkgfmt.pkg_tag.as_ref());

        let branch_tag = pkg_tag.unwrap_or("");
        println!("Updating package `{}@{}`...", &pkg_name, branch_tag);

        // TODO: make sure to update installed source files tracking the current version files first
        // TODO: make sure of update additional files based on `paket.toml`

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

        self.read_pkg_dir(&pkg_dir, &pkg_name, |src_path, dest_path| {
            fs::copy(src_path, dest_path)?;
            Ok(())
        })?;

        // Dispatch the Fish shell `paket_update` event
        let cwd = std::env::current_dir()?;
        let out = Command::new("fish", &cwd)
            .arg("-c")
            .arg("emit paket_update")
            .execute()?;

        if !out.is_empty() {
            println!("{}", out);
        }

        println!("Package was updated successfully.");
        println!("Now reload your current Fish shell session or try:");
        println!("source ~/.config/fish/config.fish");

        Ok(())
    }

    /// Command action to remove an existing package and invoke a `paket_uninstall` Fish shell event.
    pub fn remove(&self, pkg_name: &str) -> Result {
        let pkgfmt = PkgNameFmt::from(&pkg_name)?;
        let pkg_name = &pkgfmt.get_short_name();

        println!("Removing package `{}`...", &pkg_name);

        // Process Fish shell package structure
        let pkg_dir = self.git.base_dir.join(&pkg_name);
        if !self.pk.pkg_exists(pkg_name) {
            bail!(
                "package `{}` is not installed or was already removed.",
                pkg_name
            );
        }

        let pkg_dir = pkg_dir.canonicalize()?;

        // Dispatch the Fish shell `paket_uninstall` event
        let cwd = std::env::current_dir()?;
        let out = Command::new("fish", &cwd)
            .arg("-c")
            .arg("emit paket_uninstall")
            .execute()?;

        if !out.is_empty() {
            println!("{}", out);
        }

        self.read_pkg_dir(&pkg_dir, &pkg_name, |_, dest_path| {
            if dest_path.exists() {
                fs::remove_file(dest_path)?;
            }
            Ok(())
        })?;
        // TODO: make sure of remove additional files based on `paket.toml`

        // NOTE: For now just remove the package Git repository as well
        fs::remove_dir_all(pkg_dir)?;

        println!("Package was removed successfully.");
        println!("Now reload your current Fish shell session or try:");
        println!("source ~/.config/fish/config.fish");

        Ok(())
    }
}
