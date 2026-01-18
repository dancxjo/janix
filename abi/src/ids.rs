use crate::wire::ThingId;

/// Adapter trait for converting between internal u64 handles and ABI ThingIds.
/// This acts as a bridge while the kernel uses u64 internally but the ABI uses UUIDs.
pub trait HandleId {
    fn from_u64(val: u64) -> Self;
    fn to_u64_lossy(&self) -> u64;
}

impl HandleId for ThingId {
    /// Creates a ThingId from a u64 handle by zero-padding.
    /// Layout: [handle_bytes(8) | 0...0]
    fn from_u64(val: u64) -> Self {
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&val.to_le_bytes());
        ThingId(bytes)
    }

    /// Extracts the u64 handle from a ThingId.
    /// This is lossy/unsafe if the ThingId was not created via `from_u64`.
    /// For the bridge, we assume it is valid.
    fn to_u64_lossy(&self) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[0..8]);
        u64::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_id_roundtrip() {
        let original = 0xDEAD_BEEF_CAFE_BABE;
        let thing_id = ThingId::from_u64(original);
        let recovered = thing_id.to_u64_lossy();
        assert_eq!(original, recovered, "Handle roundtrip failed");
    }

    #[test]
    fn test_handle_id_layout() {
        let val = 0x0102030405060708;
        let thing_id = ThingId::from_u64(val);
        // Little endian: 08, 07, 06...
        let expected = [
            0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(thing_id.0, expected, "ThingId layout mismatch");
    }
}
