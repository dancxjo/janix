pub mod irq_ring;
use irq_ring::IrqRing;
use spin::Mutex;
use abi::wire::driver::DriverEvent;

static GLOBAL_INPUT_RING: Mutex<Option<IrqRing>> = Mutex::new(None);

pub fn init() {
    *GLOBAL_INPUT_RING.lock() = Some(IrqRing::new());
}

// Called by arch trap handler
pub fn on_ps2_scancode(scancode: u8) {
    if let Some(ref mut ring) = *GLOBAL_INPUT_RING.lock() {
        ring.push(DriverEvent::Ps2Scancode { scancode });
        // NOTE: If we had a waiting thread, we should wake it here.
        // For v0, sys_driver_wait might spin-wait or yield loop.
    }
}

pub fn on_ps2_mouse(byte: u8) {
    if let Some(ref mut ring) = *GLOBAL_INPUT_RING.lock() {
        ring.push(DriverEvent::Ps2MouseByte { byte });
    }
}

pub fn try_pop_event<B: hw::HardwareBridge>(bridge: &B) -> Option<DriverEvent> {
    bridge.irq_disable();
    let result = if let Some(ref mut ring) = *GLOBAL_INPUT_RING.lock() {
        ring.pop()
    } else {
        None
    };
    bridge.irq_enable();
    result
}
