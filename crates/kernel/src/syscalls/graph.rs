use crate::bridge::HardwareBridge;
use crate::Kernel;
use abi::wire::graph::{GraphOp, GraphReply};
use abi::wire::time::{
    TimeMonotonicReq, TimeMonotonicResp, TimeNowReq, TimeNowResp, TimeSleepNsReq, TimeSleepNsResp,
    TimeSleepUntilReq, TimeSleepUntilResp,
};
use alloc::string::String;
use postcard::to_slice;
use serde::Serialize;

pub const SYSCALL_WAIT_FLAG: usize = 1 << 62;

pub fn handle_graph_op<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    pid: abi::ids::ProcessId,
    op: GraphOp,
) -> GraphReply {
    // kernel.bridge.log(alloc::format!("GraphOp: {:?}", op).as_str()); // Commented out to avoid noise, enabled for ReadBytes
    if let GraphOp::ReadBytes { .. } = op {
        kernel.bridge.log("GraphOp::ReadBytes received");
    }
    use crate::graph::security;
    if let Err(_) = security::check_write(pid, &op) {
        // We need a PermissionDenied reply?
        // Or generic Error.
        return GraphReply::Error;
    }

    match op {
        GraphOp::SymbolIntern { text } => match kernel.symbols.intern(text) {
            Ok(id) => GraphReply::SymbolInterned { id },
            Err(_) => GraphReply::Error,
        },
        GraphOp::SymbolResolve { id } => match kernel.symbols.resolve(id) {
            Some(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(s) => GraphReply::SymbolResolved { text: s },
                Err(_) => GraphReply::Error,
            },
            None => GraphReply::Error,
        },
        GraphOp::Log { text } => {
            kernel.bridge.log(text);
            GraphReply::Ack
        }
        GraphOp::WriteTyped { path: _, value: _ } => {
            // Placeholder for graph storage integration
            GraphReply::Ack
        }
        GraphOp::Watch { .. } => GraphReply::Error,
        GraphOp::CreateThing { kind, value } => {
            match thing_models::value::ThingBody::from(&value) {
                Ok(body) => {
                    let id = kernel.graph.create_thing(kind, body);
                    GraphReply::Created { id }
                }
                Err(_) => GraphReply::Error,
            }
        }
        GraphOp::GetThing { id } => {
            if let Some(thing) = kernel.graph.get(id) {
                match thing.body.decode::<abi::wire::typed::TypedBytes>() {
                    Ok(tb) => GraphReply::TypedValue(tb),
                    Err(_) => GraphReply::Error,
                }
            } else {
                GraphReply::Error
            }
        }
        GraphOp::UpdateThing { id, value } => match thing_models::value::ThingBody::from(&value) {
            Ok(body) => match kernel.graph.update_thing(id, body) {
                Ok(_) => GraphReply::Ack,
                Err(_) => GraphReply::Error,
            },
            Err(_) => GraphReply::Error,
        },
        GraphOp::AddLink { from, to, kind } => {
            let body_struct = thing_models::link::LinkBody {
                from,
                to,
                predicate: kind,
            };
            match postcard::to_allocvec(&body_struct) {
                Ok(link_bytes) => {
                    let typed = abi::wire::typed::TypedBytes {
                        type_id: abi::wire::typed::TypeId(
                            thing_models::builtins::ids::THING_LINK_KIND.0 as u128,
                        ),
                        codec_id: abi::wire::typed::CodecId::POSTCARD,
                        bytes: link_bytes,
                    };
                    match thing_models::value::ThingBody::from(&typed) {
                        Ok(body) => {
                            let id = kernel
                                .graph
                                .create_thing(thing_models::builtins::ids::THING_LINK_KIND, body);
                            GraphReply::Created { id }
                        }
                        Err(_) => GraphReply::Error,
                    }
                }
                Err(_) => GraphReply::Error,
            }
        }
        GraphOp::ScanLinks { from, to, kind } => {
            let mut results = alloc::vec::Vec::new();
            let link_kind = thing_models::builtins::ids::THING_LINK_KIND;

            kernel.bridge.log("Kernel: ScanLinks Start");

            let mut count_all = 0;
            let mut count_links = 0;
            let mut count_decoded = 0;
            let mut count_matched = 0;

            // Optimization: iterate only links using the kind index
            for thing in kernel.graph.iter_kind(link_kind) {
                count_all += 1;
                // if thing.kind == link_kind { // Implicit in iter_kind
                count_links += 1;
                if let Ok(tb) = thing.body.decode::<abi::wire::typed::TypedBytes>() {
                    if let Ok(link) =
                        postcard::from_bytes::<thing_models::link::LinkBody>(&tb.bytes)
                    {
                        count_decoded += 1;
                        let f = link.from;
                        let t = link.to;
                        let p = link.predicate;
                        if let Some(target_from) = from {
                            if f != target_from {
                                continue;
                            }
                        }
                        if let Some(target_to) = to {
                            if t != target_to {
                                continue;
                            }
                        }
                        if let Some(target_kind) = kind {
                            if p != target_kind {
                                continue;
                            }
                        }
                        kernel.bridge.log(
                            alloc::format!("Kernel: MATCHED Link Root->{} (Pred {})", t.0, p.0)
                                .as_str(),
                        );
                        count_matched += 1;
                        results.push((f, t, p));
                    } else {
                        kernel
                            .bridge
                            .log("Kernel: Failed to decode LinkBody from TypedBytes");
                    }
                } else {
                    kernel
                        .bridge
                        .log("Kernel: Failed to decode TypedBytes from ThingBody");
                }
                // } // End if thing.kind == link_kind
            }
            kernel.bridge.log(
                alloc::format!(
                    "Kernel: Scanned {} things. Found {} Links. Decoded {}. Matched {}. Returning {}",
                    count_all, count_links, count_decoded, count_matched, results.len()
                )
                .as_str(),
            );

            GraphReply::Links(results)
        }
        GraphOp::DeleteThing { id } => match kernel.graph.delete_thing(id) {
            Ok(_) => GraphReply::Ack,
            Err(_) => GraphReply::Error,
        },
        GraphOp::ReadBytes { id, offset, len } => {
            match crate::fs::read::read_file_bytes(kernel, id, offset, len) {
                Ok(bytes) => GraphReply::Bytes { bytes },
                Err(_) => GraphReply::Error,
            }
        }
        GraphOp::MachineCall {
            iface,
            ver,
            instance,
            op,
            payload,
        } => {
            match kernel
                .machine
                .call(&kernel.bridge, iface, ver, instance, op, payload)
            {
                Ok(bytes) => GraphReply::Bytes { bytes },
                Err(_) => GraphReply::Error,
            }
        }
        GraphOp::Batch(ops) => {
            let mut results = alloc::vec::Vec::with_capacity(ops.len());
            for op in ops {
                // Recursive call (handle_graph_op is &mut self on kernel, effectively)
                // Assuming no deep recursion limit hit for now.
                results.push(handle_graph_op(kernel, pid, op));
            }
            GraphReply::BatchReply(results)
        }
    }
}

pub fn handle_graph_query<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    pid: abi::ids::ProcessId,
    query: &str,
    _params: &[u8],
    out: &mut [u8],
) -> Result<usize, isize> {
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
        }
        "time.monotonic_ns" => {
            if postcard::from_bytes::<TimeMonotonicReq>(_params).is_ok() {
                let now = crate::time::monotonic_ns();
                let resp = TimeMonotonicResp { monotonic_ns: now };
                to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                Err(-1)
            }
        }
        "time.sleep_ns" => {
            if let Ok(req) = postcard::from_bytes::<TimeSleepNsReq>(_params) {
                let now = crate::time::monotonic_ns();
                let wake = now.saturating_add(req.duration_ns);

                if now < wake {
                    kernel.scheduler.sleep_current_until(wake);
                    return Err(abi::syscall_defs::SYS_EAGAIN);
                }

                let resp = TimeSleepNsResp {
                    woke_at_monotonic_ns: wake,
                };
                to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                Err(-1)
            }
        }
        "time.sleep_until_ns" => {
            if let Ok(req) = postcard::from_bytes::<TimeSleepUntilReq>(_params) {
                let now = crate::time::monotonic_ns();
                if now < req.wake_monotonic_ns {
                    kernel.scheduler.sleep_current_until(req.wake_monotonic_ns);
                    return Err(abi::syscall_defs::SYS_EAGAIN);
                }

                let resp = TimeSleepUntilResp {
                    woke_at_monotonic_ns: req.wake_monotonic_ns,
                };
                to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                Err(-1)
            }
        }
        "graph.dump" => {
            #[derive(Serialize)]
            struct DumpResp<'a> {
                text: &'a str,
            }
            let resp = DumpResp { text: "ok\n" };
            to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
        }
        "op" => {
            // Deserialize params as GraphOp
            if let Ok(op) = postcard::from_bytes::<GraphOp>(_params) {
                let reply = handle_graph_op(kernel, pid, op);
                to_slice(&reply, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                Err(-1)
            }
        }
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
        }
        _ => Err(-1),
    }
}
