use crate::ids::SymbolId;
use alloc::vec::Vec;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SurfaceBody {
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
}

impl SurfaceBody {
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(20); // 4 + 4 + 4 + 8 = 20
        bytes.extend_from_slice(&self.width.to_le_bytes());
        bytes.extend_from_slice(&self.height.to_le_bytes());
        bytes.extend_from_slice(&self.stride_bytes.to_le_bytes());
        bytes.extend_from_slice(&self.format.0.to_le_bytes());
        bytes
    }
}

pub const BYTESPACE_FLAG_HAS_PHYS_BASE: u32 = 1 << 0;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BytespaceBody {
    pub len: u64,
    pub flags: u32,
    pub _pad: u32,
    pub phys_base: u64,
}

impl BytespaceBody {
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(24); // 8 + 4 + 4 + 8 = 24
        bytes.extend_from_slice(&self.len.to_le_bytes());
        bytes.extend_from_slice(&self.flags.to_le_bytes());
        bytes.extend_from_slice(&self._pad.to_le_bytes());
        bytes.extend_from_slice(&self.phys_base.to_le_bytes());
        bytes
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MonotonicClockBody {
    pub now_ns: u64,
    pub resolution_ns: u64,
    pub source: SymbolId,
    pub last_update_ns: u64,
}

impl MonotonicClockBody {
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&self.now_ns.to_le_bytes());
        bytes.extend_from_slice(&self.resolution_ns.to_le_bytes());
        bytes.extend_from_slice(&self.source.0.to_le_bytes());
        bytes.extend_from_slice(&self.last_update_ns.to_le_bytes());
        bytes
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemClockBody {
    pub unix_epoch_ns: i64,
    pub status: u32, // 0: Unset, 1: Set, 2: Slewing
    pub _pad: u32,
    pub last_set_mono_ns: u64,
    pub accuracy_ns: u64,
    pub source: SymbolId,
}

impl SystemClockBody {
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(40);
        bytes.extend_from_slice(&self.unix_epoch_ns.to_le_bytes());
        bytes.extend_from_slice(&self.status.to_le_bytes());
        bytes.extend_from_slice(&self._pad.to_le_bytes());
        bytes.extend_from_slice(&self.last_set_mono_ns.to_le_bytes());
        bytes.extend_from_slice(&self.accuracy_ns.to_le_bytes());
        bytes.extend_from_slice(&self.source.0.to_le_bytes());
        bytes
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeOffsetBody {
    pub offset_ns: i64,
    pub rate_ppb: i64,
    pub updated_mono_ns: u64,
}

impl TimeOffsetBody {
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(24);
        bytes.extend_from_slice(&self.offset_ns.to_le_bytes());
        bytes.extend_from_slice(&self.rate_ppb.to_le_bytes());
        bytes.extend_from_slice(&self.updated_mono_ns.to_le_bytes());
        bytes
    }
}
