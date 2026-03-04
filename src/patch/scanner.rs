use anyhow::{Result, bail};

pub fn find_sentinel(data: &[u8], sentinel: &str) -> Result<usize> {
    let sentinel_bytes = sentinel.as_bytes();
    let matches: Vec<usize> = data
        .windows(sentinel_bytes.len())
        .enumerate()
        .filter(|(_, window)| *window == sentinel_bytes)
        .map(|(i, _)| i)
        .collect();

    match matches.len() {
        0 => bail!("sentinel '{}' not found in binary", sentinel),
        1 => Ok(matches[0]),
        n => bail!(
            "sentinel '{}' found {} times in binary (expected exactly 1)",
            sentinel,
            n
        ),
    }
}
