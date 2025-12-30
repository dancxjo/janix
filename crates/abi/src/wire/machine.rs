use serde::{Deserialize, Serialize};
use alloc::vec::Vec;

// Interface IDs
pub const IFACE_RTC: &str = "machine.rtc";
pub const IFACE_FRAMEBUFFER: &str = "machine.framebuffer";
pub const IFACE_KEYBOARD: &str = "machine.input.keyboard";
pub const IFACE_MOUSE: &str = "machine.input.mouse";

// Op IDs
pub const OP_RTC_NOW_NS: u32 = 0x0101;
pub const OP_KBD_READ_EVENTS: u32 = 0x0201;
pub const OP_MOUSE_READ_EVENTS: u32 = 0x0202;
pub const OP_FB_GET_INFO: u32 = 0x0301;
pub const OP_FB_PRESENT: u32 = 0x0302;

#[derive(Serialize, Deserialize, Debug)]
pub struct RtcNowReq {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RtcNowResp {
    pub system_ns: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FbGetInfoReq {}

#[derive(Serialize, Deserialize, Debug)]
pub struct FbGetInfoResp {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub addr: u64,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FbPresentReq {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FbPresentResp {}

#[derive(Serialize, Deserialize, Debug)]
pub struct KbdReadReq {
    pub max: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyEvent {
    pub scancode: u8,
    pub pressed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KbdReadResp {
    pub events: Vec<KeyEvent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseReadReq {
    pub max: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseEvent {
    pub byte: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseReadResp {
    pub events: Vec<MouseEvent>,
}
