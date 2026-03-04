use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::manifest;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show the resolved configuration
    Show,
}

pub fn execute(args: ConfigArgs, _output: &OutputConfig) -> Result<()> {
    match args.action {
        ConfigAction::Show => {
            let manifest = manifest::load_from_cwd()?;
            println!("{:#?}", manifest);
            Ok(())
        }
    }
}
