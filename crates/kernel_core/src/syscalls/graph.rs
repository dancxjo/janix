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
        // CRUD Ops
        GraphOp::CreateThing { kind, value } => {
             match thing_models::value::ThingBody::from(&value) {
                 Ok(body) => {
                     let id = kernel.graph.create_thing(kind, body);
                     GraphReply::Created { id }
                 },
                 Err(_) => GraphReply::Error,
             }
        },
        GraphOp::GetThing { id } => {
            if let Some(thing) = kernel.graph.get(id) {
                // Decode body to TypedBytes
                match thing.body.decode::<abi::wire::typed::TypedBytes>() {
                    Ok(tb) => GraphReply::TypedValue(tb),
                    Err(_) => GraphReply::Error,
                }
            } else {
                GraphReply::Error
            }
        },
        GraphOp::UpdateThing { id, value } => {
            match thing_models::value::ThingBody::from(&value) {
                Ok(body) => {
                     match kernel.graph.update_thing(id, body) {
                         Ok(_) => GraphReply::Ack,
                         Err(_) => GraphReply::Error,
                     }
                },
                Err(_) => GraphReply::Error,
            }
        },
        GraphOp::AddLink { from, to, kind } => {
             let body_struct = thing_models::link::LinkBody { from, to, predicate: kind };
             match postcard::to_allocvec(&body_struct) {
                  Ok(link_bytes) => {
                      // Wrap in TypedBytes
                      let typed = abi::wire::typed::TypedBytes {
                          type_id: abi::wire::typed::TypeId(thing_models::builtins::ids::THING_LINK_KIND.0 as u128),
                          codec_id: abi::wire::typed::CodecId::POSTCARD,
                          bytes: link_bytes,
                      };
                      match thing_models::value::ThingBody::from(&typed) {
                          Ok(body) => {
                               let id = kernel.graph.create_thing(thing_models::builtins::ids::THING_LINK_KIND, body);
                               GraphReply::Created { id }
                          },
                          Err(_) => GraphReply::Error,
                      }
                  },
                  Err(_) => GraphReply::Error,
             }
        },
        GraphOp::ScanLinks { from, to, kind } => {
            let mut results = alloc::vec::Vec::new();
            let link_kind = thing_models::builtins::ids::THING_LINK_KIND;
            
            // Inefficient scan for v0.2 smoke test. 
            // Phase 4 should optimize using index.
            for thing in kernel.graph.list() {
                 if thing.kind == link_kind {
                     if let Ok(tb) = thing.body.decode::<abi::wire::typed::TypedBytes>() {
                          if let Ok(link) = postcard::from_bytes::<thing_models::link::LinkBody>(&tb.bytes) {
                               if let Some(f) = from { if link.from != f { continue; } }
                               if let Some(t) = to { if link.to != t { continue; } }
                               if let Some(k) = kind { if link.predicate != k { continue; } }
                               
                               results.push((link.from, link.to, link.predicate));
                          }
                     }
                 }
            }
            GraphReply::Links(results)
        },
        GraphOp::DeleteThing { id } => {
             match kernel.graph.delete_thing(id) {
                 Ok(_) => GraphReply::Ack,
                 Err(_) => GraphReply::Error,
             }
        },
    }
}

pub fn handle_graph_query<B: HardwareBridge>(kernel: &mut Kernel<B>, query: &str, _params: &[u8], out: &mut [u8]) -> Result<usize, ()> {
    match query {
        "time.now" => {
            // Verify request format (TimeNowReq is empty, but we strictly follow protocol)
            if postcard::from_bytes::<abi::wire::time::TimeNowReq>(_params).is_ok() {
                 use thing_models::builtins::ids::THING_TIME_INSTANCE;
                 use thing_models::core::time::TimeNow;
                 use abi::wire::time::TimeNowResp;

                 if let Some(thing) = kernel.graph.get(THING_TIME_INSTANCE) {
                      // Decode body
                      if let Ok(typed) = thing.body.decode::<abi::wire::typed::TypedBytes>() {
                           if let Ok(time_body) = postcard::from_bytes::<TimeNow>(&typed.bytes) {
                                let resp = TimeNowResp {
                                    system_ns: time_body.system_ns,
                                    monotonic_ns: time_body.monotonic_ns,
                                };
                                return to_slice(&resp, out).map(|s| s.len()).map_err(|_| ());
                           }
                      }
                 }
                 // If missing or decode fail, return error or fallback?
                 // Fallback to bridge for robustness during boot?
                 // No, strict dependency on graph ensures we verify the graph flow.
                 Err(())
            } else {
                Err(())
            }
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
