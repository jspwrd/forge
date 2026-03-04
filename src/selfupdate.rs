use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::util::output::{self, OutputConfig};

const REPO: &str = "jspwrd/forge";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Return the current compiled version.
pub fn current_version() -> &'static str {
    CURRENT_VERSION
}

/// Fetch the latest release tag from GitHub (e.g. "v0.2.0").
fn fetch_latest_tag() -> Result<String> {
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");

    let output = std::process::Command::new("curl")
        .args([
            "-fsSL",
            "-H",
            "Accept: application/vnd.github.v3+json",
            &url,
        ])
        .output()
        .context("failed to run curl — is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("failed to fetch latest release: {stderr}");
    }

    let body = String::from_utf8_lossy(&output.stdout);

    // Minimal JSON parsing — look for "tag_name": "vX.Y.Z"
    let tag = body
        .split("\"tag_name\"")
        .nth(1)
        .and_then(|s| s.split('"').nth(1))
        .map(|s| s.to_string())
        .context("could not parse tag_name from GitHub API response")?;

    Ok(tag)
}

/// Strip leading 'v' for comparison.
fn normalize_version(v: &str) -> &str {
    v.strip_prefix('v').unwrap_or(v)
}

/// Determine the target triple for the current platform.
fn current_target() -> Result<String> {
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        bail!("unsupported architecture");
    };

    let os = if cfg!(target_os = "linux") {
        "unknown-linux-gnu"
    } else if cfg!(target_os = "macos") {
        "apple-darwin"
    } else {
        bail!("unsupported OS");
    };

    Ok(format!("{arch}-{os}"))
}

/// Get the path of the currently running binary.
fn current_exe_path() -> Result<PathBuf> {
    env::current_exe().context("could not determine path of current executable")
}

/// Download a file from `url` to `dest`.
fn download(url: &str, dest: &Path) -> Result<()> {
    let status = std::process::Command::new("curl")
        .args(["-fsSL", "-o"])
        .arg(dest)
        .arg(url)
        .status()
        .context("failed to run curl")?;

    if !status.success() {
        bail!("download failed: {url}");
    }
    Ok(())
}

/// Check for updates and print status.
pub fn check_update(_output: &OutputConfig) -> Result<()> {
    output::status("Checking", "for updates...");

    let latest_tag = fetch_latest_tag()?;
    let latest = normalize_version(&latest_tag);
    let current = normalize_version(CURRENT_VERSION);

    if latest == current {
        output::status("Up to date", &format!("forge v{current}"));
    } else {
        output::status(
            "Update",
            &format!("v{current} -> {latest_tag} (run `forge update` to install)"),
        );
    }

    Ok(())
}

/// Self-update: download the latest release and replace the current binary.
pub fn execute_update(force: bool, output: &OutputConfig) -> Result<()> {
    output::status("Checking", "for updates...");

    let latest_tag = fetch_latest_tag()?;
    let latest = normalize_version(&latest_tag);
    let current = normalize_version(CURRENT_VERSION);

    if latest == current && !force {
        output::status("Up to date", &format!("forge v{current}"));
        return Ok(());
    }

    if latest != current {
        output::status("Updating", &format!("v{current} -> {latest_tag}"));
    } else {
        output::status("Reinstalling", &format!("{latest_tag} (--force)"));
    }

    let target = current_target()?;
    let tarball = format!("forge-{target}.tar.gz");
    let url = format!("https://github.com/{REPO}/releases/download/{latest_tag}/{tarball}");

    output::verbose(output, &format!("downloading {url}"));

    let tmpdir = tempfile::tempdir().context("failed to create temp directory")?;
    let archive_path = tmpdir.path().join(&tarball);

    download(&url, &archive_path)?;

    // Extract
    let status = std::process::Command::new("tar")
        .args(["xzf"])
        .arg(&archive_path)
        .arg("-C")
        .arg(tmpdir.path())
        .status()
        .context("failed to run tar")?;

    if !status.success() {
        bail!("failed to extract archive");
    }

    let new_binary = tmpdir.path().join("forge");
    if !new_binary.exists() {
        bail!("extracted archive does not contain 'forge' binary");
    }

    // Replace current binary
    let exe_path = current_exe_path()?;
    output::verbose(output, &format!("replacing {}", exe_path.display()));

    // Rename old binary as backup, then move new one in
    let backup = exe_path.with_extension("old");
    if backup.exists() {
        fs::remove_file(&backup).ok();
    }
    fs::rename(&exe_path, &backup)
        .with_context(|| format!("failed to back up current binary to {}", backup.display()))?;

    if let Err(e) = fs::copy(&new_binary, &exe_path) {
        // Restore backup on failure
        fs::rename(&backup, &exe_path).ok();
        bail!("failed to install new binary: {e}");
    }

    // Set executable permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&exe_path, fs::Permissions::from_mode(0o755)).ok();
    }

    // Clean up backup
    fs::remove_file(&backup).ok();

    // Update version file if it exists (for install.sh-based installs)
    if let Some(parent) = exe_path.parent() {
        let version_file = parent.join("../.forge-version");
        if version_file.exists() {
            fs::write(&version_file, &latest_tag).ok();
        }
    }

    output::status("Updated", &format!("forge {latest_tag}"));

    Ok(())
}

/// Uninstall forge by removing its binary and install directory.
pub fn execute_uninstall(_output: &OutputConfig) -> Result<()> {
    let exe_path = current_exe_path()?;
    let exe_dir = exe_path
        .parent()
        .context("could not determine binary directory")?;

    output::status("Uninstalling", &format!("forge v{CURRENT_VERSION}"));
    output::status("Binary", &format!("{}", exe_path.display()));

    // Check if this looks like a ~/.forge/bin installation
    let is_forge_dir = exe_dir.to_string_lossy().contains(".forge");

    // Remove the binary
    fs::remove_file(&exe_path).with_context(|| {
        format!(
            "failed to remove {}. You may need to run with sudo.",
            exe_path.display()
        )
    })?;

    output::status("Removed", &format!("{}", exe_path.display()));

    // Clean up .forge directory if applicable
    if is_forge_dir {
        let forge_home = exe_dir.parent().unwrap_or(exe_dir);

        // Remove version file
        let version_file = forge_home.join(".forge-version");
        if version_file.exists() {
            fs::remove_file(&version_file).ok();
        }

        // Remove bin dir if empty
        if exe_dir.read_dir().is_ok_and(|mut d| d.next().is_none()) {
            fs::remove_dir(exe_dir).ok();
        }

        // Remove .forge dir if empty
        if forge_home
            .read_dir()
            .is_ok_and(|mut d| d.next().is_none())
        {
            fs::remove_dir(forge_home).ok();
            output::status("Removed", &format!("{}", forge_home.display()));
        }
    }

    println!();
    output::status("Uninstalled", "forge has been removed.");

    if is_forge_dir {
        println!();
        println!("You can also remove the PATH entry from your shell profile:");
        println!(
            "  Remove the line: export PATH=\"{}:$PATH\"",
            exe_dir.display()
        );
    }

    Ok(())
}
