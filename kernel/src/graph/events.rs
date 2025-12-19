use abi::graph_ops::GraphEvent;
use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;

type EventHandler = fn(&GraphEvent);

lazy_static! {
    static ref LISTENERS: Mutex<Vec<EventHandler>> = Mutex::new(Vec::new());
}

pub fn init() {
    LISTENERS.lock().clear();
}

pub fn subscribe(handler: EventHandler) {
    LISTENERS.lock().push(handler);
}

pub fn dispatch_event(event: &GraphEvent) {
    let listeners = LISTENERS.lock();
    for handler in listeners.iter() {
        handler(event);
    }
}
