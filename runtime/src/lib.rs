#![cfg_attr(any(target_os = "none", feature = "kernel"), no_std)]
#![cfg_attr(
    all(target_os = "none", not(feature = "kernel")),
    feature(alloc_error_handler)
)]

use abi::{KernelRequest, KernelResponse};

#[cfg(all(target_os = "none", not(feature = "kernel")))]
mod heap;

#[cfg(all(target_os = "none", not(feature = "kernel")))]
pub use heap::init_user_heap;

#[cfg(not(all(target_os = "none", not(feature = "kernel"))))]
pub fn init_user_heap() {}

pub trait Sys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse;

    /// Monotonic time, in ns since boot.
    fn time_now_ns(&mut self) -> u64;

    /// Optional: sleep until a monotonic deadline.
    fn sleep_until_ns(&mut self, deadline_ns: u64);

    fn time_monotonic_ns(&mut self) -> u64;
    fn time_system_ns(&mut self) -> u64;

    fn sleep_for_ns(&mut self, delta_ns: u64);

    fn yield_now(&mut self);
    fn exit_thread(&mut self) -> !;
}

// New KernelSys: used inside the real kernel build
#[cfg(any(target_os = "none", feature = "kernel"))]
pub struct KernelSys;

#[cfg(any(target_os = "none", feature = "kernel"))]
impl Sys for KernelSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        // In-kernel, we just call into kernel
        kernel::handle_request(request)
    }

    fn time_now_ns(&mut self) -> u64 {
        kernel::time::monotonic_now_ns()
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        kernel::time::monotonic_now_ns()
    }

    fn time_system_ns(&mut self) -> u64 {
        kernel::time::system_time_ns().unwrap_or(0)
    }

    fn sleep_for_ns(&mut self, delta_ns: u64) {
        let start = kernel::time::monotonic_now_ns();
        while kernel::time::monotonic_now_ns() < start + delta_ns {
            core::hint::spin_loop();
        }
    }

    fn sleep_until_ns(&mut self, deadline_ns: u64) {
        while kernel::time::monotonic_now_ns() < deadline_ns {
            core::hint::spin_loop();
        }
    }

    fn yield_now(&mut self) {
        kernel::sched::yield_current_thread();
    }

    fn exit_thread(&mut self) -> ! {
        kernel::sched::exit_current_thread("runtime_exit", 0);
        loop {}
    }
}

// Convenience getters so call sites don’t have to worry about cfgs:
#[cfg(any(target_os = "none", feature = "kernel"))]
pub fn get_kernel_sys() -> KernelSys {
    KernelSys
}

mod sys;

pub use sys::UserlandSys;
