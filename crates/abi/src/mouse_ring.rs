//! Mouse ring buffer ABI types.
//!
//! Provides a lock-free ring buffer structure for high-throughput mouse input.
//! The kernel writes MouseSample structs into the ring, and userspace (Bloom)
//! consumes them by mapping the bytespace.

use core::sync::atomic::AtomicU32;

/// Magic number to identify valid mouse ring buffers
pub const MOUSE_RING_MAGIC: u32 = 0x4D4F5553; // "MOUS"

/// Version of the ring buffer format
pub const MOUSE_RING_VERSION: u32 = 1;

/// Symbol ID for MouseStream kind
pub const KIND_MOUSE_STREAM: crate::ids::SymbolId = crate::ids::sym("kind.MouseStream");

/// Calculate the total bytespace size for a ring buffer with given capacity.
pub const fn ring_bytespace_size(capacity: u32) -> usize {
    core::mem::size_of::<MouseRingHeader>() + (capacity as usize * core::mem::size_of::<MouseSample>())
}

/// Ring buffer header - placed at the start of the bytespace.
#[repr(C)]
pub struct MouseRingHeader {
    /// Magic number (MOUSE_RING_MAGIC) for validation
    pub magic: u32,
    /// Version of the ring format
    pub version: u32,
    /// Number of sample slots in the ring
    pub capacity: u32,
    /// Size of each sample in bytes
    pub sample_size: u32,
    /// Write index (atomically incremented by kernel)
    pub write: AtomicU32,
    /// Count of dropped samples (if ring overflows)
    pub dropped: AtomicU32,
    /// Reserved for future use
    pub _reserved: [u32; 2],
}

/// A single mouse sample in the ring buffer.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MouseSample {
    /// Timestamp in nanoseconds (monotonic)
    pub t_ns: u64,
    /// X movement delta
    pub dx: i16,
    /// Y movement delta
    pub dy: i16,
    /// Wheel delta
    pub wheel: i16,
    /// Button state (bit 0=left, bit 1=right, bit 2=middle)
    pub buttons: u16,
}
