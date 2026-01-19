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
#[derive(Debug, Clone, Copy, Default)]
pub struct WatchSpec {
    pub query_ptr: u64,
    pub query_len: u64,
    pub mode: u32,
    pub _padding: u32,  // Alignment padding
    pub start_seq: u64,
    /// Pointer to RootWatchFilter (0 = no filter, match all)
    pub filter_ptr: u64,
    /// Size of filter struct (for versioning, should be RootWatchFilter::SIZE)
    pub filter_len: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct WatchEvent {
    pub kind: u32, // 1=Found, 2=Lost
    pub node_id: u64,
    pub handle: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GraphEdge {
    pub rel: u64,
    pub target: u64,
}

// ============================================================================
// Watch Semantics
// ============================================================================

/// Start from next commit only (no history replay)
/// Use this to ignore historical events and only see future mutations.
pub const WATCH_START_LATEST: u64 = u64::MAX;

impl WatchMode {
    /// Convert raw u32 to WatchMode, returning None for unknown values.
    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::QueryThenStream),
            1 => Some(Self::StreamOnly),
            _ => None,
        }
    }
}
