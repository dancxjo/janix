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
            if graph::update_thing(id, props) {
                KernelResponse::Success { data: None }
            } else {
                KernelResponse::Error { message: "Failed to update thing" }
            }
        }
    }
}
