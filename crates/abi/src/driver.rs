use crate::SymbolId;

#[repr(C, packed)]
pub struct DriverDescriptor {
    pub name: SymbolId,
    pub lane: SymbolId,
    pub provides_len: u16,
    pub provides_ptr: u64,
    pub requires_len: u16,
    pub requires_ptr: u64,
    pub claims_len: u16,
    pub claims_ptr: u64,
    pub abi_version: u16,
}

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
