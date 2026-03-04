pub mod types;
pub mod validation;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use types::ForgeManifest;
use validation::validate_manifest;

pub const MANIFEST_NAME: &str = "forge.toml";

pub fn find_manifest(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        let candidate = current.join(MANIFEST_NAME);
        if candidate.is_file() {
            return Some(candidate);
        }
        if !current.pop() {
            return None;
        }
    }
}

pub fn load(path: &Path) -> Result<ForgeManifest> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let manifest: ForgeManifest =
        toml::from_str(&content).with_context(|| format!("failed to parse {}", path.display()))?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

pub fn load_from_cwd() -> Result<ForgeManifest> {
    let cwd = std::env::current_dir().context("failed to determine current directory")?;
    let path = find_manifest(&cwd).ok_or_else(|| {
        anyhow::anyhow!("no {} found in current directory or any parent", MANIFEST_NAME)
    })?;
    load(&path)
}

pub fn manifest_dir() -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("failed to determine current directory")?;
    let path = find_manifest(&cwd).ok_or_else(|| {
        anyhow::anyhow!("no {} found in current directory or any parent", MANIFEST_NAME)
    })?;
    Ok(path
        .parent()
        .expect("manifest path must have a parent")
        .to_path_buf())
}
