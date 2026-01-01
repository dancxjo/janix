

use crate::ids::ThingId;

// Syscall Numbers
pub const SYSCALL_LOG: u64 = 0x10;

pub const SYSCALL_MEM_BASE: u64 = 0x20;
pub const SYSCALL_HEAP_GROW: u64 = 0x20 + 0;
pub const SYSCALL_BYTESPACE_CREATE: u64 = 0x20 + 1;
pub const SYSCALL_SPACE_MAP: u64 = 0x20 + 2;
pub const SYSCALL_SPACE_UNMAP: u64 = 0x20 + 3;

// Types
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct HeapGrowResult {
    pub base: u64,
    pub len: u64,
    pub errno: i32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytespaceKind {
    Ram = 1,
    Device = 2,
    Module = 3,
    Framebuffer = 4,
}

bitflags::bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct MapPerms: u32 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXEC = 1 << 2;
        const USER = 1 << 3;
        const DEVICE = 1 << 4;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapResult {
    pub vaddr: u64,
    pub len: u64,
    pub map_id: ThingId,
    pub errno: i32,
}
