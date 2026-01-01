//! x86_64 16550 UART driver (COM1)

use core::arch::asm;

const COM1: u16 = 0x3F8;

#[inline]
unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    value
}

pub struct Serial;

impl Serial {
    pub const fn new() -> Self {
        Self
    }

    fn is_transmit_ready(&self) -> bool {
        unsafe { (inb(COM1 + 5) & 0x20) != 0 }
    }

    pub fn putc(&self, c: u8) {
        while !self.is_transmit_ready() {
            core::hint::spin_loop();
        }
        unsafe {
            outb(COM1, c);
        }
    }

    pub fn write(&self, bytes: &[u8]) {
        for &b in bytes {
            self.putc(b);
        }
    }
}
