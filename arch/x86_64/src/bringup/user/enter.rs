use super::UserEntryRegs;
use alloc::alloc::{alloc_zeroed, Layout};
use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::structures::paging::PhysFrame as X86PhysFrame;
use x86_64::PhysAddr;

// Hardcoded selectors corresponding to standard ThingOS layout (KCode=8, KData=16, TSS=24, UData=40, UCode=48)
// Removed hardcoded selectors. Using crate::gdt instead.

pub fn enter_user_mode(regs: &UserEntryRegs) -> ! {
    let (cs, ss) = unsafe {
        (
            crate::bringup::gdt::USER_CODE_SELECTOR.0,
            crate::bringup::gdt::USER_DATA_SELECTOR.0,
        )
    };

    let x86_regs = X86UserEntryRegs {
        rip: regs.entry_point,
        rsp: regs.user_stack & !0xF, // Align 16
        rflags: 0x202,               // IF=1, bit 1=1
        user_cs: cs as u64 | 3,
        user_ss: ss as u64 | 3,
        rdi: regs.arg0,
    };

    // Safety: Jumping to user mode. Assumes generic bridge setup calls this.
    unsafe { enter_user_mode_asm(&x86_regs as *const _) }
}

#[repr(C)]
struct X86UserEntryRegs {
    rip: u64,
    rsp: u64,
    rflags: u64,
    user_cs: u64,
    user_ss: u64,
    rdi: u64,
}

#[unsafe(naked)]
unsafe extern "C" fn enter_user_mode_asm(_regs: *const X86UserEntryRegs) -> ! {
    // rcx = regs pointer
    // ABI: rdi = regs pointer (System V). Wait, `extern "C"` on x86_64 (Linux/SystemV) uses RDI.
    // Confirm target conv.
    core::arch::naked_asm!(
        "mov rcx, rdi", // Move regs ptr to rcx (sysret convention placeholder or just GPR)
        // Load target registers
        "mov rdi, [rcx + 40]", // rdi = arg0 (offset 5 * 8)
        "mov r11, [rcx + 16]", // r11 = rflags
        // iretq stack setup
        "mov rax, [rcx + 32]", // ss
        "push rax",
        "mov rax, [rcx + 8]", // rsp
        "push rax",
        "push r11",            // rflags
        "mov rax, [rcx + 24]", // cs
        "push rax",
        "mov rax, [rcx]", // rip
        "push rax",
        // Clear segments?
        "mov ax, [rcx + 32]", // Load SS (User Data selector)
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "// mov gs, ax", // DO NOT DO THIS. Loading a selector zeros the base address in 64-bit mode.
        // swapgs if we were in kernel? usually yes if we return to user.
        "swapgs",
        "iretq"
    )
}

/// Resume user mode execution with the given context.
/// Note: FPU state should already be restored by the scheduler before calling this.
pub fn resume_user_mode(context: &[u64]) -> ! {
    // Context layout: [r15...rax, rip, cs, rflags, rsp, ss]
    // The context slice is assumed to be at the TOP of the kernel stack for the target thread.
    // (Or rather, populating the space just below top).
    // stack_top = context_ptr + 20 * 8.
    // NOTE: This assumes the context is the LAST thing on the stack.
    // In our scheduler, kernel_stack is Vec<u128>, and kernel_stack_top is the actual top.
    // The scheduler sets context into thread.context which is a field.
    // CpuBridge::switch calls into this path with &Context.
    // We should probably rely on the caller setting the stack top correctly BEFORE calling this,
    // or pass the stack top explicitly.
    // For now, let's look at Bridge::resume_user_mode in lib.rs.
    let stack_top = context.as_ptr() as u64 + (context.len() * 8) as u64;

    unsafe {
        crate::bringup::gdt::set_kernel_stack(stack_top);
        crate::bringup::interrupts::syscall::set_kernel_stack(stack_top);
        resume_user_mode_asm(context.as_ptr())
    }
}

#[unsafe(naked)]
unsafe extern "C" fn resume_user_mode_asm(_context: *const u64) -> ! {
    core::arch::naked_asm!(
        "mov rsp, rdi", // context ptr
        // Restore GPRs
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbp",
        "pop rbx",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rcx",
        "pop rdx",
        "pop rsi",
        "pop rdi",
        "pop rax",
        // Stack now has [rip, cs, rflags, rsp, ss]

        // Check CS (at rsp + 8) to decide on swapgs
        "test byte ptr [rsp + 8], 3",
        "jz 1f", // Jump if kernel (0)
        "swapgs",
        "iretq",
        "1:",
        // Kernel Return logic.
        // We have [RIP, CS, RFLAGS, RSP, SS] on stack.
        // In 64-bit mode, iretq ALWAYS pops 5 items.
        // Our kernel return logic was trying to "skip" some, which is wrong.
        "iretq"
    )
}

static mut PHYS_MEM_OFFSET: u64 = 0;

pub unsafe fn init_user_stack(phys_mem_offset: u64) {
    unsafe {
        PHYS_MEM_OFFSET = phys_mem_offset;
    }
    // Stub
}

pub fn alloc_user_stack() -> u64 {
    const USER_STACK_SIZE: usize = 64 * 1024;
    let layout = Layout::from_size_align(USER_STACK_SIZE, 16).unwrap();
    let ptr = unsafe { alloc_zeroed(layout) };
    let addr = ptr as u64;

    // Start/End mapping logic stubbed for now or reuse init code
    addr + USER_STACK_SIZE as u64
}

static KERNEL_CR3: AtomicU64 = AtomicU64::new(0);

pub fn activate_address_space(token: Option<u64>) {
    let current = Cr3::read().0.start_address().as_u64();
    if KERNEL_CR3.load(Ordering::Relaxed) == 0 {
        KERNEL_CR3.store(current, Ordering::Relaxed);
    }
    let target = token.unwrap_or_else(|| KERNEL_CR3.load(Ordering::Relaxed));
    if current != target {
        unsafe {
            let frame = X86PhysFrame::from_start_address(PhysAddr::new(target)).unwrap();
            Cr3::write(frame, Cr3Flags::empty());
        }
    }
}
