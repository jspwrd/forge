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
