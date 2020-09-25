use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::cli::{App, CommandOpts};
use crate::result::Result;

/// Packet is a package manager for the Fish shell.
pub struct Paket {
    pub fish_dir: PathBuf,
    pub paket_dir: PathBuf,
    pub opts: CommandOpts,
}

impl Paket {
    /// Create a new instance of `Paket`.
    pub fn new() -> Result<Self> {
        let (fish_dir, paket_dir) = Self::configure()?;
        let opts = CommandOpts::from_args();

        Ok(Self {
            fish_dir,
            paket_dir,
            opts,
        })
    }

    /// Configure directory paths used by `Paket`.
    fn configure() -> Result<(PathBuf, PathBuf)> {
        let home_dir = dirs::home_dir()
            .expect("Paket: config directory not found")
            .canonicalize()?;

        // Config directory
        let config_dir = home_dir.join(".config").canonicalize()?;

        // Fish directory
        let fish_dir = config_dir.join("fish").canonicalize()?;

        // Paket directory
        let paket_dir = config_dir.join("paket");

        if !paket_dir.exists() {
            fs::create_dir_all(&paket_dir)?;
        }

        let paket_dir = paket_dir.canonicalize()?;

        Ok((fish_dir, paket_dir))
    }

    /// Just run the `Paket` application.
    pub fn run(&self) -> Result {
        App::run(&self)?;

        Ok(())
    }

    /// Verify if a package directory path exists and it's not empty.
    pub fn pkg_exists(&self, pkg_name: &str) -> bool {
        let pkg_dir = self.paket_dir.join(pkg_name);
        pkg_dir.exists() && pkg_dir.is_dir() && pkg_dir.read_dir().unwrap().next().is_some()
    }
}
