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
    }
}
