//! Watch wait registry for kernel-visible waiters.
//!
//! Tracks which tasks are blocked on which WatchIds and produces wake lists
//! for the scheduler without allocating in interrupt context.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use spin::Mutex;

use abi::ids::WatchId;
use abi::types::{WaitFlags, WakeReason, MAX_WATCH_EVENTS};
use crate::sched::task::TaskId;

struct WaitEntry {
    watches: Vec<(WatchId, bool)>, // bool = fired
    flags: WaitFlags,
    timeout_at: Option<u64>,
}

struct Registry {
    by_task: BTreeMap<TaskId, WaitEntry>,
    by_watch: BTreeMap<WatchId, Vec<TaskId>>,
}

impl Registry {
    const fn new() -> Self {
        Self {
            by_task: BTreeMap::new(),
            by_watch: BTreeMap::new(),
        }
    }

    fn remove_task_locked(&mut self, task: TaskId) {
        if let Some(entry) = self.by_task.remove(&task) {
            for (watch_id, _) in entry.watches.iter() {
                if let Some(waiters) = self.by_watch.get_mut(watch_id) {
                    waiters.retain(|t| *t != task);
                    if waiters.is_empty() {
                        self.by_watch.remove(watch_id);
                    }
                }
            }
        }
    }
}

static REGISTRY: Mutex<Registry> = Mutex::new(Registry::new());

/// Bounded list of tasks to wake with their reason.
pub struct WakeList {
    entries: [(TaskId, WakeReason); MAX_WATCH_EVENTS],
    len: usize,
}

impl WakeList {
    pub const fn new() -> Self {
        Self {
            entries: [(TaskId(0), WakeReason::timeout()); MAX_WATCH_EVENTS],
            len: 0,
        }
    }

    pub fn push(&mut self, task: TaskId, reason: WakeReason) {
        if self.len >= self.entries.len() {
            return;
        }
        self.entries[self.len] = (task, reason);
        self.len += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = (TaskId, WakeReason)> + '_ {
        self.entries[..self.len].iter().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Register a task wait on a set of watches with optional timeout in ticks.
pub fn register_wait(task: TaskId, watches: &[WatchId], flags: WaitFlags, timeout_at: Option<u64>) {
    let mut guard = REGISTRY.lock();
    let registry = &mut *guard;

    registry.remove_task_locked(task);

    let mut entry = WaitEntry {
        watches: Vec::new(),
        flags,
        timeout_at,
    };

    entry.watches.reserve(watches.len());
    for &w in watches {
        entry.watches.push((w, false));
        registry.by_watch.entry(w).or_default().push(task);
    }

    registry.by_task.insert(task, entry);
}

/// Remove any outstanding wait registration for a task.
pub fn unregister_wait(task: TaskId) {
    let mut guard = REGISTRY.lock();
    guard.remove_task_locked(task);
}

/// Mark a watch as fired, filling the wake list for tasks that should resume.
pub fn fire_watch(id: WatchId, out: &mut WakeList) {
    let mut guard = REGISTRY.lock();
    let registry = &mut *guard;

    // Copy waiters into a bounded stack array to avoid heap work here.
    let mut waiters: [Option<TaskId>; MAX_WATCH_EVENTS] = [None; MAX_WATCH_EVENTS];
    let mut count = 0usize;
    {
        if let Some(list) = registry.by_watch.get(&id) {
            for &tid in list.iter() {
                if count < waiters.len() {
                    waiters[count] = Some(tid);
                    count += 1;
                }
            }
        }
    }

    for tid_opt in waiters.into_iter().flatten().take(count) {
        if let Some(entry) = registry.by_task.get_mut(&tid_opt) {
            for (watch_id, fired) in entry.watches.iter_mut() {
                if *watch_id == id {
                    *fired = true;
                }
            }
            let ready = if entry.flags.contains(WaitFlags::WAIT_ALL) {
                entry.watches.iter().all(|(_, fired)| *fired)
            } else {
                true
            };
            if ready {
                out.push(tid_opt, WakeReason::watch(id));
                registry.remove_task_locked(tid_opt);
            }
        }
    }
}

/// Scan for timed-out waits.
pub fn check_timeouts(now_ticks: u64, out: &mut WakeList) {
    let mut guard = REGISTRY.lock();
    let registry = &mut *guard;

    // Collect tasks that have expired without reallocating during the scan.
    let mut expired: [Option<TaskId>; MAX_WATCH_EVENTS] = [None; MAX_WATCH_EVENTS];
    let mut count = 0usize;

    for (task_id, entry) in registry.by_task.iter() {
        if let Some(deadline) = entry.timeout_at {
            if now_ticks >= deadline && count < expired.len() {
                expired[count] = Some(*task_id);
                count += 1;
            }
        }
    }

    for tid_opt in expired.into_iter().flatten().take(count) {
        out.push(tid_opt, WakeReason::timeout());
        registry.remove_task_locked(tid_opt);
    }
}

/// Convenience helper: fire a watch and wake any waiting tasks immediately.
pub fn fire_and_wake(id: WatchId) {
    let mut list = WakeList::new();
    fire_watch(id, &mut list);
    crate::sched::wake_from_watch_list(&list);
}
