use crate::bridge::ProviderBridge;
use crate::machine::{MachineError, ProviderMeta, ProviderVtable};
use abi::symbols::sym;
use abi::wire::machine::{
    FbGetInfoResp, FbPresentReq, FbPresentResp, OP_FB_GET_INFO, OP_FB_PRESENT,
};
use alloc::vec::Vec;

pub struct LimineFramebufferProvider {
    pub info: FbGetInfoResp,
}

impl LimineFramebufferProvider {
    pub const META: ProviderMeta = ProviderMeta {
        name: sym("driver.limine_fb"),
        kind: sym("sys.driver"),
        lane: sym("direct"),
    };

    pub fn vtable<B: ProviderBridge + ?Sized>() -> ProviderVtable<B> {
        ProviderVtable { call: Self::call::<B> }
    }

    pub fn new(info: FbGetInfoResp) -> Self {
        Self { info }
    }

    fn call<B: ProviderBridge + ?Sized>(
        ctx: *const (),
        _bridge: &B,
        op: u32,
        req: &[u8],
    ) -> Result<Vec<u8>, MachineError> {
        let provider = unsafe { &*(ctx as *const LimineFramebufferProvider) };

        match op {
            OP_FB_GET_INFO => {
                postcard::to_allocvec(&provider.info).map_err(|_| MachineError::InternalError)
            }
            OP_FB_PRESENT => {
                let _ = postcard::from_bytes::<FbPresentReq>(req)
                    .map_err(|_| MachineError::EncodingError)?;
                let resp = FbPresentResp {};
                postcard::to_allocvec(&resp).map_err(|_| MachineError::InternalError)
            }
            _ => Err(MachineError::InvalidOp),
        }
    }
}
