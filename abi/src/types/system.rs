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
    pub _padding: u32, // Alignment padding
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

// ============================================================================
// Bulk Property Fetch (Performance Optimization)
// ============================================================================

/// Maximum number of properties that can be fetched in one bulk call
pub const BULK_PROPS_MAX_KEYS: usize = 32;

/// Request for bulk property fetch
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BulkPropsRequest {
    /// Node ID to query
    pub node_id: u64,
    /// Array of property key IDs (pre-interned SymbolIds)
    pub keys: [u32; BULK_PROPS_MAX_KEYS],
    /// Number of valid keys in the array
    pub key_count: u8,
    pub _pad: [u8; 3],
}

impl Default for BulkPropsRequest {
    fn default() -> Self {
        Self {
            node_id: 0,
            keys: [0; BULK_PROPS_MAX_KEYS],
            key_count: 0,
            _pad: [0; 3],
        }
    }
}

/// Response from bulk property fetch
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BulkPropsResponse {
    /// Node ID that was queried
    pub node_id: u64,
    /// Property values (parallel to request keys array)
    pub values: [u64; BULK_PROPS_MAX_KEYS],
    /// Bitmask indicating which keys had values (bit N = key N present)
    pub present_mask: u32,
    pub _pad: u32,
}

impl Default for BulkPropsResponse {
    fn default() -> Self {
        Self {
            node_id: 0,
            values: [0; BULK_PROPS_MAX_KEYS],
            present_mask: 0,
            _pad: 0,
        }
    }
}
