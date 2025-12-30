use crate::bridge::HardwareBridge;
use super::{MachineError, ProviderMeta};
use abi::wire::machine::*;
use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub enum BuiltinEndpoint {
    Rtc,
    Framebuffer,
    Keyboard,
    Mouse,
}

pub fn dispatch<B: HardwareBridge>(
    bridge: &B,
    ep: BuiltinEndpoint,
    op: u32,
    req: &[u8],
) -> Result<Vec<u8>, MachineError> {
    match ep {
        BuiltinEndpoint::Rtc => dispatch_rtc(bridge, op, req),
        BuiltinEndpoint::Framebuffer => dispatch_fb(op, req),
        BuiltinEndpoint::Keyboard => dispatch_kbd(bridge, op, req),
        BuiltinEndpoint::Mouse => dispatch_mouse(bridge, op, req),
    }
}

pub fn meta_for(_ep: BuiltinEndpoint) -> ProviderMeta {
    use abi::symbols::sym;
    ProviderMeta {
        name: sym("driver.builtin"),
        kind: sym("sys.driver"),
        lane: sym("direct"),
    }
}

fn dispatch_rtc<B: HardwareBridge>(bridge: &B, op: u32, _req: &[u8]) -> Result<Vec<u8>, MachineError> {
    if op == OP_RTC_NOW_NS {
        let ns = bridge.monotonic_now();
        let resp = RtcNowResp { system_ns: ns };
        postcard::to_allocvec(&resp).map_err(|_| MachineError::InternalError)
    } else {
        Err(MachineError::InvalidOp)
    }
}

fn dispatch_fb(op: u32, _req: &[u8]) -> Result<Vec<u8>, MachineError> {
    if op == OP_FB_GET_INFO {
        Err(MachineError::NotFound)
    } else if op == OP_FB_PRESENT {
        let resp = FbPresentResp {};
        postcard::to_allocvec(&resp).map_err(|_| MachineError::InternalError)
    } else {
        Err(MachineError::InvalidOp)
    }
}

fn dispatch_kbd<B: HardwareBridge>(bridge: &B, op: u32, req: &[u8]) -> Result<Vec<u8>, MachineError> {
    if op == OP_KBD_READ_EVENTS {
        let req: KbdReadReq = postcard::from_bytes(req).map_err(|_| MachineError::EncodingError)?;
        let mut events = Vec::new();
        for _ in 0..req.max {
             if let Some(ev) = crate::input::try_pop_keyboard(bridge) {
                 events.push(KeyEvent { scancode: ev, pressed: (ev & 0x80) == 0 });
             } else {
                 break;
             }
        }
        let resp = KbdReadResp { events };
        postcard::to_allocvec(&resp).map_err(|_| MachineError::InternalError)
    } else {
        Err(MachineError::InvalidOp)
    }
}

fn dispatch_mouse<B: HardwareBridge>(bridge: &B, op: u32, req: &[u8]) -> Result<Vec<u8>, MachineError> {
    if op == OP_MOUSE_READ_EVENTS {
        let req: MouseReadReq = postcard::from_bytes(req).map_err(|_| MachineError::EncodingError)?;
         let mut events = Vec::new();
        for _ in 0..req.max {
             if let Some(ev) = crate::input::try_pop_mouse(bridge) {
                 events.push(MouseEvent { byte: ev });
             } else {
                 break;
             }
        }
        let resp = MouseReadResp { events };
         postcard::to_allocvec(&resp).map_err(|_| MachineError::InternalError)
    } else {
         Err(MachineError::InvalidOp)
    }
}
