use core::arch::asm;
use kernel::{IrqState, UserTaskSpec, FrameAllocatorHook, MapPerms, MapKind};
use kernel::time::MonotonicClamp;
use crate::runtime::ArchRuntime;

pub mod task;
pub mod paging;
pub mod simd;

pub struct X86_64Runtime {
    clamp: MonotonicClamp,
}

impl X86_64Runtime {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
        }
    }
}

pub use task::X86_64Context;
pub use paging::X86_64AddressSpace;

impl ArchRuntime for X86_64Runtime {
    type Context = X86_64Context;
    type AddressSpace = X86_64AddressSpace;

    fn init(&self, hhdm_offset: u64) { paging::init(hhdm_offset); }
    fn putchar(&self, c: u8) {
        unsafe {
            let port = 0x3f8;
            core::arch::asm!("out dx, al", in("dx") port, in("al") c);
        }
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        let low: u32;
        let high: u32;
        unsafe {
            core::arch::asm!("rdtsc", out("eax") low, out("edx") high);
        }
        let raw = ((high as u64) << 32) | (low as u64);
        self.clamp.clamp(raw)
    }

    fn mono_freq_hz(&self) -> u64 {
        2_000_000_000 // Placeholder
    }

    fn irq_disable(&self) -> IrqState {
        let rflags: u64;
        unsafe {
            core::arch::asm!("pushfq", "pop {}", out(reg) rflags);
            core::arch::asm!("cli");
        }
        IrqState(((rflags >> 9) & 1) as usize)
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 != 0 {
            unsafe { core::arch::asm!("sti"); }
        }
    }

    fn threads_supported(&self) -> bool { true }
    
    fn simd_init_cpu(&self) { simd::init_cpu(); }
    fn simd_state_layout(&self) -> (usize, usize) { simd::STATE_LAYOUT }
    unsafe fn simd_save(&self, dst: *mut u8) { unsafe { simd::save(dst) } }
    unsafe fn simd_restore(&self, src: *const u8) { unsafe { simd::restore(src) } }

    fn fence_full(&self) { 
        unsafe { core::arch::asm!("mfence", options(nostack, preserves_flags)); }
    }

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
        unsafe { core::arch::asm!("mov cr3, {}", in(reg) aspace.0); }
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
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
