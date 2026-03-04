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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_patch_basic() {
        let mut data = vec![0u8; 20];
        let encoded = b"hello";
        apply_patch(&mut data, 5, encoded, 10).unwrap();
        assert_eq!(&data[5..10], b"hello");
        assert_eq!(&data[10..15], &[0, 0, 0, 0, 0]); // null-padded
    }

    #[test]
    fn encoded_exceeds_field_size() {
        let mut data = vec![0u8; 20];
        let encoded = b"toolong";
        let err = apply_patch(&mut data, 0, encoded, 3).unwrap_err();
        assert!(err.to_string().contains("exceeds field size"));
    }

    #[test]
    fn patch_past_end_of_binary() {
        let mut data = vec![0u8; 10];
        let encoded = b"hi";
        let err = apply_patch(&mut data, 8, encoded, 5).unwrap_err();
        assert!(err.to_string().contains("past end of binary"));
    }

    #[test]
    fn patch_exact_fit() {
        let mut data = vec![0xFFu8; 10];
        let encoded = b"abc";
        apply_patch(&mut data, 0, encoded, 3).unwrap();
        assert_eq!(&data[0..3], b"abc");
        assert_eq!(data[3], 0xFF); // untouched
    }

    #[test]
    fn null_padding_applied() {
        let mut data = vec![0xFFu8; 10];
        let encoded = b"x";
        apply_patch(&mut data, 2, encoded, 5).unwrap();
        assert_eq!(data[2], b'x');
        assert_eq!(&data[3..7], &[0, 0, 0, 0]);
        assert_eq!(data[7], 0xFF); // untouched
    }
}
