//! FFI-safe types for syscall arguments and return values.

use crate::ids::{ThingId, SymbolId};

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

// Limits
pub const MAX_WATCH_EVENTS: usize = 64;
pub const MAX_REL_BATCH: usize = 32;
