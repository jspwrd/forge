use std::path::Path;

use anyhow::Result;

use crate::util::output::{self, OutputConfig};
use crate::util::process;

pub fn run_shellcheck(
    scripts: &[(String, String)],
    project_dir: &Path,
    output: &OutputConfig,
) -> Result<bool> {
    // Check if shellcheck is available
    if which::which("shellcheck").is_err() {
        output::warning("shellcheck not found, skipping script checks");
        return Ok(true);
    }

    let mut all_pass = true;

    for (name, path) in scripts {
        let full_path = project_dir.join(path);
        if !full_path.exists() {
            output::warning(&format!("script '{name}' not found at {path}"));
            continue;
        }

        let result = process::run_command(
            "shellcheck",
            &[full_path.to_str().unwrap_or("")],
            None,
            None,
        )?;

        if result.status.success() {
            output::verbose(output, &format!("shellcheck: {name} OK"));
        } else {
            output::error(&format!("shellcheck: {name} FAILED"));
            eprint!("{}", result.stdout);
            eprint!("{}", result.stderr);
            all_pass = false;
        }
    }

    Ok(all_pass)
}
