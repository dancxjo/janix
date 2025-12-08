#![no_std]

use abi::{KernelRequest, KernelResponse};

/// System call interface trait
pub trait Sys {
    /// Make a system call to the kernel
    fn syscall(&self, request: KernelRequest) -> KernelResponse;
}

/// Hosted implementation of Sys trait (stub for testing on host)
pub struct HostedSys;

impl Sys for HostedSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        // This is a stub implementation for hosted environments
        // In a real system, this would make actual system calls
        match request {
            KernelRequest::Log { message: _ } => {
                // On hosted, we can't actually log to kernel
                KernelResponse::Success { data: None }
            }
            KernelRequest::GraphQuery { node_id } => {
                // Return stub data
                KernelResponse::NodeData { node_id, value: node_id.0 * 10 }
            }
            KernelRequest::CreateTransaction => {
                KernelResponse::TransactionCreated { 
                    tx_id: abi::TransactionId(1) 
                }
            }
            KernelRequest::CommitTransaction { tx_id: _ } => {
                KernelResponse::Success { data: None }
            }
        }
    }
}

/// Get the system call interface
pub fn get_sys() -> &'static dyn Sys {
    &HostedSys
}
