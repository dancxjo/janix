//! Kernel data model schemas and creation helpers
//!
//! This module defines Thing schemas for memory management, scheduling, and
//! resource tracking. These are graph-only models - no actual hardware or
//! context switching is implemented here.

extern crate alloc;

use crate::sched_types::ThreadState;
use crate::{graph, graph_kinds, memory, sched_graph};
use abi::{
    FrameId, FrameInfo, MemorySummary, PropType, PropValue, SchedulerSummary, Thing, ThingId,
    ThreadId, ThreadInfo,
};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use thing_models::{
    AlarmEvent, AlarmRequest, BootProfile, BootProgram, FontModule, InterruptEvent, IoPortOp,
    IoPortRegion, ProgramImage, TimeSource,
};

#[derive(Clone, Copy)]
pub struct Thread {
    pub id: ThreadId,
    pub process_id: u64,
    pub state: ThreadState,
    pub entry_point: u64,
    pub user_arg: u64,
    pub user_stack_top: u64,
    pub address_space_token: Option<u64>,
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
                    entry_point: entry as u64,
                    user_arg: app_id,
                    user_stack_top: stack_top,
                    address_space_token: None,
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

/// Initialize all kernel model schemas.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// k::model::init_schemas();
/// assert!(k::graph::get_schema_description(k::graph_kinds::KIND_THREAD).is_some());
/// ```
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
        &[],
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
        &[],
    );

    // AddressSpace: represents a virtual address space
    static ADDRESS_SPACE_SCHEMA: &[(&str, PropType)] = &[("asid", PropType::U64)];
    let _ = graph::register_schema(
        "AddressSpace",
        "A virtual address space identified by its address space identifier (ASID)",
        ADDRESS_SPACE_SCHEMA,
        &[],
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
        &[],
    );

    // Process: represents a process
    static PROCESS_SCHEMA: &[(&str, PropType)] = &[("pid", PropType::U64), ("name", PropType::Str)];
    let _ = graph::register_schema(
        graph_kinds::KIND_PROCESS,
        "A process with process identifier (PID) and execution state",
        PROCESS_SCHEMA,
        &[],
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
        &[],
    );

    // CpuCore: represents a CPU core
    static CPU_CORE_SCHEMA: &[(&str, PropType)] = &[("index", PropType::U64)];
    let _ = graph::register_schema(
        graph_kinds::KIND_CPU_CORE,
        "A CPU core identified by its index in the system",
        CPU_CORE_SCHEMA,
        &[],
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
        &[],
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_BOOT_PROFILE,
        BootProfile::DESCRIPTION,
        BootProfile::schema(),
        &[],
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_BOOT_PROGRAM,
        BootProgram::DESCRIPTION,
        BootProgram::schema(),
        &[],
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_PROGRAM_IMAGE,
        ProgramImage::DESCRIPTION,
        ProgramImage::schema(),
        &[],
    );
    let _ = graph::register_schema(
        graph_kinds::KIND_FONT_MODULE,
        FontModule::DESCRIPTION,
        FontModule::schema(),
        &[],
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_TIME_SOURCE,
        TimeSource::DESCRIPTION,
        TimeSource::schema(),
        &[],
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_IO_PORT_REGION,
        IoPortRegion::DESCRIPTION,
        IoPortRegion::schema(),
        &[],
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_IO_PORT_OP,
        IoPortOp::DESCRIPTION,
        IoPortOp::schema(),
        &[],
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_INTERRUPT_EVENT,
        InterruptEvent::DESCRIPTION,
        InterruptEvent::schema(),
        &[],
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_ALARM_REQUEST,
        AlarmRequest::DESCRIPTION,
        AlarmRequest::schema(),
        &[],
    );

    let _ = graph::register_schema(
        graph_kinds::KIND_ALARM_EVENT,
        AlarmEvent::DESCRIPTION,
        AlarmEvent::schema(),
        &[],
    );

    // Display pipeline schemas
    static DISPLAY_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_NAME, PropType::Str),
        (graph_kinds::PROP_WIDTH, PropType::U64),
        (graph_kinds::PROP_HEIGHT, PropType::U64),
        (graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX, PropType::I64),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_DISPLAY,
        "A display sink capable of scanning out a SharedBuffer",
        DISPLAY_SCHEMA,
        &[],
    );

    static SHARED_BUFFER_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_WIDTH, PropType::U64),
        (graph_kinds::PROP_HEIGHT, PropType::U64),
        (graph_kinds::PROP_STRIDE, PropType::U64),
        (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_SHARED_BUFFER,
        "A kernel-owned shared memory buffer that can be mapped into userland",
        SHARED_BUFFER_SCHEMA,
        &[],
    );

    static DISPLAY_FRAMEBUFFER_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_NAME, PropType::Str),
        (graph_kinds::PROP_WIDTH, PropType::U64),
        (graph_kinds::PROP_HEIGHT, PropType::U64),
        (graph_kinds::PROP_STRIDE, PropType::U64),
        (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
        (graph_kinds::PROP_POWER_STATE, PropType::Str),
        (graph_kinds::PROP_REFRESH_INTERVAL_NS, PropType::U64),
        (graph_kinds::PROP_FRAMES_PRESENTED, PropType::U64),
        (graph_kinds::PROP_LAST_PRESENT_NS, PropType::U64),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_DISPLAY_FRAMEBUFFER,
        "A userland-published framebuffer description backed by a SharedBuffer",
        DISPLAY_FRAMEBUFFER_SCHEMA,
        &[],
    );

    static DISPLAY_FRAME_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_WIDTH, PropType::U64),
        (graph_kinds::PROP_HEIGHT, PropType::U64),
        (graph_kinds::PROP_STRIDE, PropType::U64),
        (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_DISPLAY_FRAME,
        "A single frame produced by a compositor targeting a framebuffer",
        DISPLAY_FRAME_SCHEMA,
        &[],
    );
    static DISPLAY_PRESENT_REQUEST_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_FRAMEBUFFER_ID, PropType::U64),
        (graph_kinds::PROP_FRAME_INDEX, PropType::U64),
        (graph_kinds::PROP_REQUESTED_AT_NS, PropType::U64),
        (graph_kinds::PROP_PRESENTED_AT_NS, PropType::U64),
        (graph_kinds::PROP_COMPLETED, PropType::Bool),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_DISPLAY_PRESENT_REQUEST,
        "A compositor request asking a framebuffer driver to present a frame",
        DISPLAY_PRESENT_REQUEST_SCHEMA,
        &[],
    );

    // Mode and windowing schemas
    static MODE_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_MODE_INDEX, PropType::U64),
        (graph_kinds::PROP_NAME, PropType::Str),
        (graph_kinds::PROP_MODE_PLACE, PropType::U64),
        (graph_kinds::PROP_MODE_ACTIVE, PropType::Bool),
        (graph_kinds::PROP_MODE_LAYOUT_POLICY, PropType::I64),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_MODE,
        "Logical system mode (F1-F12)",
        MODE_SCHEMA,
        &[],
    );

    static MODE_SWITCH_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_MODE_INDEX, PropType::U64),
        (graph_kinds::PROP_TIMESTAMP, PropType::U64),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_MODE_SWITCH_EVENT,
        "Input event requesting a mode change",
        MODE_SWITCH_SCHEMA,
        &[],
    );

    static PLACE_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_NAME, PropType::Str),
        (graph_kinds::PROP_LAYOUT_MODE, PropType::Str),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_PLACE,
        "Workspace root for windows",
        PLACE_SCHEMA,
        &[],
    );

    static WINDOW_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_PLACE_ID, PropType::U64),
        (graph_kinds::PROP_WINDOW_X, PropType::I64),
        (graph_kinds::PROP_WINDOW_Y, PropType::I64),
        (graph_kinds::PROP_WINDOW_WIDTH, PropType::I64),
        (graph_kinds::PROP_WINDOW_HEIGHT, PropType::I64),
        (graph_kinds::PROP_Z_INDEX, PropType::I64),
        (graph_kinds::PROP_WINDOW_ACTIVE, PropType::Bool),
        (graph_kinds::PROP_TITLE, PropType::Str),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_WINDOW,
        "Top-level window description",
        WINDOW_SCHEMA,
        &[],
    );

    static SURFACE_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_WINDOW_ID, PropType::U64),
        (graph_kinds::PROP_SURFACE_KIND, PropType::Str),
        (graph_kinds::PROP_SURFACE_TEXT, PropType::Str),
    ];
    let _ = graph::register_schema(
        graph_kinds::KIND_SURFACE,
        "Renderable surface attached to a window",
        SURFACE_SCHEMA,
        &[],
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
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let frame = k::model::create_phys_frame(0x1000, 4096).unwrap();
/// assert!(matches!(k::graph::get_prop(frame, "base"), Some(abi::PropValue::U64(0x1000))));
/// assert!(matches!(k::graph::get_prop(frame, "allocated"), Some(abi::PropValue::Bool(false))));
/// ```
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
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let pool = k::model::create_frame_pool(0x1000, 0x2000, 4096).unwrap();
/// assert!(matches!(k::graph::get_prop(pool, "frame_size"), Some(abi::PropValue::U64(4096))));
/// ```
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
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let cpu = k::model::create_cpu_core(0).unwrap();
/// let idx = k::graph::get_prop(cpu, "index");
/// assert!(matches!(idx, Some(abi::PropValue::U64(0))));
/// ```
pub fn create_cpu_core(index: u64) -> Option<ThingId> {
    let props = &[("index", PropValue::U64(index))];

    graph::create_thing(graph_kinds::KIND_CPU_CORE, props)
}

/// Compute memory statistics from graph state.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// k::model::create_phys_frame(0x1000, 4096);
/// k::model::create_phys_frame(0x2000, 4096);
/// let summary = k::model::compute_memory_summary();
/// assert!(summary.total_frames >= 2);
/// assert_eq!(summary.used_frames + summary.free_frames, summary.total_frames);
/// ```
pub fn compute_memory_summary() -> MemorySummary {
    let (total_frames, used_frames, free_frames) = memory::frame_stats();
    if total_frames > 0 {
        return MemorySummary {
            total_frames,
            used_frames,
            free_frames,
        };
    }

    let mut total_frames = 0_u64;
    let mut used_frames = 0_u64;

    graph::iter_things(|thing| {
        if thing.kind != "PhysFrame" {
            return;
        }
        total_frames += 1;

        for p in thing.props.iter().flatten() {
            if let (key, PropValue::Bool(allocated)) = p {
                if *key == "allocated" && *allocated {
                    used_frames += 1;
                }
            }
        }
    });

    MemorySummary {
        total_frames,
        used_frames,
        free_frames: total_frames.saturating_sub(used_frames),
    }
}

/// Compute scheduler statistics from graph state.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// k::model::create_process(1);
/// k::model::create_thread(1, 1);
/// let summary = k::model::compute_scheduler_summary();
/// assert!(summary.process_count >= 1);
/// assert!(summary.thread_count >= 1);
/// ```
pub fn compute_scheduler_summary() -> SchedulerSummary {
    let mut process_count = 0_u64;
    let mut thread_count = 0_u64;
    let mut runnable_threads = 0_u64;

    graph::iter_things(|thing| {
        match thing.kind {
            graph_kinds::KIND_PROCESS => {
                process_count += 1;
            }
            graph_kinds::KIND_THREAD => {
                thread_count += 1;

                // state == Running or Runnable counts as runnable
                let mut state = None;
                for p in thing.props.iter().flatten() {
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
    });

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
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let asid = k::model::create_address_space(7).unwrap();
/// assert!(matches!(k::graph::get_prop(asid, "asid"), Some(abi::PropValue::U64(7))));
/// ```
pub fn create_address_space(asid: u64) -> Option<ThingId> {
    let props = &[("asid", PropValue::U64(asid))];

    graph::create_thing("AddressSpace", props)
}

/// Create a Display Thing
///
/// # Arguments
/// * `name` - Display name
/// * `width` - Display width in pixels
/// * `height` - Display height in pixels
/// * `active_buffer_id` - ThingId of the currently active shared buffer
///
/// # Returns
/// ThingId of the created Display, or None if creation failed
pub fn create_display(
    name: &str,
    width: u64,
    height: u64,
    active_buffer_id: u64,
) -> Option<ThingId> {
    let props = &[
        (graph_kinds::PROP_NAME, PropValue::Str(String::from(name))),
        (graph_kinds::PROP_WIDTH, PropValue::U64(width)),
        (graph_kinds::PROP_HEIGHT, PropValue::U64(height)),
        (
            graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX,
            PropValue::I64(active_buffer_id as i64),
        ),
    ];

    graph::create_thing(graph_kinds::KIND_DISPLAY, props)
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
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let vr = k::model::create_virt_region(0x4000, 0x1000, 0x7).unwrap();
/// assert!(matches!(k::graph::get_prop(vr, "len"), Some(abi::PropValue::U64(0x1000))));
/// ```
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
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let proc = k::model::create_process(1).unwrap();
/// let pid = k::graph::get_prop(proc, "pid");
/// assert!(matches!(pid, Some(abi::PropValue::U64(1))));
/// ```
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
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let thread = k::model::create_thread(42, 10).unwrap();
/// let state = k::graph::get_prop(thread, "state");
/// assert!(matches!(state, Some(abi::PropValue::Str(s)) if s.as_str() == k::sched_types::ThreadState::Runnable.as_str()));
/// ```
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

/// Allocate the first free `PhysFrame` and mark it allocated.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let phys = k::model::create_phys_frame(0x2000, 4096).unwrap();
/// let frame = k::model::alloc_frame().unwrap();
/// assert_eq!(frame.id.0, phys.0);
/// assert!(matches!(k::graph::get_prop(phys, "allocated"), Some(abi::PropValue::Bool(true))));
/// ```
pub fn alloc_frame() -> Option<FrameInfo> {
    // For now, scan all Things for the first PhysFrame with allocated == false
    let mut found_frame = None;

    graph::iter_things(|thing| {
        if found_frame.is_some() {
            return;
        }

        if thing.kind != "PhysFrame" {
            return;
        }

        let mut base = 0_u64;
        let mut size = 0_u64;
        let mut allocated = false;

        for p in thing.props.iter().flatten() {
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
             found_frame = Some((thing.id, base, size));
        }
    });

    if let Some((tid, base, size)) = found_frame {
        // mark as allocated
        let new_props = &[("allocated", PropValue::Bool(true))];
        crate::graph::update_thing(tid, new_props);

        return Some(FrameInfo {
            id: FrameId(tid.0),
            base,
            size,
        });
    }

    None
}

/// Mark a physical frame as free again.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let phys = k::model::create_phys_frame(0x1000, 4096).unwrap();
/// let info = k::model::alloc_frame().unwrap();
/// assert_eq!(info.id.0, phys.0);
/// assert!(k::model::free_frame(info.id));
/// assert!(matches!(k::graph::get_prop(phys, "allocated"), Some(abi::PropValue::Bool(false))));
/// ```
pub fn free_frame(frame_id: FrameId) -> bool {
    let tid = ThingId(frame_id.0);
    if !is_kind(tid, "PhysFrame") {
        return false;
    }

    let props = &[("allocated", PropValue::Bool(false))];
    crate::graph::update_thing(tid, props)
}

/// ABI helper to create a process and return its raw ThingId.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let pid = k::model::create_process_abi(2).unwrap();
/// let thing = abi::ThingId(pid);
/// assert!(matches!(k::graph::get_prop(thing, "pid"), Some(abi::PropValue::U64(2))));
/// ```
pub fn create_process_abi(pid: u64) -> Option<u64> {
    create_process(pid).map(|id| id.0)
}

/// Create a thread from an ABI call, returning the ThingId raw value.
///
/// The first thread created becomes the current thread.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let tid = k::model::create_thread_abi(1, 10, 1).unwrap();
/// let thing = abi::ThingId(tid);
/// assert!(matches!(k::graph::get_prop(thing, "tid"), Some(abi::PropValue::U64(10))));
/// ```
pub fn create_thread_abi(_pid: u64, tid: u64, priority: u64) -> Option<u64> {
    // For now, we ignore pid in the graph; later we’ll add links.
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

/// Run a scheduler tick against the graph-backed model and return the selected thread info.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// k::init_boot_graph();
/// let info = k::model::scheduler_tick().unwrap();
/// assert!(info.tid > 0);
/// ```
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

pub fn init_boot_profile() {
    // Avoid creating duplicate profiles if already seeded.
    if let Some(existing) =
        graph::next_thing_of_kind(graph_kinds::KIND_BOOT_PROFILE, ThingId(u64::MAX))
    {
        let msg = format!("Found existing BootProfile Thing id={}", existing.0);
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        crate::log(leaked);
        return;
    }

    let profile_props = &[("version", PropValue::U64(1))];
    let Some(profile) = graph::create_thing(graph_kinds::KIND_BOOT_PROFILE, profile_props) else {
        crate::log("Failed to create BootProfile Thing");
        return;
    };
    let msg = format!("Created BootProfile Thing id={}", profile.0);
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    crate::log(leaked);
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
    graph::iter_things(|thing| {
        counts.total_things += 1;

        match thing.kind {
            "Process" => counts.processes += 1,
            "Thread" => counts.threads += 1,
            "PhysFrame" => counts.phys_frames += 1,
            "VirtRegion" => counts.virt_regions += 1,
            "FramePool" => counts.frame_pools += 1,
            "AddressSpace" => counts.address_spaces += 1,
            "CpuCore" => counts.cpu_cores += 1,
            _ => {}
        }
    });

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
