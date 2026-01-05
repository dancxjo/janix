#![no_std]

use serde::{Serialize, Deserialize};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrawCmd {
    FillRect { x: i16, y: i16, w: u16, h: u16, color: u32 },
    FillRoundedRect { x: i16, y: i16, w: u16, h: u16, radius: u16, color: u32 },
    StrokeRoundedRect { x: i16, y: i16, w: u16, h: u16, radius: u16, thickness: u16, color: u32 },
    // Text command is followed by `len` bytes of UTF-8 text in the buffer
    Text { x: i16, y: i16, color: u32, len: u16 },
    Clear { color: u32 },
    // Reserved for future implementation
    Shadow { x: i16, y: i16, w: u16, h: u16, radius: u16, blur: u16, color: u32 },
    Blit { x: i16, y: i16, w: u16, h: u16, asset: u64 },
    End,
}
