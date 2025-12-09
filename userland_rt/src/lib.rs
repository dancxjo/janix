#![cfg_attr(target_os = "none", no_std)]

use abi::{KernelRequest, KernelResponse, SyscallNumber};

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

// Ring3Sys: used by userland apps running in Ring 3
pub struct Ring3Sys;

impl Sys for Ring3Sys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        match request {
            KernelRequest::Log { message } => {
                let ptr = message.as_ptr() as u64;
                let len = message.len() as u64;
                unsafe { syscall_stub(SyscallNumber::Log, ptr, len, 0, 0, 0, 0) };
                KernelResponse::Success { data: None }
            }
            KernelRequest::ExitThread => {
                self.exit_thread();
            }
            KernelRequest::SchedulerTick => {
                unsafe { syscall_stub(SyscallNumber::Yield, 0, 0, 0, 0, 0, 0) };
                KernelResponse::Success { data: None }
            }
            KernelRequest::AllocFrame { pool_index } => {
                let mut frame = abi::FrameInfo {
                    id: abi::FrameId(0),
                    base: 0,
                    size: 0,
                };
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::AllocFrame,
                        pool_index as u64,
                        &mut frame as *mut _ as u64,
                        0,
                        0,
                        0,
                        0,
                    )
                };
                if ret == 0 {
                    KernelResponse::FrameAllocated { frame }
                } else {
                    KernelResponse::Error {
                        message: "AllocFrame failed",
                    }
                }
            }
            KernelRequest::FreeFrame { frame_id } => {
                let ret =
                    unsafe { syscall_stub(SyscallNumber::FreeFrame, frame_id.0, 0, 0, 0, 0, 0) };
                if ret == 0 {
                    KernelResponse::FrameFreed { frame_id }
                } else {
                    KernelResponse::Error {
                        message: "FreeFrame failed",
                    }
                }
            }
            KernelRequest::CreateProcess { pid } => {
                let ret = unsafe { syscall_stub(SyscallNumber::CreateProcess, pid, 0, 0, 0, 0, 0) };
                if ret == 0 {
                    KernelResponse::ProcessCreated { pid }
                } else {
                    KernelResponse::Error {
                        message: "CreateProcess failed",
                    }
                }
            }
            KernelRequest::CreateThread { pid, tid, priority } => {
                let ret = unsafe {
                    syscall_stub(SyscallNumber::CreateThread, pid, tid, priority, 0, 0, 0)
                };
                if ret == 0 {
                    KernelResponse::ThreadCreated { tid }
                } else {
                    KernelResponse::Error {
                        message: "CreateThread failed",
                    }
                }
            }
            KernelRequest::ThingCreate { kind, props } => {
                let kind_ptr = kind.as_ptr() as u64;
                let kind_len = kind.len() as u64;
                let props_ptr = props.as_ptr() as u64;
                let props_len = props.len() as u64;
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::ThingCreate,
                        kind_ptr,
                        kind_len,
                        props_ptr,
                        props_len,
                        0,
                        0,
                    )
                };
                KernelResponse::ThingCreated {
                    id: abi::ThingId(ret),
                }
            }
            KernelRequest::SchemaRegister { kind, props } => {
                let kind_ptr = kind.as_ptr() as u64;
                let kind_len = kind.len() as u64;
                let props_ptr = props.as_ptr() as u64;
                let props_len = props.len() as u64;
                let ret = unsafe {
                    syscall_stub(
                        SyscallNumber::SchemaRegister,
                        kind_ptr,
                        kind_len,
                        props_ptr,
                        props_len,
                        0,
                        0,
                    )
                };
                if ret == 0 {
                    KernelResponse::SchemaRegistered { kind }
                } else {
                    KernelResponse::Error {
                        message: "SchemaRegister failed",
                    }
                }
            }
            _ => KernelResponse::Error {
                message: "Syscall not implemented yet",
            },
        }
    }

    fn time_now_ns(&mut self) -> u64 {
        unsafe { syscall_stub(SyscallNumber::TimeNow, 0, 0, 0, 0, 0, 0) }
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        unsafe { syscall_stub(SyscallNumber::TimeMonotonicNs, 0, 0, 0, 0, 0, 0) }
    }

    fn time_system_ns(&mut self) -> u64 {
        unsafe { syscall_stub(SyscallNumber::TimeSystemNs, 0, 0, 0, 0, 0, 0) }
    }

    fn sleep_for_ns(&mut self, delta_ns: u64) {
        unsafe { syscall_stub(SyscallNumber::SleepForNs, delta_ns, 0, 0, 0, 0, 0) };
    }

    fn sleep_until_ns(&mut self, deadline_ns: u64) {
        unsafe { syscall_stub(SyscallNumber::SleepUntil, deadline_ns, 0, 0, 0, 0, 0) };
    }

    fn yield_now(&mut self) {
        unsafe { syscall_stub(SyscallNumber::Yield, 0, 0, 0, 0, 0, 0) };
    }

    fn exit_thread(&mut self) -> ! {
        unsafe { syscall_stub(SyscallNumber::ExitThread, 0, 0, 0, 0, 0, 0) };
        loop {}
    }
}

impl Ring3Sys {
    pub fn new() -> Self {
        Ring3Sys
    }

    pub fn exit_thread(&self) -> ! {
        unsafe {
            syscall_stub(SyscallNumber::ExitThread, 0, 0, 0, 0, 0, 0);
        }
        loop {}
    }
}

#[inline(always)]
unsafe fn syscall_stub(
    num: SyscallNumber,
    arg0: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> u64 {
    let mut ret: u64;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "int 0x80",
            inlateout("rax") num as u64 => ret,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            in("rcx") arg3,
            in("r8") arg4,
            in("r9") arg5,
            options(nostack, preserves_flags),
        );
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "svc #0",
            inlateout("x8") num as u64 => ret,
            inlateout("x0") arg0 => ret,
            in("x1") arg1,
            in("x2") arg2,
            in("x3") arg3,
            in("x4") arg4,
            in("x5") arg5,
            options(nostack, preserves_flags),
        );
    }

    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a7") num as u64 => ret,
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            options(nostack, preserves_flags),
        );
    }

    #[cfg(target_arch = "loongarch64")]
    unsafe {
        core::arch::asm!(
            "syscall 0",
            inlateout("$a7") num as u64 => ret,
            inlateout("$a0") arg0 => ret,
            in("$a1") arg1,
            in("$a2") arg2,
            in("$a3") arg3,
            in("$a4") arg4,
            in("$a5") arg5,
            options(nostack, preserves_flags),
        );
    }

    ret
}
