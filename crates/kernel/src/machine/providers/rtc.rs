use crate::bridge::ProviderBridge;
use crate::machine::{MachineError, ProviderMeta, ProviderVtable};
use abi::symbols::sym;
use abi::wire::machine::{RtcNowReq, RtcNowResp, OP_RTC_NOW_NS};
use alloc::vec::Vec;

pub struct RtcProvider;

impl RtcProvider {
    pub const META: ProviderMeta = ProviderMeta {
        name: sym("driver.rtc"),
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
            OP_RTC_NOW_NS => {
                let _ = postcard::from_bytes::<RtcNowReq>(req)
                    .map_err(|_| MachineError::EncodingError)?;
                let ns = bridge.monotonic_now();
                let resp = RtcNowResp { system_ns: ns };
                postcard::to_allocvec(&resp).map_err(|_| MachineError::InternalError)
            }
            _ => Err(MachineError::InvalidOp),
        }
    }
}
