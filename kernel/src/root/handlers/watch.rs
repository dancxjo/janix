//! Watch API handlers.
//!
//! Watches observe graph mutations via the shared CommitHistory ring buffer.
//! Each watch maintains only a cursor_seq, not a copy of commit data.

use crate::root::graph::{Graph, GlobalWatch};
use crate::root::resources::{stream, ResourceHandle};
use crate::root::symbols::Interner;
use crate::root::query::PreparedStep;
use super::HandlerResult;
use core::sync::atomic::Ordering;

/// Opens a new watch.
/// 
/// # Arguments
/// * `start_seq` - If 0, subscribe from "now" (next commit). Otherwise resume from that seq.
/// 
/// # Returns
/// (0, watch_id) on success
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
    
    // 3. Determine cursor position
    // If start_seq == 0: cursor = history.next_seq ("from now", next commit will have this seq)
    // Else: cursor = start_seq (resume/replay from that point)
    let cursor_seq = if start_seq == 0 {
        graph.commit_history.next_seq
    } else {
        start_seq
    };

    let watch = GlobalWatch {
        id: stream_id,
        spec_ptr: 0, // Unused
        stream_handle: ResourceHandle::Stream(stream_handle),
        kind_filter: bs_kind, 
        missing_fact: fact_rel,
        cursor_seq,
        overflowed: false,
    };
    
    graph.global_watches.insert(stream_id, watch);
    
    // Return the stream handle as the watch ID
    (0, stream_id)
}

/// Retrieves the next committed batch payload.
///
/// # Algorithm (exact per spec)
/// 1. Validate handle else -EINVAL
/// 2. If watch.overflowed: clear flag, return -EOVERFLOW
/// 3. Determine if cursor_seq is in history range:
///    - If history empty: return -EAGAIN
///    - If cursor_seq < oldest: set overflowed=false, cursor=oldest, return -EOVERFLOW
///    - If cursor_seq > newest: return -EAGAIN
/// 4. Fetch commit, check capacity
/// 5. Copy data, write seq, advance cursor
///
/// # Returns
/// - `>= 0`: Success, bytes written
/// - `-11`: -EAGAIN, no pending events
/// - `-22`: -EINVAL, invalid handle
/// - `-28`: -ENOSPC, buffer too small (no consume)
/// - `-75`: -EOVERFLOW, missed commits (cleared, resync)
pub fn handle_watch_next(
    graph: &mut Graph,
    msg: &crate::root::RootMsg,
    id: u64,
) -> HandlerResult {
    // Extract syscall parameters
    let out_ptr = if let crate::root::RootOp::WatchNext { out_ptr, .. } = msg.op { out_ptr } else { 0 };
    let out_len = if let crate::root::RootOp::WatchNext { out_len, .. } = msg.op { out_len } else { 0 };

    // 1. Validate handle
    let watch = match graph.global_watches.get_mut(&id) {
        Some(w) => w,
        None => return (-22, 0), // -EINVAL
    };
    
    // 2. Check overflow flag (sticky until reported)
    if watch.overflowed {
        watch.overflowed = false;
        return (-75, 0); // -EOVERFLOW
    }
    
    // 3. Check history bounds
    let oldest = graph.commit_history.oldest_seq();
    let newest = graph.commit_history.newest_seq();
    
    // Empty history: nothing to read
    if oldest.is_none() {
        return (-11, 0); // -EAGAIN
    }
    
    let oldest = oldest.unwrap();
    let newest = newest.unwrap();
    
    // Cursor behind oldest: watch missed commits (overflow)
    if watch.cursor_seq < oldest {
        watch.overflowed = false; // Do not immediately loop
        watch.cursor_seq = oldest; // Resync to earliest available
        return (-75, 0); // -EOVERFLOW (do not consume data this call)
    }
    
    // Cursor ahead of newest: no new commits yet
    if watch.cursor_seq > newest {
        return (-11, 0); // -EAGAIN
    }
    
    // 4. Fetch commit record
    let cursor = watch.cursor_seq;
    let data = match graph.commit_history.get(cursor) {
        Some(d) => d,
        None => {
            // Shouldn't happen if contiguous, treat as overflow
            watch.overflowed = false;
            return (-75, 0); // -EOVERFLOW
        }
    };
    
    // 5. Check capacity
    if (out_len as usize) < data.len() {
        return (-28, 0); // -ENOSPC (do not advance cursor)
    }
    
    // 6. Copy data to user buffer
    unsafe {
        let src = data.as_ptr();
        let dst = out_ptr as *mut u8;
        core::ptr::copy_nonoverlapping(src, dst, data.len());
    }
    
    // 7. Write seq into reply (out_seq_ptr handled by syscall layer via p0)
    msg.reply.p0.store(cursor, Ordering::Relaxed);
    
    let len = data.len();
    
    // 8. Advance cursor
    // Re-fetch mutable reference since we borrowed immutably for data
    if let Some(watch) = graph.global_watches.get_mut(&id) {
        watch.cursor_seq = cursor + 1;
    }
    
    (0, len as u64) // Success, return length
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
