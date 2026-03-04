use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use sha2::{Digest, Sha256};

#[derive(Default)]
pub struct BuildCache {
    entries: HashMap<String, String>,
}

impl BuildCache {
    pub fn load(path: &Path) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            let mut entries = HashMap::new();
            for line in content.lines() {
                if let Some((key, hash)) = line.split_once('=') {
                    entries.insert(key.to_string(), hash.to_string());
                }
            }
            Self { entries }
        } else {
            Self::default()
        }
    }

    pub fn is_up_to_date(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    pub fn update(&mut self, key: &str, _source: &Path) -> Result<()> {
        self.entries.insert(key.to_string(), key.to_string());
        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut content = String::new();
        for (key, hash) in &self.entries {
            content.push_str(&format!("{key}={hash}\n"));
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}

pub fn cache_key(source: &Path, flags: &[String]) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(source.to_string_lossy().as_bytes());
    for flag in flags {
        hasher.update(flag.as_bytes());
    }

    // Include file content hash
    let content = std::fs::read(source)?;
    hasher.update(&content);

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
    }
}
