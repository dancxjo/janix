use crate::wire::common::UserSlice;

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
