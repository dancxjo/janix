//! Root service main loop and message dispatch.

use super::graph::Graph;
use crate::root::handlers as root_handlers;
use crate::root::handlers::batch::RootBatchScratch;
use super::journal::Journal;
use super::symbols::Interner;
use super::{RootMsg, RootOp};
use crate::BootRuntime;
use core::sync::atomic::Ordering;

pub extern "C" fn root_main<R: BootRuntime>(_arg: usize) -> ! {
    crate::kinfo!("ROOT: started once");

    let mut graph = Graph::new();
    let mut journal = Journal::new();
    let mut interner = Interner::new();
    let log_symbols = root_handlers::logging::LogSymbols::new(&mut interner);
    let mut batch_scratch = RootBatchScratch::new();

    let mut iteration = 0u64;
    loop {
        iteration = iteration.wrapping_add(1);
        let mut processed = 0;
        
        while processed < 16 {
            if let Some(msg) = super::pop_msg() {
                handle_msg::<R>(&mut graph, &mut journal, &mut interner, &log_symbols, &mut batch_scratch, msg);
                processed += 1;
            } else {
                break;
            }
        }

        unsafe {
            crate::task::block_current_erased();
        }
    }
}

/// Returns a short name for the RootOp type for logging
#[allow(dead_code)]
fn msg_type_name(op: &RootOp) -> &'static str {
    match op {
        RootOp::Intern { .. } => "Intern",
        RootOp::GetKind { .. } => "GetKind",
        RootOp::CreateNode { .. } => "CreateNode",
        RootOp::ApplyBatch { .. } => "ApplyBatch",
        RootOp::Link { .. } => "Link",
        RootOp::Find { .. } => "Find",
        RootOp::Query { .. } => "Query",
        RootOp::PropGet { .. } => "PropGet",
        RootOp::PropSet { .. } => "PropSet",
        RootOp::BytespaceCreate { .. } => "BytespaceCreate",
        RootOp::BytespaceInfo { .. } => "BytespaceInfo",
        RootOp::BytespaceMap { .. } => "BytespaceMap",
        RootOp::BytespaceUnmap { .. } => "BytespaceUnmap",
        RootOp::BytespacePhys { .. } => "BytespacePhys",
        RootOp::BytespaceRead { .. } => "BytespaceRead",
        RootOp::BytespaceWrite { .. } => "BytespaceWrite",
        RootOp::BytespaceCreateFromPtr { .. } => "BytespaceCreateFromPtr",
        RootOp::WatchSubscribe { .. } => "WatchSubscribe",
        RootOp::StreamPoll { .. } => "StreamPoll",
        RootOp::WatchOpen { .. } => "WatchOpen",
        RootOp::WatchNext { .. } => "WatchNext",
        RootOp::WatchClose { .. } => "WatchClose",
        RootOp::DescribeThing { .. } => "DescribeThing",
        RootOp::DescribeSymbol { .. } => "DescribeSymbol",
        RootOp::DescribeEdge { .. } => "DescribeEdge",
        RootOp::DumpEdges { .. } => "DumpEdges",
        RootOp::GetEdges { .. } => "GetEdges",
        RootOp::DumpGraph { .. } => "DumpGraph",
        RootOp::LogEvent { .. } => "LogEvent",
        RootOp::PropsGetMany { .. } => "PropsGetMany",
    }
}


fn handle_msg<R: BootRuntime>(
    graph: &mut Graph,
    journal: &mut Journal,
    interner: &mut Interner,
    log_symbols: &root_handlers::logging::LogSymbols,
    batch_scratch: &mut RootBatchScratch,
    msg: RootMsg,
) {
    let (status, value) = match msg.op {
        // Symbol operations
        RootOp::Intern { name } => root_handlers::handle_intern(interner, &name),

        // Graph core operations
        RootOp::GetKind { id } => root_handlers::handle_get_kind(graph, id),
        RootOp::CreateNode { kind } => root_handlers::handle_create_node(graph, journal, interner, kind),
        RootOp::Link { src, rel, dst } => root_handlers::handle_link(graph, interner, src, rel, dst),
        RootOp::Find { kind, buffer, len } => root_handlers::handle_find(graph, interner, kind, buffer, len),
        RootOp::Query { plan, out_buffer, out_len } => root_handlers::handle_query(graph, &plan, out_buffer, out_len),

        // Property operations
        RootOp::PropGet { id, key } => root_handlers::handle_prop_get(graph, interner, id, key),
        RootOp::PropSet { id, key, value } => root_handlers::handle_prop_set(graph, journal, interner, id, key, value),
        RootOp::PropsGetMany { id, keys, kbuf_ptr } => {
            let response_ptr = kbuf_ptr as *mut abi::types::BulkPropsResponse;
            let response = unsafe { &mut *response_ptr };
            root_handlers::handle_props_get_many(graph, id, &keys, response)
        }

        // Bytespace operations
        RootOp::BytespaceCreate { len, flags, format } => {
            root_handlers::handle_bytespace_create::<R>(graph, journal, interner, &msg, len, flags, format)
        }
        RootOp::BytespaceCreateFromPtr { ptr, len } => {
            root_handlers::handle_bytespace_create_from_ptr::<R>(graph, journal, interner, ptr, len)
        }
        RootOp::BytespaceWrite { id, offset, ptr, len } => {
            root_handlers::handle_bytespace_write(graph, id, offset, ptr, len)
        }
        RootOp::BytespaceRead { id, offset, ptr, len } => {
            root_handlers::handle_bytespace_read(graph, id, offset, ptr, len)
        }
        RootOp::BytespaceInfo { id } => root_handlers::handle_bytespace_info(graph, &msg, id),
        RootOp::BytespaceMap { id, tid } => root_handlers::handle_bytespace_map(graph, &msg, id, tid),
        RootOp::BytespaceUnmap { id, user_va, tid } => root_handlers::handle_bytespace_unmap(id, user_va, tid),
        RootOp::BytespacePhys { id } => root_handlers::handle_bytespace_phys(graph, &msg, id),

        // Stream/Watch operations
        RootOp::WatchSubscribe { target_id, mask } => {
            root_handlers::handle_watch_subscribe(graph, interner, target_id, mask)
        }
        RootOp::StreamPoll { stream_id, max: _, out_ptr: _ } => {
            root_handlers::handle_stream_poll(graph, &msg, stream_id)
        }
        RootOp::WatchOpen { mode, start_seq, query, filter } => {
            root_handlers::handle_watch_open(graph, interner, mode, start_seq, query, filter)
        }
        RootOp::WatchNext { id, .. } => {
            root_handlers::handle_watch_next(graph, &msg, id)
        }
        RootOp::WatchClose { id } => {
            root_handlers::handle_watch_close(graph, id)
        }

        RootOp::ApplyBatch { batch } => {
            root_handlers::batch::handle_apply_batch_with_scratch(graph, interner, &batch, batch_scratch)
        }

        // Debug/Describe operations
        RootOp::DescribeThing { id, buffer, len } => {
            root_handlers::handle_describe_thing(graph, interner, id, buffer, len)
        }
        RootOp::DescribeSymbol { id, buffer, len } => {
            root_handlers::handle_describe_symbol(interner, id, buffer, len)
        }
        RootOp::DescribeEdge { src, rel, dst, buffer, len } => {
            root_handlers::handle_describe_edge(graph, interner, src, rel, dst, buffer, len)
        }
        RootOp::DumpEdges { id, buffer, len } => {
            root_handlers::handle_dump_edges(graph, interner, id, buffer, len)
        }
        RootOp::GetEdges { id, buffer, len } => {
            root_handlers::handle_get_edges(graph, id, buffer, len)
        }
        RootOp::DumpGraph { limit } => root_handlers::handle_dump_graph(graph, interner, limit),

        // Logging
        RootOp::LogEvent { level, event, message, timestamp, provenance, fields, about } => {
            root_handlers::handle_log_event(
                graph,
                interner,
                &log_symbols,
                level,
                event,
                &message,
                timestamp,
                &provenance,
                &fields,
                &about,
            )
        }
    };

    msg.reply.status.store(status, Ordering::Relaxed);
    msg.reply.value.store(value, Ordering::Relaxed);
    msg.reply.done.store(1, Ordering::Release);
}
