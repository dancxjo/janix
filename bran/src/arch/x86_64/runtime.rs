use core::arch::asm;
use kernel::IrqState;
use crate::runtime::ArchRuntime;

use super::simd;
use super::serial::{SerialPort, rdtsc};
use super::{gdt, percpu, syscall, paging, task};

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
    fn fence_full(&self) {
        unsafe { asm!("mfence", options(nostack, preserves_flags)) };
    }

    // Paging
    fn page_size(&self) -> usize {
        4096
    }
     fn kernel_virt_base(&self) -> u64 {
        0xFFFF_8000_0000_0000
    }
    
    fn phys_to_virt_offset(&self) -> u64 {
        paging::phys_to_virt_offset()
    }
    
    fn map_page(&self, handle: usize, virt: u64, phys: u64, flags: u64) -> Result<(), ()> {
        let mut aspace = paging::AddressSpace { root: kernel::memory::frame_alloc::PhysFrame(handle as u64) };
        aspace.map_page(virt, kernel::memory::frame_alloc::PhysFrame(phys), kernel::memory::paging::PageFlags::new(flags))
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
    
    fn unmap_page(&self, handle: usize, virt: u64) {
        let mut aspace = paging::AddressSpace { root: kernel::memory::frame_alloc::PhysFrame(handle as u64) };
        let _ = aspace.unmap_page(virt);
    }
    
    fn translate(&self, handle: usize, virt: u64) -> Option<u64> {
        let aspace = paging::AddressSpace { root: kernel::memory::frame_alloc::PhysFrame(handle as u64) };
        aspace.translate(virt).map(|f| f.0)
    }
    
    fn new_address_space(&self) -> usize {
        let aspace = paging::AddressSpace::new();
        aspace.root.0 as usize
    }
    
    fn switch_address_space(&self, handle: usize) {
        let aspace = paging::AddressSpace { root: kernel::memory::frame_alloc::PhysFrame(handle as u64) };
        aspace.switch();
    }
    
    fn current_address_space(&self) -> usize {
        let aspace = paging::AddressSpace::active();
        aspace.root.0 as usize
    }

    fn tlb_flush_page(&self, virt: u64) {
        paging::tlb_flush_page(virt);
    }
    
    fn tlb_flush_all(&self) {
        paging::tlb_flush_all();
    }

    // Task Context
    fn context_init(
        &self, 
        ctx_handle: &mut u64, 
        kstack_top: u64, 
        entry: extern "C" fn(usize) -> !, 
        arg: usize
    ) {
        task::context_init(ctx_handle, kstack_top, entry, arg);
    }

    unsafe fn context_switch(&self, old_handle_ptr: *mut u64, new_handle: u64) {
        unsafe {
             task::context_switch(old_handle_ptr, new_handle);
        }
    }

    fn register_syscall_handler(&self, entry: u64) {
        unsafe {
            gdt::init();
            percpu::init_gs_base();
            syscall::enable(entry);
            super::interrupt::init();
            super::timer::init();
        }
    }

    fn set_kernel_stack(&self, stack_top: u64) {
        unsafe {
            gdt::set_tss_rsp0(stack_top);
            percpu::set_kernel_rsp0(stack_top);
        }
    }
}

/// Halt and catch fire - enters an infinite halt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
