use anyhow::Result;
use clap::Parser;

use crate::package;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct PackageArgs {
    /// Package format (raw, dmg, deb, msi)
    #[arg(long, default_value = "raw")]
    pub format: String,

    /// Target to package (defaults to native)
    #[arg(long)]
    pub target: Option<String>,
}

pub fn execute(args: PackageArgs, output: &OutputConfig) -> Result<()> {
    package::execute_package(args, output)
}
