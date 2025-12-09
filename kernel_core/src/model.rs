//! Kernel data model schemas and creation helpers
//!
//! This module defines Thing schemas for memory management, scheduling, and
//! resource tracking. These are graph-only models - no actual hardware or
//! context switching is implemented here.

use crate::graph;
use abi::{
    FrameId, FrameInfo, MemorySummary, PropType, PropValue, SchedulerSummary, ThingId, ThreadId,
    ThreadInfo,
};

// State constants for Process and Thread Things
pub const STATE_RUNNING: u64 = 1;
pub const STATE_READY: u64 = 2;
pub const STATE_BLOCKED: u64 = 3;
pub const STATE_TERMINATED: u64 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    New,
    Running,
    Ready,
    Blocked,
    Terminated,
}

#[derive(Clone, Copy)]
pub struct Thread {
    pub id: ThreadId,
    pub process_id: u64,
    pub state: ThreadState,
    pub user_entry: Option<extern "C" fn(u64) -> !>,
    pub user_arg: u64,
    pub user_stack_top: u64,
}

pub const MAX_THREADS: usize = 32;
pub static mut THREAD_TABLE: [Option<Thread>; MAX_THREADS] = [None; MAX_THREADS];

pub fn create_user_thread_for_app(
    pid: u64,
    app_id: u64,
    entry: extern "C" fn(u64) -> !,
    stack_top: u64,
) -> Option<ThreadId> {
    unsafe {
        for (i, slot) in (*core::ptr::addr_of_mut!(THREAD_TABLE))
            .iter_mut()
            .enumerate()
        {
            if slot.is_none() {
                let tid = ThreadId((i as u64) + 100);
                *slot = Some(Thread {
                    id: tid,
                    process_id: pid,
                    state: ThreadState::New,
                    user_entry: Some(entry),
                    user_arg: app_id,
                    user_stack_top: stack_top,
                });

                // Sync with graph
                create_thread_abi(pid, tid.0, 1);

                return Some(tid);
            }
        }
    }
    None
}

pub fn pick_next_thread() -> Option<&'static mut Thread> {
    unsafe {
        for slot in (*core::ptr::addr_of_mut!(THREAD_TABLE)).iter_mut() {
            if let Some(thread) = slot {
                if thread.state == ThreadState::New || thread.state == ThreadState::Ready {
                    return Some(thread);
                }
            }
        }
    }
    None
}

// SAFETY: single-core, single-thread kernel model for now.
static mut CURRENT_THREAD: Option<ThingId> = None;

fn is_kind(id: ThingId, expected: &str) -> bool {
    if let Some((kind, _)) = crate::graph::get_thing(id) {
        kind == expected
    } else {
        false
    }
}

/// Initialize all kernel model schemas
pub fn init_schemas() {
    // PhysFrame: represents a physical memory frame
    static PHYS_FRAME_SCHEMA: &[(&str, PropType)] = &[
        ("base", PropType::U64),
        ("size", PropType::U64),
        ("allocated", PropType::Bool),
    ];
    let _ = graph::register_schema("PhysFrame", PHYS_FRAME_SCHEMA);

    // FramePool: represents a pool of physical frames
    static FRAME_POOL_SCHEMA: &[(&str, PropType)] = &[
        ("start", PropType::U64),
        ("end", PropType::U64),
        ("frame_size", PropType::U64),
    ];
    let _ = graph::register_schema("FramePool", FRAME_POOL_SCHEMA);

    // AddressSpace: represents a virtual address space
    static ADDRESS_SPACE_SCHEMA: &[(&str, PropType)] = &[("asid", PropType::U64)];
    let _ = graph::register_schema("AddressSpace", ADDRESS_SPACE_SCHEMA);

    // VirtRegion: represents a virtual memory region
    static VIRT_REGION_SCHEMA: &[(&str, PropType)] = &[
        ("base", PropType::U64),
        ("len", PropType::U64),
        ("flags", PropType::U64),
    ];
    let _ = graph::register_schema("VirtRegion", VIRT_REGION_SCHEMA);

    // Process: represents a process
    static PROCESS_SCHEMA: &[(&str, PropType)] =
        &[("pid", PropType::U64), ("state", PropType::U64)];
    let _ = graph::register_schema("Process", PROCESS_SCHEMA);

    // Thread: represents a thread
    static THREAD_SCHEMA: &[(&str, PropType)] = &[
        ("tid", PropType::U64),
        ("state", PropType::U64),
        ("priority", PropType::U64),
        ("runtime_ns", PropType::U64),
    ];
    let _ = graph::register_schema("Thread", THREAD_SCHEMA);

    // CpuCore: represents a CPU core
    static CPU_CORE_SCHEMA: &[(&str, PropType)] = &[("index", PropType::U64)];
    let _ = graph::register_schema("CpuCore", CPU_CORE_SCHEMA);
}

/// Create a PhysFrame Thing
///
/// # Arguments
/// * `base` - Base physical address
/// * `size` - Size in bytes
///
/// # Returns
/// ThingId of the created PhysFrame, or None if creation failed
pub fn create_phys_frame(base: u64, size: u64) -> Option<ThingId> {
    let props = &[
        ("base", PropValue::U64(base)),
        ("size", PropValue::U64(size)),
        ("allocated", PropValue::Bool(false)),
    ];

    graph::create_thing("PhysFrame", props)
}

/// Create a FramePool Thing
///
/// # Arguments
/// * `start` - Start physical address
/// * `end` - End physical address
/// * `frame_size` - Size of each frame
///
/// # Returns
/// ThingId of the created FramePool, or None if creation failed
pub fn create_frame_pool(start: u64, end: u64, frame_size: u64) -> Option<ThingId> {
    let props = &[
        ("start", PropValue::U64(start)),
        ("end", PropValue::U64(end)),
        ("frame_size", PropValue::U64(frame_size)),
    ];

    graph::create_thing("FramePool", props)
}

/// Create a CpuCore Thing
///
/// # Arguments
/// * `index` - CPU core index
///
/// # Returns
/// ThingId of the created CpuCore, or None if creation failed
pub fn create_cpu_core(index: u64) -> Option<ThingId> {
    let props = &[("index", PropValue::U64(index))];

    graph::create_thing("CpuCore", props)
}

pub fn compute_memory_summary() -> MemorySummary {
    let (total, used, free) = crate::memory::frame_stats();
    MemorySummary {
        total_frames: total,
        used_frames: used,
        free_frames: free,
    }
}

pub fn compute_scheduler_summary() -> SchedulerSummary {
    let mut process_count = 0_u64;
    let mut thread_count = 0_u64;
    let mut runnable_threads = 0_u64;

    for raw_id in 0..crate::graph::MAX_THINGS as u64 {
        let id = ThingId(raw_id);
        if let Some((kind, props)) = crate::graph::get_thing(id) {
            match kind {
                "Process" => {
                    process_count += 1;
                }
                "Thread" => {
                    thread_count += 1;

                    // state == STATE_RUNNING or STATE_READY counts as runnable
                    let mut state = 0_u64;
                    for p in props.iter().flatten() {
                        let (key, value) = p;
                        if *key == "state" {
                            if let PropValue::U64(v) = value {
                                state = *v;
                            }
                        }
                    }

                    if state == STATE_RUNNING || state == STATE_READY {
                        runnable_threads += 1;
                    }
                }
                _ => {}
            }
        }
    }

    SchedulerSummary {
        process_count,
        thread_count,
        runnable_threads,
    }
}

/// Create an AddressSpace Thing
///
/// # Arguments
/// * `asid` - Address space identifier
///
/// # Returns
/// ThingId of the created AddressSpace, or None if creation failed
pub fn create_address_space(asid: u64) -> Option<ThingId> {
    let props = &[("asid", PropValue::U64(asid))];

    graph::create_thing("AddressSpace", props)
}

/// Create a VirtRegion Thing
///
/// # Arguments
/// * `base` - Base virtual address
/// * `len` - Length in bytes
/// * `flags` - Memory region flags
///
/// # Returns
/// ThingId of the created VirtRegion, or None if creation failed
pub fn create_virt_region(base: u64, len: u64, flags: u64) -> Option<ThingId> {
    let props = &[
        ("base", PropValue::U64(base)),
        ("len", PropValue::U64(len)),
        ("flags", PropValue::U64(flags)),
    ];

    graph::create_thing("VirtRegion", props)
}

/// Create a Process Thing
///
/// # Arguments
/// * `pid` - Process identifier
///
/// # Returns
/// ThingId of the created Process, or None if creation failed
pub fn create_process(pid: u64) -> Option<ThingId> {
    let props = &[
        ("pid", PropValue::U64(pid)),
        ("state", PropValue::U64(STATE_READY)),
    ];

    graph::create_thing("Process", props)
}

/// Create a Thread Thing
///
/// # Arguments
/// * `tid` - Thread identifier
/// * `priority` - Thread priority
///
/// # Returns
/// ThingId of the created Thread, or None if creation failed
pub fn create_thread(tid: u64, priority: u64) -> Option<ThingId> {
    let props = &[
        ("tid", PropValue::U64(tid)),
        ("state", PropValue::U64(STATE_READY)),
        ("priority", PropValue::U64(priority)),
        ("runtime_ns", PropValue::U64(0)),
    ];

    graph::create_thing("Thread", props)
}

pub fn alloc_frame() -> Option<FrameInfo> {
    // For now, scan all Things for the first PhysFrame with allocated == false
    for raw_id in 0..crate::graph::MAX_THINGS as u64 {
        let tid = ThingId(raw_id);
        if let Some((kind, props)) = crate::graph::get_thing(tid) {
            if kind != "PhysFrame" {
                continue;
            }

            let mut base = 0_u64;
            let mut size = 0_u64;
            let mut allocated = false;

            for p in props.iter().flatten() {
                let (key, value) = p;
                match *key {
                    "base" => {
                        if let PropValue::U64(v) = value {
                            base = *v;
                        }
                    }
                    "size" => {
                        if let PropValue::U64(v) = value {
                            size = *v;
                        }
                    }
                    "allocated" => {
                        if let PropValue::Bool(v) = value {
                            allocated = *v;
                        }
                    }
                    _ => {}
                }
            }

            if !allocated {
                // mark as allocated
                let new_props = &[("allocated", PropValue::Bool(true))];
                crate::graph::update_thing(tid, new_props);

                return Some(FrameInfo {
                    id: FrameId(raw_id),
                    base,
                    size,
                });
            }
        }
    }

    None
}

pub fn free_frame(frame_id: FrameId) -> bool {
    let tid = ThingId(frame_id.0);
    if !is_kind(tid, "PhysFrame") {
        return false;
    }

    let props = &[("allocated", PropValue::Bool(false))];
    crate::graph::update_thing(tid, props)
}

pub fn create_process_abi(pid: u64) -> Option<u64> {
    create_process(pid).map(|id| id.0)
}

pub fn create_thread_abi(_pid: u64, tid: u64, priority: u64) -> Option<u64> {
    // For now, we ignore pid in the graph; later we’ll add edges.
    create_thread(tid, priority).map(|id| {
        // if there is no current thread yet, make this one current
        unsafe {
            // Use raw pointer to avoid creating a reference to mutable static
            if core::ptr::addr_of!(CURRENT_THREAD).read().is_none() {
                CURRENT_THREAD = Some(id);
            }
        }
        id.0
    })
}

pub fn scheduler_tick() -> Option<ThreadInfo> {
    unsafe {
        // 1. If there is a current thread, bump its runtime and mark it READY
        // Use raw pointer read to avoid reference to mutable static
        if let Some(cur) = core::ptr::addr_of!(CURRENT_THREAD).read() {
            if let Some((_kind, props)) = crate::graph::get_thing(cur) {
                let mut runtime = 0_u64;
                let mut _priority = 0_u64;
                let mut _tid = 0_u64;

                for p in props.iter().flatten() {
                    let (key, value) = p;
                    match *key {
                        "runtime_ns" => {
                            if let PropValue::U64(v) = value {
                                runtime = *v;
                            }
                        }
                        "priority" => {
                            if let PropValue::U64(v) = value {
                                _priority = *v;
                            }
                        }
                        "tid" => {
                            if let PropValue::U64(v) = value {
                                _tid = *v;
                            }
                        }
                        _ => {}
                    }
                }

                runtime = runtime.saturating_add(1); // one “tick”

                let props = &[
                    ("runtime_ns", PropValue::U64(runtime)),
                    ("state", PropValue::U64(STATE_READY)),
                ];
                crate::graph::update_thing(cur, props);

                // we’ll pick a new current below
            }
        }

        // 2. Pick the next runnable thread: smallest runtime_ns among READY/RUNNING
        let mut best: Option<(ThingId, u64, u64, u64)> = None; // (id, tid, runtime, priority)

        for raw_id in 0..crate::graph::MAX_THINGS as u64 {
            let id = ThingId(raw_id);
            if let Some((kind, props)) = crate::graph::get_thing(id) {
                if kind != "Thread" {
                    continue;
                }

                let mut state = 0_u64;
                let mut runtime = 0_u64;
                let mut priority = 0_u64;
                let mut tid = 0_u64;

                for p in props.iter().flatten() {
                    let (key, value) = p;
                    match *key {
                        "state" => {
                            if let PropValue::U64(v) = value {
                                state = *v;
                            }
                        }
                        "runtime_ns" => {
                            if let PropValue::U64(v) = value {
                                runtime = *v;
                            }
                        }
                        "priority" => {
                            if let PropValue::U64(v) = value {
                                priority = *v;
                            }
                        }
                        "tid" => {
                            if let PropValue::U64(v) = value {
                                tid = *v;
                            }
                        }
                        _ => {}
                    }
                }

                if state == STATE_READY || state == STATE_RUNNING {
                    match best {
                        None => best = Some((id, tid, runtime, priority)),
                        Some((_, _, best_runtime, _)) => {
                            if runtime < best_runtime {
                                best = Some((id, tid, runtime, priority));
                            }
                        }
                    }
                }
            }
        }

        if let Some((id, tid, _runtime, priority)) = best {
            // mark new current as RUNNING
            let props = &[("state", PropValue::U64(STATE_RUNNING))];
            crate::graph::update_thing(id, props);

            CURRENT_THREAD = Some(id);

            Some(ThreadInfo {
                tid,
                state: STATE_RUNNING,
                priority,
                // we could include runtime+1 but it's not critical
            })
        } else {
            CURRENT_THREAD = None;
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThingCounts {
    pub total_things: u64,
    pub processes: u64,
    pub threads: u64,
    pub phys_frames: u64,
    pub virt_regions: u64,
    pub frame_pools: u64,
    pub address_spaces: u64,
    pub cpu_cores: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct DashboardSnapshot {
    pub memory: MemorySummary,
    pub scheduler: SchedulerSummary,
    pub counts: ThingCounts,
}

pub fn dashboard_snapshot() -> DashboardSnapshot {
    let memory = compute_memory_summary();
    let scheduler = compute_scheduler_summary();

    let mut counts = ThingCounts {
        total_things: 0,
        processes: 0,
        threads: 0,
        phys_frames: 0,
        virt_regions: 0,
        frame_pools: 0,
        address_spaces: 0,
        cpu_cores: 0,
    };

    // Walk all Things in the graph
    for raw_id in 0..crate::graph::MAX_THINGS as u64 {
        let id = ThingId(raw_id);
        if let Some((kind, _props)) = crate::graph::get_thing(id) {
            counts.total_things += 1;

            match kind {
                "Process" => counts.processes += 1,
                "Thread" => counts.threads += 1,
                "PhysFrame" => counts.phys_frames += 1,
                "VirtRegion" => counts.virt_regions += 1,
                "FramePool" => counts.frame_pools += 1,
                "AddressSpace" => counts.address_spaces += 1,
                "CpuCore" => counts.cpu_cores += 1,
                _ => {}
            }
        }
    }

    DashboardSnapshot {
        memory,
        scheduler,
        counts,
    }
}
