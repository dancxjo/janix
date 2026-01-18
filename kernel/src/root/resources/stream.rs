use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;
use abi::wire::{ThingId, SymbolId};

#[derive(Clone)]
pub struct WatchEvent {
    pub target: ThingId,
    pub key: SymbolId,
    pub value: [u8; 16],
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
