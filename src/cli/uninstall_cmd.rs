use anyhow::Result;
use clap::Parser;

use crate::selfupdate;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct UninstallArgs;

pub fn execute(_args: UninstallArgs, output: &OutputConfig) -> Result<()> {
    selfupdate::execute_uninstall(output)
}
