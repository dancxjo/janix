//! riscv64 architecture-specific implementation.

use core::arch::asm;
use kernel::IrqState;
use crate::runtime::ArchRuntime;

mod serial;
mod paging;
mod task;
use serial::SerialPort;

/// The architecture-specific runtime for riscv64.
pub struct Riscv64Runtime {
    serial: SerialPort,
}

pub type Runtime = crate::runtime::Runtime<Riscv64Runtime>;

pub const fn create_runtime() -> Runtime {
    Runtime::new(Riscv64Runtime::new())
}

pub unsafe fn init() {}

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

    // Paging Delegates
    fn map_page(&self, _handle: usize, virt: u64, phys: u64, flags: u64) -> Result<(), ()> {
        let pflags = kernel::memory::paging::PageFlags::new(flags);
        paging::map_page(virt, kernel::memory::frame_alloc::PhysFrame(phys), pflags)
    }

    fn map_page_with_allocator(
        &self, 
        _handle: usize, 
        virt: u64, 
        phys: u64, 
        _flags: u64,
        alloc: &mut kernel::memory::boot_frame_alloc::BootFrameAllocator
    ) -> Result<(), ()> {
        paging::map_bootheap_page(virt, phys, alloc);
        Ok(())
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

    // Task Context
    fn new_context(&self) -> alloc::boxed::Box<dyn kernel::boot::ArchContext> {
        alloc::boxed::Box::new(task::Context { sp: 0 })
    }

    fn init_task_context(
        &self, 
        out: &mut dyn kernel::boot::ArchContext,
        kstack_top: u64, 
        entry: extern "C" fn(usize) -> !, 
        arg: usize
    ) -> usize {
        let sp = task::context_init(kstack_top, entry, arg);
        let ctx = out.as_any_mut().downcast_mut::<task::Context>().unwrap();
        ctx.sp = sp as u64;
        sp
    }

    unsafe fn switch_tasks(&self, old: &mut dyn kernel::boot::ArchContext, new: &dyn kernel::boot::ArchContext) {
        let old_ctx = old.as_any_mut().downcast_mut::<task::Context>().unwrap();
        let new_ctx = new.as_any().downcast_ref::<task::Context>().unwrap();
        
        unsafe {
             task::context_switch(&mut old_ctx.sp as *mut u64, new_ctx.sp);
        }
    }
}

/// Halt and catch fire - enters an infinite wait-for-interrupt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}
