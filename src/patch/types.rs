use anyhow::{Result, bail};

use crate::manifest::types::PatchFieldDef;

pub fn encode_value(field: &PatchFieldDef, value: &str) -> Result<Vec<u8>> {
    match field.r#type.as_str() {
        "ipv4" => encode_ipv4(value, field.size),
        "u8" => encode_u8(value, field.size),
        "u16" => encode_u16(value, field.size),
        "u32" => encode_u32(value, field.size),
        "string" => encode_string(value, field.size),
        other => bail!("unsupported patch type: {other}"),
    }
}

fn encode_ipv4(value: &str, size: usize) -> Result<Vec<u8>> {
    // Store as string bytes, null-padded
    let bytes = value.as_bytes();
    if bytes.len() > size {
        bail!(
            "IPv4 string '{}' ({} bytes) exceeds field size {}",
            value,
            bytes.len(),
            size
        );
    }
    let mut result = bytes.to_vec();
    result.resize(size, 0);
    Ok(result)
}

fn encode_u8(value: &str, size: usize) -> Result<Vec<u8>> {
    let n: u8 = value
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid u8: {value}"))?;
    let mut result = vec![n];
    result.resize(size, 0);
    Ok(result)
}

fn encode_u16(value: &str, size: usize) -> Result<Vec<u8>> {
    let n: u16 = value
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid u16: {value}"))?;
    let bytes = n.to_be_bytes();
    let mut result = bytes.to_vec();
    result.resize(size, 0);
    Ok(result)
}

fn encode_u32(value: &str, size: usize) -> Result<Vec<u8>> {
    let n: u32 = value
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid u32: {value}"))?;
    let bytes = n.to_be_bytes();
    let mut result = bytes.to_vec();
    result.resize(size, 0);
    Ok(result)
}

fn encode_string(value: &str, size: usize) -> Result<Vec<u8>> {
    let bytes = value.as_bytes();
    if bytes.len() > size {
        bail!(
            "string '{}' ({} bytes) exceeds field size {}",
            value,
            bytes.len(),
            size
        );
    }
    let mut result = bytes.to_vec();
    result.resize(size, 0);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::types::PatchFieldDef;

    fn field(ty: &str, size: usize) -> PatchFieldDef {
        PatchFieldDef {
            sentinel: "TEST".to_string(),
            r#type: ty.to_string(),
            size,
        }
    }

    #[test]
    fn encode_ipv4_basic() {
        let result = encode_value(&field("ipv4", 16), "10.0.0.1").unwrap();
        assert_eq!(result.len(), 16);
        assert_eq!(&result[..8], b"10.0.0.1");
        assert!(result[8..].iter().all(|&b| b == 0));
    }

    #[test]
    fn encode_ipv4_too_long() {
        let err = encode_value(&field("ipv4", 4), "192.168.100.200").unwrap_err();
        assert!(err.to_string().contains("exceeds field size"));
    }

    #[test]
    fn encode_u8_valid() {
        let result = encode_value(&field("u8", 4), "255").unwrap();
        assert_eq!(result[0], 255);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn encode_u8_invalid() {
        assert!(encode_value(&field("u8", 1), "256").is_err());
        assert!(encode_value(&field("u8", 1), "abc").is_err());
    }

    #[test]
    fn encode_u16_big_endian() {
        let result = encode_value(&field("u16", 4), "443").unwrap();
        assert_eq!(&result[..2], &443u16.to_be_bytes());
    }

    #[test]
    fn encode_u32_big_endian() {
        let result = encode_value(&field("u32", 8), "80000").unwrap();
        assert_eq!(&result[..4], &80000u32.to_be_bytes());
    }

    #[test]
    fn encode_string_basic() {
        let result = encode_value(&field("string", 10), "hello").unwrap();
        assert_eq!(&result[..5], b"hello");
        assert!(result[5..].iter().all(|&b| b == 0));
    }

    #[test]
    fn encode_string_too_long() {
        let err = encode_value(&field("string", 3), "toolong").unwrap_err();
        assert!(err.to_string().contains("exceeds field size"));
    }

    #[test]
    fn unsupported_type_rejected() {
        let err = encode_value(&field("float", 4), "1.0").unwrap_err();
        assert!(err.to_string().contains("unsupported patch type"));
    }
}
