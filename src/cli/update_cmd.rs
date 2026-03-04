use anyhow::Result;
use clap::Parser;

use crate::selfupdate;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct UpdateArgs {
    /// Force reinstall even if already on the latest version
    #[arg(long)]
    pub force: bool,

    /// Only check for updates without installing
    #[arg(long)]
    pub check: bool,
}

pub fn execute(args: UpdateArgs, output: &OutputConfig) -> Result<()> {
    if args.check {
        selfupdate::check_update(output)
    } else {
        selfupdate::execute_update(args.force, output)
    }
}
