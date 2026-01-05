//! Minimal threading API for thing-os userspace.
//!
//! Provides basic thread spawning, joining, and blocking primitives.

use crate::{syscall, ThingId};
use abi::syscall::nr;

/// Handle to a spawned thread.
#[derive(Debug, Clone, Copy)]
pub struct ThreadHandle {
    tid: u64,
}

impl ThreadHandle {
    /// Get the raw thread ID.
    pub fn tid(&self) -> u64 {
        self.tid
    }
}

/// Spawn a new thread in the current address space.
///
/// The entry function receives the argument and should never return
/// (it must call `thread_exit` or diverge).
///
/// # Arguments
/// * `entry` - Entry point function with signature `extern "C" fn(u64) -> !`
/// * `arg` - Argument passed to the thread via register
///
/// # Returns
/// A `ThreadHandle` that can be used to join the thread.
pub fn thread_spawn(entry: extern "C" fn(u64) -> !, arg: u64) -> ThreadHandle {
    let res = unsafe {
        syscall(nr::SYS_THREAD_SPAWN, entry as u64, arg, 0, 0, 0, 0)
    };
    ThreadHandle { tid: res.val0 }
}

/// Spawn a thread with a custom stack pointer.
pub fn thread_spawn_with_stack(entry: extern "C" fn(u64) -> !, arg: u64, stack_ptr: u64) -> ThreadHandle {
    let res = unsafe {
        syscall(nr::SYS_THREAD_SPAWN, entry as u64, arg, stack_ptr, 0, 0, 0)
    };
    ThreadHandle { tid: res.val0 }
}

/// Wait for a thread to exit and return its exit code.
pub fn join(handle: ThreadHandle) -> i32 {
    let res = unsafe {
        syscall(nr::SYS_THREAD_JOIN, handle.tid, 0, 0, 0, 0, 0)
    };
    if res.status == 0 {
        res.val0 as i32
    } else {
        -1 // Error case
    }
}

/// Wait for a thread with a timeout (in scheduler ticks).
///
/// Returns `Some(exit_code)` if the thread exited, `None` if timeout expired.
pub fn join_timeout(handle: ThreadHandle, timeout_ticks: u64) -> Option<i32> {
    let res = unsafe {
        syscall(nr::SYS_THREAD_JOIN, handle.tid, timeout_ticks, 0, 0, 0, 0)
    };
    if res.status == 0 {
        Some(res.val0 as i32)
    } else {
        None
    }
}

/// Exit the current thread with the given exit code.
///
/// This wakes any threads waiting to join this thread.
pub fn thread_exit(code: i32) -> ! {
    unsafe {
        syscall(nr::SYS_THREAD_EXIT, code as u64, 0, 0, 0, 0, 0);
    }
    // Should never reach here
    loop {}
}

/// Block the current thread until a watch event arrives.
///
/// This is the primary mechanism for efficient event-driven threading
/// without spinning.
pub fn sleep_on_watch(watch_id: u64) {
    unsafe {
        syscall(nr::SYS_THREAD_BLOCK_ON_WATCH, watch_id, 0, 0, 0, 0, 0);
    }
}

/// Yield the current thread's time slice to other runnable threads.
pub fn yield_now() {
    crate::sched_yield();
}
