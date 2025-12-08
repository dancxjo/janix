use abi::{KernelRequest, KernelResponse, NodeId};

/// Print a line to the kernel log
pub fn println(message: &'static str) {
    let sys = userland_rt::get_sys();
    let request = KernelRequest::Log { message };
    sys.syscall(request);
}

/// Query a node in the graph
pub fn graph_query(node_id: NodeId) -> Option<u64> {
    let sys = userland_rt::get_sys();
    let request = KernelRequest::GraphQuery { node_id };
    match sys.syscall(request) {
        KernelResponse::NodeData { node_id: _, value } => Some(value),
        _ => None,
    }
}

/// Create a transaction
pub fn create_transaction() -> Option<abi::TransactionId> {
    let sys = userland_rt::get_sys();
    let request = KernelRequest::CreateTransaction;
    match sys.syscall(request) {
        KernelResponse::TransactionCreated { tx_id } => Some(tx_id),
        _ => None,
    }
}

/// Commit a transaction
pub fn commit_transaction(tx_id: abi::TransactionId) -> bool {
    let sys = userland_rt::get_sys();
    let request = KernelRequest::CommitTransaction { tx_id };
    matches!(sys.syscall(request), KernelResponse::Success { .. })
}
