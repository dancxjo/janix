#![no_std]

extern crate alloc;

pub mod console;
pub mod graph;
pub mod graph_kinds;
pub mod log;
pub mod memory;
pub mod model;
pub mod sched;
pub mod sched_graph;
pub mod sched_tick;
pub mod sched_types;
pub mod time;
pub mod transaction;

use crate::model::{compute_memory_summary, compute_scheduler_summary, scheduler_tick};
use crate::sched_types::ThreadState;
use abi::{FrameId, FrameInfo, KernelRequest, KernelResponse, PropValue, ThingId};
use alloc::string::String;
use spin::Mutex;

type SpawnProgramHandler = fn(ThingId) -> Result<(ThingId, ThingId), &'static str>;

static SPAWN_PROGRAM_HANDLER: Mutex<Option<SpawnProgramHandler>> = Mutex::new(None);

/// Initialize the kernel core subsystems
pub fn init() {
    log::init();
    graph::init();
    transaction::init();
    model::init_schemas();
}

/// Register the function responsible for spawning programs described by BootProgram Things.
///
/// This allows the kernel core to delegate spawning to the architecture-specific
/// environment (e.g., the boot crate or the host harness). Calling this function
/// replaces any previously registered handler.
pub fn register_spawn_program_handler(handler: SpawnProgramHandler) {
    *SPAWN_PROGRAM_HANDLER.lock() = Some(handler);
}

/// Log a message to the kernel log
pub fn log(message: &'static str) {
    log::log_message(message);
}

/// Get all log entries
pub fn get_logs() -> &'static [Option<&'static str>] {
    log::get_logs()
}

/// Handle a kernel request from userland
pub fn handle_request(request: KernelRequest) -> KernelResponse {
    match request {
        KernelRequest::GraphQuery { node_id } => match graph::query_node(node_id) {
            Some(value) => KernelResponse::NodeData { node_id, value },
            None => KernelResponse::Error {
                message: "Node not found",
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
            if let Err(e) = graph::validate_props(kind, props) {
                return KernelResponse::Error { message: e };
            }
            match graph::create_thing(kind, props) {
                Some(id) => KernelResponse::ThingCreated { id },
                None => KernelResponse::Error {
                    message: "Failed to create thing",
                },
            }
        }
        KernelRequest::ThingGet { id } => match graph::get_thing(id) {
            Some((kind, props)) => KernelResponse::ThingData { id, kind, props },
            None => KernelResponse::Error {
                message: "Thing not found",
            },
        },
        KernelRequest::ThingList { kind, start_after } => KernelResponse::ThingListEntry {
            id: graph::next_thing_of_kind(kind, start_after),
        },
        KernelRequest::ThingUpdate { id, props } => {
            // Get the kind first to validate
            if let Some((kind, _)) = graph::get_thing(id) {
                if let Err(e) = graph::validate_props(kind, props) {
                    return KernelResponse::Error { message: e };
                }
            }
            if graph::update_thing(id, props) {
                KernelResponse::Success { data: None }
            } else {
                KernelResponse::Error {
                    message: "Failed to update thing",
                }
            }
        }
        KernelRequest::AddEdge {
            from,
            edge_kind,
            to,
        } => {
            if graph::add_edge(from, edge_kind, to) {
                KernelResponse::Success { data: None }
            } else {
                KernelResponse::Error {
                    message: "Failed to add edge",
                }
            }
        }
        KernelRequest::EdgeAt {
            from,
            edge_kind,
            index,
        } => KernelResponse::EdgeTarget {
            target: graph::edge_target_at(from, edge_kind, index as usize),
        },
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
        } => match graph::register_schema(kind, description, props) {
            Ok(()) => KernelResponse::SchemaRegistered { kind },
            Err(e) => KernelResponse::Error { message: e },
        },
        KernelRequest::SchemaGet { kind } => match graph::get_schema_props(kind) {
            Some(props) => KernelResponse::SchemaData { kind, props },
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
        KernelRequest::ExitThread => KernelResponse::Success { data: None },
    }
}

/// Create builtin kernel Things at boot time
pub fn create_builtin_things() {
    use abi::{PropType, PropValue};

    log("Creating kernel Things...");

    // Number of Things we'll create
    const BUILTIN_THING_COUNT: u64 = 2;

    // Register schema for KernelInfo Thing
    static KERNEL_INFO_SCHEMA: &[(&str, PropType)] =
        &[("version", PropType::U64), ("booted", PropType::Bool)];

    if let Err(e) = graph::register_schema(
        "KernelInfo",
        "Kernel version and boot status information",
        KERNEL_INFO_SCHEMA,
    ) {
        log("Failed to register KernelInfo schema");
        log(e);
    }

    // Register schema for BootStats Thing
    static BOOT_STATS_SCHEMA: &[(&str, PropType)] = &[
        ("boot_time_ms", PropType::U64),
        ("things_created", PropType::U64),
    ];

    if let Err(e) = graph::register_schema(
        "BootStats",
        "Statistics about kernel boot process including boot time and initial things created",
        BOOT_STATS_SCHEMA,
    ) {
        log("Failed to register BootStats schema");
        log(e);
    }

    // Create a KernelInfo Thing
    static KERNEL_PROPS: &[(abi::PropKey, PropValue)] = &[
        ("version", PropValue::U64(1)),
        ("booted", PropValue::Bool(true)),
    ];

    if let Some(_id) = graph::create_thing("KernelInfo", KERNEL_PROPS) {
        log("Created KernelInfo Thing");
    }

    // Create a BootStats Thing
    static STATS_PROPS: &[(abi::PropKey, PropValue)] = &[
        ("boot_time_ms", PropValue::U64(0)),
        ("things_created", PropValue::U64(BUILTIN_THING_COUNT)),
    ];

    if let Some(_id) = graph::create_thing("BootStats", STATS_PROPS) {
        log("Created BootStats Thing");
    }

    log("Kernel Things created.");
}

/// Initialize the boot graph with memory and scheduling Things
///
/// Creates a minimal "toy system" graph that represents:
/// - 1 CPU core
/// - 1 Process with 1 Thread in Running state
/// - 1 AddressSpace linked to the process
/// - 1 FramePool with several PhysFrame nodes
/// - Several VirtRegion nodes linked to AddressSpace and PhysFrames
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
        let thread_running = [
            (
                "state",
                abi::PropValue::Str(String::from(ThreadState::Running.as_str())),
            ),
            ("last_started_ns", abi::PropValue::U64(0)),
        ];
        graph::update_thing(thread_id, &thread_running);
        let _ = graph::add_edge(process, graph_kinds::EDGE_OWNS_THREAD, thread_id);
        let _ = graph::add_edge(thread_id, graph_kinds::EDGE_RUNS_ON, cpu_core);
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
    log("Created 3 PhysFrame nodes");

    // Create virtual regions (kernel space, user stack, user heap)
    let _virt1 = model::create_virt_region(KERNEL_VIRT_BASE, KERNEL_VIRT_SIZE, FLAG_RW);
    let _virt2 = model::create_virt_region(USER_STACK_BASE, USER_STACK_SIZE, FLAG_RWX);
    let _virt3 = model::create_virt_region(USER_HEAP_BASE, USER_HEAP_SIZE, FLAG_RX);
    log("Created 3 VirtRegion nodes");

    log("Boot graph initialized: 1 process, 1 thread, 1 CPU");

    if cfg!(target_os = "none") {
        verify_boot_graph_invariants();
    }
}

fn verify_boot_graph_invariants() {
    use crate::graph_kinds::{EDGE_OWNS_THREAD, EDGE_RUNS_ON};

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

    graph::iter_things(|thing| match thing.kind {
        "CpuCore" => {
            cpu_count += 1;
            cpu_node = Some(thing.id);
        }
        "Process" => {
            process_count += 1;
            process_node = Some(thing.id);
        }
        "Thread" => {
            thread_count += 1;
            if let Some(PropValue::Str(s)) = graph::get_prop(thing.id, "state") {
                if s.as_str() == ThreadState::Running.as_str() {
                    running_thread = Some(thing.id);
                }
            }
        }
        "PhysFrame" => phys_frame_count += 1,
        "VirtRegion" => virt_region_count += 1,
        "AddressSpace" => {
            addr_space_count += 1;
            addr_space_node = Some(thing.id);
        }
        _ => {}
    });

    println!(
        "Boot graph counts: CpuCore={} Process={} Thread={} PhysFrame={} VirtRegion={} AddressSpace={}",
        cpu_count,
        process_count,
        thread_count,
        phys_frame_count,
        virt_region_count,
        addr_space_count
    );

    debug_assert_eq!(
        cpu_count, 1_u64,
        "boot graph should have exactly one CpuCore"
    );
    debug_assert_eq!(
        process_count, 1_u64,
        "boot graph should have exactly one Process"
    );
    debug_assert_eq!(
        thread_count, 1_u64,
        "boot graph should have exactly one Thread"
    );
    debug_assert_eq!(
        phys_frame_count, 3_u64,
        "boot graph should create three PhysFrame nodes"
    );
    debug_assert_eq!(
        virt_region_count, 3_u64,
        "boot graph should create three VirtRegion nodes"
    );
    debug_assert_eq!(
        addr_space_count, 1_u64,
        "boot graph should create one AddressSpace"
    );

    if let Some(thread) = running_thread {
        let state = graph::get_prop(thread, "state");
        debug_assert!(
            matches!(state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Running.as_str()),
            "boot thread should be Running"
        );

        if let Some(cpu) = cpu_node {
            let mut buf = [None; 4];
            graph::neighbors(thread, EDGE_RUNS_ON, &mut buf);
            debug_assert!(
                buf.into_iter().flatten().any(|id| id == cpu),
                "boot thread should run on CpuCore(0)"
            );
        }
    }

    if let (Some(proc_id), Some(thread_id)) = (process_node, running_thread) {
        let mut buf = [None; 4];
        graph::neighbors(proc_id, EDGE_OWNS_THREAD, &mut buf);
        debug_assert!(
            buf.into_iter().flatten().any(|id| id == thread_id),
            "boot Process(1) should own the boot thread"
        );
    }

    if let Some(addr_space) = addr_space_node {
        let asid = graph::get_prop(addr_space, "asid");
        debug_assert!(
            matches!(asid, Some(PropValue::U64(1))),
            "AddressSpace should have ASID 1"
        );
    }
}
