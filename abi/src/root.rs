//! Root Graph Batch ABI
//!
//! Defines the binary format for `SYS_ROOT_APPLY_BATCH`.
//! Use safe byte-level parsing; do not cast unaligned bytes to these structs.

use crate::wire::ThingId;

/// Batch Header (8 bytes)
/// [Magic: 4] [Version: 2] [OpCount: 2]
pub const BATCH_MAGIC: u32 = 0x54485254; // "THRT"
pub const BATCH_VERSION: u16 = 1;

/// Op Tags
pub const OP_CREATE_NODE: u8 = 0x01;
pub const OP_PUT_EDGE: u8 = 0x02;
// Reserved for future: OP_SET_PROP = 0x03, OP_CREATE_BYTESPACE = 0x04

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

/// Helper for reasoning about ThingRef size
pub fn thing_ref_size(kind: u8) -> usize {
    match kind {
        REF_ABSOLUTE => 1 + 16, // tag + ThingId
        REF_LOCAL => 1 + 2,     // tag + u16 index
        _ => 0, // Invalid
    }
}
