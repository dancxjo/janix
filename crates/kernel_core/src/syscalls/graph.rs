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
        },
        GraphOp::WriteTyped { path: _, value: _ } => {
             // Placeholder for graph storage integration
             GraphReply::Ack
        },
        GraphOp::Watch { .. } => GraphReply::Error,
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
        "input.key_events.next" => {
            // Params: start_after_id: ThingId (serialized)
            // Or simple stub: we just return "next after user provided Id"
            if let Ok(last_seen) = postcard::from_bytes::<abi::ThingId>(_params) {
               // Blocking wait? 
               // "returns one event (blocking)"
               // We need to loop/yield if None.
               // V0: spin wait.
               // target kind: THING_KEY_EVENT_KIND (211)
               let kind = thing_models::builtins::ids::THING_KEY_EVENT_KIND;
               loop {
                   if let Some(next_id) = kernel.graph.next_thing_of_kind(kind, last_seen) {
                       if let Some(thing) = kernel.graph.get(next_id) {
                            // serialize Thing
                            return to_slice(thing, out).map(|s| s.len()).map_err(|_| ());
                       }
                   }
                   // Yield
                   core::hint::spin_loop(); 
               }
            } else {
                Err(())
            }
        },
        _ => Err(()),

    }
}
