#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ResidentHeader {
    pub magic: u32,   // 'RSID'
    pub version: u16, // 1
    pub flags: u16,
    pub total_len: u32, // bytes
    pub props_off: u32, // offset to ResPropEntry table (0 if none)
    pub prop_count: u32,
    pub data_off: u32,   // offset to resident payload region
    pub generation: u32, // increment on rest/evict if desired
    pub seq: u32,        // seqlock
    pub _pad: u32,
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
    pub key_id: u32, // schema key id
    pub tag: ResTag,
    pub _pad: [u8; 3],
    pub a: u32, // off for Str/Bytes
    pub b: u32, // len for Str/Bytes
    pub v: u64, // numeric / ThingId / bool in low bit
}
