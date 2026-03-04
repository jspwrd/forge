use anyhow::Result;
use clap::Parser;

use crate::build;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct BuildArgs {
    /// Target to build for
    #[arg(long)]
    pub target: Option<String>,

    /// Build all enabled targets
    #[arg(long)]
    pub all_targets: bool,

    /// Build in release mode
    #[arg(long)]
    pub release: bool,

    /// Force rebuild, ignoring cache
    #[arg(long)]
    pub force: bool,
}

pub fn execute(args: BuildArgs, output: &OutputConfig) -> Result<()> {
    build::execute_build(args, output)
}
