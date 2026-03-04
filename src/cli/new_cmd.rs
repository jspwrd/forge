use anyhow::Result;
use clap::Parser;

use crate::scaffold;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct NewArgs {
    /// Name of the new project
    pub name: String,
}

pub fn execute(args: NewArgs, output: &OutputConfig) -> Result<()> {
    scaffold::create_project(&args.name, output)
}
