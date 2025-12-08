#![no_std]

pub mod graph;
pub mod transaction;
pub mod log;

use abi::{KernelRequest, KernelResponse};

/// Initialize the kernel core subsystems
pub fn init() {
    log::init();
    graph::init();
    transaction::init();
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
