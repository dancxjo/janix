use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use spin::Mutex;

use abi::types::WakeReason;
use crate::sched::task::TaskId;
use crate::watch::WakeList;

struct SleepState {
    entries: BTreeMap<TaskId, u64>,
}

impl SleepState {
    const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    fn remove_task(&mut self, task: TaskId) {
        self.entries.remove(&task);
    }
}

static SLEEP_STATE: Mutex<SleepState> = Mutex::new(SleepState::new());

pub fn register_sleep(task: TaskId, deadline_ns: u64) {
    let mut guard = SLEEP_STATE.lock();
    guard.entries.insert(task, deadline_ns);
}

pub fn unregister_sleep(task: TaskId) {
    let mut guard = SLEEP_STATE.lock();
    guard.remove_task(task);
}

pub fn check_timeouts(now_ns: u64, out: &mut WakeList) {
    let mut guard = SLEEP_STATE.lock();
    if guard.entries.is_empty() {
        return;
    }

    let mut expired = Vec::new();
    for (&task, &deadline) in guard.entries.iter() {
        if now_ns >= deadline {
            expired.push(task);
        }
    }

    for task in expired {
        guard.entries.remove(&task);
        out.push(task, WakeReason::timeout());
    }
}
