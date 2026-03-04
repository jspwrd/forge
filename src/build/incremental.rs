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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let cache_path = dir.path().join("cache");

        let mut cache = BuildCache::default();
        cache.update("key1", Path::new("a.c")).unwrap();
        cache.update("key2", Path::new("b.c")).unwrap();
        cache.save(&cache_path).unwrap();

        let loaded = BuildCache::load(&cache_path);
        assert!(loaded.is_up_to_date("key1"));
        assert!(loaded.is_up_to_date("key2"));
        assert!(!loaded.is_up_to_date("key3"));
    }

    #[test]
    fn cache_load_missing_file() {
        let cache = BuildCache::load(Path::new("/nonexistent/path/cache"));
        assert!(!cache.is_up_to_date("anything"));
    }

    #[test]
    fn cache_key_differs_for_different_flags() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("test.c");
        std::fs::write(&src, "int main() {}").unwrap();

        let key1 = cache_key(&src, &["-O2".into()]).unwrap();
        let key2 = cache_key(&src, &["-O0".into()]).unwrap();
        assert_ne!(key1, key2);
    }

    #[test]
    fn cache_key_differs_for_different_content() {
        let dir = tempfile::tempdir().unwrap();

        let src = dir.path().join("test.c");
        std::fs::write(&src, "int main() { return 0; }").unwrap();
        let key1 = cache_key(&src, &[]).unwrap();

        std::fs::write(&src, "int main() { return 1; }").unwrap();
        let key2 = cache_key(&src, &[]).unwrap();

        assert_ne!(key1, key2);
    }

    #[test]
    fn cache_key_deterministic() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("test.c");
        std::fs::write(&src, "hello").unwrap();

        let key1 = cache_key(&src, &["-Wall".into()]).unwrap();
        let key2 = cache_key(&src, &["-Wall".into()]).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn hex_encode_works() {
        assert_eq!(hex::encode([0xDE, 0xAD, 0xBE, 0xEF]), "deadbeef");
        assert_eq!(hex::encode([0x00, 0xFF]), "00ff");
    }
}
