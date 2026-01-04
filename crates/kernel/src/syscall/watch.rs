//! Watch Syscalls

use abi::syscall::err;
use abi::wire::SyscallResult;
use core::sync::atomic::{AtomicU64, Ordering};

static NEXT_WATCH_ID: AtomicU64 = AtomicU64::new(1);

pub fn sys_watch_create(_target_low: u64, _flags: u64) -> SyscallResult {
    let id = NEXT_WATCH_ID.fetch_add(1, Ordering::Relaxed);
    SyscallResult::new(0, id, 0)
}

pub fn sys_watch_poll(_watch_id: u64, _out_ptr: u64, _len: u64) -> SyscallResult {
    // Event buffering not yet implemented; return EAGAIN so callers can fallback.
    SyscallResult::new(err::EAGAIN, 0, 0)
}
