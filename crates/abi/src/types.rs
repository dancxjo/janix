//! FFI-safe types for syscall arguments and return values.

use crate::ids::{ThingId, SymbolId, WatchId};
use bitflags::bitflags;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelationshipRef {
    pub id: ThingId, // actually RelationshipId but it's a type alias
    pub kind: SymbolId,
    pub target: ThingId,
}

// Ensure 16-byte alignment or packed if needed for arrays?
// 128 + 64 + 128 = 320 bits = 40 bytes.
// Arrays of this might be awkward to iterate if we just dump bytes.
// For `SYS_REL_GET_FROM`:
// We might return arrays of `(RelationshipId, kind, to)`
// RelationshipId is 128-bit.
// SymbolId is 64-bit.
// ThingId is 128-bit.
// Total 32 bytes + 8 bytes = 40 bytes.
// Alignment 16.
// Size 48 bytes with padding.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThingHeader {
    pub id: ThingId,
    pub kind: SymbolId,
    // flags? size?
}

bitflags! {
    /// Wait behavior flags
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WaitFlags: u32 {
        /// Wake when any watch fires (default)
        const WAIT_ANY = 0;
        /// Wake only when all watches fire
        const WAIT_ALL = 1 << 0;
    }
}

/// Reason a blocked wait was resumed.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeReasonCode {
    Watch = 1,
    Timeout = 2,
    Cancelled = 3,
}

/// Wake metadata returned from sys_wait.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WakeReason {
    pub reason: WakeReasonCode,
    pub which: WatchId,
    pub val0: u64,
    pub val1: u64,
}

impl WakeReason {
    pub const fn watch(id: WatchId) -> Self {
        Self { reason: WakeReasonCode::Watch, which: id, val0: 0, val1: 0 }
    }

    pub const fn timeout() -> Self {
        Self { reason: WakeReasonCode::Timeout, which: WatchId(0), val0: 0, val1: 0 }
    }

    pub const fn cancelled() -> Self {
        Self { reason: WakeReasonCode::Cancelled, which: WatchId(0), val0: 0, val1: 0 }
    }
}

// Limits
pub const MAX_WATCH_EVENTS: usize = 64;
pub const MAX_REL_BATCH: usize = 32;
