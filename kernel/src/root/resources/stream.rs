use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;

#[derive(Clone)]
pub struct WatchEvent {
    pub target: u64,
    pub key: u64,
    pub value: u64,
}

pub struct Stream {
    pub events: VecDeque<WatchEvent>,
    pub capacity: usize,
}

pub type StreamHandle = Arc<Mutex<Stream>>;

pub fn create(capacity: usize) -> StreamHandle {
    Arc::new(Mutex::new(Stream {
        events: VecDeque::new(),
        capacity,
    }))
}
