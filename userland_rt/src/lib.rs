#![cfg_attr(target_os = "none", no_std)]

use abi::{KernelRequest, KernelResponse};

/// System call interface trait
pub trait Sys {
    /// Make a system call to the kernel
    fn syscall(&self, request: KernelRequest) -> KernelResponse;
}

/// Hosted implementation of Sys trait (stub for testing on host)
///
/// IMPORTANT: This implementation must maintain strict parity with the kernel's `KernelSys`.
/// Any syscall supported by the kernel must be supported here (typically by delegating to `kernel_core`).
pub struct HostedSys;

impl Sys for HostedSys {
    fn syscall(&self, _request: KernelRequest) -> KernelResponse {
        // In hosted mode, we delegate directly to kernel_core
        #[cfg(not(target_os = "none"))]
        {
            // For Log requests in hosted mode, print to stdout before handling
            if let KernelRequest::Log { message } = &_request {
                println!("{}", message);
            }

            kernel_core::handle_request(_request)
        }

        // In bare-metal mode, this would use actual syscall mechanism
        #[cfg(target_os = "none")]
        {
            // Placeholder: This would be the actual syscall instruction
            KernelResponse::Error {
                message: "Syscall not implemented for bare metal",
            }
        }
    }
}

/// Get the system call interface
pub fn get_sys() -> &'static dyn Sys {
    &HostedSys
}
