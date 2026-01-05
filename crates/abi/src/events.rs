//! EventStream wire format and constants.
//!
//! Provides a single wire format: EventStreamHeader + ring of EventRecord + payload bytes.
//! Cross-arch & FFI-safe: stable repr(C), little-endian fields.

use core::sync::atomic::{AtomicU32, AtomicU64};

pub const EVENT_STREAM_MAGIC: u32 = u32::from_le_bytes(*b"EVNT");
pub const EVENT_STREAM_VERSION: u16 = 1;

/// Size of the EventStreamHeader in bytes
pub const HEADER_SIZE: usize = core::mem::size_of::<EventStreamHeader>();

/// Size of the EventRecord header (without payload)
pub const RECORD_HEADER_SIZE: usize = core::mem::size_of::<EventRecord>();

/// Header (shared memory)
#[repr(C)]
pub struct EventStreamHeader {
    pub magic: u32,
    pub version: u16,
    pub header_bytes: u16,
    pub capacity_bytes: u32,
    /// monotonic, counts records written
    pub write_seq: AtomicU64,
    /// byte offset into ring
    pub write_off: AtomicU32,
    pub dropped: AtomicU64,
    pub reserved: [u32; 2],
}

/// Record layout (variable size, aligned to 8 bytes)
#[repr(C)]
pub struct EventRecord {
    /// total record bytes including this header
    pub len: u16,
    /// small numeric kind id or SymbolId index
    pub kind: u16,
    pub flags: u16,
    pub reserved: u16,
    /// sequence number
    pub seq: u64,
    /// optional, or 0
    pub ts_mono: u64,
    // payload bytes follow...
}

/// Event Kinds
pub const EV_POINTER_DELTA: u16 = 1;
pub const EV_KEY_SCANCODE: u16 = 2;

/// Pointer delta payload
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PointerDeltaPayload {
    pub dx: i16,
    pub dy: i16,
    pub buttons: u16,
    pub wheel: i16,
    pub reserved: i16,
}

/// Size of PointerDeltaPayload
pub const POINTER_DELTA_PAYLOAD_SIZE: usize = core::mem::size_of::<PointerDeltaPayload>();

/// Key scancode payload
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyScancodePayload {
    pub set: u8,
    pub code: u8,
    pub flags: u16,
}

/// Size of KeyScancodePayload
pub const KEY_SCANCODE_PAYLOAD_SIZE: usize = core::mem::size_of::<KeyScancodePayload>();

impl EventStreamHeader {
    /// Create a new header with the given ring capacity
    pub fn new(capacity_bytes: u32) -> Self {
        Self {
            magic: EVENT_STREAM_MAGIC,
            version: EVENT_STREAM_VERSION,
            header_bytes: core::mem::size_of::<Self>() as u16,
            capacity_bytes,
            write_seq: AtomicU64::new(0),
            write_off: AtomicU32::new(0),
            dropped: AtomicU64::new(0),
            reserved: [0; 2],
        }
    }
}

/// Compute the total record size including 8-byte alignment padding
#[inline]
pub const fn record_size(payload_len: usize) -> usize {
    let raw = RECORD_HEADER_SIZE + payload_len;
    (raw + 7) & !7
}

/// Total bytespace size needed for an EventStream with given ring capacity
#[inline]
pub const fn stream_bytespace_size(ring_capacity_bytes: u32) -> usize {
    HEADER_SIZE + ring_capacity_bytes as usize
}
