use abi::wire::graph::{GraphOp, GraphReply};
use crate::Kernel;
use hw::HardwareBridge;
use alloc::string::String;
use thing_models::core::time::TimeNow;
use postcard::to_slice;
use serde::Serialize;
use abi::wire::time::{TimeNowReq, TimeNowResp, TimeMonotonicReq, TimeMonotonicResp, TimeSleepNsReq, TimeSleepNsResp, TimeSleepUntilReq, TimeSleepUntilResp};

pub const SYSCALL_WAIT_FLAG: usize = 1 << 62;

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

pub fn handle_graph_query<B: HardwareBridge>(kernel: &mut Kernel<B>, query: &str, _params: &[u8], out: &mut [u8]) -> Result<usize, isize> {
    match query {
        "time.now" => {
            // Verify request format (TimeNowReq is empty, but we strictly follow protocol)
            if postcard::from_bytes::<TimeNowReq>(_params).is_ok() {
                 let now = crate::time::monotonic_ns(); 
                 let sys = crate::time::system_ns();
                 let resp = TimeNowResp {
                     system_ns: sys,
                     monotonic_ns: now,
                  };
                 to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                Err(-1)
            }
        },
        "time.monotonic_ns" => {
            if postcard::from_bytes::<TimeMonotonicReq>(_params).is_ok() {
                 let now = crate::time::monotonic_ns();
                 let resp = TimeMonotonicResp { monotonic_ns: now };
                 to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                 Err(-1)
            }
        },
        "time.sleep_ns" => {
            if let Ok(req) = postcard::from_bytes::<TimeSleepNsReq>(_params) {
                 let now = crate::time::monotonic_ns();
                 let wake = now.saturating_add(req.duration_ns);
                 
                 if now < wake {
                     kernel.scheduler.sleep_current_until(wake);
                     return Err(abi::syscall_defs::SYS_EAGAIN);
                 }
                 
                 let resp = TimeSleepNsResp { woke_at_monotonic_ns: wake };
                 to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                 Err(-1)
            }
        },
        "time.sleep_until_ns" => {
            if let Ok(req) = postcard::from_bytes::<TimeSleepUntilReq>(_params) {
                 let now = crate::time::monotonic_ns();
                 if now < req.wake_monotonic_ns {
                     kernel.scheduler.sleep_current_until(req.wake_monotonic_ns);
                     return Err(abi::syscall_defs::SYS_EAGAIN);
                 }

                 let resp = TimeSleepUntilResp { woke_at_monotonic_ns: req.wake_monotonic_ns };
                 to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                 Err(-1)
            }
        },
        "graph.dump" => {
            #[derive(Serialize)]
            struct DumpResp<'a> {
                text: &'a str,
            }
            let resp = DumpResp { text: "ok\n" };
            to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
        },
        "op" => {
             // Deserialize params as GraphOp
             if let Ok(op) = postcard::from_bytes::<GraphOp>(_params) {
                 let reply = handle_graph_op(kernel, op);
                 to_slice(&reply, out).map(|s| s.len()).map_err(|_| -1)
             } else {
                 Err(-1)
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
                   // Non-blocking poll
                   if let Some(next_id) = kernel.graph.next_thing_of_kind(kind, last_seen) {
                       if let Some(thing) = kernel.graph.get(next_id) {
                            return to_slice(thing, out).map(|s| s.len()).map_err(|_| -1);
                       }
                   }
                   return Err(abi::syscall_defs::SYS_EAGAIN);
            } else {
                Err(-1)
            }
        },
        _ => Err(-1),
    }
}

