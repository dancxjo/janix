use crate::resident_layout::ResidentHeader;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KeyboardStreamHeader {
    // Standard ResidentHeader (40 bytes)
    pub header: ResidentHeader,
    
    // Ring buffer state
    pub head: u64,
    pub capacity: u64,
    pub _pad: [u8; 8], // Align to 64 bytes total (40 + 16 + 8 = 64)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct KeyboardEntry {
    pub scancode: u8,
    pub flags: u8,      // Bit 0: Released, Bit 1: Extended, Bit 2: HasChar
    pub _pad: u16,
    pub utf32: u32,     // Character codepoint if HasChar is set
}

impl KeyboardEntry {
    pub const FLAG_RELEASED: u8 = 1 << 0;
    pub const FLAG_EXTENDED: u8 = 1 << 1;
    pub const FLAG_HAS_CHAR: u8 = 1 << 2;
}

// Static assertions for size and alignment
const _: () = assert!(core::mem::size_of::<KeyboardEntry>() == 8);
const _: () = assert!(core::mem::size_of::<KeyboardStreamHeader>() == 64);
