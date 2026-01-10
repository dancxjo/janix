use core::arch::asm;
use kernel::IrqState;
use kernel::time::MonotonicClamp;
use crate::runtime::ArchRuntime;

mod paging;
mod simd;
mod task;
mod exception;
mod timer;
mod gic;

/// The architecture-specific runtime for aarch64.
pub struct AArch64Runtime {
    serial: SerialPort,
}

pub type Runtime = crate::runtime::Runtime<AArch64Runtime>;

pub const fn create_runtime() -> Runtime {
    Runtime::new(AArch64Runtime::new())
}

impl AArch64Runtime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort::new(),
        }
    }
}

impl ArchRuntime for AArch64Runtime {
    fn putchar(&self, c: u8) {
        self.serial.putchar(c);
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        let raw = read_cntvct_el0();
        self.serial.clamp.clamp(raw)
    }

    fn mono_freq_hz(&self) -> u64 {
        read_cntfrq_el0()
    }

    fn irq_disable(&self) -> IrqState {
        // AArch64: Mask DAIF
        let daif: u64;
        unsafe {
            asm!("mrs {}, daif", out(reg) daif, options(nomem, nostack));
            asm!("msr daifset, #2", options(nomem, nostack)); // Mask IRQ (bit 1)
        }
        // Extract original I bit (bit 7 of DAIF)
        IrqState(((daif >> 7) & 1) as usize)
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 == 0 {
             unsafe { asm!("msr daifclr, #2", options(nomem, nostack)); } // Unmask if it was 0
        } else {
             unsafe { asm!("msr daifset, #2", options(nomem, nostack)); } // Mask if it was 1
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
         unsafe { asm!("dmb sy", options(nostack, preserves_flags)); }
    }
    
    fn icache_invalidate(&self) {
         unsafe { 
             asm!("ic ialluis", options(nostack, preserves_flags));
             asm!("dsb ish", options(nostack, preserves_flags));
             asm!("isb", options(nostack, preserves_flags));
         }
    }

    // Paging Delegates
    fn map_page(&self, _handle: usize, virt: u64, phys: u64, flags: u64) -> Result<(), ()> {
        // Convert flags if needed
        let pflags = kernel::memory::paging::PageFlags::new(flags);
        paging::map_page(virt, kernel::memory::frame_alloc::PhysFrame(phys), pflags)
    }

    fn map_page_with_allocator(
        &self, 
        _handle: usize, 
        virt: u64, 
        phys: u64, 
        _flags: u64, // BootHeap usually implies Present|Writable, we hardcoded checks in paging
        allocator: &mut kernel::memory::boot_frame_alloc::BootFrameAllocator
    ) -> Result<(), ()> {
        paging::map_bootheap_page(virt, phys, allocator);
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

    // Syscall / Exceptions
    fn register_syscall_handler(&self, entry: u64) {
        unsafe {
             exception::init();
             exception::register_syscall_handler(entry);
             timer::init();
        }
    }
    
    fn set_kernel_stack(&self, _stack_top: u64) {
        // AArch64 uses SP_EL1 for kernel stack, which is switched via context_switch.
        // If we needed to update TPIDR_EL1 or similar for a fresh trap handler stack, we would do it here.
        // For now, we assume SP_EL1 is sufficient.
    }

    unsafe fn enter_user_mode(&self, context: &kernel::arch::TrapFrame) -> ! {
        unsafe {
            let context_ptr = context as *const _ as u64;
             asm!(
                "mov x0, {}",
                "b aarch64_enter_user_mode",
                in(reg) context_ptr,
                options(noreturn)
            );
        }
    }
}

/// Serial port implementation for aarch64 using Semihosting.
/// (PL011 MMIO requires identity mapping of 0x09000000 which may be missing)
pub struct SerialPort {
    pub clamp: MonotonicClamp,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
        }
    }

    fn putchar(&self, c: u8) {
        let ch = c;
        unsafe {
            // Semihosting call: SYS_WRITEC (0x03)
            // W0 = Operation 0x03
            // X1 = Pointer to character
            asm!(
                "hlt #0xF000",
                in("w0") 0x03,
                in("x1") &ch,
                options(nostack, preserves_flags)
            );
        }
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
