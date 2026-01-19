//! Root Graph Batch ABI
//!
//! Defines the binary format for `SYS_ROOT_APPLY_BATCH`.
//! Use safe byte-level parsing; do not cast unaligned bytes to these structs.
//!
//! ============================================================================
//! ApplyBatch Wire Format (canonical)
//! ============================================================================
//! All integers are little-endian.
//!
//! Batch header (8 bytes):
//!   [magic: u32][version: u16][op_count: u16]
//!
//! Then `op_count` operations, each prefixed with a 1-byte op tag.
//!
//! ThingRef encoding (variable size):
//!   tag: u8 (REF_ABSOLUTE or REF_LOCAL)
//!   if REF_ABSOLUTE: [thing_id: 16 bytes]
//!   if REF_LOCAL:    [local_index: u16]
//!
//! Operation layouts:
//!   OP_CREATE_NODE:
//!     [tag: u8][kind_id: 16 bytes][out_ref: u16]
//!
//!   OP_PUT_EDGE:
//!     [tag: u8][subject: ThingRef][predicate_id: 16 bytes][object: ThingRef]
//!
//!   OP_SET_PROP:
//!     [tag: u8][subject: ThingRef][key_id: 16 bytes][value: u64]
//!

/// Batch Header (8 bytes)
/// [Magic: 4] [Version: 2] [OpCount: 2]
pub const BATCH_MAGIC: u32 = 0x54485254; // "THRT"
pub const BATCH_VERSION: u16 = 1;

// ============================================================================
// Batch Caps (DoS protection + zero-alloc hot path)
// ============================================================================

/// Maximum batch payload size in bytes
pub const MAX_BATCH_BYTES: usize = 256 * 1024;   // 256 KiB
/// Maximum number of operations per batch
pub const MAX_BATCH_OPS: usize = 4096;
/// Maximum number of local references (CREATE_NODE outputs)
pub const MAX_LOCAL_REFS: usize = 1024;

/// Op Tags
pub const OP_CREATE_NODE: u8 = 0x01;
pub const OP_PUT_EDGE: u8 = 0x02;
pub const OP_SET_PROP: u8 = 0x03;
// Reserved for future: OP_CREATE_BYTESPACE = 0x04

/// Reference types for `ThingRef`
pub const REF_ABSOLUTE: u8 = 0x00;
pub const REF_LOCAL: u8 = 0x01;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BatchHeader {
    pub magic: u32,
    pub version: u16,
    pub op_count: u16,
}

impl BatchHeader {
    pub const SIZE: usize = 8;
    
    pub fn to_le_bytes(self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        buf[0..4].copy_from_slice(&self.magic.to_le_bytes());
        buf[4..6].copy_from_slice(&self.version.to_le_bytes());
        buf[6..8].copy_from_slice(&self.op_count.to_le_bytes());
        buf
    }
}

// ============================================================================
// Watch Filters
// ============================================================================

/// Filter flags for RootWatchFilter
pub const WATCH_F_ALL: u32 = 0;           // Match all commits (no filtering)
pub const WATCH_F_KIND: u32 = 1 << 0;     // Filter by kind_id
pub const WATCH_F_PREDICATE: u32 = 1 << 1; // Filter by predicate_id
pub const WATCH_F_SUBJECT: u32 = 1 << 2;   // Filter by subject ThingId

/// Compact watch filter (32 bytes, C-compatible)
/// 
/// Used with `SYS_ROOT_WATCH_OPEN` to filter which commits are delivered.
/// If `flags == 0`, all commits match. Otherwise, only commits containing
/// at least one operation matching the specified criteria are delivered.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RootWatchFilter {
    /// Filter flags (combination of WATCH_F_* constants)
    pub flags: u32,
    /// Kind ID to filter by (requires WATCH_F_KIND flag)
    /// Must be an interned SymbolId from SYS_ROOT_INTERN
    pub kind_id: u32,
    /// Predicate ID to filter by (requires WATCH_F_PREDICATE flag)
    /// Must be an interned SymbolId from SYS_ROOT_INTERN
    pub predicate_id: u32,
    /// Reserved for alignment
    pub _reserved: u32,
    /// Subject ThingId high bits (reserved for 128-bit ThingId expansion)
    pub subject_hi: u64,
    /// Subject ThingId to filter by (requires WATCH_F_SUBJECT flag)
    pub subject_lo: u64,
}

impl RootWatchFilter {
    pub const SIZE: usize = 32;
    
    /// Create a filter that matches all commits
    pub fn all() -> Self {
        Self::default()
    }
    
    /// Create a filter for a specific subject
    pub fn subject(id: u64) -> Self {
        Self {
            flags: WATCH_F_SUBJECT,
            subject_lo: id,
            ..Default::default()
        }
    }
    
    /// Create a filter for a specific predicate
    pub fn predicate(predicate_id: u32) -> Self {
        Self {
            flags: WATCH_F_PREDICATE,
            predicate_id,
            ..Default::default()
        }
    }
    
    /// Create a filter for a specific kind
    pub fn kind(kind_id: u32) -> Self {
        Self {
            flags: WATCH_F_KIND,
            kind_id,
            ..Default::default()
        }
    }
}

/// Helper for reasoning about ThingRef size
pub fn thing_ref_size(kind: u8) -> usize {
    match kind {
        REF_ABSOLUTE => 1 + 16, // tag + ThingId
        REF_LOCAL => 1 + 2,     // tag + u16 index
        _ => 0, // Invalid
    }
}

/// Mask of all known filter flags (for validation)
/// Unknown flag bits should be rejected with EINVAL.
pub const WATCH_F_KNOWN_MASK: u32 = WATCH_F_KIND | WATCH_F_PREDICATE | WATCH_F_SUBJECT;
