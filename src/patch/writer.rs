use anyhow::{Result, bail};

pub fn apply_patch(
    data: &mut [u8],
    offset: usize,
    encoded: &[u8],
    field_size: usize,
) -> Result<()> {
    if encoded.len() > field_size {
        bail!(
            "encoded value ({} bytes) exceeds field size ({})",
            encoded.len(),
            field_size
        );
    }
    if offset + field_size > data.len() {
        bail!(
            "patch would write past end of binary (offset={}, size={}, binary_len={})",
            offset,
            field_size,
            data.len()
        );
    }

    // Write encoded value
    data[offset..offset + encoded.len()].copy_from_slice(encoded);

    // Null-pad remainder
    for byte in &mut data[offset + encoded.len()..offset + field_size] {
        *byte = 0;
    }

    Ok(())
}
