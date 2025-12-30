use crate::bringup::gdt::{
    KERNEL_CODE_SELECTOR, KERNEL_DATA_SELECTOR, USER_CODE_SELECTOR, USER_DATA_SELECTOR,
};
use core::arch::naked_asm;
use x86_64::registers::model_specific::{
    Efer, EferFlags, GsBase, KernelGsBase, LStar, SFMask, Star,
};
use x86_64::registers::rflags::RFlags;
use x86_64::VirtAddr;
use kernel::bridge::CpuBridge;

// Scratch Layout:
// [0]: User RSP (Temporary storage)
// [1]: Kernel RSP Top (Set by Scheduler)
#[repr(align(4096))] // Page aligned to be safe/clean
struct Scratch([u64; 2]);
static mut GS_SCRATCH: Scratch = Scratch([0; 2]);

pub unsafe fn init() {
    // 0. Setup KernelGSBase
    // We are in Kernel Mode, so Active GS Base should point to Scratch.
    // The "Shadow" (MSR) should point to User (0), so swapgs loads it.
    let gs_base = VirtAddr::new(core::ptr::addr_of!(GS_SCRATCH) as u64);
    GsBase::write(gs_base);
    KernelGsBase::write(VirtAddr::zero());

    // 1. Enable syscall/sysret instruction via EFER
    let mut efer = Efer::read();
    efer.insert(EferFlags::SYSTEM_CALL_EXTENSIONS);
    unsafe {
        Efer::write(efer);
    }

    // 2. Setup LSTAR (Target address for syscall)
    LStar::write(VirtAddr::new(syscall_handler_naked as *const () as u64));

    // 3. Setup STAR
    unsafe {
        // Star::write failed validation. Using raw MSR write.
        // MSR 0xC0000081
        // 63-48: User Base (0x10 - KERNEL_DATA_SELECTOR) -> CS=0x20, SS=0x18
        // 47-32: Kernel Base (0x8 - KERNEL_CODE_SELECTOR) -> CS=0x8, SS=0x10?
        // 31-0:  Reserved (EIP)
        let star_val: u64 =
            ((KERNEL_DATA_SELECTOR.0 as u64) << 48) | ((KERNEL_CODE_SELECTOR.0 as u64) << 32);
        x86_64::registers::model_specific::Msr::new(0xC0000081).write(star_val);
    }

    // 4. Setup SFMask
    SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG);
}

#[unsafe(naked)]
unsafe extern "C" fn syscall_handler_naked() {
    naked_asm!(
        "swapgs",
        // Save User RSP to Scratch[0]
        "mov gs:[0], rsp",
        // Load Kernel RSP from Scratch[1]
        "mov rsp, gs:[8]",
        // Push User RSP (from Scratch) onto Kernel Stack
        // We need a reg? "push qword ptr gs:[0]" is valid x86.
        "push qword ptr gs:[0]",
        // Stack: [User RSP]
        // Now save user context
        "push rcx", // User RIP
        "push r11", // User RFLAGS
        "push rbp",
        "push rbx",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        // Arguments setup: (RDI=a1, RSI=a2...)
        // ABI: RAX(num), RDI, RSI, RDX, R10(a4), R8(a5), R9(a6)
        // Rust Dispatch: fn(num, a1, a2, a3, a4, a5, a6)
        // Regs: RDI, RSI, RDX, RCX, R8, R9, Stack

        // Stack Alignment Check:
        // Pushed so far:
        // UserRSP (1) + 8 Regs (8) = 9 words.
        // RSP is Misaligned (8 mod 16).
        // Pushing R9 (Arg 6) adds 1 word. Total 10 words.
        // 10 * 8 = 80 bytes. Aligned (0 mod 16).
        // So NO padding needed before call.
        "push r9", // Arg 6 (a6) -> Stack
        // Register shuffle
        // a5 (R8) -> R9
        "mov r9, r8",
        // a4 (R10) -> R8
        "mov r8, r10",
        // a3 (RDX) -> RCX
        "mov rcx, rdx",
        // a2 (RSI) -> RDX
        "mov rdx, rsi",
        // a1 (RDI) -> RSI
        "mov rsi, rdi",
        // num (RAX) -> RDI
        "mov rdi, rax",
        "call syscall_dispatch",
        "add rsp, 8", // Pop Arg6 (No Padding)
        // Restore Regs and user state
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbx",
        "pop rbp",
        "pop r11", // user RFLAGS
        "pop rcx", // user RIP
        "pop rax", // user RSP
        "mov rsp, rax",
        "swapgs",
        "sysretq"
    );
}

#[no_mangle]
extern "C" fn syscall_dispatch(
    num: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> isize {
    static LOG_COUNT: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    if LOG_COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed) < 16 {
                let bridge = crate::Bridge;
        let rsp: *const u64;
        unsafe { core::arch::asm!("mov {}, rsp", out(reg) rsp) };

        // Stack layout at entry:
        //  [0] return addr into syscall_handler_naked
        //  [1] saved r9 (arg6)
        //  [2] r15, [3] r14, [4] r13, [5] r12, [6] rbx, [7] rbp
        //  [8] user rflags (from r11), [9] user rip (from rcx), [10] user rsp
        let saved_flags = unsafe { *rsp.add(8) };
        let saved_rip = unsafe { *rsp.add(9) };
        let saved_rsp = unsafe { *rsp.add(10) };

        bridge.log("SYSCALL entry num=");
        crate::print_hex(&bridge, num as u64);
        bridge.log(" rip=");
        crate::print_hex(&bridge, saved_rip);
        bridge.log(" rflags=");
        crate::print_hex(&bridge, saved_flags);
        bridge.log(" ursp=");
        crate::print_hex(&bridge, saved_rsp);
        bridge.log("\n");
    }
    unsafe {
        if let Some(hook) = SYSCALL_HOOK {
            hook(num, a1, a2, a3, a4, a5, a6)
        } else {
            -1
        }
    }
}

pub static mut SYSCALL_HOOK: Option<fn(usize, usize, usize, usize, usize, usize, usize) -> isize> =
    None;

pub fn set_syscall_hook(hook: fn(usize, usize, usize, usize, usize, usize, usize) -> isize) {
    unsafe {
        SYSCALL_HOOK = Some(hook);
    }
}

pub fn set_kernel_stack(stack_top: u64) {
    unsafe {
        // Update Scratch[1]
        GS_SCRATCH.0[1] = stack_top;
    }
}
