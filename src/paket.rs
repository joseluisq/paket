use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::cli::{App, CommandOpts};
use crate::result::Result;

pub struct Paket {
    pub fish_dir: PathBuf,
    pub paket_dir: PathBuf,
    pub opts: CommandOpts,
}

impl Paket {
    pub fn new() -> Result<Self> {
        let (fish_dir, paket_dir) = Self::configure()?;
        let opts = CommandOpts::from_args();

        Ok(Self {
            fish_dir,
            paket_dir,
            opts,
        })
    }

    fn configure() -> Result<(PathBuf, PathBuf)> {
        let mut home_dir = dirs::home_dir()
            .expect("Paket: config directory not found")
            .canonicalize()?;

        // Config directory
        home_dir.push(".config");
        let mut config_dir = home_dir.canonicalize()?;

        // Paket directory
        let mut paket_dir = config_dir.clone();
        paket_dir.push("paket");

        // Fish directory
        config_dir.push("fish");
        let fish_dir = config_dir.canonicalize()?;

        if !paket_dir.exists() {
            fs::create_dir_all(&paket_dir)?;
        }

        let paket_dir = paket_dir.canonicalize()?;

        Ok((fish_dir, paket_dir))
    }

    pub fn run(&self) -> Result {
        App::run(&self)?;

        Ok(())
    }

    /// Verify if a package directory path exists and it's not empty.
    pub fn pkg_exists(&self, pkg_name: &str) -> bool {
        let mut pkg_dir = self.paket_dir.clone();
        pkg_dir.push(pkg_name);
        pkg_dir.exists() && pkg_dir.is_dir() && pkg_dir.read_dir().unwrap().next().is_some()
    }
}
