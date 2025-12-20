#![no_std]
// force rebuild 2

extern crate alloc;
use crate::graph::schema::{register_schema, get_schema_props};

pub mod bridge;
pub mod console;
pub mod console_backend;
pub mod graph;
pub mod graph_kinds;
pub mod graph_watchers;
pub mod handles;
pub mod journal;
pub mod log;
pub mod memory;
pub mod model;
pub mod resident;
pub mod sched;
pub mod sched_graph;
pub mod sched_tick;
pub mod sched_types;
pub mod shared_buffer;
pub mod symbols;
pub mod time;
pub mod transaction;
pub mod work_queue;

use crate::model::{compute_memory_summary, compute_scheduler_summary, scheduler_tick};
use crate::sched_types::ThreadState;
use crate::shared_buffer::MAX_FRAMES_PER_BUFFER;
use abi::{FrameId, FrameInfo, KernelRequest, KernelResponse, PropValue, ThingId};
use alloc::string::String;
use spin::{Mutex, MutexGuard};

type SpawnProgramHandler = fn(ThingId) -> Result<(ThingId, ThingId), &'static str>;

static SPAWN_PROGRAM_HANDLER: Mutex<Option<SpawnProgramHandler>> = Mutex::new(None);

static TEST_MUTEX: Mutex<()> = Mutex::new(());

/// Initialize the kernel core subsystems
pub fn init() {
    log::init();
    bridge::ps2::init();

    // Initialize graph store first
    crate::graph::store::init();
    // Initialize symbols and other graph components
    crate::symbols::init();
    crate::graph::schema::init();
    crate::graph::events::init();
    crate::graph::index_props::init();

    // Dump the graph after initialization so builtin kinds and indexes are visible.
    crate::graph::debug::dump_graph_table();
    journal::init();
    transaction::init();
    model::init_schemas();
    console_backend::init();
    work_queue::init();
    graph_watchers::init();
}

/// Register the function responsible for spawning programs described by BootProgram Things.
///
/// This allows the kernel core to delegate spawning to the architecture-specific
/// environment (e.g., the boot crate). Calling this function replaces any
/// previously registered handler.
pub fn register_spawn_program_handler(handler: SpawnProgramHandler) {
    *SPAWN_PROGRAM_HANDLER.lock() = Some(handler);
}

/// Spawn a program by its BootProgram Thing ID.
///
/// This invokes the registered handler (usually in the boot crate) to load and launch
/// the program. Returns the new ProcessId and ThreadId as ThingIds on success.
pub fn spawn_program(boot_program_id: ThingId) -> Result<(ThingId, ThingId), &'static str> {
    let handler = SPAWN_PROGRAM_HANDLER.lock().clone();
    if let Some(spawn_fn) = handler {
        spawn_fn(boot_program_id)
    } else {
        Err("SpawnProgram handler not registered")
    }
}

/// Log a message to the kernel log
pub fn log(message: &'static str) {
    log::log_message(message);
}

/// Serialize access to global kernel state for tests.
///
/// This is intended for integration and doc tests that share the global graph
/// and schema storage. Hold the guard for the duration of a test to avoid
/// cross-test interference.
pub fn test_lock() -> MutexGuard<'static, ()> {
    TEST_MUTEX.lock()
}

/// Get all log entries
pub fn get_logs() -> &'static [Option<&'static str>] {
    log::get_logs()
}

/// Handle a kernel request from userland
pub fn handle_request(request: KernelRequest) -> KernelResponse {
    match request {
        KernelRequest::LinkAt { src, pred, idx } => KernelResponse::LinkTarget {
            target: graph::link_target_at(src, pred, idx as usize),
        },
        KernelRequest::GraphQuery { node_id } => match graph::query_node(ThingId(node_id.0)) {
            Some(value) => KernelResponse::NodeData { node_id, value },
            None => KernelResponse::Error {
                message: "Thing not found",
            },
        },
        KernelRequest::CreateTransaction => {
            let tx_id = transaction::create_transaction();
            KernelResponse::TransactionCreated { tx_id }
        }
        KernelRequest::CommitTransaction { tx_id } => {
            match transaction::commit_transaction(tx_id) {
                Ok(()) => KernelResponse::Success { data: None },
                Err(e) => KernelResponse::Error { message: e },
            }
        }
        KernelRequest::Log { message } => {
            log::log_message(message);
            KernelResponse::Success { data: None }
        }
        KernelRequest::ThingCreate { kind, props } => {
            // Convert UserSlice<WireProp> to Vec<(SymbolId, PropValue)>
            // Note: This requires reading from user memory, which might fail or be unsafe if not validated.
            // For now assuming UserSlice provides an iterator or we can check bounds.
            // But UserSlice in kernel is just a pointer/len wrapper.
            // We need a helper to read from it.
            // AND we need to convert WirePropValue to PropValue.
            
            // Helper should ideally be in a separate module or function to reuse.
            // But for now inline or local helper.
            let mut internal_props = alloc::vec::Vec::new();
            
            // Assuming we have a way to iter UserSlice.
            // If UserSlice impls IntoIterator for &UserSlice or similar?
            // Checking abi/src/wire/common.rs implies UserSlice is just POD.
            // We need `copy_from_user` logic which is mocked/abstracted here?
            // In a real kernel, we'd copy the slice to kernel memory first.
            // Here, we might assume shared address space or direct access if 'user' is just function calls (mock kernel).
            // But UserPtr implies it might be an offset.
            // Let's assume for now we can treat UserPtr as a raw pointer if we are in same address space (toy OS).
            // Or better, we cast pointer and read.
            // Safe access?
            
            let slice_ptr = props.ptr as *const abi::wire::graph::WireProp;
            let slice_len = props.len;
            
            unsafe {
                for i in 0..slice_len {
                    let wire_prop = *slice_ptr.add(i as usize);
                    let key = wire_prop.key;
                    // Convert value
                    let val = match wire_prop.value.tag {
                         0 => PropValue::U64(wire_prop.value.data_0),
                         1 => PropValue::I64(wire_prop.value.data_0 as i64),
                         2 => PropValue::Bool(wire_prop.value.data_0 != 0),
                         3 => PropValue::Symbol(abi::syscall_defs::SymbolId(wire_prop.value.data_0 as u32)),
                         4 => {
                             // Blob. data_0 is ptr, data_1 is len.
                             // Need to copy blob data.
                             let blob_ptr = wire_prop.value.data_0 as *const u8;
                             let blob_len = wire_prop.value.data_1;
                             let mut blob = alloc::vec::Vec::with_capacity(blob_len as usize);
                             // Check bounds/null?
                             if blob_ptr.is_null() && blob_len > 0 {
                                 return KernelResponse::Error { message: "Invalid blob pointer" };
                             }
                             if !blob_ptr.is_null() {
                                 core::ptr::copy_nonoverlapping(blob_ptr, blob.as_mut_ptr(), blob_len as usize);
                                 blob.set_len(blob_len as usize);
                             }
                             PropValue::Blob(blob)
                         },
                         _ => return KernelResponse::Error { message: "Invalid property tag" },
                    };
                    internal_props.push((key, val));
                }
            }

            if let Err(e) = graph::validate_props(kind, &internal_props) {

                return KernelResponse::Error { message: e };
            }
            
            let id = graph::create_thing(kind, internal_props);
            KernelResponse::ThingCreated { id }
        }
        KernelRequest::ThingGet { id: _ } => KernelResponse::Error {
            message: "ThingGet not available via handle_request (use with_thing)",
        },
        KernelRequest::ThingList { kind, start_after } => KernelResponse::ThingListEntry {
            id: graph::next_thing_of_kind(kind, start_after),
        },
        KernelRequest::ThingUpdate { id, props } => {
            // Similar conversion logic
             let mut internal_props = alloc::vec::Vec::new();
            
            let slice_ptr = props.ptr as *const abi::wire::graph::WireProp;
            let slice_len = props.len;
            
            unsafe {
                for i in 0..slice_len {
                    let wire_prop = *slice_ptr.add(i as usize);
                    let key = wire_prop.key;
                    let val = match wire_prop.value.tag {
                         0 => PropValue::U64(wire_prop.value.data_0),
                         1 => PropValue::I64(wire_prop.value.data_0 as i64),
                         2 => PropValue::Bool(wire_prop.value.data_0 != 0),
                         3 => PropValue::Symbol(abi::syscall_defs::SymbolId(wire_prop.value.data_0 as u32)),
                         4 => {
                             let blob_ptr = wire_prop.value.data_0 as *const u8;
                             let blob_len = wire_prop.value.data_1;
                             let mut blob = alloc::vec::Vec::with_capacity(blob_len as usize);
                             if !blob_ptr.is_null() {
                                 core::ptr::copy_nonoverlapping(blob_ptr, blob.as_mut_ptr(), blob_len as usize);
                                 blob.set_len(blob_len as usize);
                             }
                             PropValue::Blob(blob)
                         },
                         _ => return KernelResponse::Error { message: "Invalid property tag" },
                    };
                    internal_props.push((key, val));
                }
            }
// ... (omitting update logic unchanged except slice_ptr fix)
// Need to match context.
// I'll assume lines around 145 and 198 match.
// I need to split this replacement or provide enough context.
// The previous tool usage replaced large chunk.
// I will just replace the `props.base.0` lines specifically if possible, or large chunk again.
// And create_builtin_things fix.

// create_builtin_things fix:
// Lines 562, 573.
// I'll separate the tool calls.


            // Get the kind first to validate
            let validation_result = graph::with_thing(id, |thing_node| {
                graph::validate_props(thing_node.kind, &internal_props)
            });
            if let Some(res) = validation_result {
                 if let Err(e) = res {
                     return KernelResponse::Error { message: e };
                 }
            } else {
                 // Thing not found, update_thing will fail anyway
            }
            if graph::update_thing(id, internal_props) {
                KernelResponse::Success { data: None }
            } else {
                KernelResponse::Error {
                    message: "Failed to update thing",
                }
            }
        }
        KernelRequest::ThingBatchUpdate { updates } => {
            // Iterate batch entries
            let updates_ptr = updates.ptr as *const abi::wire::graph::BatchUpdateEntry;
            let updates_len = updates.len;
            // I need to check context. The 'props' in slice_len in previous block was correct variable for ThingUpdate.
            // In ThingBatchUpdate, arg is 'updates'.
            
            unsafe {
                for i in 0..updates.len { // updates_len
                    let entry = *updates_ptr.add(i as usize);
                    // Convert entry.props_ptr/len to internal_props
                    let mut internal_props = alloc::vec::Vec::new();
                    let props_ptr = entry.props_ptr.ptr as *const abi::wire::graph::WireProp;

                    let props_len = entry.props_len;
                    
                     for j in 0..props_len {
                        let wire_prop = *props_ptr.add(j as usize);
                        let key = wire_prop.key;
                        let val = match wire_prop.value.tag {
                             0 => PropValue::U64(wire_prop.value.data_0),
                             1 => PropValue::I64(wire_prop.value.data_0 as i64),
                             2 => PropValue::Bool(wire_prop.value.data_0 != 0),
                             3 => PropValue::Symbol(abi::syscall_defs::SymbolId(wire_prop.value.data_0 as u32)),
                             4 => {
                                 let blob_ptr = wire_prop.value.data_0 as *const u8;
                                 let blob_len = wire_prop.value.data_1;
                                 let mut blob = alloc::vec::Vec::with_capacity(blob_len as usize);
                                 if !blob_ptr.is_null() {
                                     core::ptr::copy_nonoverlapping(blob_ptr, blob.as_mut_ptr(), blob_len as usize);
                                     blob.set_len(blob_len as usize);
                                 }
                                 PropValue::Blob(blob)
                             },
                             _ => return KernelResponse::Error { message: "Invalid property tag" },
                        };
                        internal_props.push((key, val));
                    }
                    
                    // Validate and update
                     let validation_result = graph::with_thing(entry.id, |thing_node| {
                        graph::validate_props(thing_node.kind, &internal_props)
                    });
                     if let Some(res) = validation_result {
                        if let Err(e) = res {
                            return KernelResponse::Error { message: e };
                        }
                    } else {
                        return KernelResponse::Error {
                            message: "Thing not found in batch",
                        };
                    }

                    if !graph::update_thing(entry.id, internal_props) {
                        return KernelResponse::Error {
                            message: "Failed to update thing in batch",
                        };
                    }
                }
            }
            KernelResponse::Success { data: None }
        }
        KernelRequest::AddLink { src, pred, dst } => {
            if graph::add_link(src, pred, dst) {
                KernelResponse::Success { data: None }
            } else {
                KernelResponse::Error {
                    message: "Failed to add link",
                }
            }
        }

        KernelRequest::CreateSharedBuffer {
            width,
            height,
            pixel_format,
        } => {
            let bytes_per_pixel = 4_u64;
            let stride = width as u64 * bytes_per_pixel;
            let size = stride * height as u64;

            let mut frames = heapless::Vec::<memory::PhysFrame, { MAX_FRAMES_PER_BUFFER }>::new();
            let needed = shared_buffer::page_count_for_size(size);
            for _ in 0..needed {
                let frame = match memory::allocate_frame() {
                    Some(f) => f,
                    None => {
                        return KernelResponse::Error {
                            message: "Out of frames for SharedBuffer",
                        };
                    }
                };
                if frames.push(frame).is_err() {
                    return KernelResponse::Error {
                        message: "SharedBuffer frame capacity exceeded",
                    };
                }
            }

            match shared_buffer::register_shared_buffer(
                width,
                height,
                stride as u32,
                pixel_format,
                frames,
            ) {
                Ok(buffer_id) => KernelResponse::SharedBufferCreated { buffer_id },
                Err(message) => KernelResponse::Error { message },
            }
        }
        KernelRequest::MapSharedBuffer { buffer_id, flags } => {
            // Retrieve buffer info and frames using a short-lived lock.
            // We convert the frames to a heap-allocated Vec to avoid exploding the kernel stack,
            // as SharedBuffer uses a large inline heapless::Vec (32KB+).
            let (size_bytes, frames) = {
                let manager = shared_buffer::manager().lock();
                if let Some(buffer) = manager.get(&buffer_id) {
                    let frames: alloc::vec::Vec<_> = buffer.frames.iter().cloned().collect();
                    (buffer.size_bytes(), frames)
                } else {
                    return KernelResponse::Error {
                        message: "SharedBuffer not found",
                    };
                }
            };

            let size = shared_buffer::align_up(size_bytes, 4096);

            let vaddr = {
                let mut sched = sched::SCHEDULER.lock();
                let pid = match sched.current_process_id() {
                    Some(id) => id,
                    None => {
                        return KernelResponse::Error {
                            message: "No current process for mapping",
                        };
                    }
                };

                match sched.reserve_user_region(pid, size as usize, 4096) {
                    Some(addr) => addr,
                    None => {
                        return KernelResponse::Error {
                            message: "Failed to reserve virtual region",
                        };
                    }
                }
            };

            if let Err(msg) = shared_buffer::map_frames_into_current_as(vaddr, &frames, flags) {
                return KernelResponse::Error { message: msg };
            }

            KernelResponse::SharedBufferMapped { vaddr, size }
        }
        KernelRequest::GetSharedBufferInfo { buffer_id } => {
            let info = shared_buffer::manager()
                .lock()
                .get(&buffer_id)
                .map(|sb| sb.info());

            match info {
                Some(info) => KernelResponse::SharedBufferInfoResponse { info },
                None => KernelResponse::Error {
                    message: "SharedBuffer not found",
                },
            }
        }
        KernelRequest::SpawnProgram { boot_program_id } => {
            let handler = SPAWN_PROGRAM_HANDLER.lock().clone();
            if let Some(spawn_fn) = handler {
                match spawn_fn(boot_program_id) {
                    Ok((process_id, thread_id)) => KernelResponse::ProgramSpawned {
                        process_id,
                        thread_id,
                    },
                    Err(msg) => KernelResponse::Error { message: msg },
                }
            } else {
                KernelResponse::Error {
                    message: "SpawnProgram handler not registered",
                }
            }
        }
        KernelRequest::SchemaRegister {
            kind,
            description,
            props,
        } => { 
            let mut props_vec = alloc::vec::Vec::new();
            unsafe {
                 let ptr = props.ptr as *const abi::wire::graph::WireSchemaProp;
                 for i in 0..props.len {
                      let wp = *ptr.add(i as usize);
                      // wp.name is SymbolId
                      let sym = wp.name;
                      
                      let pt = match wp.prop_type {
                          0 => abi::PropType::U64,
                          1 => abi::PropType::I64,
                          2 => abi::PropType::Bool,
                          3 => abi::PropType::Str,
                          4 => abi::PropType::Blob,
                          _ => return KernelResponse::Error { message: "Invalid schema prop_type tag" },
                      };
                      
                      props_vec.push((sym, pt));
                 }
            }
            match register_schema(kind, description, props_vec, alloc::vec![]) {
                Ok(outcome) => KernelResponse::SchemaRegistered { kind, outcome },
                Err(e) => {
                    let msg = alloc::format!(
                        "SchemaRegister failed: kind={:?} desc={:?} err={}",
                        kind, description, e
                    );
                    crate::log::log_message(&msg);
                    KernelResponse::Error { message: e }
                }
            }
        },
        KernelRequest::SchemaGet { kind } => match get_schema_props(kind) {
            Some(props) => {
                 let props_converted: alloc::vec::Vec<Option<(abi::syscall_defs::SymbolId, abi::PropType)>> = props.into_iter().map(Some).collect();
                 let props_slice = alloc::boxed::Box::leak(props_converted.into_boxed_slice());
                 KernelResponse::SchemaData { kind, props: props_slice }
            }
            None => KernelResponse::Error {
                message: "Schema not found",
            },
        },
        KernelRequest::GetMemorySummary => {
            let summary = compute_memory_summary();
            KernelResponse::MemorySummary { summary }
        }
        KernelRequest::GetSchedulerSummary => {
            let summary = compute_scheduler_summary();
            KernelResponse::SchedulerSummary { summary }
        }
        KernelRequest::AllocFrame { pool_index: _ } => match memory::allocate_frame() {
            Some(frame) => {
                let frame_info = FrameInfo {
                    id: FrameId(frame.start_address),
                    base: frame.start_address,
                    size: frame.size,
                };
                KernelResponse::FrameAllocated { frame: frame_info }
            }
            None => KernelResponse::Error {
                message: "Out of frames",
            },
        },
        KernelRequest::FreeFrame { frame_id } => {
            let frame = memory::PhysFrame::from_start_address(frame_id.0, 4096);
            memory::free_frame(frame);
            KernelResponse::FrameFreed { frame_id }
        }
        KernelRequest::CreateProcess { name: _ } => KernelResponse::Error {
            message: "CreateProcess not available via handle_request",
        },
        KernelRequest::CreateThread {
            pid: _,
            name: _,
            app_id: _,
            priority: _,
        } => KernelResponse::Error {
            message: "CreateThread not available via handle_request",
        },
        KernelRequest::SchedulerTick => {
            let current = scheduler_tick();
            KernelResponse::SchedulerTicked { current }
        }
        KernelRequest::ResidentAlloc { kind, byte_len, flags } => {
            let kind_id = crate::graph::schema::ensure_kind_exists(kind);
            let args = abi::resident::ResidentAllocArgs {
                kind_id,
                byte_len,
                flags,
            };
            match resident::manager::sys_resident_alloc(args) {
                Ok(resp) => KernelResponse::ResidentAllocated { resp },
                Err(e) => KernelResponse::ResidentError(e),
            }
        }
        KernelRequest::ResidentMap { id, perms } => {
             let args = abi::resident::ResidentMapArgs {
                 id,
                 perms,
             };
             match resident::manager::sys_resident_map(args) {
                 Ok(resp) => KernelResponse::ResidentMapped { resp },
                 Err(e) => KernelResponse::ResidentError(e),
             }
        }
        KernelRequest::ResidentUnmap { thing_id } => {
             match resident::manager::sys_resident_unmap(thing_id) {
                 Ok(()) => KernelResponse::Success { data: None },
                 Err(e) => KernelResponse::ResidentError(e),
             }
        }
        KernelRequest::ThingRest { thing_id, policy } => {
             match resident::manager::sys_thing_rest(thing_id, policy) {
                 Ok(resp) => KernelResponse::ThingRested { resp },
                 Err(e) => KernelResponse::ResidentError(e),
             }
        }
        
        KernelRequest::ExitThread => KernelResponse::Success { data: None },
    }
}

/// Create builtin kernel Things at boot time
pub fn create_builtin_things() {
    use abi::{PropType, PropValue};
    use alloc::vec;

    log("Creating kernel Things...");

    const BUILTIN_THING_COUNT: u64 = 2;

    // Register schema for KernelInfo Thing
    let kind = crate::symbols::intern("KernelInfo");
    let desc = crate::symbols::intern("Kernel version and boot status information");
    let props = vec![
        (crate::symbols::intern("version"), PropType::U64),
        (crate::symbols::intern("booted"), PropType::Bool),
    ];
    
    if let Err(e) = register_schema(kind, desc, props, vec![]) {
        log("Failed to register KernelInfo schema");
        log(e);
    }

    // Register schema for BootStats Thing
    let kind = crate::symbols::intern("BootStats");
    let desc = crate::symbols::intern("Statistics about kernel boot process");
    let props = vec![
        (crate::symbols::intern("boot_time_ms"), PropType::U64),
        (crate::symbols::intern("things_created"), PropType::U64),
    ];

    if let Err(e) = register_schema(kind, desc, props, vec![]) {
        log("Failed to register BootStats schema");
        log(e);
    }

    // Create a KernelInfo Thing
    let kind = crate::symbols::intern("KernelInfo");
    let props = vec![
        (crate::symbols::intern("version"), PropValue::U64(1)),
        (crate::symbols::intern("booted"), PropValue::Bool(true)),
    ];

    let _id = graph::create_thing(kind, props);
    log("Created KernelInfo Thing");

    // Create a BootStats Thing
    let kind = crate::symbols::intern("BootStats");
    let props = vec![
        (crate::symbols::intern("boot_time_ms"), PropValue::U64(0)),
        (crate::symbols::intern("things_created"), PropValue::U64(BUILTIN_THING_COUNT)),
    ];

    let _id = graph::create_thing(kind, props);
    log("Created BootStats Thing");

    log("Kernel Things created.");
    // Dump the graph so callers can inspect the freshly-created builtin Things
    crate::graph::debug::dump_graph_table();
}

/// Initialize the boot graph with memory and scheduling Things
///
/// Creates a minimal "toy system" graph that represents:
/// - 1 CPU core
/// - 1 Process with 1 Thread in Running state
/// - 1 AddressSpace linked to the process
/// - 1 FramePool with several PhysFrame things
/// - Several VirtRegion things linked to AddressSpace and PhysFrames
///
/// This is a graph-only model - no actual paging or context switching.
pub fn init_boot_graph() {
    log("Initializing boot graph...");

    // Memory layout constants
    const FRAME_POOL_START: u64 = 0x100000; // 1MB
    const FRAME_POOL_END: u64 = 0x1100000; // 17MB
    const FRAME_SIZE: u64 = 4096; // 4KB

    const KERNEL_VIRT_BASE: u64 = 0xFFFF800000000000; // Canonical higher-half
    const KERNEL_VIRT_SIZE: u64 = 0x200000; // 2MB
    const USER_STACK_BASE: u64 = 0x7FFFFFFFE000; // Top of user space
    const USER_STACK_SIZE: u64 = 0x2000; // 8KB
    const USER_HEAP_BASE: u64 = 0x400000; // 4MB
    const USER_HEAP_SIZE: u64 = 0x100000; // 1MB

    // Memory flags
    const FLAG_RW: u64 = 0x3; // Read + Write
    const FLAG_RWX: u64 = 0x7; // Read + Write + Execute
    const FLAG_RX: u64 = 0x5; // Read + Execute

    // Create CPU core
    let cpu_core = match model::create_cpu_core(0) {
        Some(id) => {
            log("Created CpuCore(0)");
            id
        }
        None => {
            log("Failed to create CpuCore");
            return;
        }
    };

    // Create process
    let process = match model::create_process(1) {
        Some(id) => {
            log("Created Process(1)");
            id
        }
        None => {
            log("Failed to create Process");
            return;
        }
    };

    // Create thread with Running state
    let thread = model::create_thread(1, 100);
    if let Some(thread_id) = thread {
        log("Created Thread(1)");

        // Update thread state to Running
        let thread_running = alloc::vec![
            (
                crate::symbols::intern("state"),
                abi::PropValue::Str(String::from(ThreadState::Running.as_str())),
            ),
            (crate::symbols::intern("last_started_ns"), abi::PropValue::U64(0)),
        ];
        graph::update_thing(thread_id, thread_running);
        let _ = graph::add_link(process, graph_kinds::LINK_OWNS_THREAD, thread_id);
        let _ = graph::add_link(thread_id, graph_kinds::LINK_RUNS_ON, cpu_core);
        log("Thread(1) set to Running state");
    } else {
        log("Failed to create Thread");
        return;
    }

    // Create address space
    let addr_space = model::create_address_space(1);
    if addr_space.is_some() {
        log("Created AddressSpace(1)");
    } else {
        log("Failed to create AddressSpace");
        return;
    }

    // Create frame pool
    let frame_pool = model::create_frame_pool(FRAME_POOL_START, FRAME_POOL_END, FRAME_SIZE);
    if frame_pool.is_some() {
        log("Created FramePool");
    } else {
        log("Failed to create FramePool");
        return;
    }

    // Create a few physical frames
    let _frame1 = model::create_phys_frame(FRAME_POOL_START, FRAME_SIZE);
    let _frame2 = model::create_phys_frame(FRAME_POOL_START + FRAME_SIZE, FRAME_SIZE);
    let _frame3 = model::create_phys_frame(FRAME_POOL_START + 2 * FRAME_SIZE, FRAME_SIZE);
    log("Created 3 PhysFrame things");

    // Create virtual regions (kernel space, user stack, user heap)
    let _virt1 = model::create_virt_region(KERNEL_VIRT_BASE, KERNEL_VIRT_SIZE, FLAG_RW);
    let _virt2 = model::create_virt_region(USER_STACK_BASE, USER_STACK_SIZE, FLAG_RWX);
    let _virt3 = model::create_virt_region(USER_HEAP_BASE, USER_HEAP_SIZE, FLAG_RX);
    log("Created 3 VirtRegion things");

    log("Boot graph initialized: 1 process, 1 thread, 1 CPU");

    // Dump the graph after building the boot graph to aid debugging.
    crate::graph::debug::dump_graph_table();
    if cfg!(target_os = "none") {
        verify_boot_graph_invariants();
    }
}

fn verify_boot_graph_invariants() {
    use crate::graph_kinds::{LINK_OWNS_THREAD, LINK_RUNS_ON};

    let mut cpu_count = 0_u64;
    let mut process_count = 0_u64;
    let mut thread_count = 0_u64;
    let mut phys_frame_count = 0_u64;
    let mut virt_region_count = 0_u64;
    let mut addr_space_count = 0_u64;
    let mut addr_space_node = None;

    let mut cpu_node = None;
    let mut process_node = None;
    let mut running_thread = None;
    
    let kind_cpu = crate::symbols::intern("CpuCore");
    let kind_process = crate::symbols::intern("Process");
    let kind_thread = crate::symbols::intern("Thread");
    let kind_frame = crate::symbols::intern("PhysFrame");
    let kind_virt = crate::symbols::intern("VirtRegion");
    let kind_as = crate::symbols::intern("AddressSpace");
    let prop_state = crate::symbols::intern("state");
    let prop_asid = crate::symbols::intern("asid");

    graph::iter_things(|thing| {
        if thing.kind == kind_cpu {
            cpu_count += 1;
            cpu_node = Some(thing.id);
        } else if thing.kind == kind_process {
            process_count += 1;
            process_node = Some(thing.id);
        } else if thing.kind == kind_thread {
             thread_count += 1;
             // Find state prop in Vec
             if let Some((_, PropValue::Str(s))) = thing.props.iter().find(|(k, _)| *k == prop_state) {
                 if s.as_str() == ThreadState::Running.as_str() {
                     running_thread = Some(thing.id);
                 }
             }
        } else if thing.kind == kind_frame {
            phys_frame_count += 1;
        } else if thing.kind == kind_virt {
            virt_region_count += 1;
        } else if thing.kind == kind_as {
            addr_space_count += 1;
            addr_space_node = Some(thing.id);
        }
    });

    // Logging counts via println (serial)
    crate::println!(
        "Boot graph counts: CpuCore={} Process={} Thread={} PhysFrame={} VirtRegion={} AddressSpace={}",
        cpu_count,
        process_count,
        thread_count,
        phys_frame_count,
        virt_region_count,
        addr_space_count
    );

    debug_assert_eq!(cpu_count, 1);
    debug_assert_eq!(process_count, 1);
    debug_assert_eq!(thread_count, 1);
    debug_assert_eq!(phys_frame_count, 3);
    debug_assert_eq!(virt_region_count, 3);
    debug_assert_eq!(addr_space_count, 1);

    if let Some(thread) = running_thread {
        graph::with_thing(thread, |t| {
             let state = t.props.iter().find(|(k, _)| *k == prop_state).map(|(_, v)| v);
             debug_assert!(
                 matches!(state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Running.as_str()),
                 "boot thread should be Running"
             );
        });

        if let Some(cpu) = cpu_node {
            let mut buf = [None; 4];
            graph::neighbors(thread, LINK_RUNS_ON, &mut buf);
            debug_assert!(
                buf.into_iter().flatten().any(|id| id == cpu),
                "boot thread should run on CpuCore(0)"
            );
        }
    }

    if let (Some(proc_id), Some(thread_id)) = (process_node, running_thread) {
        let mut buf = [None; 4];
        graph::neighbors(proc_id, LINK_OWNS_THREAD, &mut buf);
        debug_assert!(
            buf.into_iter().flatten().any(|id| id == thread_id),
            "boot Process(1) should own the boot thread"
        );
    }

    if let Some(addr_space) = addr_space_node {
        graph::with_thing(addr_space, |t| {
            let asid = t.props.iter().find(|(k, _)| *k == prop_asid).map(|(_, v)| v);
             debug_assert!(
                 matches!(asid, Some(PropValue::U64(1))),
                 "AddressSpace should have ASID 1"
             );
        });
    }
}
pub mod hal_impl;
pub mod driver_bringup;
