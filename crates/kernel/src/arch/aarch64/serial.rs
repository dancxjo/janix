//! aarch64 PL011 UART driver

use core::ptr;

const PL011_BASE: usize = 0x0900_0000;

pub struct Serial;

impl Serial {
    pub const fn new() -> Self {
        Self
    }

    fn is_transmit_ready(&self) -> bool {
        unsafe {
            let fr = ptr::read_volatile((PL011_BASE + 0x18) as *const u32);
            (fr & (1 << 5)) == 0
        }
    }

    pub fn putc(&self, c: u8) {
        while !self.is_transmit_ready() {
            core::hint::spin_loop();
        }
        unsafe {
            ptr::write_volatile((PL011_BASE + 0x00) as *mut u32, c as u32);
        }
    }

    pub fn write(&self, bytes: &[u8]) {
        for &b in bytes {
            self.putc(b);
        }
    }
}
