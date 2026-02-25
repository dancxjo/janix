//! Deferred graph work queue for scheduler graphification.
//!
//! This queue allows graph operations to be deferred until the scheduler lock
//! is released, avoiding deadlock between scheduler and Root service.

use crate::task::TaskId;
use alloc::collections::VecDeque;
use alloc::string::String;
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

/// A deferred graph operation.
#[derive(Debug)]
pub enum GraphWork {
    /// Create a new thread node in the graph
    CreateThread {
        tid: TaskId,
        priority: u8,
        is_user: bool,
        name: Option<String>,
        parent_tid: Option<TaskId>,
    },
    /// Update the state property of a task
    UpdateState { tid: TaskId, state: &'static str },
    /// Set the exit code on a terminated task
    SetExitCode { tid: TaskId, code: i32 },
    /// Update the priority property of a task
    SetPriority { tid: TaskId, priority: u8 },
    /// Set the name property of a task
    SetName { tid: TaskId, name: String },
    /// Set the location (RUNS_ON) of a task
    SetLocation { tid: TaskId, cpu_index: usize },
    /// Set the affinity (PINNED_TO) of a task
    SetAffinity { tid: TaskId, cpu_index: usize },
}

/// The global work queue for deferred graph operations.
/// Uses a separate lock from the scheduler to avoid deadlock.
static WORK_QUEUE: Mutex<VecDeque<GraphWork>> = Mutex::new(VecDeque::new());

/// Maximum work queue size to prevent OOM from feedback loops
/// (context switches generate more work items which cause more context switches)
const MAX_QUEUE_SIZE: usize = 256;

/// Pre-allocate the queue to prevent memory allocations inside the ISR
pub fn init() {
    WORK_QUEUE.lock().reserve(MAX_QUEUE_SIZE);
}

static DROPPED_NON_CRITICAL: AtomicUsize = AtomicUsize::new(0);
static EVICTED_CRITICAL: AtomicUsize = AtomicUsize::new(0);
static HIGH_WATER_MARK: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Default)]
pub struct QueueStats {
    pub dropped_non_critical: usize,
    pub evicted_critical: usize,
    pub high_water_mark: usize,
    pub current_len: usize,
}

/// Push a work item to the queue.
/// This is safe to call while holding the scheduler lock.
/// Non-critical items (UpdateState) may be dropped if queue is full.
/// UpdateState items are coalesced per TID (latest state wins).
pub fn push(work: GraphWork) {
    let mut q = WORK_QUEUE.lock();

    // Coalesce: if pushing a property update and one already exists for this TID,
    // overwrite the property in-place so we only flush the latest transition.
    let mut coalesced = false;
    match &work {
        GraphWork::UpdateState { tid, state } => {
            for item in q.iter_mut() {
                if let GraphWork::UpdateState { tid: existing_tid, state: existing_state } = item {
                    if *existing_tid == *tid {
                        *existing_state = *state;
                        coalesced = true;
                        break;
                    }
                }
            }
        }
        GraphWork::SetExitCode { tid, code } => {
            for item in q.iter_mut() {
                if let GraphWork::SetExitCode { tid: existing_tid, code: existing_code } = item {
                    if *existing_tid == *tid {
                        *existing_code = *code;
                        coalesced = true;
                        break;
                    }
                }
            }
        }
        GraphWork::SetPriority { tid, priority } => {
            for item in q.iter_mut() {
                if let GraphWork::SetPriority { tid: existing_tid, priority: existing_priority } = item {
                    if *existing_tid == *tid {
                        *existing_priority = *priority;
                        coalesced = true;
                        break;
                    }
                }
            }
        }
        GraphWork::SetName { tid, name } => {
            for item in q.iter_mut() {
                if let GraphWork::SetName { tid: existing_tid, name: existing_name } = item {
                    if *existing_tid == *tid {
                        *existing_name = name.clone();
                        coalesced = true;
                        break;
                    }
                }
            }
        }
        GraphWork::SetLocation { tid, cpu_index } => {
            for item in q.iter_mut() {
                if let GraphWork::SetLocation { tid: existing_tid, cpu_index: existing_cpu_index } = item {
                    if *existing_tid == *tid {
                        *existing_cpu_index = *cpu_index;
                        coalesced = true;
                        break;
                    }
                }
            }
        }
        GraphWork::SetAffinity { tid, cpu_index } => {
            for item in q.iter_mut() {
                if let GraphWork::SetAffinity { tid: existing_tid, cpu_index: existing_cpu_index } = item {
                    if *existing_tid == *tid {
                        *existing_cpu_index = *cpu_index;
                        coalesced = true;
                        break;
                    }
                }
            }
        }
        _ => {}
    }

    if coalesced {
        return;
    }

    // If queue is full, drop non-critical items
    if q.len() >= MAX_QUEUE_SIZE {
        match &work {
            GraphWork::UpdateState { .. }
            | GraphWork::SetExitCode { .. }
            | GraphWork::SetPriority { .. }
            | GraphWork::SetName { .. }
            | GraphWork::SetLocation { .. }
            | GraphWork::SetAffinity { .. } => {
                // Property updates are non-critical - drop silently
                DROPPED_NON_CRITICAL.fetch_add(1, Ordering::Relaxed);
                return;
            }
            _ => {
                // For critical items, evict oldest to make room
                q.pop_front();
                EVICTED_CRITICAL.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    q.push_back(work);
    let len = q.len();
    let mut prev = HIGH_WATER_MARK.load(Ordering::Relaxed);
    while len > prev {
        match HIGH_WATER_MARK.compare_exchange_weak(prev, len, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => break,
            Err(actual) => prev = actual,
        }
    }
}

/// Drain up to `n` work items from the front of the queue.
/// Limits per-flush wall time by processing in small batches.
/// Returns the items for processing. Call this WITHOUT holding the scheduler lock.
pub fn drain_n(n: usize) -> VecDeque<GraphWork> {
    let mut q = WORK_QUEUE.lock();
    if q.len() <= n {
        core::mem::take(&mut *q)
    } else {
        q.drain(..n).collect()
    }
}

pub fn stats_snapshot() -> QueueStats {
    let len = WORK_QUEUE.lock().len();
    QueueStats {
        dropped_non_critical: DROPPED_NON_CRITICAL.load(Ordering::Relaxed),
        evicted_critical: EVICTED_CRITICAL.load(Ordering::Relaxed),
        high_water_mark: HIGH_WATER_MARK.load(Ordering::Relaxed),
        current_len: len,
    }
}

/// Check if the queue is empty.
#[allow(dead_code)]
pub fn is_empty() -> bool {
    WORK_QUEUE.lock().is_empty()
}
