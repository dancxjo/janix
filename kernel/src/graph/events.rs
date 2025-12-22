use abi::GraphEvent;
use alloc::vec::Vec;
use spin::Mutex;

type EventHandler = fn(&GraphEvent);

static LISTENERS: Mutex<Option<Vec<EventHandler>>> = Mutex::new(None);

pub fn init() {
    *LISTENERS.lock() = Some(Vec::new());
}

pub fn subscribe(handler: EventHandler) {
    LISTENERS
        .lock()
        .as_mut()
        .expect("Events not initialized")
        .push(handler);
}

pub fn dispatch_event(event: &GraphEvent) {
    let guard = LISTENERS.lock();
    if let Some(listeners) = guard.as_ref() {
        for handler in listeners.iter() {
            handler(event);
        }
    }
}
