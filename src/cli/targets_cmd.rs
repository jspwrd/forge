use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use crate::manifest;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct TargetsArgs;

pub fn execute(_args: TargetsArgs, _output: &OutputConfig) -> Result<()> {
    let manifest = manifest::load_from_cwd()?;

    if manifest.targets.is_empty() {
        println!("No targets defined in forge.toml");
        return Ok(());
    }

    println!("{}", "Available targets:".bold());
    for (name, target) in &manifest.targets {
        let status = if target.enabled {
            "enabled".green()
        } else {
            "disabled".red()
        };
        let cc = target.cc.as_deref().unwrap_or("default");
        println!("  {:<30} [{}] (cc: {})", name, status, cc);
    }

    Ok(())
}
