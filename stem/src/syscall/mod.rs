mod arch;
pub mod graph;

pub use abi::syscall::*;
use abi::errors::{Errno, SysResult};
use abi::device::RtcTime;

use arch::raw_syscall6;

/// Helper to expose raw syscalls safely to other modules if needed.
#[inline(always)]
pub unsafe fn syscall6(n: u32, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
    unsafe { raw_syscall6(n, a0, a1, a2, a3, a4, a5) }
}

// Low-level wrappers

pub fn exit(code: i32) -> ! {
    unsafe {
        raw_syscall6(SYS_EXIT, code as usize, 0, 0, 0, 0, 0);
        core::hint::unreachable_unchecked();
    }
}

pub fn log_write(msg: &str) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_LOG_WRITE,
            msg.as_ptr() as usize,
            msg.len(),
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub use log_write as debug_write;

pub fn yield_now() {
    unsafe {
        raw_syscall6(SYS_YIELD, 0, 0, 0, 0, 0, 0);
    }
}

pub fn sleep_ns(ns: u64) {
    unsafe {
        raw_syscall6(SYS_SLEEP_NS, ns as usize, 0, 0, 0, 0, 0);
    }
}

pub fn sleep_ms(ms: u64) {
    // Legacy support, or use ns
    sleep_ns(ms * 1_000_000);
}

pub fn monotonic_ns() -> u64 {
    let ret = unsafe {
        raw_syscall6(SYS_TIME_MONOTONIC, 0, 0, 0, 0, 0, 0)
    };
    if ret < 0 { 0 } else { ret as u64 }
}

pub fn rtc_read(out: &mut RtcTime) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_RTC_READ,
            out as *mut _ as usize,
            0,
            0,
            0,
            0,
            0,
        )
    };
    if ret < 0 {
        abi::errors::errno(ret).map(|_| ())
    } else {
        Ok(())
    }
}

pub fn spawn_process(name: &str, arg: usize) -> Result<u64, abi::errors::Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_SPAWN_PROCESS,
            name.as_ptr() as usize,
            name.len(),
            arg,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as u64)
}

pub fn spawn_thread(entry: extern "C" fn() -> !, stack_top: usize) -> Result<u64, Errno> {
    let entry_addr = entry as usize;
    let ret = unsafe {
        raw_syscall6(SYS_SPAWN_THREAD, entry_addr, stack_top, 0, 0, 0, 0)
    };
    abi::errors::errno(ret).map(|v| v as u64)
}
