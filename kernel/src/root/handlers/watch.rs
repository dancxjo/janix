//! Watch API handlers.
//!
//! Watches observe graph mutations via the shared CommitHistory ring buffer.
//! Each watch maintains only a cursor_seq, not a copy of commit data.
//! Watches may filter commits by subject/predicate/kind using O(1) summary matching.

use crate::root::graph::{Graph, GlobalWatch, WatchFilter, WATCH_SCAN_LIMIT, commit_matches};
use crate::root::resources::{stream, ResourceHandle};
use crate::root::symbols::Interner;
use crate::root::query::PreparedStep;
use super::HandlerResult;
use core::sync::atomic::Ordering;

/// Opens a new watch with optional filtering.
/// 
/// # Arguments
/// * `start_seq` - If 0, subscribe from "now" (next commit). Otherwise resume from that seq.
/// * `filter` - Watch filter (flags=0 means match all commits)
/// 
/// # Returns
/// (0, watch_id) on success
pub fn handle_watch_open(
    graph: &mut Graph,
    interner: &mut Interner,
    _mode: u32,
    start_seq: u64,
    query: alloc::vec::Vec<PreparedStep>,
    filter: WatchFilter,
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
    // WATCH CONTRACT:
    // - start_seq == 0: from oldest available (replay history)
    // - start_seq == WATCH_START_LATEST: from next commit only (skip history)
    // - start_seq == N: resume from that sequence
    let cursor_seq = if start_seq == abi::types::WATCH_START_LATEST {
        graph.commit_history.next_seq
    } else if start_seq == 0 {
        graph.commit_history.oldest_seq().unwrap_or(1)
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
        filter,
    };
    
    graph.global_watches.insert(stream_id, watch);
    
    // Return the stream handle as the watch ID
    (0, stream_id)
}

/// Retrieves the next committed batch payload that matches the watch's filter.
///
/// # Algorithm (with filtering)
/// 1. Validate handle else -EINVAL
/// 2. If watch.overflowed: clear flag, return -EOVERFLOW
/// 3. Bounded scan loop (max WATCH_SCAN_LIMIT commits per call):
///    - Check history bounds (overflow/empty)
///    - Check if commit matches filter
///    - Skip non-matching commits
///    - Stop on match or scan limit
/// 4. If match found: check capacity, copy data, write seq, advance cursor
/// 5. If scan limit reached: return -EAGAIN (caller retries)
///
/// # Returns
/// - `>= 0`: Success, bytes written
/// - `-11`: -EAGAIN, no pending events (or scan limit reached)
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

    // 1. Validate handle and extract initial state
    let (mut cursor, filter, _was_overflowed) = {
        let watch = match graph.global_watches.get_mut(&id) {
            Some(w) => w,
            None => return (-9, 0), // -EBADF: bad/stale watch descriptor
        };
        
        // Check overflow flag (sticky until reported)
        if watch.overflowed {
            watch.overflowed = false;
            return (-75, 0); // -EOVERFLOW
        }
        
        (watch.cursor_seq, watch.filter.clone(), false)
    };
    
    // 2. Check history bounds
    let oldest = graph.commit_history.oldest_seq();
    let newest = graph.commit_history.newest_seq();
    
    // Empty history: nothing to read
    if oldest.is_none() {
        return (-11, 0); // -EAGAIN
    }
    
    let oldest = oldest.unwrap();
    let newest = newest.unwrap();
    
    // Cursor behind oldest: watch missed commits (overflow)
    if cursor < oldest {
        if let Some(watch) = graph.global_watches.get_mut(&id) {
            watch.cursor_seq = oldest; // Resync to earliest available
        }
        return (-75, 0); // -EOVERFLOW (do not consume data this call)
    }
    
    // 3. Bounded scan for matching commit
    let mut scanned = 0usize;
    let mut found_cursor: Option<u64> = None;
    
    while scanned < WATCH_SCAN_LIMIT {
        // Cursor ahead of newest: no new commits yet
        if cursor > newest {
            // Save progress before returning
            if let Some(watch) = graph.global_watches.get_mut(&id) {
                watch.cursor_seq = cursor;
            }
            return (-11, 0); // -EAGAIN
        }
        
        // Get commit record (includes summary for O(1) matching)
        let record = match graph.commit_history.get_record(cursor) {
            Some(r) => r,
            None => {
                // Gap in history (shouldn't happen), skip
                cursor += 1;
                scanned += 1;
                continue;
            }
        };
        
        // Check if commit matches filter using summary (O(1))
        if commit_matches(&filter, &record.summary) {
            // Match found!
            found_cursor = Some(cursor);
            break;
        } else {
            // No match, skip to next
            cursor += 1;
            scanned += 1;
            continue;
        }
    }
    
    // If scan limit reached without finding a match
    if found_cursor.is_none() {
        // Save progress so next call continues from where we left off
        if let Some(watch) = graph.global_watches.get_mut(&id) {
            watch.cursor_seq = cursor;
        }
        return (-11, 0); // -EAGAIN (caller will retry)
    }
    
    let match_cursor = found_cursor.unwrap();
    
    // 4. Fetch matching commit and deliver
    let data = match graph.commit_history.get(match_cursor) {
        Some(d) => d,
        None => {
            // Shouldn't happen, treat as overflow
            return (-75, 0);
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
    msg.reply.p0.store(match_cursor, Ordering::Relaxed);
    
    let len = data.len();
    
    // 8. Advance cursor past the delivered commit
    if let Some(watch) = graph.global_watches.get_mut(&id) {
        watch.cursor_seq = match_cursor + 1;
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
