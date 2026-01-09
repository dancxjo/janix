//! aarch64 architecture-specific implementation.

use core::arch::asm;
use kernel::BootRuntime;
use kernel::time::MonotonicClamp;

/// Serial port implementation for aarch64 using PL011 UART.
pub struct SerialPort {
    clamp: MonotonicClamp,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
        }
    }
}

impl BootRuntime for SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // PL011 base address is 0x09000000 for qemu-virt
            let base = 0x09000000 as *mut u8;
            base.write_volatile(c);
        }
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        let raw = read_cntvct_el0();
        self.clamp.clamp(raw)
    }

    fn mono_freq_hz(&self) -> u64 {
        read_cntfrq_el0()
    }
}

/// Halt and catch fire - enters an infinite wait-for-interrupt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}

/// Read the virtual counter frequency (CNTFRQ_EL0)
#[inline]
fn read_cntfrq_el0() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {}, cntfrq_el0", out(reg) val, options(nomem, nostack));
    }
    val
}

/// Read the virtual counter count (CNTVCT_EL0)
#[inline]
fn read_cntvct_el0() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {}, cntvct_el0", out(reg) val, options(nomem, nostack));
    }
    val
}
