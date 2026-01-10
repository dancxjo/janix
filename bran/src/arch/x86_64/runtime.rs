use core::arch::asm;
use kernel::IrqState;
use crate::runtime::ArchRuntime;

use super::simd;
use super::serial::{SerialPort, rdtsc};
use super::{gdt, percpu, syscall, paging};

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
    
    fn new_context(&self) -> alloc::boxed::Box<dyn kernel::boot::ArchContext> {
        alloc::boxed::Box::new(crate::arch::x86_64::trap::Context::default())
    }

    fn init_task_context(
        &self, 
        out: &mut dyn kernel::boot::ArchContext,
        kstack_top: u64, 
        entry: extern "C" fn(usize) -> !, 
        arg: usize
    ) -> usize {
         let ctx = out.as_any_mut().downcast_mut::<crate::arch::x86_64::trap::Context>().expect("init_task_context: not x86_64 context");
         let tf = &mut ctx.tf;
         // Minimal kernel thread context for Trap Return
         tf.rip = entry as u64; 
         tf.rsp = kstack_top;
         tf.rflags = 0x202; // IF | Reserved
         tf.cs = 0x8; // Kernel Code
         tf.ss = 0x10; // Kernel Data
         tf.rdi = arg as u64;
         0
    }

    fn save_from_trap(&self, tf: &dyn kernel::boot::ArchTrapFrame, out: &mut dyn kernel::boot::ArchContext) {
        let tf_concrete = tf.as_any().downcast_ref::<crate::arch::x86_64::trap::TrapFrame>().expect("save_from_trap: not x86_64 tf");
        let out_concrete = out.as_any_mut().downcast_mut::<crate::arch::x86_64::trap::Context>().expect("save_from_trap: not x86_64 context");
        out_concrete.tf = *tf_concrete;
    }

    fn load_into_trap(&self, ctx: &dyn kernel::boot::ArchContext, tf: &mut dyn kernel::boot::ArchTrapFrame) {
        let ctx_concrete = ctx.as_any().downcast_ref::<crate::arch::x86_64::trap::Context>().expect("load_into_trap: not x86_64 context");
        let tf_concrete = tf.as_any_mut().downcast_mut::<crate::arch::x86_64::trap::TrapFrame>().expect("load_into_trap: not x86_64 tf");
        *tf_concrete = ctx_concrete.tf;
    }
    
    unsafe fn switch_tasks(&self, old: &mut dyn kernel::boot::ArchContext, new: &dyn kernel::boot::ArchContext) {
        // Switch between two "Kernel Contexts".
        // In our model (unified), a Kernel Context is just a TrapFrame (saved state).
        // `old` is the CURRENT running context. We must save current state to it.
        // `new` is the NEXT context. We must restore state from it.
        
        let old_ctx = old.as_any_mut().downcast_mut::<crate::arch::x86_64::trap::Context>().expect("switch_tasks: not x86_64 old");
        let new_ctx = new.as_any().downcast_ref::<crate::arch::x86_64::trap::Context>().expect("switch_tasks: not x86_64 new");
        
        let old_tf_ptr = &mut old_ctx.tf as *mut crate::arch::x86_64::trap::TrapFrame;
        let new_tf_ptr = &new_ctx.tf as *const crate::arch::x86_64::trap::TrapFrame;
        
        // We use inline assembly to:
        // 1. Push current state to stack (matching TrapFrame layout).
        // 2. Mov stack to `old_tf`.
        // 3. Mov `new_tf` to stack? Or just jump to `return_from_trap` logic?
        // Reuse `return_from_trap` logic to restore `new_tf`.
        
        unsafe {
            core::arch::asm!(
                // Save current state as if we trapped.
                // We are "returning" to label `1f`.
                
                // Construct TrapFrame on stack.
                // SS (0x10 Kernel Data)
                "push 0x10",
                // RSP (Current RSP)
                "push rsp", // Wait, pushing rsp pushes *old* rsp? Yes.
                // RFLAGS
                "pushfq",
                // CS (0x8 Kernel Code)
                "push 0x8",
                // RIP (label 2f)
                "lea rax, [rip + 2f]",
                "push rax",
                
                // Error(0), Trap(0) - align with struct
                "push 0",
                "push 0",
                
                // GPRs
                "push rax", "push rbx", "push rcx", "push rdx",
                "push rbp", "push rdi", "push rsi", "push r8",
                "push r9",  "push r10", "push r11", "push r12",
                "push r13", "push r14", "push r15",
                
                // Now Stack contains a full TrapFrame.
                // Copy Stack to `old_tf`.
                // memcpy(old_tf_ptr, rsp, sizeof(TrapFrame)).
                // TrapFrame size = 15*8 + 2*8 + 5*8 = 176 bytes?
                // Struct has: 15 GPRs (120) + 2 (16) + 5 (40) = 176 bytes.
                // We can use REP MOVSB or just manual copy.
                // Or just: `*old_tf_ptr = *(rsp as *const TrapFrame)`.
                // In assembly:
                "mov rdi, {old_tf}",
                "mov rsi, rsp",
                "mov rcx, 176", // bytes
                "cld",
                "rep movsb", // copy stack to heap
                
                // Now switch to new task.
                // Call return_from_trap logic on `new_tf_ptr`.
                // We can just jump to a helper or replicate logic.
                // Logic:
                "mov rsp, {new_tf}", // Pivot stack to new TrapFrame (on heap)
                // Restore GPRs
                "pop r15", "pop r14", "pop r13", "pop r12",
                "pop r11", "pop r10", "pop r9",  "pop r8",
                "pop rsi", "pop rdi", "pop rbp", "pop rdx",
                "pop rcx", "pop rbx", "pop rax",
                
                // Skip TrapNum/Error
                "add rsp, 16",
                
                // Check CS? Assuming correct.
                // Check CS (RSP + 8) for User Mode (3)
                "test byte ptr [rsp + 8], 3",
                "jz 2f",
                "swapgs",
                "2:",
                "iretq",
                
                "2:",
                // We returned here!
                // Clean up stack?
                // We pushed 176 bytes. We need to pop them or reset RSP.
                // Actually, if we just returned via IRETQ from `old_tf`, IRETQ popped RIP, CS, RFLAGS, RSP, SS.
                // Wait. `iretq` restores RSP.
                // If we saved RSP as "Before pushes", then `iretq` restores RSP to THAT point.
                // So the pushed stuff is GONE (popped logically).
                // So we are good.
                
                old_tf = in(reg) old_tf_ptr,
                new_tf = in(reg) new_tf_ptr,
                out("rdi") _, out("rsi") _, out("rcx") _, out("rax") _
                // options(noreturn) removed - switch returns!
            );
        }
    }

    unsafe fn return_from_trap(&self, tf: &dyn kernel::boot::ArchTrapFrame) -> ! {
        let tf = tf.as_any().downcast_ref::<crate::arch::x86_64::trap::TrapFrame>().expect("return_from_trap: not x86_64 tf");
        // We need a pointer to the trap frame. It MUST be valid.
        let tf_ptr = tf as *const crate::arch::x86_64::trap::TrapFrame;
        
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
                // trap_num (8), error_code (8)
                // rip (user_rip)
                // cs
                // rflags (user_rflags)
                // rsp
                // ss
                
                // Skip trap_num + error_code
                "add rsp, 16",
                
                "test byte ptr [rsp + 8], 3",
                "jz 3f",
                "swapgs",
                "3:",
                "iretq",
                
                tf = in(reg) tf_ptr,
                options(noreturn)
            );
        }
    }

    fn make_user_trapframe(&self, rip: u64, rsp: u64) -> alloc::boxed::Box<dyn kernel::boot::ArchTrapFrame> {
         alloc::boxed::Box::new(crate::arch::x86_64::trap::TrapFrame::new_user(rip, rsp))
    }
    
    fn new_trapframe(&self) -> alloc::boxed::Box<dyn kernel::boot::ArchTrapFrame> {
         alloc::boxed::Box::new(crate::arch::x86_64::trap::TrapFrame::default())
    }
    
    fn register_syscall_handler(&self, entry: u64) {
        unsafe { syscall::enable(entry) }
    }
    
    fn set_kernel_stack(&self, stack_top: u64) {
        unsafe { gdt::set_kernel_stack(stack_top) }
    }
}

/// Halt and catch fire - enters an infinite halt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
