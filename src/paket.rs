use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::cli::{App, CommandOpts};
use crate::result::Result;

/// Defines directory paths used by `Paket`.
pub struct PaketPaths {
    pub config_dir: PathBuf,
    pub fish_dir: PathBuf,
    pub fish_snippets_dir: PathBuf,
    pub fish_completions_dir: PathBuf,
    pub fish_functions_dir: PathBuf,
    pub paket_dir: PathBuf,
}

/// Packet is a package manager for the Fish shell.
pub struct Paket {
    pub paths: PaketPaths,
    pub opts: CommandOpts,
}

impl Paket {
    /// Create a new instance of `Paket`.
    pub fn new() -> Result<Self> {
        // TODO: Check if Fish shell is installed and this tool is running on top
        // For example using `echo $FISH_VERSION`
        // See https://github.com/fish-shell/fish-shell/issues/374

        let paths = Self::configure_paths()?;
        let opts = CommandOpts::from_args();

        Ok(Self { paths, opts })
    }

    /// Configure directory paths used by `Paket`.
    fn configure_paths() -> Result<PaketPaths> {
        let home_dir = dirs::home_dir()
            .expect("user config directory was not found")
            .canonicalize()?;

        // Config directory
        let config_dir = home_dir.join(".config").canonicalize()?;

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
