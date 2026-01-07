use super::*;

pub fn exit(code: i32) -> ! {
    unsafe {
        syscall(nr::SYS_PROC_EXIT, code as u64, 0, 0, 0, 0, 0);
    }
    loop {}
}

pub fn spawn(name: &str) -> Result<ThingId, i32> {
    unsafe {
        let res = syscall(
            nr::SYS_PROC_SPAWN,
            name.as_ptr() as u64,
            name.len() as u64,
            0,
            0,
            0,
            0,
        );
        if res.status != 0 {
            Err(res.status as i32)
        } else {
            Ok(ThingId::from_parts(res.val1, res.val0)) // High is val1!
        }
    }
}

pub fn sched_yield() {
    unsafe {
        syscall(nr::SYS_SCHED_YIELD, 0, 0, 0, 0, 0, 0);
    }
}

/// Set boot progress (triggers screen color update from kernel)
pub fn boot_progress(step: u32, max_step: u32) {
    unsafe {
        crate::syscall(
            abi::syscall::nr::SYS_BOOT_PROGRESS,
            step as u64, max_step as u64, 0, 0, 0, 0
        );
    }
}

use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;

/// A simple one-shot communication channel that allows sending a single value.
pub struct OneShotMailbox<T> {
    ready: AtomicBool,
    data: UnsafeCell<Option<T>>,
}

unsafe impl<T: Send> Sync for OneShotMailbox<T> {}

impl<T> OneShotMailbox<T> {
    pub const fn new() -> Self {
        Self {
            ready: AtomicBool::new(false),
            data: UnsafeCell::new(None),
        }
    }

    pub fn send(&self, val: T) -> Result<(), T> {
        if self.ready.load(Ordering::Acquire) {
            return Err(val);
        }
        unsafe {
            *self.data.get() = Some(val);
        }
        self.ready.store(true, Ordering::Release);
        Ok(())
    }

    pub fn try_take(&self) -> Option<T> {
        if !self.ready.load(Ordering::Acquire) {
            return None;
        }
        let data = unsafe { (*self.data.get()).take() };
        self.ready.store(false, Ordering::Release);
        data
    }
    
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }
}
