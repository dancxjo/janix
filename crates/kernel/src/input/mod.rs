pub mod irq_ring;

use abi::wire::driver::DriverEvent;
use irq_ring::IrqRing;
use spin::Mutex;

static GLOBAL_KBD_RING: Mutex<Option<IrqRing>> = Mutex::new(None);
static GLOBAL_MOUSE_RING: Mutex<Option<IrqRing>> = Mutex::new(None);

pub fn init() {
    *GLOBAL_KBD_RING.lock() = Some(IrqRing::new());
    *GLOBAL_MOUSE_RING.lock() = Some(IrqRing::new());
}

// Called by arch trap handler
pub fn on_ps2_scancode(scancode: u8) {
    if let Some(ref mut ring) = *GLOBAL_KBD_RING.lock() {
        ring.push(DriverEvent::Ps2Scancode { scancode });
    }
}

pub fn on_ps2_mouse(byte: u8) {
    if let Some(ref mut ring) = *GLOBAL_MOUSE_RING.lock() {
        ring.push(DriverEvent::Ps2MouseByte { byte });
    }
}

pub fn try_pop_keyboard<B: crate::bridge::HardwareBridge>(bridge: &B) -> Option<u8> {
    bridge.irq_disable();
    let result = if let Some(ref mut ring) = *GLOBAL_KBD_RING.lock() {
        match ring.pop() {
            Some(DriverEvent::Ps2Scancode { scancode }) => Some(scancode),
            _ => None,
        }
    } else {
        None
    };
    bridge.irq_enable();
    result
}

pub fn try_pop_mouse<B: crate::bridge::HardwareBridge>(bridge: &B) -> Option<u8> {
    bridge.irq_disable();
    let result = if let Some(ref mut ring) = *GLOBAL_MOUSE_RING.lock() {
        match ring.pop() {
            Some(DriverEvent::Ps2MouseByte { byte }) => Some(byte),
            _ => None,
        }
    } else {
        None
    };
    bridge.irq_enable();
    result
}

pub fn try_pop_event<B: crate::bridge::HardwareBridge>(bridge: &B) -> Option<DriverEvent> {
    bridge.irq_disable();
    // Check Keyboard first
    let kbd = if let Some(ref mut ring) = *GLOBAL_KBD_RING.lock() {
        ring.pop()
    } else {
        None
    };
    if kbd.is_some() {
        bridge.irq_enable();
        return kbd;
    }

    // Check Mouse
    let mouse = if let Some(ref mut ring) = *GLOBAL_MOUSE_RING.lock() {
        ring.pop()
    } else {
        None
    };
    bridge.irq_enable();
    mouse
}
