use std::path::Path;

use anyhow::{Result, bail};

use crate::util::output::{self, OutputConfig};
use crate::util::process;

pub fn package_dmg(
    project_name: &str,
    version: &str,
    binary_path: &Path,
    output_dir: &Path,
    _output: &OutputConfig,
) -> Result<()> {
    if !cfg!(target_os = "macos") {
        bail!("DMG packaging is only supported on macOS");
    }

    if !binary_path.exists() {
        bail!(
            "binary not found: {}. Run `forge build` first.",
            binary_path.display()
        );
    }

    crate::util::fs::ensure_dir(output_dir)?;

    let dmg_name = format!("{project_name}-{version}.dmg");
    let dmg_path = output_dir.join(&dmg_name);

    // Create a temp directory for DMG content
    let staging = tempfile::tempdir()?;
    std::fs::copy(binary_path, staging.path().join(project_name))?;

    output::status("Packaging", &format!("{dmg_name} (macOS disk image)"));

    process::run_command_checked(
        "hdiutil",
        &[
            "create",
            "-volname", project_name,
            "-srcfolder", staging.path().to_str().unwrap_or(""),
            "-ov",
            dmg_path.to_str().unwrap_or(""),
        ],
        None,
        None,
    )?;

    output::status("Packaged", &dmg_path.display().to_string());

    Ok(())
}
