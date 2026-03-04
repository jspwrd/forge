use std::fs;
use std::path::Path;

use anyhow::{Result, bail};

use crate::util::output::{self, OutputConfig};
use crate::util::process;

pub fn package_deb(
    project_name: &str,
    version: &str,
    binary_path: &Path,
    output_dir: &Path,
    _output: &OutputConfig,
) -> Result<()> {
    if !cfg!(target_os = "linux") {
        bail!("DEB packaging is only supported on Linux");
    }

    if !binary_path.exists() {
        bail!(
            "binary not found: {}. Run `forge build` first.",
            binary_path.display()
        );
    }

    crate::util::fs::ensure_dir(output_dir)?;

    let staging = tempfile::tempdir()?;
    let debian_dir = staging.path().join("DEBIAN");
    let bin_dir = staging.path().join("usr").join("local").join("bin");

    fs::create_dir_all(&debian_dir)?;
    fs::create_dir_all(&bin_dir)?;

    // Write control file
    let control = format!(
        "Package: {project_name}\nVersion: {version}\nArchitecture: amd64\nMaintainer: forge\nDescription: {project_name}\n"
    );
    fs::write(debian_dir.join("control"), control)?;

    // Copy binary
    fs::copy(binary_path, bin_dir.join(project_name))?;

    let deb_name = format!("{project_name}_{version}_amd64.deb");
    let deb_path = output_dir.join(&deb_name);

    output::status("Packaging", &format!("{deb_name} (Debian package)"));

    process::run_command_checked(
        "dpkg-deb",
        &[
            "--build",
            staging.path().to_str().unwrap_or(""),
            deb_path.to_str().unwrap_or(""),
        ],
        None,
        None,
    )?;

    output::status("Packaged", &deb_path.display().to_string());

    Ok(())
}
