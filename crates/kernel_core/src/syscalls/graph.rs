use abi::wire::graph::{GraphOp, GraphReply};
use crate::Kernel;
use hw::HardwareBridge;
use alloc::string::String;

pub fn handle_graph_op<B: HardwareBridge>(kernel: &mut Kernel<B>, op: GraphOp) -> GraphReply {
    match op {
        GraphOp::SymbolIntern { text } => {
             match kernel.symbols.intern(text) {
                 Ok(id) => GraphReply::SymbolInterned { id },
                 Err(_) => GraphReply::Error,
             }
        },
        GraphOp::SymbolResolve { id } => {
             match kernel.symbols.resolve(id) {
                 Some(bytes) => {
                     // We validate utf8 on insert, so this should remain valid
                     match String::from_utf8(bytes.to_vec()) {
                         Ok(s) => GraphReply::SymbolResolved { text: s },
                         Err(_) => GraphReply::Error,
                     }
                 },
                 None => GraphReply::Error,
             }
        }
    }
}
