use clap::Parser;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use sysinfo::{ProcessExt, System, SystemExt};

use crate::cli::{App, CommandOpts};
use crate::helpers::{file as helper_file, process, Command};
use crate::pkg::config;
use crate::result::{Context, Result};

/// Packet events supported transferable to Fish shell events.
pub enum PaketEvents {
    AfterInstall,
    AfterUpdate,
    BeforeUninstall,
}

/// Defines directory paths used by `Paket`.
pub struct PaketPaths {
    /// User configuration directory.
    pub config_dir: PathBuf,
    /// Fish configuration directory.
    pub fish_dir: PathBuf,
    /// Fish snippets directory.
    pub fish_snippets_dir: PathBuf,
    /// Fish completions directory.
    pub fish_completions_dir: PathBuf,
    /// Fish functions directory.
    pub fish_functions_dir: PathBuf,
    /// Paket config directory.
    pub paket_dir: PathBuf,
}

/// Paket is a package manager for the Fish shell.
pub struct Paket {
    /// Contain directory paths used by `Paket`.
    pub paths: PaketPaths,
    /// Contain `Paket` command options.
    pub opts: CommandOpts,
}

impl<'a> Paket {
    /// Create a new instance of `Paket`.
    pub fn new() -> Result<Self> {
        // Check if Git and Fish shell binaries are available on system
        Command::new("git", None).spawn().with_context(|| {
            "`git` was not found! Please check if the latest binary is installed on system."
                .to_string()
        })?;
        Command::new("fish", None).spawn().with_context(|| {
            "`fish` was not found! Please check if the latest binary is installed on system."
                .to_string()
        })?;

        // Check if `paket` is running on top of a Fish shell session
        let pid = process::getppid().to_string();
        let on_fish = System::new_all()
            .processes_by_name("fish")
            .any(|p| p.pid().to_string() == pid);

        if !on_fish {
            bail!("Paket is not running on top of a Fish shell session. Just run `fish` and then use `paket` from there.")
        }

        let paths = Self::configure_paths()?;
        let opts = CommandOpts::parse();

        Ok(Self { paths, opts })
    }

    /// Configure directory paths used by `Paket`.
    fn configure_paths() -> Result<PaketPaths> {
        // User's home directory
        let home_dir = dirs::home_dir()
            .expect("User home directory was not found or inaccessible.")
            .canonicalize()?;

        // User's home config directory
        let config_dir = home_dir.join(".config");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .with_context(|| "Home config directory can not be created.")?;
        }
        let config_dir = config_dir
            .canonicalize()
            .with_context(|| "Home config directory was not found or inaccessible.")?;

        // Fish config directories
        let fish_dir = config_dir
            .join("fish")
            .canonicalize()
            .with_context(|| "Fish config directory was not found or inaccessible.")?;

        // Fish config snippets directory
        let fish_snippets_dir = fish_dir.join("conf.d");
        if !fish_snippets_dir.exists() {
            fs::create_dir_all(&fish_snippets_dir)
                .with_context(|| "Fish snippets directory can not be created.")?;
        }

        // Fish config completions directory
        let fish_completions_dir = fish_dir.join("completions");
        if !fish_completions_dir.exists() {
            fs::create_dir_all(&fish_completions_dir)
                .with_context(|| "Fish completions directory can not be created.")?;
        }

        // Fish config functions directory
        let fish_functions_dir = fish_dir.join("functions");
        if !fish_functions_dir.exists() {
            fs::create_dir_all(&fish_functions_dir)
                .with_context(|| "Fish functions directory can not be created.")?;
        }

        // Paket config directory
        let paket_dir = config_dir.join("paket");
        if !paket_dir.exists() {
            fs::create_dir_all(&paket_dir)
                .with_context(|| "Paket config directory can not be created.")?;
        }
        let paket_dir = paket_dir
            .canonicalize()
            .with_context(|| "Paket config directory was not found or inaccessible.")?;

        Ok(PaketPaths {
            config_dir,
            fish_dir,
            fish_snippets_dir,
            fish_completions_dir,
            fish_functions_dir,
            paket_dir,
        })
    }

    /// Just run the `Paket` application.
    pub fn run(&'a mut self) -> Result {
        App::run(self)?;

        Ok(())
    }

    /// Verify if a package directory path exists and it's not empty.
    pub fn pkg_exists(&'a self, pkg_name: &str) -> bool {
        let pkg_dir = self.paths.paket_dir.join(pkg_name);
        pkg_dir.exists() && pkg_dir.is_dir() && pkg_dir.read_dir().unwrap().next().is_some()
    }

    /// Read a valid package directory along with its Paket manifest file (paket.toml).
    pub fn read_pkg_dir_with_manifest(
        &'a self,
        pkg_dir: &Path,
        pkg_name: &str,
        is_local: bool,
    ) -> Result<config::TomlManifest> {
        let pkg_dir = pkg_dir.to_path_buf();
        let pkg_toml_path = pkg_dir.join("paket.toml").canonicalize().with_context(|| {
            let pkg_name = if is_local {
                pkg_dir.as_os_str().to_str().unwrap_or_default()
            } else {
                pkg_name
            };
            format!(
                "`paket.toml` file was not found on package `{}` or inaccessible.",
                pkg_name
            )
        })?;

        // Detect and read the `paket.toml` file
        let manifest = config::read_pkg_file(&pkg_toml_path)?;

        // Read `package` toml section
        let toml_pkg = if manifest.package.is_some() {
            manifest.package.clone().unwrap()
        } else {
            bail!("`paket.toml` file is empty or can not be read.")
        };

        // Verify if package input name is equal to manifest package name
        // checking for remote packages only
        if !is_local && pkg_name != toml_pkg.name {
            bail!(
                "package name `{}` in `paket.toml` doesn't match with given input package name.",
                pkg_name
            )
        }

        Ok(manifest)
    }

    /// Read a given package directory with its extra include directories and
    /// then call a function passing a source file path per every read along with
    /// its equivalent destination file path.
    pub fn scan_pkg_dir<F>(
        &'a self,
        pkg_dir: PathBuf,
        pkg_include: &Option<Vec<String>>,
        mut func: F,
    ) -> Result
    where
        F: FnMut(&PathBuf, &PathBuf) -> Result,
    {
        // Read `include` toml property of `package` section
        let pkg_include = &pkg_include.clone().unwrap_or_default();

        // TODO: support Git glob-like file's reading on `include` toml array.
        // Plain file paths only for now.

        // `configuration snippets` -> conf.d/*.fish
        // `completions` -> completions/*.fish
        // `functions` -> functions/*.fish
        let snippets_dir = &self.paths.fish_snippets_dir;
        let completions_dir = &self.paths.fish_completions_dir;
        let functions_dir = &self.paths.fish_functions_dir;

        let mut stack_paths = vec![pkg_dir];
        let path_suffixes = vec!["conf.d", "completions", "functions"];

        // Copy only files contained on "conf.d", "completions" or "functions" directories
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
                    if !helper_file::has_path_any_suffixes(parent, &path_suffixes) {
                        continue;
                    }

                    let mut fish_dir = snippets_dir;
                    if parent.ends_with("completions") {
                        fish_dir = completions_dir;
                    }
                    if parent.ends_with("functions") {
                        fish_dir = functions_dir;
                    }

                    // call callback function with a source and destination paths
                    match path.file_name() {
                        Some(filename) => {
                            let filename = filename.to_str();
                            let is_fish_file = match filename {
                                Some(f) => f.ends_with(".fish"),
                                _ => false,
                            };

                            let dest_path = fish_dir.join(filename.unwrap());

                            // Copy Fish shell files
                            if is_fish_file {
                                func(&path, &dest_path)?;
                                continue;
                            }

                            // Try to copy included non Fish shell files
                            if pkg_include.is_empty() {
                                continue;
                            }
                            let is_included = pkg_include.iter().any(|f| path.ends_with(f));
                            if is_included {
                                func(&path, &dest_path)?;
                            }
                        }
                        None => {
                            bail!("failed to get file name for path {}", path.display());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Process a Paket event definition, validating it with manifest file and
    /// finally dispatching the corresponding Fish shell event.
    pub fn emit_event(
        &self,
        pkgname: &str,
        events: &config::TomlEvents,
        event_type: PaketEvents,
    ) -> Result {
        let pkg_event_val = match event_type {
            PaketEvents::AfterInstall => &events.after_install,
            PaketEvents::AfterUpdate => &events.after_update,
            PaketEvents::BeforeUninstall => &events.before_uninstall,
        };

        if let Some(pkg_event_val) = pkg_event_val {
            let event_parts = pkg_event_val.splitn(2, '_').collect::<Vec<&str>>();
            if !event_parts.is_empty() {
                let pkgname_e = event_parts[0].trim_start();
                if pkgname_e != pkgname {
                    bail!(
                        "Paket event value `{}` defined in `paket.toml` has invalid format. \nUse the format `[package_name]_[event_name]` without the brackets and spaces (underscores instead). \nAlso make sure that the whole value matches with your package's Fish shell event (--on-event).",
                        pkg_event_val
                    );
                }

                let eventname = event_parts[1].trim_end();
                let emit_event = format!("emit {}_{}", pkgname, eventname);
                let out = Command::new("fish", None)
                    .arg("-c")
                    .arg(emit_event)
                    .execute()?;

                if !out.is_empty() {
                    print!("{}", out);
                }
            }
        }

        Ok(())
    }
}
