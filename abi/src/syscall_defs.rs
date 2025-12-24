use core::marker::PhantomData;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SysError {
    pub code: u32,
    pub detail: u32,
}

// Keep these codes stable and documented.
impl SysError {
    pub const OK: u32 = 0;
    pub const INVALID_ARG: u32 = 1;
    pub const NOT_FOUND: u32 = 2;
    pub const PERMISSION: u32 = 3;
    pub const BAD_HANDLE: u32 = 4;
    pub const BUFFER_TOO_SMALL: u32 = 5;
    pub const WOULD_BLOCK: u32 = 6;
    pub const INTERNAL: u32 = 7;
    pub const NOT_IMPLEMENTED: u32 = 8;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SysRet<T> {
    pub ok: u32,       // 1 = success, 0 = error
    pub val: T,        // valid if ok=1
    pub err: SysError, // valid if ok=0
}

impl<T: Default> SysRet<T> {
    pub fn ok(val: T) -> Self {
        Self {
            ok: 1,
            val,
            err: SysError { code: 0, detail: 0 },
        }
    }
    pub fn err(code: u32, detail: u32) -> Self {
        Self {
            ok: 0,
            val: T::default(),
            err: SysError { code, detail },
        }
    }
}

pub use crate::wire::common::{UserPtr, UserSlice};

pub use crate::wire::dev::{
    DevOpenArgs, DevOpenRet, DevReadArgs, DevReadRet, DeviceHandle, DeviceKind,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolId(pub u32);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WireStr {
    pub ptr: u64,
    pub len: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SymbolInternReq {
    pub s: WireStr,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SymbolInternResp {
    pub id: SymbolId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SymbolResolveReq {
    pub id: SymbolId,
    pub out_ptr: u64,
    pub out_cap: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SymbolResolveResp {
    pub written: u64, // bytes written
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PciReadConfigArgs {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
    pub offset: u16,
    pub width: u8, // 1, 2, or 4
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PciReadConfigRet {
    pub value: u32,
}

pub const SYSCALL_WATCH_OPEN: u64 = 600;
pub const SYSCALL_WATCH_NEXT: u64 = 601;
pub const SYSCALL_WATCH_CLOSE: u64 = 602;
