use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use tempfile::NamedTempFile;

pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("failed to create directory {}", path.display()))?;
    }
    Ok(())
}

pub fn atomic_write(path: &Path, data: &[u8]) -> Result<()> {
    let parent = path.parent().unwrap_or(Path::new("."));
    ensure_dir(parent)?;

    let temp = NamedTempFile::new_in(parent)
        .with_context(|| format!("failed to create temp file in {}", parent.display()))?;

    fs::write(temp.path(), data)
        .with_context(|| format!("failed to write temp file {}", temp.path().display()))?;

    temp.persist(path)
        .with_context(|| format!("failed to persist file to {}", path.display()))?;

    Ok(())
}
