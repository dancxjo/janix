use abi::wire::driver::DriverEvent;
use alloc::vec::Vec;
use alloc::collections::VecDeque;

pub struct IrqRing {
    events: VecDeque<DriverEvent>,
    // Wakers could be stored here if we implement proper blocking
    // For now, we might spin or use a condvar-like mechanism if the scheduler supported it
    // v0: we might just return immediately if empty? 
    // User requested "blocks current thread until event available".
    // This requires scheduler cooperation.
}

impl IrqRing {
    pub const fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: DriverEvent) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<DriverEvent> {
        self.events.pop_front()
    }
}
