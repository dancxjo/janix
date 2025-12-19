use abi::graph_ops::{GraphOp, GraphEvent};
use abi::{ThingId, PropKey, PropValue, syscall_defs::SymbolId};

pub struct KernelGraphSink;

impl KernelGraphSink {
    pub fn new() -> Self {
        Self
    }
}

// TODO: Implement sink for kernel->user replication if needed.
// For now, kernel is the source of truth.
