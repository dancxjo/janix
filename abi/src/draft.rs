#![no_std]

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftHandle(pub u64);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DraftAllocResp {
    pub handle: DraftHandle,
    pub user_addr: u64,     // user virtual address where draft is mapped
    pub byte_len: u32,      // total writable bytes
    pub _pad: u32,
}

pub const DRAFT_MAGIC: u32 = 0x31564454; // 'TDv1'

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ThingDraftHeader {
    pub magic: u32,         // 'TDv1' = 0x31564454
    pub version: u16,       // 1
    pub flags: u16,         // reserved
    pub kind_str_off: u32,  // offset to utf8 kind name bytes in same buffer
    pub kind_str_len: u32,
    pub prop_count: u32,
    pub prop_table_off: u32,// offset to [PropEntry; prop_count]
    pub total_len: u32,     // total bytes used by writer
    pub _pad: u32,
}

// Value tags for PropEntry
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftValueTag {
    U64 = 1,
    I64 = 2,
    Bool = 3,
    Str = 4,        // (off,len)
    ThingId = 5,    // in v field
    Bytes = 6,      // (off,len)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PropEntry {
    pub key_str_off: u32,   // offset to key name
    pub key_str_len: u32,
    pub tag: u8,
    pub _pad: [u8; 7],
    pub a: u32,             // for Str/Bytes: off
    pub b: u32,             // for Str/Bytes: len
    pub v: u64,             // for numeric/bool/thingid
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftSealErrorCode {
    Success = 0,
    BadHandle = 1,
    NotMapped = 2,
    BadHeader = 3,
    Bounds = 4,
    InvalidUtf8 = 5,
    UnknownKind = 6,
    PropertyNotInSchema = 7,
    MissingRequired = 8,
    WrongType = 9,
    TooManyProps = 10,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DraftSealError {
    pub code: DraftSealErrorCode,
    pub aux0: u64, // e.g. offending offset/len or hash of key
    pub aux1: u64, // optional
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SysDraftAllocReq { pub byte_len: u32, pub _pad: u32 }

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SysDraftSealReq { pub handle: DraftHandle }

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SysDraftAbortReq { pub handle: DraftHandle }
