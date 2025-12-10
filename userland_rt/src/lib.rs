#![cfg_attr(target_os = "none", no_std)]
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

    fn yield_now(&mut self) {
        #[cfg(not(target_os = "none"))]
        {
            // In hosted mode, we simulate a syscall.
            // We need to yield execution back to the scheduler.
            // Since we are likely running in a thread (or need to be), we can park or panic.
            // For now, we'll assume the harness handles this via a thread-local or global mechanism
            // that we can trigger.
            // But wait, we can just call the kernel function directly?
            kernel_core::sched::yield_current_thread();

            // Now we need to actually stop execution.
            // If we are using threads, we park.
            std::thread::park();
        }
    }

    fn exit_thread(&mut self) -> ! {
        #[cfg(not(target_os = "none"))]
        {
            kernel_core::sched::exit_current_thread();
            // Stop execution
            std::thread::park();
            loop {}
        }
        #[cfg(target_os = "none")]
        loop {}
    }

    fn time_now_ns(&mut self) -> u64 {
        #[cfg(not(target_os = "none"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        }
        #[cfg(target_os = "none")]
        0
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn time_system_ns(&mut self) -> u64 {
        #[cfg(not(target_os = "none"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        }
        #[cfg(target_os = "none")]
        0
    }

    fn sleep_for_ns(&mut self, delta_ns: u64) {
        #[cfg(not(target_os = "none"))]
        {
            std::thread::sleep(std::time::Duration::from_nanos(delta_ns));
        }
    }

    fn sleep_until_ns(&mut self, deadline_ns: u64) {
        #[cfg(not(target_os = "none"))]
        {
            use std::thread;
            use std::time::{Duration, SystemTime, UNIX_EPOCH};
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
            if deadline_ns > now {
                thread::sleep(Duration::from_nanos(deadline_ns - now));
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

    fn time_now_ns(&mut self) -> u64 {
        kernel_core::time::monotonic_now_ns()
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        kernel_core::time::monotonic_now_ns()
    }

    fn time_system_ns(&mut self) -> u64 {
        kernel_core::time::system_time_ns().unwrap_or(0)
    }

    fn sleep_for_ns(&mut self, delta_ns: u64) {
        let start = kernel_core::time::monotonic_now_ns();
        while kernel_core::time::monotonic_now_ns() < start + delta_ns {
            core::hint::spin_loop();
        }
    }

    fn sleep_until_ns(&mut self, deadline_ns: u64) {
        while kernel_core::time::monotonic_now_ns() < deadline_ns {
            core::hint::spin_loop();
        }
    }

    fn yield_now(&mut self) {
        kernel_core::sched::yield_current_thread();
    }

    fn exit_thread(&mut self) -> ! {
        kernel_core::sched::exit_current_thread();
        loop {}
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

mod sys;

pub use sys::UserlandSys;
