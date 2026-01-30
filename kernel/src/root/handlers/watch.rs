//! Watch API handlers.
//!
//! Watches observe graph mutations via the shared CommitHistory ring buffer.
//! Each watch maintains only a cursor_seq, not a copy of commit data.
//! Watches may filter commits by subject/predicate/kind using O(1) summary matching.

use super::HandlerResult;
use crate::root::graph::{GlobalWatch, Graph, WATCH_SCAN_LIMIT, WatchFilter, commit_matches};
use crate::root::handlers::watch_payload::filter_watch_payload;
use crate::root::query::PreparedStep;
use crate::root::resources::{ResourceHandle, stream};
use crate::root::symbols::Interner;
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
    let stream_handle = stream::create(512); // Buffer size
    let kid = interner.intern("stream.watch");
    let stream_id = graph.alloc(kid);

    // Attach resource to stream node
    if let Some(node) = graph.get_node_mut(stream_id) {
        node.resource = Some(ResourceHandle::Stream(stream_handle.clone()));
    }

    // 2. Parse Query to determine filter
    // Extract kind from query's first Scan step, fallback to bytespace
    let bs_kind = query
        .iter()
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

/// Retrieves the next committed watch payload that matches the watch's filter.
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
pub fn handle_watch_next(graph: &mut Graph, msg: &crate::root::RootMsg, id: u64) -> HandlerResult {
    // Extract syscall parameters
    let out_ptr = if let crate::root::RootOp::WatchNext { out_ptr, .. } = msg.op {
        out_ptr
    } else {
        0
    };
    let out_len = if let crate::root::RootOp::WatchNext { out_len, .. } = msg.op {
        out_len
    } else {
        0
    };

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
        if !commit_matches(&filter, &record.summary) {
            // No match, skip to next
            cursor += 1;
            scanned += 1;
            continue;
        }

        let match_cursor = cursor;
        let data = match graph.commit_history.get(match_cursor) {
            Some(d) => d,
            None => return (-75, 0), // Shouldn't happen, treat as overflow
        };

        let filtered = if filter.matches_all() {
            None
        } else {
            match filter_watch_payload(data, &filter) {
                Ok(bytes) => Some(bytes),
                Err(_) => return (-22, 0),
            }
        };

        let payload = match filtered.as_ref() {
            Some(bytes) if bytes.is_empty() => {
                cursor += 1;
                scanned += 1;
                continue;
            }
            Some(bytes) => bytes.as_slice(),
            None => data,
        };

        if (out_len as usize) < payload.len() {
            return (-28, 0); // -ENOSPC (do not advance cursor)
        }

        unsafe {
            let src = payload.as_ptr();
            let dst = out_ptr as *mut u8;
            core::ptr::copy_nonoverlapping(src, dst, payload.len());
        }

        msg.reply.p0.store(match_cursor, Ordering::Relaxed);

        if let Some(watch) = graph.global_watches.get_mut(&id) {
            watch.cursor_seq = match_cursor + 1;
        }

        return (0, payload.len() as u64);
    }

    // Save progress so next call continues from where we left off
    if let Some(watch) = graph.global_watches.get_mut(&id) {
        watch.cursor_seq = cursor;
    }
    (-11, 0) // -EAGAIN (caller will retry)
}

pub fn handle_watch_close(graph: &mut Graph, id: u64) -> HandlerResult {
    if graph.global_watches.remove(&id).is_some() {
        (0, 0)
    } else {
        (-1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::root::graph::CommitSummary;
    use crate::root::{ReplyCell, RootMsg, RootOp};
    use abi::watch::{self, ValueEncoding, WatchEvent, WatchOp};
    use abi::wire::ThingId as WireThingId;
    use alloc::sync::Arc;
    use alloc::vec;
    use alloc::vec::Vec;

    fn create_kind_event(subject: u64, kind: u32) -> Vec<u8> {
        let mut subj_bytes = [0u8; 16];
        subj_bytes[0..8].copy_from_slice(&subject.to_le_bytes());
        let subject = WireThingId(subj_bytes);

        let predicate = watch::WATCH_PRED_KIND;
        let value = kind.to_le_bytes();

        let event = WatchEvent {
            op: WatchOp::Upsert,
            flags: 0,
            subject,
            predicate,
            value_encoding: ValueEncoding::Bytes,
            value: &value,
        };

        let mut buf = vec![0u8; watch::encoded_len(value.len())];
        watch::encode_event(&mut buf, &event).expect("encode failed");
        buf
    }

    #[test]
    fn test_handle_watch_next_basic() {
        let mut graph = Graph::new();
        // Setup history with one commit (seq 1)
        graph
            .commit_history
            .push(1, vec![10, 20, 30], CommitSummary::default());

        // Setup watch manually
        let watch_id = 100;
        let stream_handle = stream::create(1);
        let watch = GlobalWatch {
            id: watch_id,
            spec_ptr: 0,
            stream_handle: ResourceHandle::Stream(stream_handle),
            kind_filter: 0,
            missing_fact: 0,
            cursor_seq: 1, // Ready to read seq 1
            overflowed: false,
            filter: WatchFilter::default(),
        };
        graph.global_watches.insert(watch_id, watch);

        // Setup buffer
        let mut out_buf = [0u8; 16];
        let out_ptr = out_buf.as_mut_ptr() as u64;
        let out_len = out_buf.len() as u64;

        // Setup Msg
        let op = RootOp::WatchNext {
            id: watch_id,
            out_seq_ptr: 0, // unused in handler body for reply, only p0 stored
            out_ptr,
            out_len,
        };
        let reply = Arc::new(ReplyCell::new());
        let msg = RootMsg {
            op,
            reply: reply.clone(),
        };

        // Call handler
        let (status, written) = handle_watch_next(&mut graph, &msg, watch_id);

        assert_eq!(status, 0);
        assert_eq!(written, 3);
        assert_eq!(out_buf[0..3], [10, 20, 30]);

        // Check cursor updated to 2
        let watch = graph
            .global_watches
            .get(&watch_id)
            .expect("Watch missing");
        assert_eq!(watch.cursor_seq, 2);
    }

    #[test]
    fn test_handle_watch_next_no_data() {
        let mut graph = Graph::new();
        // Setup history with one commit (seq 1)
        graph
            .commit_history
            .push(1, vec![10, 20, 30], CommitSummary::default());

        // Setup watch that has already consumed seq 1 (cursor = 2)
        let watch_id = 101;
        let stream_handle = stream::create(1);
        let watch = GlobalWatch {
            id: watch_id,
            spec_ptr: 0,
            stream_handle: ResourceHandle::Stream(stream_handle),
            kind_filter: 0,
            missing_fact: 0,
            cursor_seq: 2, // Expecting seq 2, but only 1 exists
            overflowed: false,
            filter: WatchFilter::default(),
        };
        graph.global_watches.insert(watch_id, watch);

        let op = RootOp::WatchNext {
            id: watch_id,
            out_seq_ptr: 0,
            out_ptr: 0,
            out_len: 0,
        };
        let reply = Arc::new(ReplyCell::new());
        let msg = RootMsg {
            op,
            reply: reply.clone(),
        };

        let (status, _written) = handle_watch_next(&mut graph, &msg, watch_id);

        // Expect -11 (EAGAIN)
        assert_eq!(status, -11);

        // Cursor should remain 2
        let watch = graph.global_watches.get(&watch_id).unwrap();
        assert_eq!(watch.cursor_seq, 2);
    }

    #[test]
    fn test_handle_watch_next_filtering() {
        let mut graph = Graph::new();
        let mut summary_match = CommitSummary::default();
        summary_match.kinds.insert(100);

        let mut summary_no_match = CommitSummary::default();
        summary_no_match.kinds.insert(200);

        // Seq 1: No match
        graph
            .commit_history
            .push(1, vec![1], summary_no_match.clone());
        // Seq 2: No match
        graph
            .commit_history
            .push(2, vec![2], summary_no_match.clone());
        // Seq 3: Match!
        let event_data = create_kind_event(3, 100);
        graph
            .commit_history
            .push(3, event_data.clone(), summary_match.clone());
        // Seq 4: No match
        graph
            .commit_history
            .push(4, vec![4], summary_no_match.clone());

        let watch_id = 999;
        let mut filter = WatchFilter::default();
        filter.flags = abi::root::WATCH_F_KIND;
        filter.kind_id = 100;

        let stream_handle = stream::create(1);
        let watch = GlobalWatch {
            id: watch_id,
            spec_ptr: 0,
            stream_handle: ResourceHandle::Stream(stream_handle),
            kind_filter: 0,
            missing_fact: 0,
            cursor_seq: 1,
            overflowed: false,
            filter,
        };
        graph.global_watches.insert(watch_id, watch);

        let mut out_buf = [0u8; 64];
        let out_ptr = out_buf.as_mut_ptr() as u64;
        let op = RootOp::WatchNext {
            id: watch_id,
            out_seq_ptr: 0,
            out_ptr,
            out_len: out_buf.len() as u64,
        };
        let reply = Arc::new(ReplyCell::new());
        let msg = RootMsg {
            op,
            reply: reply.clone(),
        };

        let (status, written) = handle_watch_next(&mut graph, &msg, watch_id);

        assert_eq!(status, 0);
        assert_eq!(written, event_data.len() as u64);
        assert_eq!(out_buf[0..event_data.len()], event_data[..]);

        let watch = graph.global_watches.get(&watch_id).unwrap();
        assert_eq!(watch.cursor_seq, 4);
        assert_eq!(reply.p0.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn test_handle_watch_next_scan_limit() {
        let mut graph = Graph::new();
        let mut summary_no_match = CommitSummary::default();
        summary_no_match.kinds.insert(200);

        // Push 70 non-matching commits (limit is 64)
        for i in 1..=70 {
            graph
                .commit_history
                .push(i, vec![i as u8], summary_no_match.clone());
        }

        let watch_id = 888;
        let mut filter = WatchFilter::default();
        filter.flags = abi::root::WATCH_F_KIND;
        filter.kind_id = 100;

        let stream_handle = stream::create(1);
        let watch = GlobalWatch {
            id: watch_id,
            spec_ptr: 0,
            stream_handle: ResourceHandle::Stream(stream_handle),
            kind_filter: 0,
            missing_fact: 0,
            cursor_seq: 1,
            overflowed: false,
            filter,
        };
        graph.global_watches.insert(watch_id, watch);

        let op = RootOp::WatchNext {
            id: watch_id,
            out_seq_ptr: 0,
            out_ptr: 0,
            out_len: 0,
        };
        let reply = Arc::new(ReplyCell::new());
        let msg = RootMsg { op, reply };

        let (status, _written) = handle_watch_next(&mut graph, &msg, watch_id);

        assert_eq!(status, -11); // EAGAIN

        let watch = graph.global_watches.get(&watch_id).unwrap();
        // Should have scanned 64 items, starting from 1 -> ends at 65
        assert_eq!(watch.cursor_seq, 1 + 64);
    }
}
