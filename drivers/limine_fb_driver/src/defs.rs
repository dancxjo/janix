// Copied from abi to avoid dependencies

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SymbolId(pub u64);

pub const fn sym(s: &str) -> SymbolId {
    let mut hash = 0xcbf29ce484222325;
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x1099511628211999);
        i += 1;
    }
    SymbolId(hash)
}

#[repr(C, packed)]
pub struct DriverDescriptor {
    pub name: SymbolId,
    pub lane: SymbolId,
    pub provides_len: u16,
    pub provides_ptr: *const DriverProvides, // Changed to pointer
    pub requires_len: u16,
    pub requires_ptr: u64,
    pub claims_len: u16,
    pub claims_ptr: u64,
    pub abi_version: u16,
}

// Sync is required for static pointers
unsafe impl Sync for DriverDescriptor {}

#[repr(C)]
pub struct DriverProvides {
    pub iface: SymbolId,
    pub ver: u16,
    pub instance: SymbolId,
}

#[repr(C)]
pub struct DriverContext {
    pub register: unsafe extern "C" fn(iface: SymbolId, ver: u16, instance: SymbolId) -> i32,
    pub log: unsafe extern "C" fn(ptr: *const u8, len: usize),
    pub get_boot_info: unsafe extern "C" fn(ptr: *mut DriverBootInfo) -> i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DriverBootInfo {
    pub fb_addr: u64,
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_stride: u32,
    pub fb_format: u32,
    pub fb_size: u64,
}

pub const IFACE_FRAMEBUFFER: &str = "machine.framebuffer";
