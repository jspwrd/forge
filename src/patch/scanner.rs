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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_sentinel_at_start() {
        let data = b"SENTINEL_rest_of_data";
        assert_eq!(find_sentinel(data, "SENTINEL_").unwrap(), 0);
    }

    #[test]
    fn find_sentinel_in_middle() {
        let data = b"prefixSENTINELsuffix";
        assert_eq!(find_sentinel(data, "SENTINEL").unwrap(), 6);
    }

    #[test]
    fn find_sentinel_at_end() {
        let data = b"dataSENT";
        assert_eq!(find_sentinel(data, "SENT").unwrap(), 4);
    }

    #[test]
    fn sentinel_not_found() {
        let data = b"no match here";
        let err = find_sentinel(data, "MISSING").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn duplicate_sentinel_rejected() {
        let data = b"AAAA_CALLBACK_AAAA__AAAA_CALLBACK_AAAA";
        let err = find_sentinel(data, "AAAA_CALLBACK_AAAA").unwrap_err();
        assert!(err.to_string().contains("found 2 times"));
    }

    #[test]
    fn empty_data() {
        let err = find_sentinel(b"", "SENT").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }
}
