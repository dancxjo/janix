
.section .text
.global timer_interrupt_trampoline
.global keyboard_interrupt_trampoline
.extern sched_tick_wrapper
.extern keyboard_handler_asm_helper
.extern timer_ack_asm_helper

// IDT frame (pushed by CPU):
// [SP+40] SS
// [SP+32] RSP
// [SP+24] RFLAGS
// [SP+16] CS
// [SP+ 8] RIP
// [SP+ 0] Error Code (if any, but IRQ doesn't push one usually? Vectors 32+ don't.)
// So for Vector 32, we assume NO Error code.
// Current Stack Top = RIP.

.align 16
timer_interrupt_trampoline:
    // 0. Normalize Stack Frame (Landmark 6.1)
    test byte ptr [rsp + 8], 3  // Check CS RPL
    jnz .Ltimer_norm_done        // User Mode: 5 words pushed. Done.
    
    // Kernel Mode: 3 words pushed [RFLAGS][CS][RIP]. We want 5.
    sub rsp, 16
    push rax
    mov rax, [rsp + 24] // Original RIP
    mov [rsp + 8], rax  // Target RIP
    mov rax, [rsp + 32] // Original CS
    mov [rsp + 16], rax // Target CS
    mov rax, [rsp + 40] // Original RFLAGS
    mov [rsp + 24], rax // Target RFLAGS
    
    mov qword ptr [rsp + 40], 0x10 // SS (Target +32)
    lea rax, [rsp + 48]            // Original RSP (Target +24)
    mov [rsp + 32], rax
    pop rax
.Ltimer_norm_done:

    // 1. Save GPRs (Matches TrapFrame)
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rbp
    push rdi
    push rsi
    push rdx
    push rcx
    push rbx
    push rax
    
    // Check if we came from User Mode (CS RPL=3)
    test byte ptr [rsp + 128], 3
    jz .Ltimer_skip_swap_entry
    swapgs
.Ltimer_skip_swap_entry:
    
    // 2. Scheduler Tick
    mov rdi, rsp
    cld
    call sched_tick_asm_helper
    
    // rax = 0 or new RSP
    test rax, rax
    jz .Ltimer_no_switch
    mov rsp, rax
.Ltimer_no_switch:
    
    // 3. Ack Timer
    call timer_ack_asm_helper
    
    // 4. Restore Context
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rbp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
    
    // Check if returning to User Mode
    test byte ptr [rsp + 8], 3
    jz .Ltimer_skip_swap_exit
    swapgs
.Ltimer_skip_swap_exit:
    
    // EMERGENCY FIX (Landmark 6.3): Correct CS if it became 0 (corrupted)
    // CS is at [RSP + 8]
    cmp qword ptr [rsp + 8], 0
    jne .Ltimer_iret
    
    // Infer mode from RIP sign bit
    test byte ptr [rsp + 7], 0x80
    jz 31f
    
    // Kernel (High Half) -> CS 8
    mov qword ptr [rsp + 8], 8
    jmp .Ltimer_iret
    
31: // User (Low Half) -> CS 0x2B (UCode64)
    mov qword ptr [rsp + 8], 0x2B
    
.Ltimer_iret:
    iretq

cs_is_zero_panic:
    xchg bx, bx
    cli
    hlt
    jmp cs_is_zero_panic

.align 16
keyboard_interrupt_trampoline:
    // 0. Normalize Stack Frame (Landmark 6.1)
    test byte ptr [rsp + 8], 3
    jnz .Lkeyboard_norm_done
    
    sub rsp, 16
    push rax
    mov rax, [rsp + 24]
    mov [rsp + 8], rax
    mov rax, [rsp + 32]
    mov [rsp + 16], rax
    mov rax, [rsp + 40]
    mov [rsp + 24], rax
    
    mov qword ptr [rsp + 40], 0x10
    lea rax, [rsp + 48]
    mov [rsp + 32], rax
    pop rax
.Lkeyboard_norm_done:

    // 1. Save GPRs
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rbp
    push rdi
    push rsi
    push rdx
    push rcx
    push rbx
    push rax
    
    test byte ptr [rsp + 128], 3
    jz .Lkeyboard_skip_swap_entry
    swapgs
.Lkeyboard_skip_swap_entry:
    
    cld
    call keyboard_handler_asm_helper
    
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rbp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
    
    test byte ptr [rsp + 8], 3
    jz .Lkeyboard_skip_swap_exit
    swapgs
.Lkeyboard_skip_swap_exit:

    // CS correction (Landmark 6.3)
    cmp qword ptr [rsp + 8], 0
    jne .Lkeyboard_iret
    test byte ptr [rsp + 7], 0x80
    jz 32f
    mov qword ptr [rsp + 8], 8
    jmp .Lkeyboard_iret
32:
    mov qword ptr [rsp + 8], 0x2B // UCode64
.Lkeyboard_iret:
    iretq

.global syscall_entry
.extern dispatch
syscall_entry:
    // 1. Swap to Kernel GS
    swapgs
    
    // 2. Switch to Kernel Stack (Safe against preemption)
    mov gs:[40], rax // scratch_rax
    mov rax, rsp     // Save User RSP
    mov rsp, gs:[32] // Load Kernel RSP
    
    // 3. Save User Context on Kernel Stack
    // Layout: [UserRDI][UserRSP][UserRFLAGS/R11][UserRIP/RCX]
    push rax         // User RSP
    push rdi         // User RDI (A0) - save it before we use RDI for ResPtr
    push r11         // User RFLAGS
    push rcx         // User RIP (Entry point)
    
    // Restore RAX for syscall number
    mov rax, gs:[40]
    
    // 4. Enable Interrupts (Now safe because context is on Task stack)
    sti
    
    // 5. Map User Regs to Dispatch Args
    // Nr (RAX), A0 (Stack), A1 (RSI), A2 (RDX), A3 (R10)
    // Targets: RSI(Nr), RDX(A0), RCX(A1), R8(A2), R9(A3)
    
    // Save User A2 to R8
    mov r8, rdx
    // A3 -> R9
    mov r9, r10
    // A1 -> RCX
    mov rcx, rsi
    // Nr -> RSI
    mov rsi, rax
    
    // 6. Setup call frame (128 bytes for Result buffer + alignment)
    // Overkill to ensure no ABI overflow issues clobber User Context.
    sub rsp, 128
    mov rdi, rsp // Arg0: hidden pointer to SyscallResult
    
    // A0 (from stack) -> RDX (Arg2)
    // Offset calculation:
    // [RSP+0..127]: Result buffer
    // [RSP+128]: User RIP
    // [RSP+136]: User RFLAGS
    // [RSP+144]: User RDI
    // [RSP+152]: User RSP
    mov rdx, [rsp + 144] 
    
    cld
    call dispatch

    // 7. Extract results
    mov rax, [rsp + 0]
    mov rdx, [rsp + 8]
    mov rsi, [rsp + 16]
    add rsp, 128 // Remove result buffer
    
    // 8. Restore User Context
    pop rcx // User RIP
    pop r11 // User RFLAGS
    pop rdi // User RDI (Cleanup)
    pop rsp // User RSP
    
    // 9. Return to User
    cli
    swapgs
    sysretq

.global user_mode_trampoline
user_mode_trampoline:
    # 1. Restore GPRs (RAX..R15)
    # Stack must contain [R15..RAX] (RAX at Top)
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rbp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
    
    # Check CS for User Mode
    test byte ptr [rsp + 8], 3
    jz .Luser_trampoline_no_swap
    swapgs
.Luser_trampoline_no_swap:

    # CS Fixup (Landmark 6.3)
    cmp qword ptr [rsp + 8], 0
    jne .Luser_trampoline_iret
    test byte ptr [rsp + 7], 0x80
    jz 11f
    mov qword ptr [rsp + 8], 8
    jmp .Luser_trampoline_iret
11: 
    mov qword ptr [rsp + 8], 0x2B // UCode64
.Luser_trampoline_iret:
    iretq

.global x86_return_from_trap
x86_return_from_trap:
    # Set RSP to point at the TrapFrame
    mov rsp, rdi
    
    # Restore GPRs
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rbp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
    
    # Check CS for user mode
    test byte ptr [rsp + 8], 3
    jz .Ltrap_no_swap
    swapgs
.Ltrap_no_swap:
    
    # CS Fixup
    cmp qword ptr [rsp + 8], 0
    jne .Ltrap_iret
    test byte ptr [rsp + 7], 0x80
    jz 3f
    mov qword ptr [rsp + 8], 8
    jmp .Ltrap_iret
3:  
    mov qword ptr [rsp + 8], 0x2B // UCode64
.Ltrap_iret:
    iretq
