use clap::Parser;

/// A simple and fast package manager for the Fish shell 📦
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandOpts {
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Install a new package from a local or remote repository.
    #[command(name = "add")]
    Add {
        #[arg(short = 'p', long, default_value = "github")]
        /// A Git host provider like github, bitbucket or gitlab.
        provider: String,
        /// Package name. E.g joseluisq/gitnow
        pkg_name: String,
    },

    /// Update an existing package from a local or remote repository.
    #[command(name = "up")]
    Update {
        /// Package name. E.g joseluisq/gitnow
        pkg_name: String,
    },

    /// Uninstall an existing package from a local or remote repository.
    #[command(name = "rm")]
    Remove {
        /// Package name. E.g joseluisq/gitnow
        pkg_name: String,
    },
}
