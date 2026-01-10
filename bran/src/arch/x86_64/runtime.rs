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
    type Context = kernel::arch::x86_64::Context;
    type TrapFrame = kernel::arch::x86_64::TrapFrame;

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
    fn init_task_context(
        &self, 
        out: &mut Self::Context,
        kstack_top: u64, 
        entry: extern "C" fn(usize) -> !, 
        arg: usize
    ) {
         let tf = &mut out.0;
         // Minimal kernel thread context for Trap Return
         tf.rip = entry as u64;
         tf.rsp = kstack_top;
         tf.rflags = 0x202; // IF | Reserved
         tf.cs = 0x8; // Kernel Code
         tf.ss = 0x10; // Kernel Data
         tf.rdi = arg as u64;
    }

    fn save_from_trap(&self, tf: &Self::TrapFrame, out: &mut Self::Context) {
        out.0 = tf.clone();
    }

    fn load_into_trap(&self, ctx: &Self::Context, tf: &mut Self::TrapFrame) {
        *tf = ctx.0.clone();
    }

    unsafe fn return_from_trap(&self, tf: *const Self::TrapFrame) -> ! {
        unsafe {
            core::arch::asm!(
                "cli",
                "mov rsp, {tf}",
                
                // Restore GPRs
                "pop r15", "pop r14", "pop r13", "pop r12",
                "pop r11", "pop r10", "pop r9",  "pop r8",
                "pop rsi", "pop rdi", "pop rbp", "pop rdx",
                "pop rcx", "pop rbx", "pop rax",

                // TrapFrame layout:
                // ... GPRs ...
                // trap_num (8)
                // error_code (8)
                // rip (8)
                // cs (8)
                // rflags (8)
                // rsp (8)
                // ss (8)
                
                // RSP is now at trap_num.
                // We need to check CS (at rsp + 16 + 8).
                // trap_num (8) + error_code (8) + rip (8) + cs (8).
                // Offset to CS from current RSP = 24.
                
                "cmp qword ptr [rsp+24], 0x8", // Check CS against Kernel Code Selector
                "je 2f", 
                
                // Returning to User
                "swapgs",
                
                "2:",
                // Skip trap_num and error_code to point to RIP
                "add rsp, 16", 
                
                "iretq",
                
                tf = in(reg) tf,
                options(noreturn)
            );
        }
    }

    fn make_user_trapframe(&self, rip: u64, rsp: u64) -> Self::TrapFrame {
        kernel::arch::x86_64::TrapFrame::new_user(rip, rsp)
    }
}

/// Halt and catch fire - enters an infinite halt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
