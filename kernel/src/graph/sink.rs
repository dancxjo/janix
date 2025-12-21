use crate::graph::ops::{GraphOp, GraphEvent};
use abi::{ThingId, syscall_defs::SymbolId};
use thing_models::{PropKey, PropValue};

pub struct KernelGraphSink;

impl KernelGraphSink {
    pub fn new() -> Self {
        Self
    }
}

// TODO: Implement sink for kernel->user replication if needed.
// For now, kernel is the source of truth.
