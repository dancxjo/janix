use abi::wire::graph::{GraphOp, GraphReply};
use crate::Kernel;
use hw::HardwareBridge;
use alloc::string::String;
use thing_models::core::time::TimeNow;
use postcard::to_slice;
use serde::Serialize;

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
        },
        GraphOp::Log { text } => {
             kernel.bridge.log(text);
             GraphReply::Ack
        }
    }
}

pub fn handle_graph_query<B: HardwareBridge>(kernel: &mut Kernel<B>, query: &str, _params: &[u8], out: &mut [u8]) -> Result<usize, ()> {
    match query {
        "time.now" => {
            let resp = TimeNow {
                system_ns: kernel.bridge.system_now(),
                monotonic_ns: kernel.bridge.ticks(),
            };
            to_slice(&resp, out).map(|s| s.len()).map_err(|_| ())
        },
        "graph.dump" => {
            #[derive(Serialize)]
            struct DumpResp<'a> {
                text: &'a str,
            }
            let resp = DumpResp { text: "ok\n" };
            to_slice(&resp, out).map(|s| s.len()).map_err(|_| ())
        },
        "op" => {
             // Deserialize params as GraphOp
             if let Ok(op) = postcard::from_bytes::<GraphOp>(_params) {
                 let reply = handle_graph_op(kernel, op);
                 to_slice(&reply, out).map(|s| s.len()).map_err(|_| ())
             } else {
                 Err(())
             }
        },
        _ => Err(()),
    }
}
