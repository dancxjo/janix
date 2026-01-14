//! Shared types used in syscall payloads.
//! Must be #[repr(C)] to ensure stable layout.

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TimeSpec {
    pub seconds: u64,
    pub nanoseconds: u32,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ThingId(pub u64);

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

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RootWatchEvent {
    pub target: u64,
    pub key: u64,
    pub value: u64,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Unknown = 0,
    Runnable = 1,
    Running = 2,
    Blocked = 3,
    Dead = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct StackInfo {
    pub guard_start: usize,
    pub guard_end: usize,
    pub reserve_start: usize,
    pub reserve_end: usize,
    pub committed_start: usize,
    pub grow_chunk_bytes: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SpawnThreadReq {
    pub entry: usize,
    pub sp: usize,
    pub stack: StackInfo,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchMode {
    QueryThenStream = 0,
    StreamOnly = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WatchSpec {
    pub query_ptr: u64,
    pub query_len: u64,
    pub mode: u32, 
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WatchEvent {
    pub kind: u32, // 1=Found, 2=Lost
    pub node_id: u64,
    pub handle: u64,
    pub size: u64,
}
