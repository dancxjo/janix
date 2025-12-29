pub const USER_HEAP_START: usize = 0x0000_0000_4000_0000;
pub const USER_HEAP_SIZE: usize = 32 * 1024 * 1024;
pub const USER_HEAP_END: usize = USER_HEAP_START + USER_HEAP_SIZE;

pub const USER_RESIDENT_BASE: usize = 0x5000_0000;
pub const USER_RESIDENT_LIMIT: usize = 0x6000_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysFrame(pub u64);
