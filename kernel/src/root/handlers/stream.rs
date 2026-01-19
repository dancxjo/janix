//! DEPRECATED: Legacy stream and watch subscription handlers.
//!
//! These syscalls have been superseded by the Root watch API:
//! - watch_subscribe -> root_watch_open
//! - stream_poll -> root_watch_next
//!
//! These stubs return -ENOSYS to signal callers to migrate.

use super::HandlerResult;
use crate::root::graph::Graph;
use crate::root::symbols::Interner;
use crate::root::RootMsg;

/// REMOVED: Use root_watch_open instead.
pub fn handle_watch_subscribe(
    _graph: &mut Graph,
    _interner: &mut Interner,
    target_id: u64,
    mask: u64,
) -> HandlerResult {
    crate::kwarn!(
        "ENOSYS: watch_subscribe(target={}, mask={}). Use root_watch_open.",
        target_id, mask
    );
    (-38, 0) // -ENOSYS
}

/// REMOVED: Use root_watch_next instead.
pub fn handle_stream_poll(
    _graph: &mut Graph,
    _msg: &RootMsg,
    stream_id: u64,
) -> HandlerResult {
    crate::kwarn!(
        "ENOSYS: stream_poll(stream={}). Use root_watch_next.",
        stream_id
    );
    (-38, 0) // -ENOSYS
}
