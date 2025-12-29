use core::sync::atomic::{AtomicU64, Ordering};

pub static HHDM_OFFSET: AtomicU64 = AtomicU64::new(0);

static mut UART_BASE: u64 = 0x0900_0000;

pub fn init(hhdm: u64) {
    HHDM_OFFSET.store(hhdm, Ordering::Relaxed);
    unsafe {
        set_uart_base(0x0900_0000 + hhdm);
    }
}

pub fn hhdm_offset() -> u64 {
    HHDM_OFFSET.load(Ordering::Relaxed)
}

pub unsafe fn set_uart_base(base: u64) {
    UART_BASE = base;
}

pub fn log(msg: &str) {
    let uart_base = unsafe { UART_BASE };
    let uart_ptr = uart_base as *mut u8;

    for b in msg.bytes() {
        unsafe {
            core::ptr::write_volatile(uart_ptr, b);
        }
    }
}
