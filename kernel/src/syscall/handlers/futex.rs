//! Futex (fast userspace mutex) syscall handlers.
//!
//! Provides kernel-backed wait/wake on a userspace `u32` address.
//! This is the backbone for Rust std's sync primitives (Mutex, Condvar,
//! RwLock, thread parking).

use abi::errors::{Errno, SysResult};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use spin::Mutex;

use crate::syscall::validate::validate_user_range;

/// A futex wait queue entry: the thread ID that is blocked.
type TaskId = u64;

/// Key for the wait queue: (process address-space id, userspace address).
/// For now we use the raw virtual address since all threads in a process
/// share the same address space.
type FutexKey = usize;

/// Global futex wait-queue table.
static FUTEX_TABLE: Mutex<BTreeMap<FutexKey, Vec<TaskId>>> = Mutex::new(BTreeMap::new());

/// `sys_futex_wait(uaddr, expected, timeout_ns)`
///
/// If `*uaddr == expected`, block the current thread until woken or
/// until `timeout_ns` nanoseconds elapse (0 = wait forever).
///
/// Returns:
/// - `Ok(0)` on wake or spurious wakeup
/// - `Err(EAGAIN)` if `*uaddr != expected` (value changed)
/// - `Err(ETIMEDOUT)` on timeout
pub fn sys_futex_wait(uaddr: usize, expected: u32, timeout_ns: u64) -> SysResult<usize> {
    // Validate the userspace address is readable.
    validate_user_range(uaddr, 4, false)?;

    // Read the current value at the userspace address.
    let current_val = unsafe { core::ptr::read_volatile(uaddr as *const u32) };

    if current_val != expected {
        return Err(Errno::EAGAIN);
    }

    let tid = unsafe { crate::task::scheduler::current_tid_current() };

    // Add ourselves to the wait queue.
    {
        let mut table = FUTEX_TABLE.lock();
        table.entry(uaddr).or_insert_with(Vec::new).push(tid);
    }

    if timeout_ns == 0 {
        // Indefinite wait — block until woken.
        unsafe {
            crate::task::scheduler::block_current_erased();
        }
    } else {
        // Timed wait — use sleep, which blocks for up to the given duration.
        // The wake side will unblock us early if needed.
        // Convert ns to ticks (100Hz = 10ms per tick), rounding up.
        let ticks = (timeout_ns + 9_999_999) / 10_000_000;
        if ticks == 0 {
            unsafe {
                crate::task::scheduler::yield_now_current();
            }
        } else {
            crate::task::scheduler::sleep_ticks_current(ticks);
        }

        // After waking, remove ourselves from the wait queue if still there
        // (we may have been woken by the timer, not by futex_wake).
        let mut table = FUTEX_TABLE.lock();
        if let Some(waiters) = table.get_mut(&uaddr) {
            if let Some(pos) = waiters.iter().position(|&w| w == tid) {
                waiters.remove(pos);
                if waiters.is_empty() {
                    table.remove(&uaddr);
                }
                // We timed out (still in queue = nobody woke us).
                return Err(Errno::ETIMEDOUT);
            }
        }
        // If we're not in the queue, we were woken by futex_wake — success.
    }

    Ok(0)
}

/// `sys_futex_wake(uaddr, count)`
///
/// Wake up to `count` threads waiting on `uaddr`.
/// Returns the number of threads actually woken.
pub fn sys_futex_wake(uaddr: usize, count: u32) -> SysResult<usize> {
    let mut woken = 0u32;

    let mut table = FUTEX_TABLE.lock();
    if let Some(waiters) = table.get_mut(&uaddr) {
        while woken < count {
            if let Some(tid) = waiters.pop() {
                unsafe {
                    crate::task::scheduler::wake_task_erased(tid);
                }
                woken += 1;
            } else {
                break;
            }
        }
        if waiters.is_empty() {
            table.remove(&uaddr);
        }
    }

    Ok(woken as usize)
}
