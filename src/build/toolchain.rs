use anyhow::{Result, bail};

#[allow(dead_code)]
pub fn find_compiler(name: &str) -> Result<std::path::PathBuf> {
    which::which(name)
        .map_err(|_| anyhow::anyhow!("compiler '{}' not found in PATH", name))
}

#[allow(dead_code)]
pub fn verify_compiler(name: &str) -> Result<()> {
    let path = find_compiler(name)?;
    let output = std::process::Command::new(&path)
        .arg("--version")
        .output()?;

    if !output.status.success() {
        bail!("compiler '{}' failed version check", name);
    }

    Ok(())
}
