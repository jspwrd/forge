mod deb;
mod dmg;
mod raw;

use std::path::Path;

use anyhow::{Result, bail};

use crate::cli::package_cmd::PackageArgs;
use crate::manifest;
use crate::util::output::{self, OutputConfig};

pub fn execute_package(args: PackageArgs, output: &OutputConfig) -> Result<()> {
    let manifest = manifest::load_from_cwd()?;
    let project_dir = manifest::manifest_dir()?;
    let bin_dir = project_dir.join("bin");
    let pkg_dir = project_dir.join("bin").join("pkg");

    let project_name = &manifest.project.name;
    let version = &manifest.project.version;

    // Find the binary
    let binary_name = if let Some(ref target) = args.target {
        format!("{project_name}-{target}")
    } else {
        project_name.clone()
    };
    let binary_path = bin_dir.join(&binary_name);

    output::status(
        "Packaging",
        &format!("{project_name} v{version} ({} format)", args.format),
    );

    match args.format.as_str() {
        "raw" => raw::package_raw(project_name, version, &binary_path, &pkg_dir, output)?,
        "dmg" => dmg::package_dmg(project_name, version, &binary_path, &pkg_dir, output)?,
        "deb" => deb::package_deb(project_name, version, &binary_path, &pkg_dir, output)?,
        "msi" => bail!("MSI packaging is not yet supported"),
        other => bail!("unknown package format: {other}"),
    }

    Ok(())
}

pub fn find_binary(project_dir: &Path, project_name: &str, target: Option<&str>) -> Result<std::path::PathBuf> {
    let bin_dir = project_dir.join("bin");
    let binary_name = match target {
        Some(t) => format!("{project_name}-{t}"),
        None => project_name.to_string(),
    };
    let path = bin_dir.join(&binary_name);
    if !path.exists() {
        bail!("binary not found: {}. Run `forge build` first.", path.display());
    }
    Ok(path)
}
