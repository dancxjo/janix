use core::arch::asm;
use kernel::{IrqState, UserTaskSpec, FrameAllocatorHook, MapPerms, MapKind};
use kernel::time::MonotonicClamp;
use crate::runtime::ArchRuntime;

pub mod serial;
pub mod task;
pub mod paging;

pub struct RISCV64Runtime {
    serial: serial::SerialPort,
    clamp: MonotonicClamp,
}

impl RISCV64Runtime {
    pub const fn new() -> Self {
        Self {
            serial: serial::SerialPort::new(),
            clamp: MonotonicClamp::new(),
        }
    }
}

pub use task::RISCV64Context;
pub use paging::RISCV64AddressSpace;

impl ArchRuntime for RISCV64Runtime {
    type Context = RISCV64Context;
    type AddressSpace = RISCV64AddressSpace;

    fn init(&self, hhdm_offset: u64) { paging::init(hhdm_offset); }
    fn putchar(&self, c: u8) { self.serial.putchar(c); }
    fn halt(&self) -> ! { hcf() }

    fn mono_ticks(&self) -> u64 {
        let raw = read_time();
        self.clamp.clamp(raw)
    }

    fn mono_freq_hz(&self) -> u64 { 10_000_000 }

    fn irq_disable(&self) -> IrqState {
        let sstatus: usize;
        unsafe { asm!("csrrci {}, sstatus, 2", out(reg) sstatus); }
        IrqState((sstatus >> 1) & 1)
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 != 0 { unsafe { asm!("csrrs x0, sstatus, 2"); } }
        else { unsafe { asm!("csrrci x0, sstatus, 2"); } }
    }

    fn threads_supported(&self) -> bool { true }

    // Tasking
    fn init_kernel_context(&self, entry: extern "C" fn(usize) -> !, stack_top: u64, arg: usize) -> Self::Context {
        task::init_kernel_context(entry, stack_top, arg)
    }

    fn init_user_context(&self, spec: UserTaskSpec<Self::AddressSpace>, kstack_top: u64) -> Self::Context {
        task::init_user_context(spec, kstack_top)
    }

    unsafe fn switch(&self, from: &mut Self::Context, to: &Self::Context) {
        unsafe { task::switch(from, to) }
    }

    // Paging
    fn make_user_address_space(&self) -> Self::AddressSpace {
        paging::make_user_address_space(self.active_address_space(), &DumbKernelAlloc)
    }

    fn active_address_space(&self) -> Self::AddressSpace {
        paging::active_address_space()
    }
    
    fn activate_address_space(&self, aspace: Self::AddressSpace) {
        let satp = (8 << 60) | (aspace.0 >> 12); // Sv39
        unsafe { 
            asm!("csrw satp, {}", in(reg) satp);
            asm!("sfence.vma");
        }
    }

    fn map_page(&self, aspace: Self::AddressSpace, virt: u64, phys: u64, perms: MapPerms, kind: MapKind, allocator: &dyn FrameAllocatorHook) -> Result<(), ()> {
        paging::map_page(aspace, virt, phys, perms, kind, allocator)
    }

    fn unmap_page(&self, aspace: Self::AddressSpace, virt: u64) -> Result<Option<u64>, ()> {
        paging::unmap_page(aspace, virt)
    }

    fn translate(&self, aspace: Self::AddressSpace, virt: u64) -> Option<u64> {
        paging::translate(aspace, virt)
    }

    fn tlb_flush_page(&self, virt: u64) {
        paging::tlb_flush_page(virt)
    }
}

struct DumbKernelAlloc;
impl FrameAllocatorHook for DumbKernelAlloc {
    fn alloc_frame(&self) -> Option<u64> { None }
}

pub fn hcf() -> ! {
    loop { unsafe { asm!("wfi"); } }
}

#[inline]
fn read_time() -> u64 {
    let val: u64;
    unsafe { asm!("csrr {}, time", out(reg) val); }
    val
}
