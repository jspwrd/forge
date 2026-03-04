use anyhow::Result;
use clap::Parser;

use crate::clean;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct CleanArgs {
    /// Securely overwrite files before deletion
    #[arg(long)]
    pub secure: bool,
}

pub fn execute(args: CleanArgs, output: &OutputConfig) -> Result<()> {
    clean::execute_clean(args, output)
}
