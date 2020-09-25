use crate::cli::{Actions, Commands};
use crate::paket::Paket;
use crate::result::Result;

pub struct App {}

impl App {
    pub fn run(pk: &Paket) -> Result {
        let mut actions = Actions::new(pk)?;

        if let Some(commands) = &pk.opts.commands {
            match commands {
                Commands::Add { pkg_name } => actions.install(pkg_name),
                Commands::Update { pkg_name } => actions.update(pkg_name),
                Commands::Remove { pkg_name } => actions.remove(pkg_name),
            }?
        }

        Ok(())
    }
}
