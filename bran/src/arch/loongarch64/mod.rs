use core::arch::asm;
use kernel::IrqState;
use crate::runtime::ArchRuntime;

/// The architecture-specific runtime for loongarch64.
pub struct LoongArchRuntime {
    serial: SerialPort,
}

pub type Runtime = crate::runtime::Runtime<LoongArchRuntime>;

pub const fn create_runtime() -> Runtime {
    Runtime::new(LoongArchRuntime::new())
}

impl LoongArchRuntime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort,
        }
    }
}

impl ArchRuntime for LoongArchRuntime {
    fn putchar(&self, c: u8) {
        self.serial.putchar(c);
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        let mut count: u64;
        unsafe { asm!("rdtime.d {}, $r0", out(reg) count) };
        count
    }

    fn mono_freq_hz(&self) -> u64 {
        100_000_000
    }

    fn irq_disable(&self) -> IrqState {
        let mut val: usize = 0;
        let mask: usize = 0x4; // CRMD.IE (bit 2)
        unsafe {
            asm!("csrxchg {}, {}, 0x0", inout(reg) val, in(reg) mask);
        }
        IrqState(val)
    }

    fn irq_restore(&self, state: IrqState) {
        let mut val = state.0;
        let mask: usize = 0x4; // CRMD.IE (bit 2)
        unsafe {
            asm!("csrxchg {}, {}, 0x0", inout(reg) val, in(reg) mask);
        }
        let _ = val;
    }
}

/// Serial port implementation for loongarch64 using NS16550A-compatible UART.
pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self {
        Self
    }
}

impl SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // LoongArch QEMU virt machine UART base (NS16550A compatible)
            let base = 0x1fe001e0 as *mut u8;
            base.write_volatile(c);
        }
    }
}

/// Halt and catch fire - enters an infinite idle loop.
pub fn hcf() -> ! {
    // Disable interrupts to prevent waking up and crashing if handlers aren't set
    unsafe {
        let mut _val: usize = 0;
        let mask: usize = 0x4; // CRMD.IE
        asm!("csrxchg {}, {}, 0x0", inout(reg) _val, in(reg) mask);
    }
    loop {
        unsafe { asm!("idle 0") };
    }
}
