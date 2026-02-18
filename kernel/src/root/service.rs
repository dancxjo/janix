//! Root service main loop and message dispatch.

use super::graph::Graph;
use super::journal::Journal;
use super::symbols::Interner;
use super::{RootMsg, RootOp};
use crate::BootRuntime;
use crate::root::handlers as root_handlers;
use crate::root::handlers::batch::RootBatchScratch;
use core::sync::atomic::Ordering;

pub extern "C" fn root_main<R: BootRuntime>(_arg: usize) -> ! {
    // DIAG: raw serial marker - bypasses log infrastructure entirely
    crate::runtime_base().putchar(b'R');
    crate::runtime_base().putchar(b'M');
    crate::runtime_base().putchar(b'\n');
    crate::kinfo!("ROOT: started once");

    crate::contract!("ROOT: Initializing components...");
    let mut graph = Graph::new();
    crate::contract!("ROOT: Graph initialized");
    let mut journal = Journal::new();
    crate::contract!("ROOT: Journal initialized");
    let mut interner = Interner::new();
    crate::contract!("ROOT: Interner initialized");
    let mut log_symbols = root_handlers::logging::LogSymbols::new(&mut interner);
    crate::contract!("ROOT: LogSymbols initialized");
    let mut batch_scratch = RootBatchScratch::new();
    crate::contract!("ROOT: BatchScratch initialized");
    let mut query_scratch = crate::root::query::QueryScratch::new();
    crate::contract!("ROOT: QueryScratch initialized");

    let mut iteration = 0u64;
    crate::contract!("ROOT: Entering main loop");
    loop {
        iteration = iteration.wrapping_add(1);
        let mut processed_this_round = 0;

        // Process messages until queue is empty or we hit a safety limit.
        // We use a larger limit (128) than before to improve throughput for logging storms.
        while processed_this_round < 128 {
            if let Some(msg) = super::pop_msg() {
                handle_msg::<R>(
                    &mut graph,
                    &mut journal,
                    &mut interner,
                    &mut log_symbols,
                    &mut batch_scratch,
                    &mut query_scratch,
                    msg,
                );
                processed_this_round += 1;
            } else {
                break;
            }
        }

        // Periodic memory stats (check once per round if it's time)
        if iteration % 1000 == 0 {
            let node_count = graph.nodes.len();
            let watch_count = graph.global_watches.len();
            let history_len = graph.commit_history.len();
            let journal_len = journal.entries.len();
            let symbol_count = interner.names.len();
            crate::kinfo!(
                "ROOT STATS: iter={} nodes={} watches={} history={} journal={} symbols={} drops={}",
                iteration,
                node_count,
                watch_count,
                history_len,
                journal_len,
                symbol_count,
                super::inbox_drop_count()
            );
        }

        if processed_this_round == 0 {
            // Only block if we truly ran out of work
            unsafe {
                crate::task::block_current_erased();
            }
        } else if processed_this_round >= 128 {
            // If we hit the limit, there might be more but we should yield to let other
            // high-priority tasks run (like the compositor or driver) then resume.
            unsafe {
                crate::task::scheduler::yield_now_current();
            }
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
        RootOp::GetProps { .. } => "GetProps",
        RootOp::DumpGraph { .. } => "DumpGraph",
        RootOp::LogEvent { .. } => "LogEvent",
        RootOp::PropsGetMany { .. } => "PropsGetMany",
        RootOp::BytespaceTruncate { .. } => "BytespaceTruncate",
        RootOp::ResolvePath { .. } => "ResolvePath",
        RootOp::Unlink { .. } => "Unlink",
        RootOp::DirList { .. } => "DirList",
        RootOp::OrphanThing { .. } => "OrphanThing",
        RootOp::CleanupTaskThings { .. } => "CleanupTaskThings",
    }
}

fn handle_msg<R: BootRuntime>(
    graph: &mut Graph,
    journal: &mut Journal,
    interner: &mut Interner,
    log_symbols: &mut root_handlers::logging::LogSymbols,
    batch_scratch: &mut RootBatchScratch,
    query_scratch: &mut crate::root::query::QueryScratch,
    msg: RootMsg,
) {
    // ApplyBatch gets special handling: we need to return the first created ID
    // via reply.p0 so callers can batch node creation + property sets in one call.
    if let RootOp::ApplyBatch { ref batch } = msg.op {
        let result = root_handlers::batch::handle_apply_batch_with_scratch_full(
            graph,
            interner,
            batch,
            batch_scratch,
        );
        msg.reply.status.store(result.status, Ordering::Relaxed);
        msg.reply.value.store(result.seq, Ordering::Relaxed);
        // Return the first created ID (if any) in p0 so callers can
        // batch CreateNode + PropSet in a single IPC call.
        if let Some(&first_id) = result.created_ids.first() {
            msg.reply.p0.store(first_id, Ordering::Relaxed);
        }
        msg.reply.done.store(1, Ordering::Release);
        return;
    }

    let (status, value) = match msg.op {
        // Symbol operations
        RootOp::Intern { name } => root_handlers::handle_intern(interner, &name),

        // Graph core operations
        RootOp::GetKind { id } => root_handlers::handle_get_kind(graph, id),
        RootOp::CreateNode { kind, creator_tid, owner_thing_id } => {
            root_handlers::handle_create_node(graph, journal, interner, kind, creator_tid, owner_thing_id)
        }
        RootOp::Link { src, rel, dst } => {
            root_handlers::handle_link(graph, interner, src, rel, dst)
        }
        RootOp::Find { kind, buffer, len } => {
            root_handlers::handle_find(graph, interner, kind, buffer, len)
        }
        RootOp::Query {
            plan,
            out_buffer,
            out_len,
        } => root_handlers::handle_query(graph, &plan, out_buffer, out_len, query_scratch),

        // Property operations
        RootOp::PropGet { id, key } => root_handlers::handle_prop_get(graph, interner, id, key),
        RootOp::PropSet { id, key, value } => {
            root_handlers::handle_prop_set(graph, journal, interner, id, key, value)
        }
        RootOp::PropsGetMany { id, keys, kbuf_ptr } => {
            let response_ptr = kbuf_ptr as *mut abi::types::BulkPropsResponse;
            let response = unsafe { &mut *response_ptr };
            root_handlers::handle_props_get_many(graph, id, &keys, response)
        }

        // Bytespace operations
        RootOp::BytespaceCreate { len, flags, format } => {
            root_handlers::handle_bytespace_create::<R>(
                graph, journal, interner, &msg, len, flags, format,
            )
        }
        RootOp::BytespaceCreateFromPtr { ptr, len } => {
            root_handlers::handle_bytespace_create_from_ptr::<R>(graph, journal, interner, ptr, len)
        }
        RootOp::BytespaceWrite {
            id,
            offset,
            ptr,
            len,
        } => root_handlers::handle_bytespace_write(graph, id, offset, ptr, len),
        RootOp::BytespaceRead {
            id,
            offset,
            ptr,
            len,
        } => root_handlers::handle_bytespace_read(graph, id, offset, ptr, len),
        RootOp::BytespaceInfo { id } => root_handlers::handle_bytespace_info(graph, &msg, id),
        RootOp::BytespaceMap { id, tid } => {
            root_handlers::handle_bytespace_map(graph, &msg, id, tid)
        }
        RootOp::BytespaceUnmap { id, user_va, tid } => {
            root_handlers::handle_bytespace_unmap(id, user_va, tid)
        }
        RootOp::BytespacePhys { id } => root_handlers::handle_bytespace_phys(graph, &msg, id),
        RootOp::BytespaceTruncate { id, new_len } => {
            root_handlers::handle_bytespace_truncate(graph, id, new_len)
        }
        RootOp::ResolvePath { path } => {
            root_handlers::handle_resolve_path(graph, interner, &path)
        }
        RootOp::Unlink { src, rel, dst } => {
            root_handlers::handle_unlink(graph, interner, src, rel, dst)
        }
        RootOp::DirList { id, out_ptr, out_len } => {
            root_handlers::handle_dir_list(graph, interner, id, out_ptr, out_len)
        }
        RootOp::OrphanThing { thing_id } => {
            root_handlers::handle_orphan_thing(graph, thing_id)
        }
        RootOp::CleanupTaskThings { owner_thing_id } => {
            root_handlers::handle_cleanup_task_things(graph, owner_thing_id)
        }

        // Stream/Watch operations
        RootOp::WatchSubscribe { target_id, mask } => {
            root_handlers::handle_watch_subscribe(graph, interner, target_id, mask)
        }
        RootOp::StreamPoll {
            stream_id,
            max: _,
            out_ptr: _,
        } => root_handlers::handle_stream_poll(graph, &msg, stream_id),
        RootOp::WatchOpen {
            mode,
            start_seq,
            query,
            filter,
        } => root_handlers::handle_watch_open(graph, interner, mode, start_seq, query, filter),
        RootOp::WatchNext { id, .. } => root_handlers::handle_watch_next(graph, &msg, id),
        RootOp::WatchClose { id } => root_handlers::handle_watch_close(graph, id),

        // ApplyBatch handled above — this arm is unreachable but needed for exhaustiveness
        RootOp::ApplyBatch { .. } => unreachable!(),

        // Debug/Describe operations
        RootOp::DescribeThing { id, buffer, len } => {
            root_handlers::handle_describe_thing(graph, interner, id, buffer, len)
        }
        RootOp::DescribeSymbol { id, buffer, len } => {
            root_handlers::handle_describe_symbol(interner, id, buffer, len)
        }
        RootOp::DescribeEdge {
            src,
            rel,
            dst,
            buffer,
            len,
        } => root_handlers::handle_describe_edge(graph, interner, src, rel, dst, buffer, len),
        RootOp::DumpEdges { id, buffer, len } => {
            root_handlers::handle_dump_edges(graph, interner, id, buffer, len)
        }
        RootOp::GetEdges { id, buffer, len } => {
            root_handlers::handle_get_edges(graph, id, buffer, len)
        }
        RootOp::GetProps { id, buffer, len } => {
            root_handlers::handle_get_props(graph, id, buffer, len)
        }
        RootOp::DumpGraph { limit } => root_handlers::handle_dump_graph(graph, interner, limit),

        // Logging
        RootOp::LogEvent {
            level,
            event,
            message,
            timestamp,
            provenance,
            fields,
            about,
        } => root_handlers::handle_log_event(
            graph,
            interner,
            log_symbols,
            level,
            event,
            &message,
            timestamp,
            &provenance,
            &fields,
            &about,
        ),
    };

    msg.reply.status.store(status, Ordering::Relaxed);
    msg.reply.value.store(value, Ordering::Relaxed);
    msg.reply.done.store(1, Ordering::Release);
}
