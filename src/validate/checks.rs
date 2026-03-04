use std::path::Path;

use anyhow::Result;

use crate::manifest::types::ValidateConfig;
use crate::util::process;

#[derive(Debug)]
pub enum CheckResult {
    Pass,
    Fail(String),
    Warn(String),
}

impl CheckResult {
    pub fn is_fail(&self) -> bool {
        matches!(self, CheckResult::Fail(_))
    }

    pub fn is_warn(&self) -> bool {
        matches!(self, CheckResult::Warn(_))
    }
}

pub fn check_debug_symbols(binary: &Path, config: &ValidateConfig) -> Result<CheckResult> {
    if !config.no_debug_symbols {
        return Ok(CheckResult::Pass);
    }

    // Use `file` command to check for debug info
    let result = process::run_command("file", &[binary.to_str().unwrap_or("")], None, None)?;

    if result.stdout.contains("with debug_info")
        || result.stdout.contains("not stripped")
        || result.stdout.contains("debug_info")
    {
        return Ok(CheckResult::Fail(
            "binary contains debug symbols".to_string(),
        ));
    }

    // Also scan for .debug_ sections
    let data = std::fs::read(binary)?;
    if data.windows(7).any(|w| w == b".debug_") {
        return Ok(CheckResult::Fail(
            "binary contains .debug_ sections".to_string(),
        ));
    }

    Ok(CheckResult::Pass)
}

pub fn check_plaintext_strings(binary: &Path, config: &ValidateConfig) -> Result<CheckResult> {
    if config.no_plaintext_strings.is_empty() {
        return Ok(CheckResult::Pass);
    }

    let data = std::fs::read(binary)?;

    for needle in &config.no_plaintext_strings {
        let needle_bytes = needle.as_bytes();
        if data.windows(needle_bytes.len()).any(|w| w == needle_bytes) {
            return Ok(CheckResult::Fail(format!(
                "binary contains plaintext string: '{needle}'"
            )));
        }
    }

    Ok(CheckResult::Pass)
}

pub fn check_compiler_watermarks(binary: &Path, config: &ValidateConfig) -> Result<CheckResult> {
    if !config.no_compiler_watermarks {
        return Ok(CheckResult::Pass);
    }

    let data = std::fs::read(binary)?;
    let watermarks = [b"GCC: " as &[u8], b"clang version", b"LLVM"];

    for watermark in &watermarks {
        if data.windows(watermark.len()).any(|w| w == *watermark) {
            let name = String::from_utf8_lossy(watermark);
            return Ok(CheckResult::Fail(format!(
                "binary contains compiler watermark: '{name}'"
            )));
        }
    }

    Ok(CheckResult::Pass)
}

pub fn check_binary_size(binary: &Path, config: &ValidateConfig) -> Result<CheckResult> {
    let max_str = match &config.max_binary_size {
        Some(s) => s,
        None => return Ok(CheckResult::Pass),
    };

    let max_bytes = parse_size(max_str)?;
    let actual = std::fs::metadata(binary)?.len();

    if actual > max_bytes {
        return Ok(CheckResult::Fail(format!(
            "binary size ({actual} bytes) exceeds maximum ({max_str} = {max_bytes} bytes)"
        )));
    }

    Ok(CheckResult::Pass)
}

pub fn check_unpatched_sentinels(binary: &Path, sentinels: &[String]) -> Result<CheckResult> {
    if sentinels.is_empty() {
        return Ok(CheckResult::Pass);
    }

    let data = std::fs::read(binary)?;

    for sentinel in sentinels {
        let sentinel_bytes = sentinel.as_bytes();
        if data
            .windows(sentinel_bytes.len())
            .any(|w| w == sentinel_bytes)
        {
            return Ok(CheckResult::Warn(format!(
                "unpatched sentinel found: '{sentinel}'"
            )));
        }
    }

    Ok(CheckResult::Pass)
}

pub fn check_rpath(binary: &Path, config: &ValidateConfig) -> Result<CheckResult> {
    if !config.no_rpath {
        return Ok(CheckResult::Pass);
    }

    // Try readelf on Linux, otool on macOS
    let result = if cfg!(target_os = "macos") {
        process::run_command("otool", &["-l", binary.to_str().unwrap_or("")], None, None)
    } else {
        process::run_command(
            "readelf",
            &["-d", binary.to_str().unwrap_or("")],
            None,
            None,
        )
    };

    match result {
        Ok(r) => {
            if r.stdout.contains("RPATH") || r.stdout.contains("RUNPATH") {
                return Ok(CheckResult::Fail(
                    "binary contains RPATH/RUNPATH".to_string(),
                ));
            }
        }
        Err(_) => {
            return Ok(CheckResult::Warn(
                "could not check RPATH (readelf/otool not available)".to_string(),
            ));
        }
    }

    Ok(CheckResult::Pass)
}

pub fn check_buildpaths(binary: &Path, config: &ValidateConfig) -> Result<CheckResult> {
    if !config.no_buildpaths {
        return Ok(CheckResult::Pass);
    }

    let data = std::fs::read(binary)?;

    // Common build path patterns
    let patterns = [b"/home/" as &[u8], b"/Users/", b"/tmp/", b"/build/"];

    for pattern in &patterns {
        if data.windows(pattern.len()).any(|w| w == *pattern) {
            let name = String::from_utf8_lossy(pattern);
            return Ok(CheckResult::Fail(format!(
                "binary contains build path: '{name}'"
            )));
        }
    }

    Ok(CheckResult::Pass)
}

fn parse_size(s: &str) -> Result<u64> {
    let s = s.trim();
    let (num_str, multiplier) = if let Some(n) = s.strip_suffix("GB") {
        (n.trim(), 1024 * 1024 * 1024)
    } else if let Some(n) = s.strip_suffix("MB") {
        (n.trim(), 1024 * 1024)
    } else if let Some(n) = s.strip_suffix("KB") {
        (n.trim(), 1024)
    } else if let Some(n) = s.strip_suffix('B') {
        (n.trim(), 1)
    } else {
        (s, 1)
    };

    let num: u64 = num_str
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid size: {s}"))?;

    Ok(num * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::types::ValidateConfig;

    #[test]
    fn parse_size_bytes() {
        assert_eq!(parse_size("100B").unwrap(), 100);
        assert_eq!(parse_size("100").unwrap(), 100);
    }

    #[test]
    fn parse_size_kilobytes() {
        assert_eq!(parse_size("1KB").unwrap(), 1024);
        assert_eq!(parse_size("10KB").unwrap(), 10240);
    }

    #[test]
    fn parse_size_megabytes() {
        assert_eq!(parse_size("1MB").unwrap(), 1048576);
        assert_eq!(parse_size("5MB").unwrap(), 5 * 1024 * 1024);
    }

    #[test]
    fn parse_size_gigabytes() {
        assert_eq!(parse_size("1GB").unwrap(), 1024 * 1024 * 1024);
    }

    #[test]
    fn parse_size_with_whitespace() {
        assert_eq!(parse_size("  10 MB  ").unwrap(), 10 * 1024 * 1024);
    }

    #[test]
    fn parse_size_invalid() {
        assert!(parse_size("abcMB").is_err());
    }

    #[test]
    fn check_result_is_fail() {
        assert!(CheckResult::Fail("x".into()).is_fail());
        assert!(!CheckResult::Pass.is_fail());
        assert!(!CheckResult::Warn("x".into()).is_fail());
    }

    #[test]
    fn check_result_is_warn() {
        assert!(CheckResult::Warn("x".into()).is_warn());
        assert!(!CheckResult::Pass.is_warn());
        assert!(!CheckResult::Fail("x".into()).is_warn());
    }

    #[test]
    fn debug_symbols_check_skipped_when_disabled() {
        let config = ValidateConfig {
            no_debug_symbols: false,
            ..Default::default()
        };
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"some binary data").unwrap();
        let result = check_debug_symbols(tmp.path(), &config).unwrap();
        assert!(matches!(result, CheckResult::Pass));
    }

    #[test]
    fn plaintext_strings_detected() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"contains password in binary").unwrap();
        let config = ValidateConfig {
            no_plaintext_strings: vec!["password".to_string()],
            ..Default::default()
        };
        let result = check_plaintext_strings(tmp.path(), &config).unwrap();
        assert!(result.is_fail());
    }

    #[test]
    fn plaintext_strings_not_found() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"clean binary data").unwrap();
        let config = ValidateConfig {
            no_plaintext_strings: vec!["password".to_string()],
            ..Default::default()
        };
        let result = check_plaintext_strings(tmp.path(), &config).unwrap();
        assert!(matches!(result, CheckResult::Pass));
    }

    #[test]
    fn unpatched_sentinels_found() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"data AAAA_CALLBACK_IP_AAAA rest").unwrap();
        let sentinels = vec!["AAAA_CALLBACK_IP_AAAA".to_string()];
        let result = check_unpatched_sentinels(tmp.path(), &sentinels).unwrap();
        assert!(result.is_warn());
    }

    #[test]
    fn unpatched_sentinels_clean() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"clean binary without sentinels").unwrap();
        let sentinels = vec!["AAAA_CALLBACK_IP_AAAA".to_string()];
        let result = check_unpatched_sentinels(tmp.path(), &sentinels).unwrap();
        assert!(matches!(result, CheckResult::Pass));
    }

    #[test]
    fn buildpaths_detected() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"binary with /home/user/build path").unwrap();
        let config = ValidateConfig {
            no_buildpaths: true,
            ..Default::default()
        };
        let result = check_buildpaths(tmp.path(), &config).unwrap();
        assert!(result.is_fail());
    }

    #[test]
    fn binary_size_within_limit() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), &vec![0u8; 100]).unwrap();
        let config = ValidateConfig {
            max_binary_size: Some("1KB".to_string()),
            ..Default::default()
        };
        let result = check_binary_size(tmp.path(), &config).unwrap();
        assert!(matches!(result, CheckResult::Pass));
    }

    #[test]
    fn binary_size_exceeds_limit() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), &vec![0u8; 2048]).unwrap();
        let config = ValidateConfig {
            max_binary_size: Some("1KB".to_string()),
            ..Default::default()
        };
        let result = check_binary_size(tmp.path(), &config).unwrap();
        assert!(result.is_fail());
    }
}
