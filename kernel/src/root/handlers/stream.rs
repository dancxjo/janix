//! Stream and watch subscription handlers.

use crate::root::graph::Graph;
use crate::root::resources::{ResourceHandle, stream};
use crate::root::symbols::Interner;
use crate::root::RootMsg;
use abi::wire::ThingId;

use super::HandlerResult;

pub fn handle_watch_subscribe(
    graph: &mut Graph,
    interner: &mut Interner,
    target_id: ThingId,
    mask: u64,
    out_ptr: u64,
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

        if out_ptr != 0 {
            let _ = unsafe { crate::memory::copy_to_user(out_ptr as usize, &stream_id.0) };
        }
        (0, 0)
    } else {
        (-1, 0)
    }
}

pub fn handle_stream_poll(
    graph: &mut Graph,
    msg: &RootMsg,
    stream_id: ThingId,
) -> HandlerResult {
    let out_ptr = if let crate::root::RootOp::StreamPoll { out_ptr, .. } = msg.op { out_ptr } else { 0 };

    if let Some(node) = graph.get_node_mut(stream_id) {
        if let Some(ResourceHandle::Stream(handle)) = &node.resource {
            let mut lock = handle.lock();
            if let Some(evt) = lock.events.pop_front() {
                if out_ptr != 0 {
                    let event = abi::types::RootWatchEvent {
                        target: evt.target,
                        key: evt.key,
                        value: evt.value,
                    };
                    unsafe {
                        let slice = core::slice::from_raw_parts(&event as *const _ as *const u8, core::mem::size_of::<abi::types::RootWatchEvent>());
                        let _ = crate::memory::copy_to_user(out_ptr as usize, slice);
                    }
                }
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
