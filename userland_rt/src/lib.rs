#![cfg_attr(target_os = "none", no_std)]

use abi::{KernelRequest, KernelResponse};

pub trait Sys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse;
}

// Existing HostedSys for host_harness (may be cfg(std) or cfg(feature = "host"))
#[cfg(feature = "host")]
pub struct HostedSys;

#[cfg(feature = "host")]
impl Sys for HostedSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        // existing hosted behavior (possibly forwarding to kernel_core in tests)
        #[cfg(not(target_os = "none"))]
        {
            // For Log requests in hosted mode, print to stdout before handling
            if let KernelRequest::Log { message } = &request {
                println!("{}", message);
            }
            kernel_core::handle_request(request)
        }
        #[cfg(target_os = "none")]
        {
            // Should not happen if feature=host is only used on host
            KernelResponse::Error {
                message: "HostedSys not supported on bare metal",
            }
        }
    }
}

// New KernelSys: used inside the real kernel build
#[cfg(any(target_os = "none", feature = "kernel"))]
pub struct KernelSys;

#[cfg(any(target_os = "none", feature = "kernel"))]
impl Sys for KernelSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        // In-kernel, we just call into kernel_core
        kernel_core::handle_request(request)
    }
}

// Convenience getters so call sites don’t have to worry about cfgs:
#[cfg(feature = "host")]
pub fn get_host_sys() -> HostedSys {
    HostedSys
}

#[cfg(any(target_os = "none", feature = "kernel"))]
pub fn get_kernel_sys() -> KernelSys {
    KernelSys
}
