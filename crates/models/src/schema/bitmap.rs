use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitmapBody {
    pub width: u32,
    pub height: u32,
    pub format: u32, // 0 = ARGB8888 (Little Endian: B G R A)
    pub pixels: Vec<u8>,
}
