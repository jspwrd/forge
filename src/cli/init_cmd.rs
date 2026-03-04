use anyhow::Result;
use clap::Parser;

use crate::scaffold;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct InitArgs;

pub fn execute(_args: InitArgs, output: &OutputConfig) -> Result<()> {
    scaffold::init_project(output)
}
