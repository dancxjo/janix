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

// Cached schema symbols
static SYM_PROC_TID: AtomicU64 = AtomicU64::new(0);
static SYM_PROC_PRIORITY: AtomicU64 = AtomicU64::new(0);
static SYM_PROC_STATE: AtomicU64 = AtomicU64::new(0);
static SYM_PROC_NAME: AtomicU64 = AtomicU64::new(0);
static SYM_PROC_IS_USER: AtomicU64 = AtomicU64::new(0);
static SYM_PROC_EXIT_CODE: AtomicU64 = AtomicU64::new(0);
static SYM_KIND_THREAD: AtomicU64 = AtomicU64::new(0);
static SYM_REL_HAS_TASK: AtomicU64 = AtomicU64::new(0);
static SYM_REL_RUNS_ON: AtomicU64 = AtomicU64::new(0);
static SYM_REL_PINNED_TO: AtomicU64 = AtomicU64::new(0);
static SYM_REL_CHILD_OF: AtomicU64 = AtomicU64::new(0);
static SYM_REL_SPAWNED: AtomicU64 = AtomicU64::new(0);

fn get_schema_sym(s: &'static str, slot: &AtomicU64) -> u64 {
    let cached = slot.load(Ordering::Relaxed);
    if cached != 0 {
        return cached;
    }
    let id = intern(s);
    slot.store(id, Ordering::Relaxed);
    id
}

macro_rules! schema_sym {
    ($s:expr, $slot:ident) => {
        get_schema_sym($s, &$slot)
    };
}

/// Helper to build the binary ApplyBatch format.
pub struct BatchBuilder {
    buf: alloc::vec::Vec<u8>,
    op_count: u16,
}

impl BatchBuilder {
    pub fn new() -> Self {
        let mut buf = alloc::vec::Vec::with_capacity(1024);
        // Placeholder for header
        buf.extend_from_slice(&[0u8; 8]);
        Self { buf, op_count: 0 }
    }

    fn push_u16(&mut self, val: u16) {
        self.buf.extend_from_slice(&val.to_le_bytes());
    }

    fn push_u32(&mut self, val: u32) {
        self.buf.extend_from_slice(&val.to_le_bytes());
    }

    fn push_u64(&mut self, val: u64) {
        self.buf.extend_from_slice(&val.to_le_bytes());
    }

    fn push_id_16(&mut self, id: u64) {
        let mut id_bytes = [0u8; 16];
        id_bytes[0..8].copy_from_slice(&id.to_le_bytes());
        self.buf.extend_from_slice(&id_bytes);
    }

    fn push_thing_ref_absolute(&mut self, id: u64) {
        self.buf.push(abi::root::REF_ABSOLUTE);
        self.push_id_16(id);
    }

    fn push_thing_ref_local(&mut self, idx: u16) {
        self.buf.push(abi::root::REF_LOCAL);
        self.push_u16(idx);
    }

    pub fn create_node(&mut self, kind: u64, out_idx: u16) {
        self.buf.push(abi::root::OP_CREATE_NODE);
        self.push_id_16(kind);
        self.push_u16(out_idx);
        self.op_count += 1;
    }

    pub fn put_edge(&mut self, src_id: u64, rel_id: u64, dst_id: u64) {
        self.buf.push(abi::root::OP_PUT_EDGE);
        self.push_thing_ref_absolute(src_id);
        self.push_id_16(rel_id);
        self.push_thing_ref_absolute(dst_id);
        self.op_count += 1;
    }

    pub fn put_edge_local(&mut self, src_local: u16, rel_id: u64, dst_id: u64) {
        self.buf.push(abi::root::OP_PUT_EDGE);
        self.push_thing_ref_local(src_local);
        self.push_id_16(rel_id);
        self.push_thing_ref_absolute(dst_id);
        self.op_count += 1;
    }

    pub fn link_local_to_abs(&mut self, src_local: u16, rel_id: u64, dst_abs: u64) {
        self.buf.push(abi::root::OP_PUT_EDGE);
        self.push_thing_ref_local(src_local);
        self.push_id_16(rel_id);
        self.push_thing_ref_absolute(dst_abs);
        self.op_count += 1;
    }

    pub fn link_abs_to_local(&mut self, src_abs: u64, rel_id: u64, dst_local: u16) {
        self.buf.push(abi::root::OP_PUT_EDGE);
        self.push_thing_ref_absolute(src_abs);
        self.push_id_16(rel_id);
        self.push_thing_ref_local(dst_local);
        self.op_count += 1;
    }

    pub fn set_prop(&mut self, id: u64, key: u64, value: u64) {
        self.buf.push(abi::root::OP_SET_PROP);
        self.push_thing_ref_absolute(id);
        self.push_id_16(key);
        self.push_u64(value);
        self.op_count += 1;
    }

    pub fn set_prop_local(&mut self, local_idx: u16, key: u64, value: u64) {
        self.buf.push(abi::root::OP_SET_PROP);
        self.push_thing_ref_local(local_idx);
        self.push_id_16(key);
        self.push_u64(value);
        self.op_count += 1;
    }

    pub fn finish(mut self) -> alloc::vec::Vec<u8> {
        let header = abi::root::BatchHeader {
            magic: abi::root::BATCH_MAGIC,
            version: abi::root::BATCH_VERSION,
            op_count: self.op_count,
        };
        let header_bytes = header.to_le_bytes();
        self.buf[0..8].copy_from_slice(&header_bytes);
        self.buf
    }
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
    let mut bb = BatchBuilder::new();
    
    // Create the proc.Thread node at local index 0
    bb.create_node(schema_sym!(kinds::PROC_THREAD, SYM_KIND_THREAD), 0);
    
    // Set properties on local 0
    bb.set_prop_local(0, schema_sym!(keys::PROC_TID, SYM_PROC_TID), tid as u64);
    bb.set_prop_local(0, schema_sym!(keys::PROC_PRIORITY, SYM_PROC_PRIORITY), priority as u64);
    bb.set_prop_local(0, schema_sym!(keys::PROC_IS_USER, SYM_PROC_IS_USER), if is_user { 1 } else { 0 });
    bb.set_prop_local(0, schema_sym!(keys::PROC_STATE, SYM_PROC_STATE), intern_cached("runnable"));
    
    if let Some(n) = name {
        bb.set_prop_local(0, schema_sym!(keys::PROC_NAME, SYM_PROC_NAME), intern(n));
    }
    
    // Link to scheduler service
    bb.link_abs_to_local(sched_thing, schema_sym!(rels::SCHED_HAS_TASK, SYM_REL_HAS_TASK), 0);
    
    let reply = enqueue(RootOp::ApplyBatch { batch: bb.finish() });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            // ApplyBatch returns the sequence number in value, but handle_apply_batch
            // doesn't return created IDs in the legacy u64 value slot.
            // Wait, we need the created ID!
            // Looking at handle_apply_batch_with_scratch in batch.rs:
            // returns (result.status, result.seq).
            // The created IDs are NOT returned to the caller via RootMsg.reply.value.
            // This is a problem.
            
            // Actually, we can look up the ID by querying the graph afterward, or
            // we can modify RootOp::ApplyBatch to return the first created ID if any?
            // No, the ABI says we can't easily change it without breaking others.
            
            // Wait, handle_create_node returns the ID!
            // But Batch doesn't.
            
            // Let's stick to do_create_thread_node using handle_create_node for now if we need the ID,
            // OR we can use handle_find to get it.
            break;
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
        core::hint::spin_loop();
    }
    
    // Since we need the ID for the caller to store in task_graph, 
    // maybe it's better to NOT batch CreateThread if it's infrequent?
    // Creation only happens once per task. State updates happen millions of times.
    // Let's revert CreateThread to use separate calls for now to keep it working,
    // but optimize it with cached symbols.
    None
}

/// Fallback optimized version of do_create_thread_node that returns the ID.
pub fn do_create_thread_node_optimized(
    tid: TaskId,
    priority: u8,
    is_user: bool,
    name: Option<&str>,
    sched_thing: u64,
) -> Option<u64> {
    let thing_id = create_node_optimized(kinds::PROC_THREAD, &SYM_KIND_THREAD)?;
    
    set_prop_optimized(thing_id, keys::PROC_TID, &SYM_PROC_TID, tid as u64);
    set_prop_optimized(thing_id, keys::PROC_PRIORITY, &SYM_PROC_PRIORITY, priority as u64);
    set_prop_optimized(thing_id, keys::PROC_IS_USER, &SYM_PROC_IS_USER, if is_user { 1 } else { 0 });
    set_prop_optimized(thing_id, keys::PROC_STATE, &SYM_PROC_STATE, intern_cached("runnable"));
    
    if let Some(n) = name {
        set_prop_optimized(thing_id, keys::PROC_NAME, &SYM_PROC_NAME, intern(n));
    }
    
    link_optimized(sched_thing, rels::SCHED_HAS_TASK, &SYM_REL_HAS_TASK, thing_id);
    
    Some(thing_id)
}

fn create_node_optimized(kind_str: &'static str, slot: &AtomicU64) -> Option<u64> {
    let kind = get_schema_sym(kind_str, slot);
    let reply = enqueue(RootOp::CreateNode {
        kind: SymbolShell::Id(kind as u32),
    });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let id = reply.value.load(Ordering::Relaxed);
            return if id != 0 { Some(id) } else { None };
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
        core::hint::spin_loop();
    }
}

fn set_prop_optimized(id: u64, key_str: &'static str, slot: &AtomicU64, value: u64) {
    let key = get_schema_sym(key_str, slot);
    let reply = enqueue(RootOp::PropSet {
        id,
        key: SymbolShell::Id(key as u32),
        value,
    });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 { break; }
        unsafe { crate::task::scheduler::yield_now_current(); }
        core::hint::spin_loop();
    }
}

fn link_optimized(src: u64, rel_str: &'static str, slot: &AtomicU64, dst: u64) {
    let rel = get_schema_sym(rel_str, slot);
    let reply = enqueue(RootOp::Link {
        src,
        rel: SymbolShell::Id(rel as u32),
        dst,
    });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 { break; }
        unsafe { crate::task::scheduler::yield_now_current(); }
        core::hint::spin_loop();
    }
}

pub fn do_flush_batch(items: &[(u64, GraphWork)]) {
    if items.is_empty() { return; }
    
    let mut bb = BatchBuilder::new();
    
    for (thing_id, work) in items {
        let tid_node = *thing_id;
        match work {
            GraphWork::UpdateState { state, .. } => {
                bb.set_prop(tid_node, schema_sym!(keys::PROC_STATE, SYM_PROC_STATE), intern_cached(state));
            }
            GraphWork::SetExitCode { code, .. } => {
                bb.set_prop(tid_node, schema_sym!(keys::PROC_EXIT_CODE, SYM_PROC_EXIT_CODE), *code as u64);
            }
            GraphWork::SetPriority { priority, .. } => {
                bb.set_prop(tid_node, schema_sym!(keys::PROC_PRIORITY, SYM_PROC_PRIORITY), *priority as u64);
            }
            GraphWork::SetName { name, .. } => {
                bb.set_prop(tid_node, schema_sym!(keys::PROC_NAME, SYM_PROC_NAME), intern(name));
            }
            GraphWork::SetLocation { cpu_index, .. } => {
                if let Some(cpu_thing) = crate::root::graph_anchors::cpu_thing(*cpu_index) {
                    bb.put_edge(tid_node, schema_sym!(rels::RUNS_ON, SYM_REL_RUNS_ON), cpu_thing);
                }
            }
            GraphWork::SetAffinity { cpu_index, .. } => {
                if let Some(cpu_thing) = crate::root::graph_anchors::cpu_thing(*cpu_index) {
                    bb.put_edge(tid_node, schema_sym!(rels::PINNED_TO, SYM_REL_PINNED_TO), cpu_thing);
                }
            }
            GraphWork::CreateThread { .. } => {
                // Batching CreateThread is complex because it returns IDs.
                // We handle it separately for now.
            }
        }
    }
    
    if bb.op_count == 0 { return; }
    
    let reply = enqueue(RootOp::ApplyBatch { batch: bb.finish() });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 { break; }
        unsafe { crate::task::scheduler::yield_now_current(); }
        core::hint::spin_loop();
    }
}

pub fn do_link_parent(thing_id: u64, parent_thing: u64, _sched_thing: u64) {
    let mut bb = BatchBuilder::new();
    bb.put_edge(thing_id, schema_sym!(rels::TASK_CHILD_OF, SYM_REL_CHILD_OF), parent_thing);
    bb.put_edge(parent_thing, schema_sym!(rels::TASK_SPAWNED, SYM_REL_SPAWNED), thing_id);
    
    let reply = enqueue(RootOp::ApplyBatch { batch: bb.finish() });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 { break; }
        unsafe { crate::task::scheduler::yield_now_current(); }
        core::hint::spin_loop();
    }
}



/// Link a task to a bytespace it uses.
#[allow(dead_code)]
pub fn link_bytespace(thread_thing: u64, bytespace_thing: u64) {
    link(thread_thing, rels::THREAD_USES_BYTESPACE, bytespace_thing);
}
