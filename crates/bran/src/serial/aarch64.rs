//! aarch64 PL011 UART driver (QEMU virt machine at 0x0900_0000)

use core::ptr;

/// PL011 base address on QEMU virt machine
const PL011_BASE: usize = 0x0900_0000;

/// Data Register offset
const DR: usize = 0x00;

/// Flag Register offset
const FR: usize = 0x18;

/// Transmit FIFO Full flag
const FR_TXFF: u32 = 1 << 5;

/// Wait until the transmitter is ready
#[inline]
fn wait_for_transmit_ready() {
    unsafe {
        let fr_addr = (PL011_BASE + FR) as *const u32;
        while (ptr::read_volatile(fr_addr) & FR_TXFF) != 0 {
            core::hint::spin_loop();
        }
    }
}

/// Write a single byte to PL011
pub fn putc(c: u8) {
    wait_for_transmit_ready();
    unsafe {
        let dr_addr = (PL011_BASE + DR) as *mut u32;
        ptr::write_volatile(dr_addr, c as u32);
    }
}
