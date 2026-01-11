#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    RtcCmos = 1,
    Keyboard = 2,
    Mouse = 3,
    Framebuffer = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceHandle(pub u32);

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RootCaps;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceCall {
    pub kind: DeviceKind,
    pub op: u32,
    pub in_ptr: u64,
    pub in_len: u32,
    pub out_ptr: u64,
    pub out_len: u32,
}

// RTC OPs
pub const RTC_OP_READ_TIME: u32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RtcTime {
    pub year: u16,   // e.g. 2026
    pub month: u8,   // 1-12
    pub day: u8,     // 1-31
    pub hour: u8,    // 0-23
    pub minute: u8,  // 0-59
    pub second: u8,  // 0-59
    pub weekday: u8, // 0-6
    pub flags: u8,   // Status flags
}
