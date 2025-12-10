//! Kernel data model schemas and creation helpers
//!
//! This module defines Thing schemas for memory management, scheduling, and
//! resource tracking. These are graph-only models - no actual hardware or
//! context switching is implemented here.

extern crate alloc;

use crate::sched_types::ThreadState;
use crate::{graph, graph_kinds, sched_graph};
use abi::{
    FrameId, FrameInfo, MemorySummary, PropType, PropValue, SchedulerSummary, Thing, ThingId,
    ThreadId, ThreadInfo,
};
use alloc::string::String;
use thing_models::{BootProfile, BootProgram, ProgramImage};

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
                if thread.state == ThreadState::New || thread.state == ThreadState::Runnable {
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
    let _ = graph::register_schema(
        "PhysFrame",
        "A region of physical memory with base address, size, and allocation status",
        PHYS_FRAME_SCHEMA,
    );

    // FramePool: represents a pool of physical frames
    static FRAME_POOL_SCHEMA: &[(&str, PropType)] = &[
        ("start", PropType::U64),
        ("end", PropType::U64),
        ("frame_size", PropType::U64),
    ];
    let _ = graph::register_schema(
        "FramePool",
        "A pool of physical memory frames with defined start, end, and frame size",
        FRAME_POOL_SCHEMA,
    );

    // AddressSpace: represents a virtual address space
    static ADDRESS_SPACE_SCHEMA: &[(&str, PropType)] = &[("asid", PropType::U64)];
    let _ = graph::register_schema(
        "AddressSpace",
        "A virtual address space identified by its address space identifier (ASID)",
        ADDRESS_SPACE_SCHEMA,
    );

    // VirtRegion: represents a virtual memory region
    static VIRT_REGION_SCHEMA: &[(&str, PropType)] = &[
        ("base", PropType::U64),
        ("len", PropType::U64),
        ("flags", PropType::U64),
    ];
    let _ = graph::register_schema(
        "VirtRegion",
        "A virtual memory region with base address, length, and access flags",
        VIRT_REGION_SCHEMA,
    );

    // Process: represents a process
    static PROCESS_SCHEMA: &[(&str, PropType)] = &[("pid", PropType::U64), ("name", PropType::Str)];
    let _ = graph::register_schema(
        graph_kinds::KIND_PROCESS,
        "A process with process identifier (PID) and execution state",
        PROCESS_SCHEMA,
    );

    // Thread: represents a thread
    static THREAD_SCHEMA: &[(&str, PropType)] = &[
        ("tid", PropType::U64),
        ("name", PropType::Str),
        ("state", PropType::Str),
        ("priority", PropType::U64),
        ("runtime_ns", PropType::U64),
        ("last_started_ns", PropType::U64),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_THREAD,
        "A thread of execution with thread identifier, state, priority, runtime tracking, and last start time",
        THREAD_SCHEMA,
    );

    // CpuCore: represents a CPU core
    static CPU_CORE_SCHEMA: &[(&str, PropType)] = &[("index", PropType::U64)];
    let _ = graph::register_schema(
        graph_kinds::KIND_CPU_CORE,
        "A CPU core identified by its index in the system",
        CPU_CORE_SCHEMA,
    );

    // SleepEvent: represents a wakeup deadline for a thread
    static SLEEP_SCHEMA: &[(&str, PropType)] = &[
        ("wake_at_ns", PropType::U64),
        ("created_at_ns", PropType::U64),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_SLEEP_EVENT,
        "A scheduled wakeup for a sleeping thread",
        SLEEP_SCHEMA,
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_BOOT_PROFILE,
        BootProfile::DESCRIPTION,
        BootProfile::schema(),
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_BOOT_PROGRAM,
        BootProgram::DESCRIPTION,
        BootProgram::schema(),
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_PROGRAM_IMAGE,
        ProgramImage::DESCRIPTION,
        ProgramImage::schema(),
    );
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

    graph::create_thing(graph_kinds::KIND_CPU_CORE, props)
}

pub fn compute_memory_summary() -> MemorySummary {
    let mut total_frames = 0_u64;
    let mut used_frames = 0_u64;

    for raw_id in 0..crate::graph::MAX_THINGS as u64 {
        let id = ThingId(raw_id);
        if let Some((kind, props)) = crate::graph::get_thing(id) {
            if kind != "PhysFrame" {
                continue;
            }
            total_frames += 1;

            for p in props.iter().flatten() {
                if let (key, PropValue::Bool(allocated)) = p {
                    if *key == "allocated" && *allocated {
                        used_frames += 1;
                    }
                }
            }
        }
    }

    MemorySummary {
        total_frames,
        used_frames,
        free_frames: total_frames.saturating_sub(used_frames),
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
                graph_kinds::KIND_PROCESS => {
                    process_count += 1;
                }
                graph_kinds::KIND_THREAD => {
                    thread_count += 1;

                    // state == Running or Runnable counts as runnable
                    let mut state = None;
                    for p in props.iter().flatten() {
                        let (key, value) = p;
                        if *key == "state" {
                            if let PropValue::Str(s) = value {
                                state = ThreadState::from_str(s.as_str());
                            }
                        }
                    }

                    if matches!(state, Some(ThreadState::Running | ThreadState::Runnable)) {
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
    let props = &[("pid", PropValue::U64(pid))];

    graph::create_thing(graph_kinds::KIND_PROCESS, props)
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
        (
            "state",
            PropValue::Str(String::from(ThreadState::Runnable.as_str())),
        ),
        ("priority", PropValue::U64(priority)),
        ("runtime_ns", PropValue::U64(0)),
        ("last_started_ns", PropValue::U64(0)),
    ];

    graph::create_thing(graph_kinds::KIND_THREAD, props)
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
    static mut FAKE_TIME: u64 = 0;

    unsafe {
        FAKE_TIME = FAKE_TIME.saturating_add(1_000_000);
        let now = FAKE_TIME;
        let mut g = graph::Graph::new();
        let next = sched_graph::sched_tick(&mut g, 0, now)?;

        let state = crate::graph::get_prop(next, "state")
            .and_then(|v| match v {
                PropValue::Str(s) => ThreadState::from_str(s.as_str()),
                _ => None,
            })
            .unwrap_or(ThreadState::Runnable);
        let priority = crate::graph::get_prop(next, "priority")
            .and_then(|v| match v {
                PropValue::U64(v) => Some(v),
                _ => None,
            })
            .unwrap_or(0);
        let tid = crate::graph::get_prop(next, "tid")
            .and_then(|v| match v {
                PropValue::U64(v) => Some(v),
                _ => None,
            })
            .unwrap_or(next.0);

        CURRENT_THREAD = Some(next);

        Some(ThreadInfo {
            tid,
            state: encode_state(state),
            priority,
        })
    }
}

static BOOT_PROGRAMS: &[(&str, u64, u64, &str)] = &[
    ("hello", 1, 0, "hello"),
    ("heartbeat", 2, 0, "heartbeat"),
    ("thread_dashboard", 3, 0, "thread_dashboard"),
];

pub fn init_boot_profile() {
    let profile_props = &[("version", PropValue::U64(1))];
    let Some(profile) = graph::create_thing(graph_kinds::KIND_BOOT_PROFILE, profile_props) else {
        crate::log("Failed to create BootProfile Thing");
        return;
    };

    for (name, app_id, priority, binary) in BOOT_PROGRAMS {
        let props = &[
            ("name", PropValue::Str(String::from(*name))),
            ("app_id", PropValue::U64(*app_id)),
            ("priority", PropValue::U64(*priority)),
            ("binary", PropValue::Str(String::from(*binary))),
        ];
        if let Some(program) = graph::create_thing(graph_kinds::KIND_BOOT_PROGRAM, props) {
            let _ = graph::add_edge(profile, graph_kinds::EDGE_LAUNCHES, program);
        } else {
            crate::log("Failed to create BootProgram Thing");
        }
    }
}

fn encode_state(state: ThreadState) -> u64 {
    match state {
        ThreadState::Running => 1,
        ThreadState::Runnable => 2,
        ThreadState::Sleeping => 3,
        ThreadState::Blocked => 4,
        ThreadState::Exited => 5,
        ThreadState::New => 0,
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
pub fn create_program_image(
    identifier: &str,
    module_index: u64,
    base_phys: u64,
    size: u64,
) -> Option<ThingId> {
    let props = &[
        ("identifier", PropValue::Str(String::from(identifier))),
        ("module_index", PropValue::U64(module_index)),
        ("base_phys", PropValue::U64(base_phys)),
        ("size", PropValue::U64(size)),
    ];
    graph::create_thing(graph_kinds::KIND_PROGRAM_IMAGE, props)
}
