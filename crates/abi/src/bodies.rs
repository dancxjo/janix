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
