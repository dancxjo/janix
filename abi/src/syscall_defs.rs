#![no_std]

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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UserPtr<T> {
    pub addr: u64, // user virtual address
    pub _phantom: PhantomData<T>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UserSlice<T> {
    pub ptr: UserPtr<T>,
    pub len: u64, // number of T elements
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceHandle {
    pub raw: u32,
}

impl Default for DeviceHandle {
    fn default() -> Self {
        Self { raw: 0 }
    }
}

/// Enumerated device kinds. No strings across ABI.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceKind {
    Ps2Keyboard = 1,
    Ps2Mouse = 2,
    // Future: SerialRx, VirtioNetRx, etc.
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DevOpenArgs {
    pub kind: u32,  // DeviceKind
    pub index: u32, // for multi-instance devices (0 for now)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DevOpenRet {
    pub handle: DeviceHandle,
}

impl Default for DevOpenRet {
    fn default() -> Self {
        Self {
            handle: DeviceHandle { raw: 0 },
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DevReadArgs {
    pub handle: DeviceHandle,
    pub out: UserSlice<u8>, // buffer in user memory
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DevReadRet {
    pub bytes_read: u32, // 0..out.len
}

impl Default for DevReadRet {
    fn default() -> Self {
        Self { bytes_read: 0 }
    }
}
