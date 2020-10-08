use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::cli::{App, CommandOpts};
use crate::result::Result;

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

/// Packet is a package manager for the Fish shell.
pub struct Paket {
    /// Contain directory paths used by `Paket`.
    pub paths: PaketPaths,
    /// Contain `Paket` command options.
    pub opts: CommandOpts,
}

impl Paket {
    /// Create a new instance of `Paket`.
    pub fn new() -> Result<Self> {
        // TODO: Check if Git and Fish shell binaries are installed

        // TODO: Check if this tool is running on top of a Fish shell session
        // For example using `echo $FISH_VERSION` which is exclusive to Fish shell
        // See https://github.com/fish-shell/fish-shell/issues/374

        let paths = Self::configure_paths()?;
        let opts = CommandOpts::from_args();

        Ok(Self { paths, opts })
    }

    /// Configure directory paths used by `Paket`.
    fn configure_paths() -> Result<PaketPaths> {
        let home_dir = dirs::home_dir()
            .expect("user home directory was not found or is not accessible.")
            .canonicalize()?;

        // Config directory
        let config_dir = home_dir.join(".config");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let config_dir = config_dir.canonicalize()?;

        // Fish config directories
        let fish_dir = config_dir.join("fish").canonicalize()?;
        let fish_snippets_dir = fish_dir.join("conf.d").canonicalize()?;
        let fish_completions_dir = fish_dir.join("completions").canonicalize()?;
        let fish_functions_dir = fish_dir.join("functions").canonicalize()?;

        // Paket directory
        let paket_dir = config_dir.join("paket");

        if !paket_dir.exists() {
            fs::create_dir_all(&paket_dir)?;
        }

        let paket_dir = paket_dir.canonicalize()?;

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
    pub fn run(&self) -> Result {
        App::run(&self)?;

        Ok(())
    }

    /// Verify if a package directory path exists and it's not empty.
    pub fn pkg_exists(&self, pkg_name: &str) -> bool {
        let pkg_dir = self.paths.paket_dir.join(pkg_name);
        pkg_dir.exists() && pkg_dir.is_dir() && pkg_dir.read_dir().unwrap().next().is_some()
    }
}
