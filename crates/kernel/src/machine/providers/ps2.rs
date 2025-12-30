use crate::bridge::ProviderBridge;
use crate::machine::{MachineError, ProviderMeta, ProviderVtable};
use abi::symbols::sym;
use abi::wire::machine::{
    KbdReadReq, KbdReadResp, KeyEvent, MouseEvent, MouseReadReq, MouseReadResp, OP_KBD_READ_EVENTS,
    OP_MOUSE_READ_EVENTS,
};
use alloc::vec::Vec;

pub struct Ps2Provider;

impl Ps2Provider {
    pub const META: ProviderMeta = ProviderMeta {
        name: sym("driver.ps2_controller"),
        kind: sym("sys.driver"),
        lane: sym("direct"),
    };

    pub const VTABLE: ProviderVtable = ProviderVtable { call: Self::call };

    pub fn new() -> Self {
        Self
    }

    fn call(
        _ctx: *const (),
        bridge: &dyn ProviderBridge,
        op: u32,
        req: &[u8],
    ) -> Result<Vec<u8>, MachineError> {
        match op {
            OP_KBD_READ_EVENTS => {
                let req: KbdReadReq =
                    postcard::from_bytes(req).map_err(|_| MachineError::EncodingError)?;
                let mut events = Vec::new();
                for _ in 0..req.max {
                    if let Some(ev) = crate::input::try_pop_keyboard(bridge) {
                        events.push(KeyEvent {
                            scancode: ev,
                            pressed: (ev & 0x80) == 0,
                        });
                    } else {
                        break;
                    }
                }
                let resp = KbdReadResp { events };
                postcard::to_allocvec(&resp).map_err(|_| MachineError::InternalError)
            }
            OP_MOUSE_READ_EVENTS => {
                let req: MouseReadReq =
                    postcard::from_bytes(req).map_err(|_| MachineError::EncodingError)?;
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
            }
            _ => Err(MachineError::InvalidOp),
        }
    }
}
