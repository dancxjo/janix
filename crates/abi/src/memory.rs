// Memory layout constants (Common ABI)

// User heap end address (example value, adjust based on actual donor if needed)
pub const USER_HEAP_END: u64 = 0x0000_7FFF_FFFF_FFFF;

pub const USER_RESIDENT_BASE: u64 = 0x0000_6000_0000_0000;
pub const USER_RESIDENT_LIMIT: u64 = 0x0000_6FFF_FFFF_FFFF;
