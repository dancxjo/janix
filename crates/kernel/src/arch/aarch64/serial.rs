//! aarch64 PL011 UART driver

use core::ptr;
use core::sync::atomic::{AtomicU64, Ordering};

static BASE: AtomicU64 = AtomicU64::new(0x0900_0000);

pub struct Serial;

impl Serial {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(&self, offset: u64) {
        BASE.store(0x0900_0000 + offset, Ordering::Relaxed);
    }

    fn base(&self) -> u64 {
        BASE.load(Ordering::Relaxed)
    }

    fn is_transmit_ready(&self) -> bool {
        unsafe {
            let fr = ptr::read_volatile((self.base() + 0x18) as *const u32);
            (fr & (1 << 5)) == 0
        }
    }

    pub fn putc(&self, c: u8) {
        while !self.is_transmit_ready() {
            core::hint::spin_loop();
        }
        unsafe {
            ptr::write_volatile((self.base() + 0x00) as *mut u32, c as u32);
        }
    }

    pub fn write(&self, bytes: &[u8]) {
        for &b in bytes {
            self.putc(b);
        }
    }
}
