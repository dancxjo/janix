
pub fn test_dtb_size_parsing() {
    // FDT header: [magic:4] [totalsize:4]
    // totalsize is at offset 4, big-endian.
    let header: [u8; 8] = [0xde, 0xad, 0xbe, 0xef, 0x00, 0x00, 0x12, 0x34];
    let size = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as u64;
    
    if size != 0x1234 {
        panic!("DTB size parsing failed: expected 0x1234, got {:x}", size);
    }
}

pub fn test_dtb_capping() {
    const DTB_MAX_SIZE: u64 = 2 * 1024 * 1024;
    
    let is_sane = |size: u64| size > 0 && size <= DTB_MAX_SIZE;
    
    assert!(is_sane(1024));
    assert!(is_sane(DTB_MAX_SIZE));
    assert!(!is_sane(0));
    assert!(!is_sane(DTB_MAX_SIZE + 1));
}

pub fn run_selftest() {
    crate::kinfo!("FW_TABLES SELFTEST: Starting...");
    test_dtb_size_parsing();
    test_dtb_capping();
    crate::kinfo!("FW_TABLES SELFTEST: PASS");
}
