//! Broadcast topics for one-to-many message fanout.
//!
//! Topics are lightweight registries of subscriber port write handles.
//! Publishing performs best-effort fanout using atomic `send_all` to each
//! subscriber port, with stale-handle pruning.

use super::{get_port, Handle, HandleMode, GLOBAL_HANDLE_TABLE};
use abi::errors::Errno;
use alloc::vec::Vec;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopicId(pub u32);

#[derive(Debug, Default)]
struct Topic {
    subscribers: Vec<Handle>,
}

static TOPICS: Mutex<Vec<Option<Topic>>> = Mutex::new(Vec::new());

pub fn create_topic() -> TopicId {
    let mut topics = TOPICS.lock();
    for (i, slot) in topics.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(Topic::default());
            return TopicId(i as u32);
        }
    }
    let id = topics.len() as u32;
    topics.push(Some(Topic::default()));
    TopicId(id)
}

pub fn subscribe_topic(topic_id: TopicId, handle: Handle) -> Result<(), Errno> {
    // Validate that this is a write handle to a live port.
    let entry = {
        let table = GLOBAL_HANDLE_TABLE.lock();
        table
            .get(handle, HandleMode::Write)
            .copied()
            .ok_or(Errno::EBADF)?
    };
    let _ = get_port(entry.port_id).ok_or(Errno::EBADF)?;

    let mut topics = TOPICS.lock();
    let topic = topics
        .get_mut(topic_id.0 as usize)
        .and_then(Option::as_mut)
        .ok_or(Errno::ENOENT)?;

    if !topic.subscribers.contains(&handle) {
        topic.subscribers.push(handle);
    }
    Ok(())
}

pub fn publish_topic(topic_id: TopicId, data: &[u8]) -> Result<usize, Errno> {
    let subscribers = {
        let topics = TOPICS.lock();
        let topic = topics
            .get(topic_id.0 as usize)
            .and_then(Option::as_ref)
            .ok_or(Errno::ENOENT)?;
        topic.subscribers.clone()
    };

    let mut delivered = 0usize;
    let mut stale = Vec::new();

    for handle in subscribers {
        let entry = {
            let table = GLOBAL_HANDLE_TABLE.lock();
            table.get(handle, HandleMode::Write).copied()
        };
        let Some(entry) = entry else {
            stale.push(handle);
            continue;
        };

        let Some(port) = get_port(entry.port_id) else {
            stale.push(handle);
            continue;
        };

        if port.send_all(data) {
            delivered += 1;
        }
    }

    if !stale.is_empty() {
        let mut topics = TOPICS.lock();
        if let Some(Some(topic)) = topics.get_mut(topic_id.0 as usize) {
            topic.subscribers.retain(|h| !stale.contains(h));
        }
    }

    Ok(delivered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topic_fanout_delivers_to_subscriber_port() {
        let port_id = crate::ipc::create_port(64);
        let (write_handle, read_handle) = {
            let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
            let w = table
                .alloc(port_id, crate::ipc::HandleMode::Write)
                .expect("alloc write handle");
            let r = table
                .alloc(port_id, crate::ipc::HandleMode::Read)
                .expect("alloc read handle");
            (w, r)
        };

        let topic = create_topic();
        subscribe_topic(topic, write_handle).expect("subscribe");
        let delivered = publish_topic(topic, b"hello").expect("publish");
        assert_eq!(delivered, 1);

        let port = crate::ipc::get_port(port_id).expect("port exists");
        let mut buf = [0u8; 16];
        let n = port.recv(&mut buf);
        assert_eq!(n, 5);
        assert_eq!(&buf[..n], b"hello");

        let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        assert!(table.close(write_handle));
        assert!(table.close(read_handle));
    }
}
