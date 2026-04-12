use core::arch::{asm, global_asm};

// MSR Constants
const MSR_EFER: u32 = 0xC0000080;
const MSR_STAR: u32 = 0xC0000081;
const MSR_LSTAR: u32 = 0xC0000082;
const MSR_SFMASK: u32 = 0xC0000084;
const MSR_GS_BASE: u32 = 0xC0000101;
const MSR_KERNEL_GS_BASE: u32 = 0xC0000102;

const EFER_SCE: u64 = 1; // Syscall Enable
const EFER_NXE: u64 = 1 << 11; // No-Execute Enable

// GDT Selectors (Must match what we assume in userspace/trampolines)
// Kernel Code: 0x08 (1)
// Kernel Data: 0x10 (2)
// User Code32: 0x18 (3)
// User Data:   0x20 (4)
// User Code64: 0x28 (5) -> Wait, usually Linux uses:
//   32-bit STAR: [31:16] CS (user 32), [15:0] Target CS (kernel)
//   48-bit STAR: [63:48] CS (user 32/64 ret).
// Let's assume:
//  Kernel Code = 0x08
//  Kernel Data = 0x10
//  User Data   = 0x18 | 3  (RPL3)
//  User Code   = 0x20 | 3  (RPL3)
// Limine GDT might differ.
// Just for v0.5, let's hardcode what we use.
#[allow(dead_code)]
const _KERNEL_CS: u16 = 0x08;
#[allow(dead_code)]
const _KERNEL_DS: u16 = 0x10;
// When SYSRET loads CS/SS:
//  CS = (STAR[63:48] + 16) | 3
//  SS = (STAR[63:48] + 8) | 3
// So if STAR[63:48] = 0x08 (Kernel Code), then:
//  CS = 0x18 | 3 = User Code 32? No.
// Let's check AMD manuals.
// SYSRET:
//   CS_Sel = STAR[63:48] + 16.
//   SS_Sel = STAR[63:48] + 8.
// User SS is 0x23 (User Data selector), User CS is 0x2B (User Code64 selector).
//  Diff is 8. So SS should be lower index than CS?
//  Usually User Code is *after* User Data in GDT for automatic SYSRET.
//  If User Data = 0x18, User Code = 0x20.
//  Then STAR[63:48] should be 0x10 (Kernel Data? No).
//  STAR[63:48] = 0x10.
//    CS = 0x10 + 16 = 0x20.
//    SS = 0x10 + 8 = 0x18.
//  Yes. So Base Selector = 0x10.
// SYSCALL:
//   CS = STAR[31:47] = 0x08.
//   SS = STAR[31:47] + 8 = 0x10.
// So STAR = (0x10 << 48) | (0x08 << 32).

const MAX_CPUS: usize = 32;

#[derive(Copy, Clone)]
#[repr(C)]
struct CpuLocal {
    scratch_rsp: u64, // offset 0
    kstack_top: u64,  // offset 8
    cpu_index: u64,   // offset 16
    current_tid: u64, // offset 24
}

static mut CPU_LOCAL: [CpuLocal; MAX_CPUS] = [CpuLocal {
    scratch_rsp: 0,
    kstack_top: 0,
    cpu_index: 0,
    current_tid: 0,
}; MAX_CPUS];

pub unsafe fn init(cpu_index: usize) {
    unsafe {
        CPU_LOCAL[cpu_index].cpu_index = cpu_index as u64;

        // 1. Setup GS Base for this specific CPU
        let gs_base = (&raw mut CPU_LOCAL[cpu_index]) as u64;

        // Keep both GS base MSRs pointing at CPU_LOCAL for now so swapgs is safe.
        wrmsr(MSR_GS_BASE, gs_base);
        wrmsr(MSR_KERNEL_GS_BASE, gs_base);

        // 2. Enable SCE (Syscall) and NXE (No-Execute) in EFER
        let efer = rdmsr(MSR_EFER);
        wrmsr(MSR_EFER, efer | EFER_SCE | EFER_NXE);

        // 3. Setup STAR
        let star = ((crate::arch::x86_64::gdt::KERNEL_CODE_SEL as u64) << 32)
            | (((crate::arch::x86_64::gdt::USER_CODE32_SEL ^ 3) as u64) << 48);
        wrmsr(MSR_STAR, star);

        // 4. Setup LSTAR (Entry point)
        wrmsr(MSR_LSTAR, syscall_entry as *const () as usize as u64);

        // 5. Setup SFMASK (Mask Interrupts 0x200)
        wrmsr(MSR_SFMASK, 0x200);
    }
}

unsafe fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high);
    }
    ((high as u64) << 32) | (low as u64)
}

unsafe fn wrmsr(msr: u32, val: u64) {
    let low = val as u32;
    let high = (val >> 32) as u32;
    unsafe {
        asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
    }
}

unsafe extern "C" {
    fn syscall_entry();
}

global_asm!(
    r#"
.section .text
.global syscall_entry
syscall_entry:
    // Enters with CS=Kernel, SS=Kernel.
    // RCX=User RIP, R11=User RFLAGS.
    // RSP=User Stack.
    
    swapgs
    
    // Save User RSP to scratch (offset 0)
    mov %rsp, %gs:0
    
    // Load Kernel RSP from offset 8
    mov %gs:8, %rsp
    
    // Now on Kernel Stack.
    // Build UserTrapFrame. 
    // Struct layout: r15..r8, rbp, rdi, rsi, rdx, rcx, rbx, rax, error_code, int_no, rip, cs, rflags, rsp, ss
    
    // We need to manufacture SS, RSP, RFLAGS, CS, RIP
    // User SS = 0x23 (Hardcoded matches task.rs) | OR we could just save what we think it is. 
    // But 'syscall' doesn't save SS. We assume standard user SS.
    pushq ${user_ss}        // SS
    pushq %gs:0         // User RSP (from scratch)
    pushq %r11          // RFLAGS
    pushq ${user_cs}        // CS
    pushq %rcx          // RIP
    
    // Error Code / Int No
    pushq $4           // int_no (SYS_DEVICE_CALL=4? No, just using 0x80 or similar marker) -> Let's use 0x80 as "Synch Trap" marker
    pushq $0           // error_code
    
    // GPRs
    pushq %rax
    pushq %rbx
    pushq %rcx  // Note: RCX contains User RIP, but we push it as GPR anyway to match struct
    pushq %rdx
    pushq %rsi
    pushq %rdi
    pushq %rbp
    pushq %r8
    pushq %r9
    pushq %r10
    pushq %r11 // Note: R11 contains User RFLAGS
    pushq %r12
    pushq %r13
    pushq %r14
    pushq %r15
    // Arguments for dispatch(n, args)
    // Rust ABI: RDI, RSI.
    // dispatch signature: fn dispatch(n: usize, args: [usize; 6]) -> isize
    
    // Mapping:
    // Syscall ABI (Linux/Standard we chose):
    // RAX = Number
    // RDI = Arg0
    // RSI = Arg1
    // RDX = Arg2
    // R10 = Arg3
    // R8  = Arg4
    // R9  = Arg5
    
    // Rust Call to dispatch(n, a0, a1, a2, a3, a4, a5)
    // RDI = n (from RAX)
    // RSI = a0 (from RDI - wait, conflict)
    // RDX = a1 (from RSI)
    // RCX = a2 (from RDX)
    // R8  = a3 (from R10)
    // R9  = a4 (from R8)
    // Stack = a5 (from R9)
    
    // We need to shuffle.
    // Saved Regs are on stack. We can read from there if needed, or move directly.
    
    // We need to preserve RAX (syscall num) to pass as 1st arg.
    mov %rdi, %r12 // Temp save Arg0 (RDI)
    mov %rsi, %r13 // Temp save Arg1 (RSI)
    mov %rdx, %r14 // Temp save Arg2 (RDX) - Fix: Preserve RDX before overwrite
    
    // Check for SYS_SIGRETURN (0x1505) before shuffling args.
    // If this is sigreturn, call kernel_sigreturn(frame_ptr=rsp) directly.
    cmp $0x1505, %rax
    jne 0f
    mov %rsp, %rdi      // frame_ptr = trap frame on stack
    call kernel_sigreturn
    // RAX = new syscall return value (normally 0); write into frame and restore.
    mov %rax, 112(%rsp)
    jmp .Lrestore_regs
0:
    
    mov %rax, %rdi // 1st Arg: n
    
    mov %r12, %rsi // 2nd Arg: a0
    mov %r13, %rdx // 3rd Arg: a1
    // RDX (Arg2) needs to go to RCX (4th Arg slot for Rust function)
    mov %r14, %rcx 
    
    // R9 (Arg5) needs to go to Stack (7th Arg slot/a5)
    // We must push R9 (A5) BEFORE we overwrite it with A4 (from R8).
    // And we must move R8 (A4) to R9 BEFORE we overwrite R8 with A3 (from R10).
    
    // 1. Save A5 (R9) to Stack
    pushq %r9
    
    // 2. Move A4 (R8) to R9
    mov %r8, %r9
    
    // 3. Move A3 (R10) to R8
    mov %r10, %r8
    
    // Align stack? We pushed odd number of args? 
    // We pushed 1 arg (8 bytes).
    // Previous stack alignment:
    // We pushed 15 regs (8*15) + Err/Int (2*8) + IRET (5*8).
    // 22 * 8 = 176. Divisible by 16. So aligned.
    // Pushing 1 arg -> Not aligned.
    // Sub 8.
    sub $8, %rsp
    // Wait, pushq %r9 put it at RSP.
    // So we need to ensure RSP+8 is aligned 16 call.
    // Before push R9, RSP was aligned.
    // After push R9, RSP is -8.
    // We need RSP to be 16-byte aligned BEFORE call? 
    // No, call pushes RIP (8 bytes), making it 16-byte aligned inside function.
    // So on 'call', RSP should be +8 aligned (ending in 8).
    // If RSP was 0 aligned. Push R9 -> 8 aligned.
    // Perfect.
    
    call kernel_dispatch_flat
    
    // Cleanup stack arg and alignment padding
    // We pushed %r9 (8 bytes) AND sub $8 (8 bytes) = 16 bytes total.
    add $16, %rsp
    
    // Call signal check before returning to user mode.
    // kernel_signal_check(frame_ptr=rsp, syscall_ret=rax) -> modified_ret
    mov %rsp, %rdi      // frame_ptr
    mov %rax, %rsi      // syscall return value
    call kernel_signal_check
    
    // RAX has return value (isize).
    // We need to put it into UserTrapFrame's RAX slot so it gets restored.
    // TrapFrame layout: ... rbx, rax, error_code ...
    // RAX is at top of GPRs (lowest address).
    // Stack top is R15.
    // Struct:
    // R15 (0), R14 (8), R13 (16), R12 (24), R11 (32), R10 (40), R9 (48), R8 (56),
    // RBP (64), RDI (72), RSI (80), RDX (88), RCX (96), RBX (104), RAX (112).
    // So [rsp + 112] is RAX.
    mov %rax, 112(%rsp)
    
.Lrestore_regs:
    
    // Restore
    popq %r15
    popq %r14
    popq %r13
    popq %r12
    popq %r11
    popq %r10
    popq %r9
    popq %r8
    popq %rbp
    popq %rdi
    popq %rsi
    popq %rdx
    popq %rcx
    popq %rbx
    popq %rax
    
    // Switch to sysretq for return (faster and assumes consistent GDT)
    // We need to restore RCX (RIP) and R11 (RFLAGS) for sysretq.
    // GPRs popped above restored User RCX/R11 (clobbered/arguments).
    // The "True" RIP/RFLAGS are in the IRET frame on stack.
    // Stack Check: [Error(0), Int(8), RIP(16), CS(24), RFLAGS(32), RSP(40), SS(48)]
    
    mov 16(%rsp), %rcx  // Load RIP into RCX
    mov 32(%rsp), %r11  // Load RFLAGS into R11
    mov 40(%rsp), %rsp  // Restore User RSP from the saved frame instead of GS scratch
    
    cli
    swapgs
    sysretq
"#,
    options(att_syntax),
    user_ss = const crate::arch::x86_64::gdt::USER_DATA_SEL,
    user_cs = const crate::arch::x86_64::gdt::USER_CODE_SEL,
);

// ─── Signal frame support ──────────────────────────────────────────────────

use super::trap::UserTrapFrame;
use abi::signal::{SigFrame, SigSet, SIGFRAME_MAGIC};

/// Validate that a user-space address range is canonical (non-null, below kernel space).
#[inline(always)]
fn check_user_range(addr: usize, size: usize) -> bool {
    if addr == 0 {
        return false;
    }
    let end = match addr.checked_add(size) {
        Some(e) => e,
        None => return false,
    };
    end < 0x0000_8000_0000_0000
}

/// x86_64 signal frame injection (called via INJECT_FRAME_HOOK).
///
/// Pushes a SigFrame onto the user stack and redirects the trap frame
/// so that sysretq will enter the signal handler.
unsafe fn x86_64_inject_signal_frame(
    frame_ptr: *mut u8,
    signum: u32,
    handler: usize,
    saved_mask: SigSet,
) -> bool {
    let tf = unsafe { &mut *(frame_ptr as *mut UserTrapFrame) };
    let user_rsp = tf.rsp;

    let frame_size = core::mem::size_of::<SigFrame>();
    // Layout (stack grows down): [SigFrame][return_addr(8)]
    let new_rsp_raw = user_rsp.wrapping_sub(frame_size + 8);
    let new_rsp = new_rsp_raw & !0xF_usize; // 16-byte align

    if !check_user_range(new_rsp, frame_size + 16) {
        return false;
    }

    // Trampoline: movq $SYS_SIGRETURN, %rax; syscall; nop*7
    let mut trampoline = [0u8; 16];
    let sysno = abi::syscall::SYS_SIGRETURN;
    trampoline[0] = 0x48; trampoline[1] = 0xC7; trampoline[2] = 0xC0;
    trampoline[3] = (sysno & 0xFF) as u8;
    trampoline[4] = ((sysno >> 8) & 0xFF) as u8;
    trampoline[5] = ((sysno >> 16) & 0xFF) as u8;
    trampoline[6] = ((sysno >> 24) & 0xFF) as u8;
    trampoline[7] = 0x0F; trampoline[8] = 0x05;
    for b in &mut trampoline[9..] { *b = 0x90; }

    let trampoline_addr = new_rsp + core::mem::offset_of!(SigFrame, trampoline);

    // Write return address above SigFrame.
    unsafe { ((new_rsp + frame_size) as *mut usize).write_volatile(trampoline_addr) };

    // Build and write SigFrame.
    let sig_frame = SigFrame {
        magic: SIGFRAME_MAGIC,
        saved_rip: tf.rip as u64,
        saved_rsp: tf.rsp as u64,
        saved_rflags: tf.rflags as u64,
        saved_r15: tf.r15 as u64, saved_r14: tf.r14 as u64,
        saved_r13: tf.r13 as u64, saved_r12: tf.r12 as u64,
        saved_r11: tf.r11 as u64, saved_r10: tf.r10 as u64,
        saved_r9: tf.r9 as u64,   saved_r8: tf.r8 as u64,
        saved_rbp: tf.rbp as u64, saved_rdi: tf.rdi as u64,
        saved_rsi: tf.rsi as u64, saved_rdx: tf.rdx as u64,
        saved_rcx: tf.rcx as u64, saved_rbx: tf.rbx as u64,
        saved_rax: tf.rax as u64,
        saved_mask,
        signum,
        _pad: 0,
        trampoline,
    };
    unsafe { (new_rsp as *mut SigFrame).write_volatile(sig_frame) };

    // Redirect trap frame to handler.
    tf.rip = handler;
    tf.rsp = new_rsp + frame_size; // RSP at the return-address slot
    tf.rdi = signum as usize;      // first handler argument
    tf.rflags &= !(1 << 10);       // clear DF

    true
}

/// Restore register state from a SigFrame during SYS_SIGRETURN.
///
/// Called directly from the x86_64 syscall stub (not through dispatch).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sigreturn(frame_ptr: *mut u8) -> isize {
    let tf = unsafe { &mut *(frame_ptr as *mut UserTrapFrame) };
    let user_rsp = tf.rsp;

    // After `ret` in the handler: RSP advanced by 8 (popped return addr).
    // Then trampoline `syscall` was executed with RSP still at that value.
    // So: SigFrame is at user_rsp - size_of::<SigFrame>() - 8
    let frame_size = core::mem::size_of::<SigFrame>();
    let frame_base = match user_rsp.checked_sub(frame_size + 8) {
        Some(b) => b,
        None => return -(abi::errors::Errno::EFAULT as isize),
    };

    if !check_user_range(frame_base, frame_size) {
        return -(abi::errors::Errno::EFAULT as isize);
    }

    let saved = unsafe { (frame_base as *const SigFrame).read_volatile() };
    if saved.magic != SIGFRAME_MAGIC {
        unsafe { kernel::sched::exit_current(128 + abi::signal::SIGSEGV as i32) };
    }

    tf.rip    = saved.saved_rip as usize;
    tf.rsp    = saved.saved_rsp as usize;
    tf.rflags = saved.saved_rflags as usize;
    tf.r15    = saved.saved_r15 as usize;  tf.r14 = saved.saved_r14 as usize;
    tf.r13    = saved.saved_r13 as usize;  tf.r12 = saved.saved_r12 as usize;
    tf.r11    = saved.saved_r11 as usize;  tf.r10 = saved.saved_r10 as usize;
    tf.r9     = saved.saved_r9 as usize;   tf.r8  = saved.saved_r8 as usize;
    tf.rbp    = saved.saved_rbp as usize;  tf.rdi = saved.saved_rdi as usize;
    tf.rsi    = saved.saved_rsi as usize;  tf.rdx = saved.saved_rdx as usize;
    tf.rcx    = saved.saved_rcx as usize;  tf.rbx = saved.saved_rbx as usize;
    tf.rax    = saved.saved_rax as usize;

    kernel::sched::hooks::set_thread_blocked_current(saved.saved_mask);

    // Clear sigsuspend state if we were inside sigsuspend.
    if let Some(mask) = kernel::sched::hooks::get_sigsuspend_mask_current() {
        kernel::sched::hooks::clear_sigsuspend_current(mask);
    }

    saved.saved_rax as isize
}

/// Register x86_64 signal frame hooks during scheduler init.
pub fn init_signal_hooks() {
    unsafe {
        kernel::signal::deliver::INJECT_FRAME_HOOK = Some(x86_64_inject_signal_frame);
    }
}
