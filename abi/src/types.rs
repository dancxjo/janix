//! Shared types for system calls.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StreamId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WatchId(pub u64);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EventHeader {
    pub seq: u64,
    pub kind: u16,
    pub flags: u16,
    pub len: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MouseDelta {
    pub dx: i32,
    pub dy: i32,
}
