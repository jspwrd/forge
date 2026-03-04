use anyhow::Result;
use clap::Parser;

use crate::util::output::OutputConfig;
use crate::validate;

#[derive(Parser)]
pub struct ValidateArgs {
    /// Path to the binary to validate
    pub binary: String,
}

pub fn execute(args: ValidateArgs, output: &OutputConfig) -> Result<()> {
    validate::execute_validate(args, output)
}
