use core::arch::{asm, global_asm};
use kernel::syscall::dispatch;
use super::trap::UserTrapFrame;

// MSR Constants
const MSR_EFER: u32 = 0xC0000080;
const MSR_STAR: u32 = 0xC0000081;
const MSR_LSTAR: u32 = 0xC0000082;
const MSR_SFMASK: u32 = 0xC0000084;
const MSR_GS_BASE: u32 = 0xC0000101;
const MSR_KERNEL_GS_BASE: u32 = 0xC0000102;

const EFER_SCE: u64 = 1; // Syscall Enable

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
const KERNEL_CS: u16 = 0x08;
const KERNEL_DS: u16 = 0x10;
// When SYSRET loads CS/SS:
//  CS = (STAR[63:48] + 16) | 3
//  SS = (STAR[63:48] + 8) | 3
// So if STAR[63:48] = 0x08 (Kernel Code), then:
//  CS = 0x18 | 3 = User Code 32? No.
// Let's check AMD manuals.
// SYSRET: 
//   CS_Sel = STAR[63:48] + 16.
//   SS_Sel = STAR[63:48] + 8.
// If we want User CS = 0x23 (0x20|3) and User SS = 0x1B (0x18|3)? 
//  0x1B = 27, 0x23 = 35. 
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

#[repr(C)]
struct CpuLocal {
    scratch_rsp: u64, // offset 0
    kstack_top: u64,  // offset 8
}

static mut CPU_LOCAL: CpuLocal = CpuLocal {
    scratch_rsp: 0,
    kstack_top: 0, // Will be set by scheduler/task switch
};

pub unsafe fn init() {
    // 1. Setup GS Base
    let gs_base = &raw mut CPU_LOCAL as *mut _ as u64;
    wrmsr(MSR_GS_BASE, gs_base);
    // Also Kernel GS Base? No, swapgs swaps them. 
    // We are in kernel now. GS points to kernel struct.
    // When we go to user, we swapgs. GS points to ... user stuff (usually 0).
    // When we execute syscall (entry from user), we swapgs immediately.
    // So MSR_KERNEL_GS_BASE should hold the address of CPU_LOCAL... 
    // WAIT. 
    // Current (Kernel) GS Base = CPU_LOCAL.
    // Target (User) GS Base = 0 (or TCB).
    // syscall instruction DOES NOT swapgs. 
    // We do swapgs explicitly in the handler.
    // So on entry (User GS active), we swapgs -> loads Kernel GS Base (CPU_LOCAL).
    // So MSR_KERNEL_GS_BASE MUST hold CPU_LOCAL.
    // MSR_GS_BASE MUST hold User GS Base.
    // Since we are in kernel now, GS_BASE should be CPU_LOCAL.
    // So we write CPU_LOCAL to GS_BASE.
    // And what about KERNEL_GS_BASE? 
    // If we use swapgs, it exchanges them.
    // If we are in kernel, GS_BASE=CPU_LOCAL. KERNEL_GS_BASE=UserGS.
    // On exit to user: swapgs. GS_BASE=UserGS, KERNEL_GS_BASE=CPU_LOCAL.
    // Correct.
    
    // 2. Enable SCE (SysCall Extension) in EFER
    let efer = rdmsr(MSR_EFER);
    wrmsr(MSR_EFER, efer | EFER_SCE);
    
    // 3. Setup STAR
    // Kernel CS = 0x08
    // User Base = 0x10 (Data=0x18, Code=0x20) - Wait, we need to match what task logic uses.
    // task.rs uses:
    //   push 0x23 // User SS (0x20 | 3) -> Index 4
    //   push 0x1B // User CS (0x18 | 3) -> Index 3
    // This is REVERSED from standard SYSRET requirements.
    // If SS=0x20 and CS=0x18.
    // SYSRET expects CS = Base+16, SS = Base+8.
    // CS(0x18) = Base+16 => Base = 0x08.
    // SS(0x20) = Base+8 => Base = 0x18.
    // CONTRADICTION.
    // Standard GDT usually has: Null, KCode, KData, UData, UCode.
    // 0x00, 0x08, 0x10, 0x18, 0x20.
    // If we used that:
    //   User SS = 0x18 | 3 = 0x1B
    //   User CS = 0x20 | 3 = 0x23
    // But task.rs says:
    //   push 0x23 // User SS 
    //   push 0x1B // User CS
    // So currently task.rs assumes CS=0x18, SS=0x20. (Code before Data? No, 0x18 is 24, 0x20 is 32)
    // 0x18 = 0001 1000 (Index 3)
    // 0x20 = 0010 0000 (Index 4)
    // So Code at 3, Data at 4.
    // SYSRET requires Code at Base+16 (Index X+2), SS at Base+8 (Index X+1).
    // So Code must be AFTER Data.
    // Current setup (implied by task.rs numbers): Code (3) BEFORE Data (4).
    // SYSRET WILL NOT WORK with these selectors if we rely on the offset math.
    // So we must fix GDT or use IRETQ for return.
    // Using IRETQ is slower but safer for now if we don't control GDT.
    // BUT we used the 'syscall' instruction to enter.
    // 'syscall' saves RIP->RCX, RFLAGS->R11.
    // We can use 'sysretq' ONLY if selectors work out.
    // Or we can construct IRET frame and 'iretq'.
    // Given the potential GDT mess, IRETQ is robust.
    // So:
    //   Entry: syscall (fast)
    //   Exit: iretq (slower, but works with any selectors)
    // For v0.5, this is acceptable. "Neat & Real" favors correctness first.
    
    let star = ((0x08 as u64) << 32) | ((0x10 as u64) << 48); // We set 48 anyway just in case
    wrmsr(MSR_STAR, star);
    
    // 4. Setup LSTAR (Entry point)
    wrmsr(MSR_LSTAR, syscall_entry as usize as u64);
    
    // 5. Setup SFMASK (Mask Interrupts 0x200)
    wrmsr(MSR_SFMASK, 0x200);
}

unsafe fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high);
    ((high as u64) << 32) | (low as u64)
}

unsafe fn wrmsr(msr: u32, val: u64) {
    let low = val as u32;
    let high = (val >> 32) as u32;
    asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
}

unsafe extern "C" {
    fn syscall_entry();
}

global_asm!(r#"
.section .text
.att_syntax
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
    pushq $0x23        // SS
    pushq %gs:0         // User RSP (from scratch)
    pushq %r11          // RFLAGS
    pushq $0x1B        // CS (Hardcoded matches task.rs)
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
    
    mov %rax, %rdi // 1st Arg: n
    
    mov %r12, %rsi // 2nd Arg: a0
    mov %r13, %rdx // 3rd Arg: a1
    // RDX (Arg2) needs to go to RCX (4th Arg slot for Rust function)
    mov %rdx, %rcx 
    
    // R10 (Arg3) needs to go to R8 (5th Arg slot)
    mov %r10, %r8
    
    // R8 (Arg4) needs to go to R9 (6th Arg slot)
    // We saved R8 on stack at offset ... let's just trust registers are preserved enough
    // But wait, we just pushed R8. It's on stack.
    // We can read R8 from stack or just move it. R8 is not clobbered yet.
    mov %r8, %r9 
    
    // R9 (Arg5) needs to go to Stack (7th Arg slot, which is 6th arg 'a5')
    // push R9? No, System V ABI puts 7th+ arg on stack.
    // But wait, dispatch(n, a0, a1, a2, a3, a4, a5) has 7 args.
    // RDI, RSI, RDX, RCX, R8, R9. That's 6 regs.
    // So 'a5' (7th arg) goes on stack.
    pushq %r9
    
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
    
    // Cleanup stack arg
    add $8, %rsp
    
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
    
    // Skip error_code, int_no
    add $16, %rsp
    
    // SWAPGS back to User GS
    swapgs
    
    // IRETQ
    iretq
"#);
