// #![no_std] removed (crate level only)

// use crate::resident_layout::ResidentHeader; // unused

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MouseStreamHeader {
    pub head: u32,
    pub capacity: u32,
    pub _pad: [u8; 8], // align to 16 bytes if needed, or just padding
}

// Ensure explicit 8-byte layout for MouseEntry as per plan
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MouseEntry {
    pub dx: i16,
    pub dy: i16,
    pub buttons: u8,
    pub flags: u8,
    pub _pad: u16,
}

// Static assertion for size (will fail compile if not 8)
const _: () = assert!(core::mem::size_of::<MouseEntry>() == 8);
