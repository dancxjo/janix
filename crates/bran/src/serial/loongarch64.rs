//! loongarch64 16550 UART driver (QEMU virt machine)
//!
//! LoongArch64 QEMU virt uses NS16550A-compatible UART at 0x1fe001e0

use core::ptr;

/// 16550 base address on QEMU loongarch64 virt machine
const UART_BASE: usize = 0x1fe001e0;

/// Transmitter Holding Register offset
const THR: usize = 0x00;

/// Line Status Register offset
const LSR: usize = 0x05;

/// Transmitter Holding Buffer Empty bit in LSR
const LSR_THRE: u8 = 0x20;

/// Wait until the transmitter is ready
#[inline]
fn wait_for_transmit_ready() {
    unsafe {
        let lsr_addr = (UART_BASE + LSR) as *const u8;
        while (ptr::read_volatile(lsr_addr) & LSR_THRE) == 0 {
            core::hint::spin_loop();
        }
    }
}

/// Write a single byte to UART
pub fn putc(c: u8) {
    wait_for_transmit_ready();
    unsafe {
        let thr_addr = (UART_BASE + THR) as *mut u8;
        ptr::write_volatile(thr_addr, c);
    }
}
