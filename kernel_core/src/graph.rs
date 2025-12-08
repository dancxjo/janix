use abi::NodeId;

/// Initialize the graph subsystem
pub fn init() {
    // For now, just a stub
}

/// Query a node in the graph
pub fn query_node(node_id: NodeId) -> Option<u64> {
    // Simple stub implementation - returns a value based on node_id
    if node_id.0 < 100 {
        Some(node_id.0 * 10)
    } else {
        None
    }
}

/// Add a node to the graph
pub fn add_node(value: u64) -> NodeId {
    // Stub implementation
    NodeId(value / 10)
}
