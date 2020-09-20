use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::cli::{App, CommandOpts};
use crate::result::Result;

pub struct Paket {
    pub paket_dir: PathBuf,
    pub opts: CommandOpts,
}

impl Paket {
    pub fn new() -> Result<Self> {
        let paket_dir = Self::configure()?;
        let opts = CommandOpts::from_args();

        Ok(Self { paket_dir, opts })
    }

    fn configure() -> Result<PathBuf> {
        let conf_dir = dirs::home_dir().expect("Paket: config directory not found");

        // NOTE: using `~/.config` for unix-like systems only
        let mut paket_dir = conf_dir.canonicalize()?;
        paket_dir.push(".config");
        paket_dir.push("paket");

        if !paket_dir.exists() {
            fs::create_dir_all(&paket_dir)?;
        }

        Ok(paket_dir.canonicalize()?)
    }

    pub fn run(&self) -> Result {
        App::run(&self)?;

        Ok(())
    }

    pub fn pkg_exists(&self, pkg_name: &str) -> bool {
        let mut pkg_dir = self.paket_dir.clone();
        pkg_dir.push(pkg_name);
        pkg_dir.exists() && pkg_dir.is_dir() && pkg_dir.read_dir().unwrap().next().is_some()
    }
}
