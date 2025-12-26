#![no_std]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

#[cfg(target_arch = "aarch64")]
use core::arch::asm;
use hw::HardwareBridge;

pub mod interrupts;
pub mod paging;
pub mod user;

pub struct Bridge;

static mut UART_BASE: u64 = 0x09000000;

pub unsafe fn set_uart_base(base: u64) {
    UART_BASE = base;
}

#[cfg(target_arch = "aarch64")]
impl HardwareBridge for Bridge {
    fn log(&self, msg: &str) {
        // PL011 UART
        let uart_base = unsafe { UART_BASE };
        let uart_ptr = uart_base as *mut u8;
        // FR Register offset 0x18. TXFF is bit 5.
        let uart_fr = (uart_base + 0x18) as *mut u32;

        for b in msg.bytes() {
            unsafe {
                // Wait while TXFF (bit 5) is set
                while (core::ptr::read_volatile(uart_fr) & (1 << 5)) != 0 {
                    core::hint::spin_loop();
                }
                core::ptr::write_volatile(uart_ptr, b);
            }
        }
    }

    fn ticks(&self) -> u64 {
        0 // TODO
    }

    fn idle(&self) {
        unsafe {
            asm!("wfi");
        }
    }

    fn shutdown(&self) -> ! {
        // PSCI SYSTEM_OFF or QEMU semihosting logic could go here.
        // For now, loop forever.
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }

    fn irq_disable(&self) {
        unsafe {
            asm!("msr daifset, #2");
        }
    }

    fn irq_enable(&self) {
        unsafe {
            asm!("msr daifclr, #2");
        }
    }

    fn system_now(&self) -> u64 {
        0
    }

    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> [u64; 20] {
        [0; 20]
    }

    fn resume_user_mode(&self, _context: &[u64]) -> ! {
        loop {}
    }

    fn set_kernel_stack(&self, _stack: u64) {}
}

#[cfg(not(target_arch = "aarch64"))]
impl HardwareBridge for Bridge {
    fn log(&self, _msg: &str) {}
    fn ticks(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn shutdown(&self) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
    fn system_now(&self) -> u64 {
        0
    }
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> [u64; 20] {
        [0; 20]
    }
    fn resume_user_mode(&self, _context: &[u64]) -> ! {
        loop {}
    }
    fn set_kernel_stack(&self, _stack: u64) {}
}
