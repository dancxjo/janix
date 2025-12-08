#![no_std]

pub mod graph;
pub mod transaction;
pub mod log;
pub mod model;

use abi::{KernelRequest, KernelResponse};

/// Initialize the kernel core subsystems
pub fn init() {
    log::init();
    graph::init();
    transaction::init();
    model::init_schemas();
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
        KernelRequest::GraphQuery { node_id } => {
            match graph::query_node(node_id) {
                Some(value) => KernelResponse::NodeData { node_id, value },
                None => KernelResponse::Error { message: "Node not found" },
            }
        }
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
                None => KernelResponse::Error { message: "Failed to create thing" },
            }
        }
        KernelRequest::ThingGet { id } => {
            match graph::get_thing(id) {
                Some((kind, props)) => KernelResponse::ThingData { id, kind, props },
                None => KernelResponse::Error { message: "Thing not found" },
            }
        }
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
                KernelResponse::Error { message: "Failed to update thing" }
            }
        }
        KernelRequest::SchemaRegister { kind, props } => {
            match graph::register_schema(kind, props) {
                Ok(()) => KernelResponse::SchemaRegistered { kind },
                Err(e) => KernelResponse::Error { message: e },
            }
        }
        KernelRequest::SchemaGet { kind } => {
            match graph::get_schema_props(kind) {
                Some(props) => KernelResponse::SchemaData { kind, props },
                None => KernelResponse::Error { message: "Schema not found" },
            }
        }
        KernelRequest::MemorySummary => {
            let summary = model::compute_memory_summary();
            KernelResponse::MemorySummary { summary }
        }
        KernelRequest::SchedulerSummary => {
            let summary = model::compute_scheduler_summary();
            KernelResponse::SchedulerSummary { summary }
        }
    }
}

/// Create builtin kernel Things at boot time
pub fn create_builtin_things() {
    use abi::{PropValue, PropType};
    
    log("Creating kernel Things...");
    
    // Number of Things we'll create
    const BUILTIN_THING_COUNT: u64 = 2;
    
    // Register schema for KernelInfo Thing
    static KERNEL_INFO_SCHEMA: &[(&str, PropType)] = &[
        ("version", PropType::U64),
        ("booted", PropType::Bool),
    ];
    
    if let Err(e) = graph::register_schema("KernelInfo", KERNEL_INFO_SCHEMA) {
        log("Failed to register KernelInfo schema");
        log(e);
    }
    
    // Register schema for BootStats Thing
    static BOOT_STATS_SCHEMA: &[(&str, PropType)] = &[
        ("boot_time_ms", PropType::U64),
        ("things_created", PropType::U64),
    ];
    
    if let Err(e) = graph::register_schema("BootStats", BOOT_STATS_SCHEMA) {
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
    const FRAME_POOL_START: u64 = 0x100000;      // 1MB
    const FRAME_POOL_END: u64 = 0x1100000;       // 17MB
    const FRAME_SIZE: u64 = 4096;                 // 4KB
    
    const KERNEL_VIRT_BASE: u64 = 0xFFFF800000000000;  // Canonical higher-half
    const KERNEL_VIRT_SIZE: u64 = 0x200000;            // 2MB
    const USER_STACK_BASE: u64 = 0x7FFFFFFFE000;       // Top of user space
    const USER_STACK_SIZE: u64 = 0x2000;               // 8KB
    const USER_HEAP_BASE: u64 = 0x400000;              // 4MB
    const USER_HEAP_SIZE: u64 = 0x100000;              // 1MB
    
    // Memory flags
    const FLAG_RW: u64 = 0x3;   // Read + Write
    const FLAG_RWX: u64 = 0x7;  // Read + Write + Execute
    const FLAG_RX: u64 = 0x5;   // Read + Execute
    
    // Create CPU core
    let cpu_core = model::create_cpu_core(0);
    if cpu_core.is_some() {
        log("Created CpuCore(0)");
    } else {
        log("Failed to create CpuCore");
        return;
    }
    
    // Create process
    let process = model::create_process(1);
    if process.is_none() {
        log("Failed to create Process");
        return;
    }
    log("Created Process(1)");
    
    // Create thread with Running state
    let thread = model::create_thread(1, 100);
    if let Some(thread_id) = thread {
        log("Created Thread(1)");
        
        // Update thread state to Running
        let thread_running = [
            ("state", abi::PropValue::U64(model::STATE_RUNNING)),
        ];
        graph::update_thing(thread_id, &thread_running);
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
}
