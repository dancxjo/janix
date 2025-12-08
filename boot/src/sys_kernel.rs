use abi::{KernelRequest, KernelResponse};
use userland_rt::Sys;
use kernel_core;

/// Kernel-side implementation of the Sys trait.
/// SAFETY: called only from single-threaded kernel context.
pub struct KernelSys;

impl Sys for KernelSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        kernel_core::handle_request(request)
    }
}
