mod secure;

use std::fs;

use anyhow::Result;

use crate::cli::clean_cmd::CleanArgs;
use crate::manifest;
use crate::util::output::{self, OutputConfig};

pub fn execute_clean(args: CleanArgs, _output: &OutputConfig) -> Result<()> {
    let project_dir = manifest::manifest_dir()?;
    let bin_dir = project_dir.join("bin");

    if !bin_dir.exists() {
        output::status("Clean", "nothing to clean");
        return Ok(());
    }

    if args.secure {
        output::status("Cleaning", "bin/ (secure delete)");
        secure::secure_delete(&bin_dir)?;
    } else {
        output::status("Cleaning", "bin/");
        fs::remove_dir_all(&bin_dir)?;
    }

    output::status("Cleaned", "build artifacts removed");
    Ok(())
}
