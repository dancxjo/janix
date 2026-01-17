//! Watch API handlers.

use crate::root::graph::{Graph, GlobalWatch};
use crate::root::resources::{stream, ResourceHandle};
use crate::root::symbols::Interner;
use crate::root::query::PreparedStep;
use super::HandlerResult;
use core::sync::atomic::Ordering;

/// Opens a new watch.
/// 1. Allocates a stream resource.
/// 2. Registers the global watch in the graph.
/// 3. (Todo: Runs the query to populate initial state?)
/// For v0, we might skip the initial population if not strictly required, 
/// but the plan says "immediately emit current matches".
pub fn handle_watch_open(
    graph: &mut Graph,
    interner: &mut Interner,
    _mode: u32,
    query: alloc::vec::Vec<PreparedStep>,
) -> HandlerResult {
    // 1. Create Stream
    let stream_handle = stream::create(128); // Buffer size
    let kid = interner.intern("stream.watch");
    let stream_id = graph.alloc(kid);
    
    // Attach resource to stream node
    if let Some(node) = graph.get_node_mut(stream_id) {
        node.resource = Some(ResourceHandle::Stream(stream_handle.clone()));
    }

    // 2. Parse Query to determine filter
    // Extract kind from query's first Scan step, fallback to bytespace
    let bs_kind = query.iter()
        .find(|s| s.op == abi::query::QueryOpKind::Scan as u64)
        .map(|s| s.symbol)
        .unwrap_or_else(|| interner.intern("thing.bytespace"));
    let fact_rel = interner.intern("has_fact");
    
    // 3. Create Global Watch
    // We use stream_id as the watch_id conceptually for the user, 
    // or we allocate a separate ID?
    // Using stream_id is convenient.
    
    let watch = GlobalWatch {
        id: stream_id,
        spec_ptr: 0, // Unused
        stream_handle: ResourceHandle::Stream(stream_handle),
        kind_filter: bs_kind, 
        missing_fact: fact_rel,
    };
    
    graph.global_watches.insert(stream_id, watch);

    // 4. Initial Query (Pre-fill)
    // We scan existing nodes.
    // This is expensive O(N).
    // Constraints: kind == Bytespace
    if let Some(ids) = graph.kind_index.get(&bs_kind) {
         // Copy to avoid borrow issues if possible, or just iterate
         // We can't iterate `ids` while modifying `requests` in stream (locking).
         // Stream is distinct from Graph lock (Graph is &mut here).
         // But we are in handle_msg, holding Graph &mut.
         
         // We need to write to the stream.
         // Retrieve handle from watch logic?
         if let Some(w) = graph.global_watches.get(&stream_id) {
            if let ResourceHandle::Stream(sh) = &w.stream_handle {
                let mut lock = sh.lock();
                
                for &node_id in ids {
                    // Check if missing fact
                    // Logic: does node_id have edge with rel == fact_rel?
                    let has_fact = if let Some(n) = graph.nodes.get(&node_id) {
                        n.edges.iter().any(|(r, _)| *r == fact_rel)
                    } else {
                        false
                    };
                    
                    if !has_fact {
                         // Emit "Match Found"
                        lock.events.push_back(crate::root::resources::stream::WatchEvent {
                            target: node_id, // "node_id"
                            key: 1, // 1 = MatchFound
                            value: 0, // Handle? (TODO: Create handle?)
                        });
                        // For v0, we assume handle is created by client using node_id? Not efficient but ok.
                        // Or we pass node_id as handle?
                    }
                }
            }
         }
    }

    (0, stream_id)
}

pub fn handle_watch_next(
    graph: &mut Graph,
    msg: &crate::root::RootMsg,
    id: u64,
) -> HandlerResult {
    if let Some(watch) = graph.global_watches.get(&id) {
        if let ResourceHandle::Stream(handle) = &watch.stream_handle {
            let mut lock = handle.lock();
             if let Some(evt) = lock.events.pop_front() {
                 msg.reply.p0.store(evt.target, Ordering::Relaxed);
                 msg.reply.p1.store(evt.key, Ordering::Relaxed);
                 msg.reply.p2.store(evt.value, Ordering::Relaxed);
                 // Return (Status=0, Value=1 (count))
                 return (0, 1);
             }
        }
    }
    
    // Check if watch node exists but not global?
    // User might have passed garbage ID.
    (0, 0) // No events
}

pub fn handle_watch_close(
    graph: &mut Graph,
    id: u64,
) -> HandlerResult {
    if graph.global_watches.remove(&id).is_some() {
        (0, 0)
    } else {
        (-1, 0)
    }
}
