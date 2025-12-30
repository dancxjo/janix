pub mod irq_ring;

use abi::wire::driver::DriverEvent;
use irq_ring::IrqRing;
use spin::Mutex;

pub struct Ps2Controller {
    kbd_ring: IrqRing,
    mouse_ring: IrqRing,
}

impl Ps2Controller {
    pub fn new() -> Self {
        Self {
            kbd_ring: IrqRing::new(),
            mouse_ring: IrqRing::new(),
        }
    }

    fn push_kbd(&mut self, scancode: u8) {
        self.kbd_ring.push(DriverEvent::Ps2Scancode { scancode });
    }

    fn push_mouse(&mut self, byte: u8) {
        self.mouse_ring.push(DriverEvent::Ps2MouseByte { byte });
    }

    fn pop_kbd(&mut self) -> Option<DriverEvent> {
        self.kbd_ring.pop()
    }

    fn pop_mouse(&mut self) -> Option<DriverEvent> {
        self.mouse_ring.pop()
    }
}

static PS2_CONTROLLER: Mutex<Option<Ps2Controller>> = Mutex::new(None);

pub fn init() {
    *PS2_CONTROLLER.lock() = Some(Ps2Controller::new());
}

// Called by arch trap handler
pub fn on_ps2_scancode(scancode: u8) {
    if let Some(ref mut ctrl) = *PS2_CONTROLLER.lock() {
        ctrl.push_kbd(scancode);
    }
}

pub fn on_ps2_mouse(byte: u8) {
    if let Some(ref mut ctrl) = *PS2_CONTROLLER.lock() {
        ctrl.push_mouse(byte);
    }
}

pub fn try_pop_keyboard<B: crate::bridge::ProviderBridge + ?Sized>(bridge: &B) -> Option<u8> {
    let irq = bridge.irq_disable();
    let result = PS2_CONTROLLER
        .lock()
        .as_mut()
        .and_then(|ctrl| match ctrl.pop_kbd() {
            Some(DriverEvent::Ps2Scancode { scancode }) => Some(scancode),
            _ => None,
        });
    bridge.irq_restore(irq);
    result
}

pub fn try_pop_mouse<B: crate::bridge::ProviderBridge + ?Sized>(bridge: &B) -> Option<u8> {
    let irq = bridge.irq_disable();
    let result = PS2_CONTROLLER
        .lock()
        .as_mut()
        .and_then(|ctrl| match ctrl.pop_mouse() {
            Some(DriverEvent::Ps2MouseByte { byte }) => Some(byte),
            _ => None,
        });
    bridge.irq_restore(irq);
    result
}

pub fn try_pop_event<B: crate::bridge::ProviderBridge + ?Sized>(bridge: &B) -> Option<DriverEvent> {
    let irq = bridge.irq_disable();
    // Check Keyboard first
    let kbd = PS2_CONTROLLER
        .lock()
        .as_mut()
        .and_then(|ctrl| ctrl.pop_kbd());
    if kbd.is_some() {
        bridge.irq_restore(irq);
        return kbd;
    }

    // Check Mouse
    let mouse = PS2_CONTROLLER
        .lock()
        .as_mut()
        .and_then(|ctrl| ctrl.pop_mouse());
    bridge.irq_restore(irq);
    mouse
}

pub fn controller_mutex() -> &'static Mutex<Option<Ps2Controller>> {
    &PS2_CONTROLLER
}
