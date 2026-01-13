//! Root service main loop and message dispatch.

use super::graph::Graph;
use super::handlers;
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

    loop {
        let mut processed = 0;
        while processed < 16 {
            if let Some(msg) = super::pop_msg() {
                handle_msg::<R>(&mut graph, &mut journal, &mut interner, msg);
                processed += 1;
            } else {
                break;
            }
        }

        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

fn handle_msg<R: BootRuntime>(
    graph: &mut Graph,
    journal: &mut Journal,
    interner: &mut Interner,
    msg: RootMsg,
) {
    let (status, value) = match msg.op {
        // Symbol operations
        RootOp::Intern { name } => handlers::handle_intern(interner, &name),

        // Graph core operations
        RootOp::GetKind { id } => handlers::handle_get_kind(graph, id),
        RootOp::CreateNode { kind } => handlers::handle_create_node(graph, journal, interner, kind),
        RootOp::Link { src, rel, dst } => handlers::handle_link(graph, interner, src, rel, dst),
        RootOp::Find { kind, buffer, len } => handlers::handle_find(graph, interner, kind, buffer, len),
        RootOp::Query { plan, out_buffer, out_len } => handlers::handle_query(graph, &plan, out_buffer, out_len),

        // Property operations
        RootOp::PropGet { id, key } => handlers::handle_prop_get(graph, interner, id, key),
        RootOp::PropSet { id, key, value } => handlers::handle_prop_set(graph, journal, interner, id, key, value),

        // Bytespace operations
        RootOp::BytespaceCreate { len, flags, format } => {
            handlers::handle_bytespace_create::<R>(graph, journal, interner, &msg, len, flags, format)
        }
        RootOp::BytespaceCreateFromPtr { ptr, len } => {
            handlers::handle_bytespace_create_from_ptr::<R>(graph, journal, interner, ptr, len)
        }
        RootOp::BytespaceWrite { id, offset, ptr, len } => {
            handlers::handle_bytespace_write(graph, id, offset, ptr, len)
        }
        RootOp::BytespaceRead { id, offset, ptr, len } => {
            handlers::handle_bytespace_read(graph, id, offset, ptr, len)
        }
        RootOp::BytespaceInfo { id } => handlers::handle_bytespace_info(graph, &msg, id),
        RootOp::BytespaceMap { id, tid } => handlers::handle_bytespace_map(graph, &msg, id, tid),
        RootOp::BytespaceUnmap { id, user_va, tid } => handlers::handle_bytespace_unmap(id, user_va, tid),
        RootOp::BytespacePhys { id } => handlers::handle_bytespace_phys(graph, &msg, id),

        // Stream/Watch operations
        RootOp::WatchSubscribe { target_id, mask } => {
            handlers::handle_watch_subscribe(graph, interner, target_id, mask)
        }
        RootOp::StreamPoll { stream_id, max: _, out_ptr: _ } => {
            handlers::handle_stream_poll(graph, &msg, stream_id)
        }

        // Debug/Describe operations
        RootOp::DescribeThing { id, buffer, len } => {
            handlers::handle_describe_thing(graph, interner, id, buffer, len)
        }
        RootOp::DescribeEdge { src, rel, dst, buffer, len } => {
            handlers::handle_describe_edge(graph, interner, src, rel, dst, buffer, len)
        }
        RootOp::DumpEdges { id, buffer, len } => {
            handlers::handle_dump_edges(graph, interner, id, buffer, len)
        }
        RootOp::DumpGraph { limit } => handlers::handle_dump_graph(graph, interner, limit),

        // Logging
        RootOp::LogEvent { level, event, message, timestamp, provenance, fields, about } => {
            handlers::handle_log_event(
                graph,
                interner,
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
