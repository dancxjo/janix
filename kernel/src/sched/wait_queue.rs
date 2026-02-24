//! Generic WaitQueue for task synchronization
//!
//! Provides FIFO waking to avoid thundering herd issues and ensure fairness.

use alloc::collections::VecDeque;
use spin::Mutex;

pub struct WaitQueue {
    waiters: Mutex<VecDeque<u64>>,
}

impl WaitQueue {
    pub const fn new() -> Self {
        Self {
            waiters: Mutex::new(VecDeque::new()),
        }
    }

    /// Add a task to the wait queue
    pub fn push_back(&self, tid: u64) {
        let mut waiters = self.waiters.lock();
        if !waiters.contains(&tid) {
            waiters.push_back(tid);
        }
    }

    /// Wake the first task in the queue
    pub fn wake_one(&self) {
        let tid = {
            let mut waiters = self.waiters.lock();
            waiters.pop_front()
        };

        if let Some(tid) = tid {
            unsafe {
                crate::sched::wake_task_erased(tid);
            }
        }
    }

    /// Wake all tasks in the queue
    pub fn wake_all(&self) {
        let mut waiters = self.waiters.lock();
        while let Some(tid) = waiters.pop_front() {
            unsafe {
                crate::sched::wake_task_erased(tid);
            }
        }
    }

    /// Remove a task from the wait queue (e.g. on timeout or interrupt)
    pub fn remove(&self, tid: u64) {
        let mut waiters = self.waiters.lock();
        if let Some(pos) = waiters.iter().position(|&id| id == tid) {
            waiters.remove(pos);
        }
    }
}
