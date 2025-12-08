use abi::{KernelRequest, KernelResponse};
use kernel_core;
use userland_rt::Sys;

/// Kernel-side implementation of the Sys trait.
/// SAFETY: called only from single-threaded kernel context.
///
/// IMPORTANT: Any functionality implemented here must also be mirrored in `userland_rt::HostedSys`
/// to ensure the host harness remains in sync with the kernel.
pub struct KernelSys;

impl Sys for KernelSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        // Mirror HostedSys behavior: print logs to console (framebuffer)
        if let KernelRequest::Log { message } = &request {
            crate::console::print(message);
            crate::console::print("\n");
        }

        kernel_core::handle_request(request)
    }
}
