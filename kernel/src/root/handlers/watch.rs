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
    start_seq: u64,
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
    // We use stream_id as the watch_id conceptually for the user.
    
    let next_seq = if start_seq == 0 {
        graph.root_seq.load(Ordering::Relaxed) + 1
    } else {
        start_seq
    };

    let watch = GlobalWatch {
        id: stream_id,
        spec_ptr: 0, // Unused
        stream_handle: ResourceHandle::Stream(stream_handle),
        kind_filter: bs_kind, 
        missing_fact: fact_rel,
        next_seq,
        pending: alloc::collections::VecDeque::new(),
        pending_bytes: 0,
        overflowed: false,
    };
    
    graph.global_watches.insert(stream_id, watch);
    
    // Return the stream handle as the watch ID
    (0, stream_id)
}

pub fn handle_watch_next(
    graph: &mut Graph,
    msg: &crate::root::RootMsg,
    id: u64,
) -> HandlerResult {
    let _out_seq_ptr = if let crate::root::RootOp::WatchNext { out_seq_ptr, .. } = msg.op { out_seq_ptr } else { 0 };
    let out_ptr = if let crate::root::RootOp::WatchNext { out_ptr, .. } = msg.op { out_ptr } else { 0 };
    let out_len = if let crate::root::RootOp::WatchNext { out_len, .. } = msg.op { out_len } else { 0 };

    
    if let Some(watch) = graph.global_watches.get_mut(&id) {
        // 1. Check Overflow
        if watch.overflowed {
            watch.overflowed = false;
            // Return -EOVERFLOW (mapped to -75)
            return (-75, 0); 
        }

        // 2. Check Queue
        if let Some(commit) = watch.pending.front() {
            if commit.data.len() as u64 > out_len {
                return (-28, 0); // -ENOSPC (28 in Linux)
            }
            
            // 3. Copy Out
            // Unsafe copy to kernel ptr provided by syscall handler
            unsafe {
                let src = commit.data.as_ptr();
                let dst = out_ptr as *mut u8;
                core::ptr::copy_nonoverlapping(src, dst, commit.data.len());
            }

            // Return seq in p0
            msg.reply.p0.store(commit.seq, Ordering::Relaxed);
            
            let len = commit.data.len() as u64;
            
            // 4. Pop
            if let Some(popped) = watch.pending.pop_front() {
                watch.pending_bytes -= popped.data.len();
            }
            
            return (0, len); // Success, return length
        } else {
             return (-11, 0); // -EAGAIN
        }
    }
    
    (-22, 0) // -EINVAL (Invalid handle)
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
