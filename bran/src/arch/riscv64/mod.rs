//! riscv64 architecture-specific implementation.

use core::arch::asm;
use kernel::IrqState;
use crate::runtime::ArchRuntime;

mod serial;
use serial::SerialPort;

/// The architecture-specific runtime for riscv64.
pub struct Riscv64Runtime {
    serial: SerialPort,
}

pub type Runtime = crate::runtime::Runtime<Riscv64Runtime>;

pub const fn create_runtime() -> Runtime {
    Runtime::new(Riscv64Runtime::new())
}

impl Riscv64Runtime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort::new(),
        }
    }
}

impl ArchRuntime for Riscv64Runtime {
    fn putchar(&self, c: u8) {
        self.serial.putchar(c);
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        let time: u64;
        unsafe {
            asm!("csrr {}, time", out(reg) time);
        }
        time
    }

    fn mono_freq_hz(&self) -> u64 {
        10_000_000 // Assumed default for QEMU virt
    }

    fn irq_disable(&self) -> IrqState {
        let sstatus: usize;
        unsafe {
            // Read and clear SIE (bit 1)
            asm!("csrrci {}, sstatus, 0x2", out(reg) sstatus);
        }
        IrqState((sstatus >> 1) & 1)
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 != 0 {
            unsafe { asm!("csrrs x0, sstatus, 0x2"); } // Set SIE
        } else {
            unsafe { asm!("csrrc x0, sstatus, 0x2"); } // Clear SIE
        }
    }

    // Barriers
    fn fence_full(&self) {
        unsafe { asm!("sfence.vma"); }
    }

    fn icache_invalidate(&self) {
        unsafe { asm!("fence.i"); }
    }
}

/// Halt and catch fire - enters an infinite wait-for-interrupt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}
