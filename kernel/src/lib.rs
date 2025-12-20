#![no_std]
// force rebuild 3

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

#[cfg(test)]
mod syscalls_test;

use crate::model::{compute_memory_summary, compute_scheduler_summary, scheduler_tick};
use crate::sched_types::ThreadState;
use crate::shared_buffer::MAX_FRAMES_PER_BUFFER;
use abi::{FrameId, FrameInfo, ThingId, KernelRequest, KernelResponse};
use thing_models::{PropType, PropValue, SchedThreadInfo};
use alloc::string::String;
use spin::{Mutex, MutexGuard};

type SpawnProgramHandler = fn(ThingId) -> Result<(ThingId, ThingId), &'static str>;

static SPAWN_PROGRAM_HANDLER: Mutex<Option<SpawnProgramHandler>> = Mutex::new(None);

static TEST_MUTEX: Mutex<()> = Mutex::new(());

/// Initialize the kernel core subsystems
pub fn init() {
    log::init();
    #[cfg(all(not(test), target_arch = "x86_64"))]
    bridge::ps2::init();

    // Initialize graph store first
    crate::graph::store::init();
    // Initialize symbols and other graph components
    crate::symbols::init();
    crate::graph::schema::init();
    crate::graph::events::init();
    crate::graph::index_props::init();

    // Dump the graph after initialization so builtin kinds and indexes are visible.
    #[cfg(not(test))]
    crate::graph::debug::dump_graph_table();
    #[cfg(not(test))]
    journal::init();
    #[cfg(not(test))]
    transaction::init();
    model::init_schemas();
    #[cfg(not(test))]
    console_backend::init();
    #[cfg(not(test))]
    work_queue::init();
    #[cfg(not(test))]
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

/// Helper to read a string from UserSlice (assuming safe access in this mocked env)
unsafe fn read_user_string(slice: abi::wire::common::UserSlice<u8>) -> String {
    if slice.len == 0 {
        return String::new();
    }
    let ptr = slice.ptr as *const u8;
    // Check null/alignment/bounds if real kernel
    let s = unsafe { core::slice::from_raw_parts(ptr, slice.len as usize) };
    String::from_utf8_lossy(s).into_owned()
}

/// Helper to write bytes to UserSlice (assuming safe access)
unsafe fn write_user_bytes(slice: abi::wire::common::UserSlice<u8>, data: &[u8]) -> u64 {
    let len = core::cmp::min(slice.len as usize, data.len());
    let ptr = slice.ptr as *mut u8;
    unsafe { core::ptr::copy_nonoverlapping(data.as_ptr(), ptr, len) };
    len as u64
}

/// Handle a kernel request from userland
pub fn handle_request(request: KernelRequest) -> KernelResponse {
    match request {
        KernelRequest::LinkAt { src, pred, idx } => {
            let target = graph::link_target_at(src, pred, idx as usize);
            match target {
                Some(id) => KernelResponse::LinkTarget { found: 1, target: id },
                None => KernelResponse::LinkTarget { found: 0, target: ThingId(0) },
            }
        },
        KernelRequest::GraphQuery { node_id, out } => match graph::query_node(ThingId(node_id.0)) {
            Some(value) => {
                let written = unsafe { write_user_bytes(out, &value) };
                KernelResponse::NodeData { written }
            },
            None => KernelResponse::Error {
                err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::NOT_FOUND, detail: 0 },
            },
        },
        KernelRequest::CreateTransaction => {
            let tx_id = transaction::create_transaction();
            KernelResponse::TransactionCreated { tx_id }
        }
        KernelRequest::CommitTransaction { tx_id } => {
            match transaction::commit_transaction(tx_id) {
                Ok(()) => KernelResponse::Success { data: None },
                Err(e) => {
                    log::log_message(e); // log internal error message
                    KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 0 } }
                },
            }
        }
        KernelRequest::Log { message } => {
            let msg = unsafe { read_user_string(message) };
            // We need to leak it to satisfy log which takes &'static str, or change log.
            // crate::log takes &'static str.
            // This is a memory leak if called repeatedly.
            // In a real kernel, log buffer is ring buffer and copies data.
            // crate::log::log_message calls LOG_BUFFER which is static.
            // But it takes &str, not &'static str. Wait.
            // definition: pub fn log_message(msg: &str)
            // But handle_request calls log::log_message.
            log::log_message(&msg);
            KernelResponse::Success { data: None }
        }
        KernelRequest::ThingCreate { kind, props } => {
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
                             if blob_ptr.is_null() && blob_len > 0 {
                                 return KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INVALID_ARG, detail: 1 } };
                             }
                             if !blob_ptr.is_null() {
                                 core::ptr::copy_nonoverlapping(blob_ptr, blob.as_mut_ptr(), blob_len as usize);
                                 blob.set_len(blob_len as usize);
                             }
                             PropValue::Blob(blob)
                         },
                         _ => return KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INVALID_ARG, detail: 2 } },
                    };
                    internal_props.push((key, val));
                }
            }

            if let Err(e) = graph::validate_props(kind, &internal_props) {
                log::log_message(e);
                return KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INVALID_ARG, detail: 3 } };
            }
            
            let id = graph::create_thing(kind, internal_props);
            KernelResponse::ThingCreated { id }
        }
        KernelRequest::ThingGet { id: _, out: _ } => KernelResponse::Error {
            err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 99 }, // Not implemented
        },
        KernelRequest::ThingList { kind, start_after } => {
            let next = graph::next_thing_of_kind(kind, start_after);
            match next {
                Some(id) => KernelResponse::ThingListEntry { valid: 1, id },
                None => KernelResponse::ThingListEntry { valid: 0, id: ThingId(0) },
            }
        },
        KernelRequest::ThingUpdate { id, props } => {
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
                         _ => return KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INVALID_ARG, detail: 2 } },
                    };
                    internal_props.push((key, val));
                }
            }

            let validation_result = graph::with_thing(id, |thing_node| {
                graph::validate_props(thing_node.kind, &internal_props)
            });
            if let Some(res) = validation_result {
                 if let Err(e) = res {
                     log::log_message(e);
                     return KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INVALID_ARG, detail: 3 } };
                 }
            } else {
                 // Thing not found, update_thing will fail anyway, but safer to error here
            }
            if graph::update_thing(id, internal_props) {
                KernelResponse::Success { data: None }
            } else {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::NOT_FOUND, detail: 0 },
                }
            }
        }
        KernelRequest::ThingBatchUpdate { updates } => {
            let updates_ptr = updates.ptr as *const abi::wire::graph::BatchUpdateEntry;
            
            unsafe {
                for i in 0..updates.len {
                    let entry = *updates_ptr.add(i as usize);
                    let mut internal_props = alloc::vec::Vec::new();
                    // entry.props_ptr is UserPtr<WireProp>. UserPtr has 'ptr' field.
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
                             _ => return KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INVALID_ARG, detail: 2 } },
                        };
                        internal_props.push((key, val));
                    }
                    
                    let validation_result = graph::with_thing(entry.id, |thing_node| {
                        graph::validate_props(thing_node.kind, &internal_props)
                    });
                     if let Some(res) = validation_result {
                        if let Err(e) = res {
                            log::log_message(e);
                            return KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INVALID_ARG, detail: 3 } };
                        }
                    } else {
                        return KernelResponse::Error {
                            err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::NOT_FOUND, detail: 0 },
                        };
                    }

                    if !graph::update_thing(entry.id, internal_props) {
                        return KernelResponse::Error {
                            err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 0 },
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
                    err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 0 },
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
                            err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 1 },
                        };
                    }
                };
                if frames.push(frame).is_err() {
                    return KernelResponse::Error {
                        err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 2 },
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
                Err(message) => {
                    log::log_message(message);
                    KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 3 } }
                },
            }
        }
        KernelRequest::MapSharedBuffer { buffer_id, flags } => {
            let (size_bytes, frames) = {
                let manager = shared_buffer::manager().lock();
                if let Some(buffer) = manager.get(&buffer_id) {
                    let frames: alloc::vec::Vec<_> = buffer.frames.iter().cloned().collect();
                    (buffer.size_bytes(), frames)
                } else {
                    return KernelResponse::Error {
                        err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::NOT_FOUND, detail: 0 },
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
                             err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::PERMISSION, detail: 0 },
                        };
                    }
                };

                match sched.reserve_user_region(pid, size as usize, 4096) {
                    Some(addr) => addr,
                    None => {
                        return KernelResponse::Error {
                            err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 4 },
                        };
                    }
                }
            };

            if let Err(msg) = shared_buffer::map_frames_into_current_as(vaddr, &frames, flags) {
                log::log_message(msg);
                return KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 5 } };
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
                    err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::NOT_FOUND, detail: 0 },
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
                    Err(msg) => {
                        log::log_message(msg);
                        KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 6 } }
                    },
                }
            } else {
                KernelResponse::Error {
                    err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 7 },
                }
            }
        }
        KernelRequest::SchemaRegisterPackage {
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
                          0 => PropType::U64,
                          1 => PropType::I64,
                          2 => PropType::Bool,
                          3 => PropType::Str,
                          4 => PropType::Blob,
                          _ => return KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INVALID_ARG, detail: 8 } },
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
                    KernelResponse::Error { err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 9 } }
                }
            }
        },
        KernelRequest::SchemaGet { kind, out } => match get_schema_props(kind) {
            Some(props) => {
                 // Convert Internal PropType to WireSchemaProp
                 // We need to write to `out`.
                 // Max items = out.len
                 let count = core::cmp::min(props.len(), out.len as usize);
                 let ptr = out.ptr as *mut abi::wire::graph::WireSchemaProp;
                 unsafe {
                     for i in 0..count {
                         let (sym, pt) = props[i];
                         let pt_raw = match pt {
                             PropType::U64 => 0,
                             PropType::I64 => 1,
                             PropType::Bool => 2,
                             PropType::Str => 3,
                             PropType::Blob => 4,
                             PropType::Symbol => 5,
                             _ => 0,
                         };
                         let wsp = abi::wire::graph::WireSchemaProp {
                             name: sym,
                             prop_type: pt_raw,
                         };
                         *ptr.add(i) = wsp;
                     }
                 }
                 KernelResponse::SchemaData { written: count as u64, fingerprint: 0 }
            }
            None => KernelResponse::Error {
                err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::NOT_FOUND, detail: 0 },
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
                err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 10 },
            },
        },
        KernelRequest::FreeFrame { frame_id } => {
            let frame = memory::PhysFrame::from_start_address(frame_id.0, 4096);
            memory::free_frame(frame);
            KernelResponse::FrameFreed { frame_id }
        }
        KernelRequest::CreateProcess { name: _ } => KernelResponse::Error {
            err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 99 },
        },
        KernelRequest::CreateThread {
            pid: _,
            name: _,
            app_id: _,
            priority: _,
        } => KernelResponse::Error {
            err: abi::syscall_defs::SysError { code: abi::syscall_defs::SysError::INTERNAL, detail: 99 },
        },
        KernelRequest::SchedulerTick => {
            let current = scheduler_tick();
            match current {
                Some(c) => KernelResponse::SchedulerTicked { has_current: 1, current: c },
                None => KernelResponse::SchedulerTicked { has_current: 0, current: SchedThreadInfo { tid: 0, state: 0, priority: 0 } },
            }
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
    use thing_models::{PropType, PropValue};
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
                PropValue::Str(String::from(ThreadState::Running.as_str())),
            ),
            (crate::symbols::intern("last_started_ns"), PropValue::U64(0)),
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
