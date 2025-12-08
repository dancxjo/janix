//! Kernel data model schemas and creation helpers
//! 
//! This module defines Thing schemas for memory management, scheduling, and
//! resource tracking. These are graph-only models - no actual hardware or
//! context switching is implemented here.

use crate::graph;
use abi::{PropType, PropValue, ThingId};

// State constants for Process and Thread Things
pub const STATE_RUNNING: u64 = 1;
pub const STATE_READY: u64 = 2;
pub const STATE_BLOCKED: u64 = 3;
pub const STATE_TERMINATED: u64 = 4;

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
    static ADDRESS_SPACE_SCHEMA: &[(&str, PropType)] = &[
        ("asid", PropType::U64),
    ];
    let _ = graph::register_schema("AddressSpace", ADDRESS_SPACE_SCHEMA);

    // VirtRegion: represents a virtual memory region
    static VIRT_REGION_SCHEMA: &[(&str, PropType)] = &[
        ("base", PropType::U64),
        ("len", PropType::U64),
        ("flags", PropType::U64),
    ];
    let _ = graph::register_schema("VirtRegion", VIRT_REGION_SCHEMA);

    // Process: represents a process
    static PROCESS_SCHEMA: &[(&str, PropType)] = &[
        ("pid", PropType::U64),
        ("state", PropType::U64),
    ];
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
    static CPU_CORE_SCHEMA: &[(&str, PropType)] = &[
        ("index", PropType::U64),
    ];
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

/// Create an AddressSpace Thing
/// 
/// # Arguments
/// * `asid` - Address space identifier
/// 
/// # Returns
/// ThingId of the created AddressSpace, or None if creation failed
pub fn create_address_space(asid: u64) -> Option<ThingId> {
    let props = &[
        ("asid", PropValue::U64(asid)),
    ];
    
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

/// Create a CpuCore Thing
/// 
/// # Arguments
/// * `index` - CPU core index
/// 
/// # Returns
/// ThingId of the created CpuCore, or None if creation failed
pub fn create_cpu_core(index: u64) -> Option<ThingId> {
    let props = &[
        ("index", PropValue::U64(index)),
    ];
    
    graph::create_thing("CpuCore", props)
}
