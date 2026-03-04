use anyhow::Result;
use clap::Parser;

use crate::patch;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct PatchArgs {
    /// Path to the binary to patch
    pub binary: String,

    /// Field=value pairs to patch (e.g., callback-ip=10.0.0.1)
    #[arg(trailing_var_arg = true)]
    pub fields: Vec<String>,
}

pub fn execute(args: PatchArgs, output: &OutputConfig) -> Result<()> {
    patch::execute_patch(args, output)
}
