use crate::ids::SymbolId;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadFormat {
    Postcard = 0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ThingEnvelopeV1 {
    /// magic: u32 = b"THNG"
    pub magic: u32,
    /// env_version: u16 = 1
    pub env_version: u16,
    pub flags: u16,
    /// kind: u64 (SymbolId raw)
    pub kind: u64,
    /// schema_hash: u64
    pub schema_hash: u64,
    /// schema_version: u32
    pub schema_version: u32,
    /// schema_str_len: u16
    pub schema_str_len: u16,
    /// payload_format: u8 (0 = postcard)
    pub payload_format: u8,
    pub reserved0: u8,
    /// payload_len: u32
    pub payload_len: u32,
    /// body_len: u32 (total bytes including header + schema_str + payload + padding)
    pub body_len: u32,
    /// integrity: u64 (CRC64 digest)
    pub integrity: u64,
}

impl ThingEnvelopeV1 {
    pub const MAGIC: u32 = 0x474E4854; // "THNG" in little endian
    pub const VERSION: u16 = 1;

    pub const fn size() -> usize {
        core::mem::size_of::<Self>()
    }
}


pub const BYTESPACE_FLAG_HAS_PHYS_BASE: u32 = 1 << 0;
