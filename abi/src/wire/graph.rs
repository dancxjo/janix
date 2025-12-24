use super::common::UserPtr;
use crate::syscall_defs::SymbolId;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WireValueTag {
    U64 = 0,
    I64 = 1,
    Bool = 2,
    Str = 3,  // data is SymbolId
    Blob = 4, // data_0 is ptr, data_1 is len
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueFormat {
    Postcard = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireSchemaLink {
    pub pred: crate::Predicate,
    pub target_kind: SymbolId,
    pub min: i32,
    pub max: i32, // -1 => unbounded
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ValueBlobHeader {
    pub format: u8, // ValueFormat
    pub _pad: [u8; 3],
    pub type_id: SymbolId, // interned type name
    pub len: u32,          // payload length (bytes)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WireBlob {
    pub ptr: u64,
    pub len: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WirePropValue {
    pub tag: u8,
    pub _pad: [u8; 7],
    pub data_0: u64, // u64 value, or Blob ptr
    pub data_1: u64, // Blob len
}

impl WirePropValue {
    pub const fn u64(v: u64) -> Self {
        Self {
            tag: WireValueTag::U64 as u8,
            _pad: [0; 7],
            data_0: v,
            data_1: 0,
        }
    }
    pub const fn i64(v: i64) -> Self {
        Self {
            tag: WireValueTag::I64 as u8,
            _pad: [0; 7],
            data_0: v as u64,
            data_1: 0,
        }
    }
    pub const fn bool(v: bool) -> Self {
        Self {
            tag: WireValueTag::Bool as u8,
            _pad: [0; 7],
            data_0: if v { 1 } else { 0 },
            data_1: 0,
        }
    }
    pub const fn sym(id: SymbolId) -> Self {
        Self {
            tag: WireValueTag::Str as u8,
            _pad: [0; 7],
            data_0: id.0 as u64,
            data_1: 0,
        }
    }
    pub const fn blob(ptr: u64, len: u64) -> Self {
        Self {
            tag: WireValueTag::Blob as u8,
            _pad: [0; 7],
            data_0: ptr,
            data_1: len,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireProp {
    pub key: SymbolId,
    pub _pad: u32, // SymbolId is u32, need padding to align next u64?
    // If WirePropValue is 16 bytes and align 8.
    // SymbolId(4) + pad(4) + WirePropValue(16) = 24 bytes.
    pub value: WirePropValue,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireSchemaProp {
    pub name: SymbolId,
    pub prop_type: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatchUpdateEntry {
    pub id: crate::ThingId,
    pub props_ptr: UserPtr<WireProp>,
    pub props_len: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatchUpdateReq {
    pub updates_ptr: UserPtr<BatchUpdateEntry>,
    pub updates_len: u64,
}
