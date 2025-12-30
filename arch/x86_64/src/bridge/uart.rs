//! UART (COM1) interface for x86_64.
//!
//! This module provides serial output via the 16550 UART.

use core::arch::asm;

const COM1: u16 = 0x3F8;

/// Initialize the UART.
pub fn init() {
    unsafe {
        // Disable interrupts
        asm!("out dx, al", in("dx") COM1 + 1, in("al") 0u8);
        // Enable DLAB
        asm!("out dx, al", in("dx") COM1 + 3, in("al") 0x80u8);
        // Set baud rate divisor (115200 baud)
        asm!("out dx, al", in("dx") COM1 + 0, in("al") 1u8);
        asm!("out dx, al", in("dx") COM1 + 1, in("al") 0u8);
        // 8 bits, no parity, one stop bit
        asm!("out dx, al", in("dx") COM1 + 3, in("al") 0x03u8);
        // Enable FIFO, clear them
        asm!("out dx, al", in("dx") COM1 + 2, in("al") 0xC7u8);
        // Enable IRQs, set RTS/DSR set
        asm!("out dx, al", in("dx") COM1 + 4, in("al") 0x0Bu8);
    }
}

/// Write bytes to the UART.
pub fn write_bytes(bytes: &[u8]) {
    for &b in bytes {
        write_byte(b);
    }
}

fn write_byte(b: u8) {
    unsafe {
        // Wait for transmit buffer to be empty
        loop {
            let status: u8;
            asm!("in al, dx", out("al") status, in("dx") COM1 + 5);
            if status & 0x20 != 0 {
                break;
            }
            core::hint::spin_loop();
        }
        asm!("out dx, al", in("dx") COM1, in("al") b);
    }
}
