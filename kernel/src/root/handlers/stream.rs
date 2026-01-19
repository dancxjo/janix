//! Stream and watch subscription handlers.

use crate::root::RootMsg;
use crate::root::graph::Graph;
use crate::root::resources::{ResourceHandle, stream};
use crate::root::symbols::Interner;
use core::sync::atomic::Ordering;

use super::HandlerResult;

pub fn handle_watch_subscribe(
    graph: &mut Graph,
    interner: &mut Interner,
    target_id: u64,
    mask: u64,
) -> HandlerResult {
    let kid = interner.intern("stream.watch");
    let exists = graph.get_kind(target_id).is_some();
    if exists {
        let stream_id = graph.alloc(kid);
        let handle = stream::create(64);
        if let Some(stream_node) = graph.get_node_mut(stream_id) {
            stream_node.resource = Some(ResourceHandle::Stream(handle));
        }
        if let Some(target_node) = graph.get_node_mut(target_id) {
            target_node.watches.push((mask, stream_id));
        }
        (0, stream_id)
    } else {
        (-1, 0)
    }
}

pub fn handle_stream_poll(graph: &mut Graph, msg: &RootMsg, stream_id: u64) -> HandlerResult {
    if let Some(node) = graph.get_node_mut(stream_id) {
        if let Some(ResourceHandle::Stream(handle)) = &node.resource {
            let mut lock = handle.lock();
            if let Some(evt) = lock.events.pop_front() {
                msg.reply.p0.store(evt.target, Ordering::Relaxed);
                msg.reply.p1.store(evt.key, Ordering::Relaxed);
                msg.reply.p2.store(evt.value, Ordering::Relaxed);
                (0, 1)
            } else {
                (0, 0)
            }
        } else {
            (-1, 0)
        }
    } else {
        (-1, 0)
    }
}
