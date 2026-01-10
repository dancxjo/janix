use core::arch::asm;
use kernel::IrqState;
use crate::runtime::ArchRuntime;

mod paging;
mod task;

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

    // Paging Delegates
    fn map_page(&self, _handle: usize, virt: u64, phys: u64, flags: u64) -> Result<(), ()> {
        let pflags = kernel::memory::paging::PageFlags::from_bits_truncate(flags);
        paging::map_page(virt, kernel::memory::frame_alloc::PhysFrame(phys), pflags)
    }

    fn unmap_page(&self, _handle: usize, virt: u64) {
        let _ = paging::unmap_page(virt);
    }

    fn translate(&self, _handle: usize, virt: u64) -> Option<u64> {
        paging::translate(virt).map(|f| f.0)
    }
    
    fn tlb_flush_page(&self, virt: u64) {
        paging::tlb_flush_page(virt);
    }
    
    fn tlb_flush_all(&self) {
        paging::tlb_flush_all();
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

    // Task Context
    fn context_init(
        &self, 
        kstack_top: u64, 
        entry: extern "C" fn(usize) -> !, 
        arg: usize
    ) -> usize {
        task::context_init(kstack_top, entry, arg)
    }

    unsafe fn context_switch(&self, old_handle_ptr: *mut usize, new_handle: usize) {
        unsafe {
             task::context_switch(old_handle_ptr as *mut u64, new_handle as u64);
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
