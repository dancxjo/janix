//! Deferred graph work queue for scheduler graphification.
//!
//! This queue allows graph operations to be deferred until the scheduler lock
//! is released, avoiding deadlock between scheduler and Root service.

use crate::task::TaskId;
use alloc::collections::VecDeque;
use alloc::string::String;
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
    UpdateState {
        tid: TaskId,
        state: &'static str,
    },
    /// Set the exit code on a terminated task
    SetExitCode {
        tid: TaskId,
        code: i32,
    },
    /// Update the priority property of a task
    SetPriority {
        tid: TaskId,
        priority: u8,
    },
    /// Set the name property of a task
    SetName {
        tid: TaskId,
        name: String,
    },
}

/// The global work queue for deferred graph operations.
/// Uses a separate lock from the scheduler to avoid deadlock.
static WORK_QUEUE: Mutex<VecDeque<GraphWork>> = Mutex::new(VecDeque::new());

/// Maximum work queue size to prevent OOM from feedback loops
/// (context switches generate more work items which cause more context switches)
const MAX_QUEUE_SIZE: usize = 256;

/// Push a work item to the queue.
/// This is safe to call while holding the scheduler lock.
/// Non-critical items (UpdateState) may be dropped if queue is full.
pub fn push(work: GraphWork) {
    let mut q = WORK_QUEUE.lock();
    
    // If queue is full, drop non-critical items
    if q.len() >= MAX_QUEUE_SIZE {
        match &work {
            GraphWork::UpdateState { .. } => {
                // State updates are non-critical - drop silently
                return;
            }
            _ => {
                // For critical items, evict oldest to make room
                q.pop_front();
            }
        }
    }
    
    q.push_back(work);
}

/// Drain all work items from the queue.
/// Returns the items for processing. Call this WITHOUT holding the scheduler lock.
pub fn drain() -> VecDeque<GraphWork> {
    core::mem::take(&mut *WORK_QUEUE.lock())
}

/// Check if the queue is empty.
#[allow(dead_code)]
pub fn is_empty() -> bool {
    WORK_QUEUE.lock().is_empty()
}
