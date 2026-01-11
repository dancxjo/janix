//! Shared types used in syscall payloads.
//! Must be #[repr(C)] to ensure stable layout.

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TimeSpec {
    pub seconds: u64,
    pub nanoseconds: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct StreamStatus {
    pub readable: bool,
    pub writable: bool,
    pub closed: bool,
    pub error: bool,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StreamId(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct WatchId(pub u64);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct EventHeader {
    pub type_: u32,
    pub size: u32,
    pub stream_id: StreamId,
}
