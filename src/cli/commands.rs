use structopt::StructOpt;

/// A experimental package manager for the Fish shell
#[derive(Debug, StructOpt)]
pub struct CommandOpts {
    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Debug, StructOpt)]
pub enum Commands {
    /// Add a new package from a local or remote repository
    #[structopt(name = "add")]
    Add {
        /// Package name. E.g joseluisq/gitnow
        pkg_name: String,
    },

    /// Update an existing package
    #[structopt(name = "up")]
    Update {
        /// Package name. E.g joseluisq/gitnow
        pkg_name: String,
    },

    /// Remove an existing package
    #[structopt(name = "rm")]
    Remove {
        /// Package name. E.g joseluisq/gitnow
        pkg_name: String,
    },
}
