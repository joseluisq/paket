use crate::cli::{Actions, Commands};
use crate::paket::Paket;
use crate::result::Result;

pub struct App {}

impl App {
    pub fn run(pk: &mut Paket) -> Result {
        let mut actions = Actions::new(pk)?;
        if let Some(commands) = &pk.opts.commands {
            match commands {
                Commands::Add { pkg_name, provider } => {
                    actions.install(pkg_name.as_str(), provider.as_str())
                }
                Commands::Update { pkg_name } => actions.update(pkg_name.as_str()),
                Commands::Remove { pkg_name } => actions.remove(pkg_name.as_str()),
            }?
        }

        Ok(())
    }
}
