
.global syscall_entry
.global enter_user_sysret_asm
.extern syscall_dispatch

// PerCpu Offsets (Must match bran/src/arch/x86_64/percpu.rs)
// kernel_rsp0: 8
// user_rsp: 16

syscall_entry:
    swapgs
    mov gs:[16], rsp        // Save user RSP
    mov rsp, gs:[8]         // Load kernel RSP

    // Build TrapFrame
    // Top of stack is high address. TrapFrame struct ends with user_rflags.
    // Push order: user_rflags, user_rsp, user_rip, rax, rbx...

    push r11                // user_rflags (from syscall)
    push qword ptr gs:[16]  // user_rsp
    push rcx                // user_rip (from syscall)

    // General registers
    push rax
    push rbx
    push rcx
    push rdx
    push rbp
    push rdi
    push rsi
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    // Call Dispatcher
    mov rdi, rsp            // &mut TrapFrame
    call syscall_dispatch

    // Fallthrough to restore

.global restore_and_sysret
restore_and_sysret:
    // Restore general registers
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rsi
    pop rdi
    pop rbp
    pop rdx
    pop rcx
    pop rbx
    pop rax

    // Restore user state for SYSRETQ
    // Stack: [user_rip, user_rsp, user_rflags] (top is low addr now?)
    // No, we popped down to user_rip.
    
    // Structure:
    // user_rip
    // user_rsp
    // user_rflags (highest)
    
    // We pushed in order: rflags, rsp, rip.
    // Stack grew down.
    // So [rsp] = rip.
    
    pop rcx     // RCX = user_rip (needed for SYSRETQ)
    
    // Now [rsp] = user_rsp.
    // We need to switch stacks to user_rsp.
    // But we also need R11 = user_rflags.
    
    // Load user_rsp into temporary reg. RDX is free, we already restored it but we can reload it?
    // No, we just restored RDX! We can't clobber it.
    // SYSRETQ clobbers RCX (RIP) and R11 (RFLAGS).
    // So we can use RCX and R11 freely as inputs.
    // But we need other regs intact.
    
    // We need to load user_rsp from stack.
    // We CANNOT use a general register to hold user_rsp if we need that register to be restored.
    // SYSRETQ does not touch RSP. We must set RSP.
    
    // WAIT. If we change RSP, we lose access to the stack where other regs might be?
    // No, everything is restored except RCX and R11.
    // So we just need:
    // 1. Pop RCX (user_rip).
    // 2. Pop user_rsp into a scratch reg?
    // 3. Pop R11 (user_rflags).
    // 4. MOV RSP, scratch_reg.
    
    // Is there a scratch reg? 
    // SYSCALL clobbers RCX and R11. 
    // Does it preserve others? Yes.
    // So we can't clobber anything else.
    // BUT user_rsp is destined for RSP.
    // Does SYSRET restore RSP? NO.
    
    // Strategy:
    // We need to restore user's stack.
    // We can use the user's stack to holding temporary data? unsafe.
    
    // Wait, `swapgs` happens at the end.
    // We can use GS scratch?
    // We have `gs:[16]` (user_rsp) used for saving.
    // Optimally, user_rsp is already in `gs:[16]` from entry?
    // But `enter_user_sysret` might change it (e.g. exec).
    // So we must trust the TrapFrame's user_rsp.
    
    // Solution:
    // Pop user_rsp into `gs:[16]`?
    // pop qword ptr gs:[16] ?
    // No, that uses GS base. Is GS kernel base? Yes, we haven't swapped yet.
    // Good!
    
    pop qword ptr gs:[16]   // Update PerCpu.user_rsp with new value from TrapFrame
    pop r11                 // R11 = user_rflags
    
    mov rsp, gs:[16]        // Switch to user stack
    
    swapgs
    sysretq

// enter_user_sysret_asm(tf: *const TrapFrame) -> !
// Custom entry point that switches stack to TF and jumps to restore code
enter_user_sysret_asm:
    mov rsp, rdi // Switch stack to point to TrapFrame
    jmp restore_and_sysret
