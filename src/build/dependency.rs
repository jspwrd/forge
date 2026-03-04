use std::path::{Path, PathBuf};

use anyhow::Result;
use glob::glob;

pub fn discover_sources(
    project_dir: &Path,
    patterns: &[String],
    excludes: &[String],
) -> Result<Vec<PathBuf>> {
    let mut sources = Vec::new();

    for pattern in patterns {
        let full_pattern = project_dir.join(pattern);
        let pattern_str = full_pattern.to_string_lossy();

        for entry in glob(&pattern_str)? {
            let path = entry?;
            if path.is_file() {
                sources.push(path);
            }
        }
    }

    // Remove excluded files
    if !excludes.is_empty() {
        let mut excluded_paths = Vec::new();
        for pattern in excludes {
            let full_pattern = project_dir.join(pattern);
            let pattern_str = full_pattern.to_string_lossy();
            for entry in glob(&pattern_str)? {
                let path = entry?;
                excluded_paths.push(path);
            }
        }

        sources.retain(|s| !excluded_paths.contains(s));
    }

    sources.sort();
    sources.dedup();
    Ok(sources)
}
