
.section .text
.global timer_trampoline
.extern timer_handler
.extern check_preemption

// Reference: Old code's User Mode Trampoline logic
// Checks CS [rsp + 8??? In IRETQ frame: RIP, CS, RFLAGS...]
// wait.
// IRETQ Frame:
// [RSP + 40] SS
// [RSP + 32] RSP
// [RSP + 24] RFLAGS
// [RSP + 16] CS
// [RSP + 8]  RIP
// [RSP]      Error Code (if exists) or top if pushed?
// No, invalid.
// Pushed by CPU on interrupt:
// SS
// RSP
// RFLAGS
// CS
// RIP
// Error Code (optional)

// Timer has NO error code.
// So:
// [RSP + 32] SS
// [RSP + 24] RSP
// [RSP + 16] RFLAGS
// [RSP + 8]  CS
// [RSP + 0]  RIP

timer_trampoline:
    // 1. Save all registers (TrapFrame style, match system_abi calling convention)
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
    
    // Stack is now 15 * 8 = 120 bytes deeper.
    // Original RSP is at [RSP + 120].
    // CS is at [RSP + 120 + 8] = [RSP + 128].
    
    // 2. Check source mode (SwapGS if user)
    // CS & 3 == 3 => User
    test byte ptr [rsp + 128], 3
    jz 1f
    swapgs
1:

    // 3. Call Rust handler
    cld // System V ABI
    call timer_handler
    
    // 4. Preemption Check
    // If we are returning to user, we check if we need to reschedule.
    // If CS & 3 == 3 (User)
    test byte ptr [rsp + 128], 3
    jz 2f
    
    // We are returning to user. Check preemption.
    // This is `kernel::task::check_preemption()` exposed via trampoline.
    // BUT `check_preemption` is in kernel crate.
    // We are in bran crate.
    // We need to call via `kernel` symbol?
    // Symbols are flat in the final binary.
    // Let's assume `_ZN6kernel4task16check_preemption17h...`? No, rust name mangling hell.
    // I should expose `check_preemption` as `extern "C"`.
    
    // Temporary Hack: Just call the mangled name? No.
    // I will add `#[no_mangle] pub extern "C" fn check_preemption_shim()` in `interrupt.rs`
    // which calls `kernel::task::check_preemption()`.
    
    call check_preemption_shim

2:
    // 5. Restore SwapGS if needed
    test byte ptr [rsp + 128], 3
    jz 3f
    swapgs
3:

    // 6. Restore registers
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    
    iretq
