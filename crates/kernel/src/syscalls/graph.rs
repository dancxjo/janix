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
use thing_models::payload::ThingPayload;

pub const SYSCALL_WAIT_FLAG: usize = 1 << 62;

pub fn handle_graph_op<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    pid: abi::ids::ProcessId,
    op: GraphOp,
) -> GraphReply {
    if let GraphOp::ReadBytes { .. } = op {
        kernel.bridge.log("GraphOp::ReadBytes received");
    }
    use crate::graph::security;
    if let Err(_) = security::check_write(pid, &op) {
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
        },
        GraphOp::WriteTyped { .. } => GraphReply::Ack,
        GraphOp::Watch { .. } => GraphReply::Error,
        GraphOp::CreateThing { kind, value } => {
            let id = kernel.graph.create_thing(kind, value);
            GraphReply::Created { id }
        },
        GraphOp::GetThing { id } => {
            if let Some(thing) = kernel.graph.get(id) {
                GraphReply::Thing { bytes: thing.payload.clone() }
            } else {
                GraphReply::Error
            }
        },
        GraphOp::UpdateThing { id, value } => {
            match kernel.graph.update_thing(id, value) {
                Ok(_) => GraphReply::Ack,
                Err(_) => GraphReply::Error,
            }
        },
        GraphOp::AddLink { from, to, kind } => {
            let body = thing_models::link::LinkBody {
                from,
                to,
                predicate: kind,
            };
            match postcard::to_allocvec(&body) {
                Ok(bytes) => {
                    let id = kernel.graph.create_thing(thing_models::link::LinkBody::KIND, bytes);
                    GraphReply::Created { id }
                },
                Err(_) => GraphReply::Error,
            }
        },
        GraphOp::ScanLinks { from, to, kind } => {
            let mut results = alloc::vec::Vec::new();
            let link_kind = thing_models::link::LinkBody::KIND;

            for thing in kernel.graph.iter_kind(link_kind) {
                 if let Ok(link) = postcard::from_bytes::<thing_models::link::LinkBody>(&thing.payload) {
                    let f = link.from;
                    let t = link.to;
                    let p = link.predicate;
                    if let Some(target_from) = from {
                        if f != target_from { continue; }
                    }
                    if let Some(target_to) = to {
                        if t != target_to { continue; }
                    }
                    if let Some(target_kind) = kind {
                        if p != target_kind { continue; }
                    }
                    results.push((f, t, p));
                 }
            }
            GraphReply::Links(results)
        },
        GraphOp::DeleteThing { id } => match kernel.graph.delete_thing(id) {
            Ok(_) => GraphReply::Ack,
            Err(_) => GraphReply::Error,
        },
        GraphOp::ReadBytes { id, offset, len } => {
            match crate::fs::read::read_file_bytes(kernel, id, offset, len) {
                Ok(bytes) => GraphReply::Bytes { bytes },
                Err(_) => GraphReply::Error,
            }
        },
        GraphOp::Batch(ops) => {
            let mut results = alloc::vec::Vec::with_capacity(ops.len());
            for op in ops {
                results.push(handle_graph_op(kernel, pid, op));
            }
            GraphReply::BatchReply(results)
        },
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
            if postcard::from_bytes::<TimeNowReq>(_params).is_ok() {
                let now = crate::time::monotonic_ns();
                let sys = crate::time::system_ns();
                let resp = TimeNowResp { system_ns: sys, monotonic_ns: now };
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
                let resp = TimeSleepNsResp { woke_at_monotonic_ns: wake };
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
                let resp = TimeSleepUntilResp { woke_at_monotonic_ns: req.wake_monotonic_ns };
                to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                Err(-1)
            }
        }
        "graph.dump" => {
            #[derive(Serialize)]
            struct DumpResp<'a> { text: &'a str }
            let resp = DumpResp { text: "ok\n" };
            to_slice(&resp, out).map(|s| s.len()).map_err(|_| -1)
        }
        "op" => {
            if let Ok(op) = postcard::from_bytes::<GraphOp>(_params) {
                let reply = handle_graph_op(kernel, pid, op);
                to_slice(&reply, out).map(|s| s.len()).map_err(|_| -1)
            } else {
                Err(-1)
            }
        }
        "input.key_events.next" => {
             Err(-1)
        }
        _ => Err(-1),
    }
}
