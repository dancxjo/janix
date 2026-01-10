use core::arch::asm;
use kernel::{IrqState, TaskContext, UserTaskSpec, AddressSpaceHandle, FrameAllocatorHook};
use crate::runtime::ArchRuntime;

use super::simd;
use super::serial::{SerialPort, rdtsc};
use super::task;
use super::paging;

/// The architecture-specific runtime for x86_64.
pub struct X86_64Runtime {
    serial: SerialPort,
}

pub type Runtime = crate::runtime::Runtime<X86_64Runtime>;

impl X86_64Runtime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort::new(),
        }
    }
}

impl ArchRuntime for X86_64Runtime {
    fn putchar(&self, c: u8) {
        self.serial.putchar(c);
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        unsafe {
            let raw = rdtsc();
            self.serial.clamp.clamp(raw)
        }
    }

    fn mono_freq_hz(&self) -> u64 {
        self.serial.calibrate()
    }

    fn irq_disable(&self) -> IrqState {
        let rflags: usize;
        unsafe {
            asm!("pushfq; pop {}", out(reg) rflags, options(nomem, preserves_flags));
            asm!("cli", options(nomem, nostack));
        }
        IrqState((rflags >> 9) & 1) 
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 != 0 {
            unsafe { asm!("sti", options(nomem, nostack)) };
        } else {
            unsafe { asm!("cli", options(nomem, nostack)) };
        }
    }

    // SIMD
    fn simd_init_cpu(&self) {
        simd::init_cpu();
    }

    fn simd_state_layout(&self) -> (usize, usize) {
        simd::STATE_LAYOUT
    }

    unsafe fn simd_save(&self, dst: *mut u8) {
        unsafe { simd::save(dst) };
    }

    unsafe fn simd_restore(&self, src: *const u8) {
        unsafe { simd::restore(src) };
    }

    // Barriers
    fn threads_supported(&self) -> bool { true }
    
    fn fence_full(&self) {
        unsafe { asm!("mfence", options(nostack, preserves_flags)) };
    }

    // Tasking
    fn init_kernel_context(&self, entry: extern "C" fn(usize) -> !, stack_top: u64, arg: usize) -> TaskContext {
        task::init_kernel_context(entry, stack_top, arg)
    }

    fn init_user_context(&self, spec: UserTaskSpec, kstack_top: u64) -> TaskContext {
        task::init_user_context(spec, kstack_top)
    }

    unsafe fn switch(&self, from: &mut TaskContext, to: &TaskContext) {
        unsafe { task::switch(from, to) }
    }

    // Paging
    fn make_user_address_space(&self) -> AddressSpaceHandle {
        paging::make_user_address_space(paging::active_address_space(), &DumbKernelAlloc)
    }

    fn active_address_space(&self) -> AddressSpaceHandle {
        paging::active_address_space()
    }

    fn map_page(&self, aspace: AddressSpaceHandle, virt: u64, phys: u64, flags: kernel::memory::paging::PageFlags, allocator: &dyn FrameAllocatorHook) -> Result<(), ()> {
        paging::map_page(aspace, virt, phys, flags, allocator)
    }

    fn unmap_page(&self, aspace: AddressSpaceHandle, virt: u64) -> Result<Option<u64>, ()> {
        paging::unmap_page(aspace, virt)
    }

    fn translate(&self, aspace: AddressSpaceHandle, virt: u64) -> Option<u64> {
        paging::translate(aspace, virt)
    }

    fn tlb_flush_page(&self, virt: u64) {
        paging::tlb_flush_page(virt)
    }
}

struct DumbKernelAlloc;
impl FrameAllocatorHook for DumbKernelAlloc {
    fn alloc_frame(&self) -> Option<u64> {
        None
    }
}

pub fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
