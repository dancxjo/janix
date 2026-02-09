//! Graph integration for scheduler task tracking.
//!
//! This module provides helper functions for creating graph nodes for tasks
//! and updating their properties on state transitions.
//!
//! Uses a deferred work queue to avoid deadlock: public functions queue work,
//! and `do_*` functions perform the actual graph operations.

use crate::root::{RootOp, SymbolShell, enqueue};
use crate::task::TaskId;
use abi::schema::{keys, kinds, rels};
use core::sync::atomic::Ordering;
use super::graph_queue::{self, GraphWork};

// ============================================================================
// Public API - these queue work items (safe to call with scheduler lock held)
// ============================================================================

/// Queue creation of a graph node for a new thread.
pub fn create_thread_node(
    tid: TaskId,
    priority: u8,
    is_user: bool,
    name: Option<&str>,
    parent_tid: Option<TaskId>,
) {
    graph_queue::push(GraphWork::CreateThread {
        tid,
        priority,
        is_user,
        name: name.map(|s| alloc::string::String::from(s)),
        parent_tid,
    });
}

/// Queue update of the state property of a task's graph node.
pub fn update_task_state(tid: TaskId, state: &'static str) {
    graph_queue::push(GraphWork::UpdateState { tid, state });
}

/// Queue setting the exit code on a task's graph node.
pub fn set_exit_code(tid: TaskId, code: i32) {
    graph_queue::push(GraphWork::SetExitCode { tid, code });
}

/// Queue update of the priority property of a task's graph node.
pub fn update_task_priority(tid: TaskId, priority: u8) {
    graph_queue::push(GraphWork::SetPriority { tid, priority });
}

/// Queue setting the name property of a task's graph node.
pub fn set_name(tid: TaskId, name: &str) {
    graph_queue::push(GraphWork::SetName {
        tid,
        name: alloc::string::String::from(name),
    });
}

/// Queue update of the location (RUNS_ON) for a task.
pub fn update_task_location(tid: TaskId, cpu_index: usize) {
    graph_queue::push(GraphWork::SetLocation { tid, cpu_index });
}

/// Queue update of the affinity (PINNED_TO) for a task.
pub fn set_affinity_node(tid: TaskId, cpu_index: usize) {
    graph_queue::push(GraphWork::SetAffinity { tid, cpu_index });
}

// ============================================================================
// Internal implementation - these do the actual graph work (no scheduler lock)
// ============================================================================

// Cached interned symbol IDs for frequently used state strings.
// 0 = not yet interned.  After the first IPC round-trip the value is
// stored and all subsequent lookups are a single atomic load.
use core::sync::atomic::AtomicU64;

static INTERN_RUNNABLE: AtomicU64 = AtomicU64::new(0);
static INTERN_BLOCKED:  AtomicU64 = AtomicU64::new(0);
static INTERN_SLEEPING: AtomicU64 = AtomicU64::new(0);
static INTERN_DEAD:     AtomicU64 = AtomicU64::new(0);
static INTERN_RUNNING:  AtomicU64 = AtomicU64::new(0);

/// Intern a string via the Root service (blocking).
fn intern(s: &str) -> u64 {
    let reply = enqueue(RootOp::Intern {
        name: alloc::string::String::from(s),
    });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            return reply.value.load(Ordering::Relaxed);
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
        core::hint::spin_loop();
    }
}

/// Intern with per-string caching for the small set of known state strings.
/// Falls back to the blocking `intern()` IPC for unknown strings.
fn intern_cached(s: &'static str) -> u64 {
    let slot = match s {
        "runnable" => &INTERN_RUNNABLE,
        "blocked"  => &INTERN_BLOCKED,
        "sleeping" => &INTERN_SLEEPING,
        "dead"     => &INTERN_DEAD,
        "running"  => &INTERN_RUNNING,
        _          => return intern(s),
    };
    let cached = slot.load(Ordering::Relaxed);
    if cached != 0 {
        return cached;
    }
    let id = intern(s);
    slot.store(id, Ordering::Relaxed);
    id
}

/// Create a graph node with the given kind (blocking).
fn create_node(kind: &str) -> Option<u64> {
    let reply = enqueue(RootOp::CreateNode {
        kind: SymbolShell::Str(alloc::string::String::from(kind)),
    });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let id = reply.value.load(Ordering::Relaxed);
            return if id != 0 { Some(id) } else { None };
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
        core::hint::spin_loop();
    }
}

/// Set a property on a graph node (blocking).
fn set_prop(id: u64, key: &str, value: u64) {
    let reply = enqueue(RootOp::PropSet {
        id,
        key: SymbolShell::Str(alloc::string::String::from(key)),
        value,
    });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            break;
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
        core::hint::spin_loop();
    }
}

/// Create a link between two graph nodes (blocking).
fn link(src: u64, rel: &str, dst: u64) {
    let reply = enqueue(RootOp::Link {
        src,
        rel: SymbolShell::Str(alloc::string::String::from(rel)),
        dst,
    });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            break;
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
        core::hint::spin_loop();
    }
}

/// Actually create a thread node in the graph (called from flush_graph_queue).
pub fn do_create_thread_node(
    tid: TaskId,
    priority: u8,
    is_user: bool,
    name: Option<&str>,
    sched_thing: u64,
) -> Option<u64> {
    // Create the proc.Thread node
    let thing_id = create_node(kinds::PROC_THREAD)?;
    
    // Set properties
    let tid_sym = intern(keys::PROC_TID);
    set_prop(thing_id, keys::PROC_TID, tid as u64);
    
    let priority_sym = intern(keys::PROC_PRIORITY);
    set_prop(thing_id, keys::PROC_PRIORITY, priority as u64);
    
    let is_user_val = if is_user { 1 } else { 0 };
    set_prop(thing_id, keys::PROC_IS_USER, is_user_val);
    
    // Set initial state to runnable
    let state_sym = intern_cached("runnable");
    set_prop(thing_id, keys::PROC_STATE, state_sym);
    
    // Set name if provided
    if let Some(n) = name {
        let name_sym = intern(n);
        set_prop(thing_id, keys::PROC_NAME, name_sym);
    }
    
    // Link to scheduler service
    link(sched_thing, rels::SCHED_HAS_TASK, thing_id);
    
    // Suppress unused warnings for pre-interned symbols
    let _ = tid_sym;
    let _ = priority_sym;
    
    Some(thing_id)
}

/// Link a thread to its parent (called from flush_graph_queue).
pub fn do_link_parent(thing_id: u64, parent_thing: u64, _sched_thing: u64) {
    // Child links to parent with TASK_CHILD_OF
    link(thing_id, rels::TASK_CHILD_OF, parent_thing);
    // Parent links to child with TASK_SPAWNED
    link(parent_thing, rels::TASK_SPAWNED, thing_id);
}

/// Update the state property of a task's graph node (called from flush_graph_queue).
pub fn do_update_state(thing_id: u64, state: &'static str) {
    let state_sym = intern_cached(state);
    set_prop(thing_id, keys::PROC_STATE, state_sym);
}

/// Set the exit code on a task's graph node (called from flush_graph_queue).
pub fn do_set_exit_code(thing_id: u64, code: i32) {
    set_prop(thing_id, keys::PROC_EXIT_CODE, code as u64);
}

/// Set the priority on a task's graph node (called from flush_graph_queue).
pub fn do_set_priority(thing_id: u64, priority: u8) {
    set_prop(thing_id, keys::PROC_PRIORITY, priority as u64);
}

/// Set the name on a task's graph node (called from flush_graph_queue).
pub fn do_set_name(thing_id: u64, name: &str) {
    let name_sym = intern(name);
    set_prop(thing_id, keys::PROC_NAME, name_sym);
}

/// Update task location (RUNS_ON) (called from flush_graph_queue).
pub fn do_update_task_location(thing_id: u64, cpu_index: usize) {
    if let Some(cpu_thing) = crate::root::graph_anchors::cpu_thing(cpu_index) {
        link(thing_id, rels::RUNS_ON, cpu_thing);
    }
}

/// Set the affinity on a task's graph node (called from flush_graph_queue).
pub fn do_set_affinity(thing_id: u64, cpu_index: usize) {
    if let Some(cpu_thing) = crate::root::graph_anchors::cpu_thing(cpu_index) {
        link(thing_id, rels::PINNED_TO, cpu_thing);
    }
}



/// Link a task to a bytespace it uses.
#[allow(dead_code)]
pub fn link_bytespace(thread_thing: u64, bytespace_thing: u64) {
    link(thread_thing, rels::THREAD_USES_BYTESPACE, bytespace_thing);
}
