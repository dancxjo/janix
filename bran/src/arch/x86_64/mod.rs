//! x86_64 architecture-specific implementation.

use core::arch::asm;
use kernel::BootRuntime;

/// Serial port implementation for x86_64 using I/O port 0x3F8 (COM1).
pub struct SerialPort;

impl BootRuntime for SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // Wait for transmit empty
            while (inb(0x3F8 + 5) & 0x20) == 0 {}
            outb(0x3F8, c);
        }
    }
}

#[inline]
unsafe fn outb(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    unsafe {
        asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    ret
}

/// Halt and catch fire - enters an infinite halt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
