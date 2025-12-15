#![no_std]

pub const SYS_DEV_OPEN: u32 = 0x40;
pub const SYS_DEV_READ: u32 = 0x41;

pub const SYS_RESIDENT_ALLOC: u32 = 0x60;
pub const SYS_RESIDENT_MAP: u32 = 0x61;
pub const SYS_RESIDENT_UNMAP: u32 = 0x62;
pub const SYS_THING_REST: u32 = 0x63;
