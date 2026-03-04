mod env;

use std::process::Command;

use anyhow::{Result, bail};

use crate::manifest;
use crate::util::output::{self, OutputConfig};

pub fn run_script(name: &str, extra_args: &[String], output: &OutputConfig) -> Result<()> {
    let manifest = manifest::load_from_cwd()?;
    let project_dir = manifest::manifest_dir()?;

    let script_path = manifest
        .scripts
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("unknown script '{name}'"))?;

    let full_path = project_dir.join(script_path);
    if !full_path.exists() {
        bail!("script file not found: {}", full_path.display());
    }

    output::status("Running", &format!("script '{name}' ({script_path})"));

    let env_vars = env::build_env(&manifest, &project_dir);

    let mut cmd = Command::new(&full_path);
    cmd.current_dir(&project_dir);

    for (k, v) in &env_vars {
        cmd.env(k, v);
    }

    for arg in extra_args {
        cmd.arg(arg);
    }

    let status = cmd.status()?;

    if !status.success() {
        bail!("script '{name}' exited with status {status}");
    }

    output::verbose(output, &format!("script '{name}' completed successfully"));
    Ok(())
}

pub fn list_scripts(_output: &OutputConfig) -> Result<()> {
    let manifest = manifest::load_from_cwd()?;

    if manifest.scripts.is_empty() {
        println!("No scripts defined in forge.toml");
        return Ok(());
    }

    println!("Available scripts:");
    for (name, path) in &manifest.scripts {
        println!("  {:<20} {}", name, path);
    }

    Ok(())
}
