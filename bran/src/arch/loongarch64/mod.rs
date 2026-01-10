use core::arch::asm;
use kernel::IrqState;
use crate::runtime::ArchRuntime;

mod paging;
mod task;
mod trap;

/// The architecture-specific runtime for loongarch64.
pub struct LoongArchRuntime {
    serial: SerialPort,
}

pub type Runtime = crate::runtime::Runtime<LoongArchRuntime>;

pub const fn create_runtime() -> Runtime {
    Runtime::new(LoongArchRuntime::new())
}

pub unsafe fn init() {}

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
        alloc::boxed::Box::new(task::Context::default())
    }

    fn new_trapframe(&self) -> alloc::boxed::Box<dyn kernel::boot::ArchTrapFrame> {
        alloc::boxed::Box::new(trap::TrapFrame::default())
    }

    fn init_task_context(
        &self, 
        out: &mut dyn kernel::boot::ArchContext,
        kstack_top: u64, 
        entry: extern "C" fn(usize) -> !, 
        arg: usize
    ) -> usize {
        let ctx = out.as_any_mut().downcast_mut::<task::Context>().expect("init_task_context: not loongarch64 context");
        ctx.sp = task::context_init(kstack_top, entry, arg) as u64;
        0
    }

    fn save_from_trap(&self, _tf: &dyn kernel::boot::ArchTrapFrame, _out: &mut dyn kernel::boot::ArchContext) {
        // Unimplemented for now
    }

    fn load_into_trap(&self, _ctx: &dyn kernel::boot::ArchContext, _tf: &mut dyn kernel::boot::ArchTrapFrame) {
        // Unimplemented for now
    }

    unsafe fn switch_tasks(&self, old: &mut dyn kernel::boot::ArchContext, new: &dyn kernel::boot::ArchContext) {
        let old_ctx = old.as_any_mut().downcast_mut::<task::Context>().expect("switch_tasks: not loongarch64 context");
        let new_ctx = new.as_any().downcast_ref::<task::Context>().expect("switch_tasks: not loongarch64 context");
        
        unsafe {
            task::context_switch(&mut old_ctx.sp as *mut u64, new_ctx.sp);
        }
    }

    unsafe fn return_from_trap(&self, _tf: &dyn kernel::boot::ArchTrapFrame) -> ! {
        hcf()
    }
    
    fn make_user_trapframe(&self, _rip: u64, _rsp: u64) -> alloc::boxed::Box<dyn kernel::boot::ArchTrapFrame> {
         panic!("make_user_trapframe not implemented");
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
