use anyhow::Result;
use clap::Parser;

use crate::scripts;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct ScriptsArgs {
    /// Script name to run (omit to list all)
    pub name: Option<String>,

    /// Extra arguments passed to the script
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

pub fn execute(args: ScriptsArgs, output: &OutputConfig) -> Result<()> {
    if let Some(ref name) = args.name {
        scripts::run_script(name, &args.args, output)
    } else {
        scripts::list_scripts(output)
    }
}
