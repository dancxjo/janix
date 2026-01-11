mod arch;

pub use abi::syscall::*;
use abi::errors::Errno;

use arch::raw_syscall6;

/// Helper to expose raw syscalls safely to other modules if needed (unlikely, but here for completeness).
#[inline(always)]
pub unsafe fn syscall6(n: u32, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
    // In Rust 2024 unsafe fn requires unsafe block for unsafe ops, but we switched to 2021.
    // However, it's good practice.
    // Since we are 2021 now, we don't strictly need the block if raw_syscall6 is unsafe and this is unsafe fn,
    // BUT the previous warning was complaining about it.
    // Wait, in 2021, `unsafe fn` implies the body is unsafe scope? No, that's pre-2024.
    // Actually, `unsafe_op_in_unsafe_fn` lint is enabled by default in 2024.
    // Let's just use the block to be forward compatible.
    unsafe { raw_syscall6(n, a0, a1, a2, a3, a4, a5) }
}

// Wrappers will go here in Phase 4

pub fn exit(code: i32) -> ! {
    unsafe {
        raw_syscall6(SYS_EXIT, code as usize, 0, 0, 0, 0, 0);
        core::hint::unreachable_unchecked();
    }
}

pub fn debug_write(msg: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_DEBUG_WRITE,
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

pub fn sleep_ms(ms: u64) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(SYS_SLEEP_MS, ms as usize, 0, 0, 0, 0, 0)
    };
    abi::errors::errno(ret).map(|_| ())
}

pub fn yield_now() -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(SYS_YIELD, 0, 0, 0, 0, 0, 0)
    };
    abi::errors::errno(ret).map(|_| ())
}

pub fn spawn_thread(entry: extern "C" fn() -> !, stack_top: usize) -> Result<u64, Errno> {
    let entry_addr = entry as usize;
    let ret = unsafe {
        raw_syscall6(SYS_SPAWN_THREAD, entry_addr, stack_top, 0, 0, 0, 0)
    };
    abi::errors::errno(ret).map(|v| v as u64)
}
