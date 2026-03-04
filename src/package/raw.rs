use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::util::output::{self, OutputConfig};

pub fn package_raw(
    project_name: &str,
    version: &str,
    binary_path: &Path,
    output_dir: &Path,
    _output: &OutputConfig,
) -> Result<()> {
    if !binary_path.exists() {
        bail!(
            "binary not found: {}. Run `forge build` first.",
            binary_path.display()
        );
    }

    crate::util::fs::ensure_dir(output_dir)?;

    let archive_name = format!("{project_name}-{version}.tar.gz");
    let archive_path = output_dir.join(&archive_name);

    output::status("Packaging", &format!("{archive_name} (raw tarball)"));

    let binary_name = binary_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    let result = Command::new("tar")
        .args([
            "czf",
            archive_path.to_str().unwrap_or(""),
            "-C",
            binary_path
                .parent()
                .unwrap_or(Path::new("."))
                .to_str()
                .unwrap_or(""),
            &binary_name,
        ])
        .output()
        .context("failed to execute tar")?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        bail!("tar failed: {}", stderr.trim());
    }

    output::status("Packaged", &archive_path.display().to_string());

    Ok(())
}
