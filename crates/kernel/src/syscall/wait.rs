//! Wait syscall: block current task until a watch fires or timeout elapses.

use alloc::vec::Vec;
use core::sync::atomic::Ordering;

use abi::ids::WatchId;
use abi::syscall::err;
use abi::types::WaitFlags;
use abi::wire::SyscallResult;

use crate::sched::{self, BlockReason};
use crate::syscall::user_mem;
use abi::cap::CapOp;

pub fn sys_wait(
    watches_ptr: u64,
    watch_len: u64,
    flags_raw: u64,
    timeout_ticks: u64,
) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::GraphWatch, None) {
        return SyscallResult::new(code, 0, 0);
    }

    // 1. Validate inputs
    let count = match usize::try_from(watch_len) {
        Ok(len) => len,
        Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
    };
    if watches_ptr == 0 && count > 0 {
        return SyscallResult::new(err::EFAULT, 0, 0);
    }

    let flags = WaitFlags::from_bits(flags_raw as u32).unwrap_or(WaitFlags::WAIT_ANY);
    let watches = if count == 0 {
        Vec::new()
    } else {
        let byte_len = match count.checked_mul(core::mem::size_of::<u64>()) {
            Some(len) => len,
            None => return SyscallResult::new(err::EINVAL, 0, 0),
        };
        let mut raw = alloc::vec![0u64; count];
        let raw_bytes = unsafe {
            core::slice::from_raw_parts_mut(raw.as_mut_ptr() as *mut u8, byte_len)
        };
        if let Err(code) = user_mem::copy_from_user(raw_bytes, watches_ptr, byte_len) {
            return SyscallResult::new(code, 0, 0);
        }
        raw.into_iter().map(WatchId).collect::<Vec<_>>()
    };

    // 2. Compute timeout absolute tick
    let timeout_at = if timeout_ticks == 0 {
        None
    } else {
        Some(
            sched::TIMER_TICKS
                .load(Ordering::Relaxed)
                .saturating_add(timeout_ticks),
        )
    };

    // 3. Register wait and block current task
    let Some(task_id) = sched::current_task_handle() else {
        return SyscallResult::new(err::EFAULT, 0, 0);
    };

    crate::watch::register_wait(task_id, &watches, flags, timeout_at);
    sched::block_current(BlockReason::WatchWait);

    // 4. Suspend until a wake reason is delivered
    loop {
        if let Some(wake_reason) = sched::take_wake_reason() {
            // Registry should already be cleaned by the wake path; unregister defensively.
            crate::watch::unregister_wait(task_id);
            let reason = wake_reason.reason as u64;
            let which = wake_reason.which.0;
            return SyscallResult::new(0, reason, which);
        }
        crate::machine::idle();
    }
}
