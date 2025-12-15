#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ResidentHeader {
    pub magic: u32,      // 'RSID' = 0x44495352
    pub version: u16,    // 1
    pub flags: u16,      // reserved
    pub kind_id: u32,    // schema kind id (or string-id)
    pub prop_count: u16,
    pub _pad0: u16,
    pub props_off: u32,  // offset to PropEntry[prop_count]
    pub data_off: u32,   // start of variable region
    pub total_len: u32,  // bytes used
    pub generation: u32,        // generation counter (handle invalidation)
    pub seq: u32,        // seqlock-style: writer makes it odd while writing, even when stable
    pub _pad1: u32,
}

impl ResidentHeader {
    pub const MAGIC: u32 = 0x44495352;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResTag {
    U64 = 1,
    I64 = 2,
    Bool = 3,
    Str = 4,   // off/len
    Bytes = 5, // off/len
    ThingId = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ResPropEntry {
    pub key_id: u32,    // schema key id
    pub tag: u8,
    pub _pad: [u8; 3],
    pub a: u32,         // off for Str/Bytes
    pub b: u32,         // len for Str/Bytes
    pub v: u64,         // numeric / ThingId / bool in low bit
}
