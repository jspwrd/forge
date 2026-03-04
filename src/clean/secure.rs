use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use rand::Rng;

pub fn secure_delete(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            secure_delete(&entry.path())?;
        }
        fs::remove_dir(path)
            .with_context(|| format!("failed to remove directory {}", path.display()))?;
    } else {
        let len = fs::metadata(path)?.len() as usize;

        let mut file = fs::OpenOptions::new()
            .write(true)
            .open(path)
            .with_context(|| format!("failed to open {} for secure delete", path.display()))?;

        // Pass 1: zeros
        file.write_all(&vec![0u8; len])?;
        file.flush()?;

        // Pass 2: 0xFF
        file.seek_from_start(0)?;
        file.write_all(&vec![0xFFu8; len])?;
        file.flush()?;

        // Pass 3: random
        file.seek_from_start(0)?;
        let mut rng = rand::rng();
        let random_data: Vec<u8> = (0..len).map(|_| rng.random()).collect();
        file.write_all(&random_data)?;
        file.flush()?;

        drop(file);
        fs::remove_file(path).with_context(|| format!("failed to remove {}", path.display()))?;
    }

    Ok(())
}

trait SeekFromStart {
    fn seek_from_start(&mut self, pos: u64) -> std::io::Result<u64>;
}

impl SeekFromStart for fs::File {
    fn seek_from_start(&mut self, pos: u64) -> std::io::Result<u64> {
        use std::io::Seek;
        self.seek(std::io::SeekFrom::Start(pos))
    }
}
