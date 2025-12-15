
#[cfg(test)]
mod tests {
    use abi::mouse_stream::{MouseEntry, MouseStreamHeader};
    use abi::resident_layout::{ResidentHeader, ResPropEntry};

    #[test]
    fn test_mouse_entry_layout() {
        assert_eq!(core::mem::size_of::<MouseEntry>(), 8, "MouseEntry must be 8 bytes");
        assert_eq!(core::mem::align_of::<MouseEntry>(), 2, "MouseEntry align should be at least 2");
    }

    #[test]
    fn test_mouse_stream_header_layout() {
        assert_eq!(core::mem::size_of::<MouseStreamHeader>(), 16, "MouseStreamHeader must be 16 bytes");
    }

    #[test]
    fn test_resident_header_layout() {
        // defined in resident_layout.rs, currently 32 bytes
        assert_eq!(core::mem::size_of::<ResidentHeader>(), 40, "ResidentHeader size mismatch");
    }
}
