//! x86_64 16550 UART driver (COM1 at 0x3F8)

use core::arch::asm;

/// COM1 base port address
const COM1: u16 = 0x3F8;

/// Line Status Register offset
const LSR: u16 = 5;

/// Transmitter Holding Buffer Empty bit in LSR
const LSR_THRE: u8 = 0x20;

/// Write a byte to I/O port
#[inline]
unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
}

/// Read a byte from I/O port  
#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    value
}

/// Wait until the transmitter is ready
#[inline]
fn wait_for_transmit_ready() {
    unsafe {
        while (inb(COM1 + LSR) & LSR_THRE) == 0 {
            core::hint::spin_loop();
        }
    }
}

/// Write a single byte to COM1
pub fn putc(c: u8) {
    wait_for_transmit_ready();
    unsafe {
        outb(COM1, c);
    }
}
