
.section .text
.global timer_interrupt_trampoline
.global keyboard_interrupt_trampoline
.global syscall_entry
.global user_mode_trampoline
.global x86_return_from_trap

.extern sched_tick_asm_helper
.extern keyboard_handler_asm_helper
.extern timer_ack_asm_helper
.extern dispatch

// Landmark 6.1: Symmetric Normalization
// 3-word HW frame -> 5-word TrapFrame
.macro NORMALIZE name
    test byte ptr [rsp + 8], 3
    jnz .L\name\()_norm_done
    
    sub rsp, 16
    push rax
    mov rax, [rsp + 24]; mov [rsp + 8], rax  // RIP
    mov rax, [rsp + 32]; mov [rsp + 16], rax // CS
    mov rax, [rsp + 40]; mov [rsp + 24], rax // FL
    lea rax, [rsp + 48]; mov [rsp + 32], rax // RSP (X)
    mov qword ptr [rsp + 40], 0x10           // SS
    pop rax
.L\name\()_norm_done:
.endm

// 5-word TrapFrame -> 3-word HW frame (Kernel Only)
.macro DENORMALIZE
    test byte ptr [rsp + 8], 3
    jnz 100f
    
    push rax
    // Shift: 3 words up by 16 bytes
    // Frame: [RSP+8] RIP, [16] CS, [24] FL, [32] RSP, [40] SS
    // Move FL(24) -> SS(40)
    mov rax, [rsp + 24]; mov [rsp + 40], rax
    // Move CS(16) -> RSP(32)
    mov rax, [rsp + 16]; mov [rsp + 32], rax
    // Move RIP(8)  -> FL(24)
    mov rax, [rsp + 8];  mov [rsp + 24], rax
    
    pop rax
    add rsp, 16 // Now RSP points to RIP at [orig_rsp+8]? No.
    // Wait. My shift logic above:
    // RIP is at [new_rsp + 24].
    // If new_rsp = old_rsp - 16.
    // Then RIP is at old_rsp + 8.
    // CS is at old_rsp + 16.
    // FL is at old_rsp + 24.
    // YES! This is exactly correct.
100:
.endm

.align 16
timer_interrupt_trampoline:
    NORMALIZE timer
    push r15; push r14; push r13; push r12; push r11; push r10; push r9; push r8
    push rbp; push rdi; push rsi; push rdx; push rcx; push rbx; push rax
    
    test byte ptr [rsp + 128], 3
    jz 1f
    swapgs
1:
    // Increment irq_depth (gs[64])
    inc dword ptr gs:[64]

    mov rdi, rsp
    cld
    call sched_tick_asm_helper
    test rax, rax
    jz 2f
    mov rsp, rax
2:
    call timer_ack_asm_helper

    // Decrement irq_depth
    dec dword ptr gs:[64]

    test byte ptr [rsp + 128], 3
    jz 3f
    swapgs
3:
    pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp; pop r8
    pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15
    DENORMALIZE
    iretq

.align 16
keyboard_interrupt_trampoline:
    NORMALIZE kbd
    push r15; push r14; push r13; push r12; push r11; push r10; push r9; push r8
    push rbp; push rdi; push rsi; push rdx; push rcx; push rbx; push rax
    
    test byte ptr [rsp + 128], 3
    jz 1f
    swapgs
1:
    // Increment irq_depth
    inc dword ptr gs:[64]

    cld
    call keyboard_handler_asm_helper

    // Decrement irq_depth
    dec dword ptr gs:[64]

    test byte ptr [rsp + 128], 3
    jz 3f
    swapgs
3:
    pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp; pop r8
    pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15
    DENORMALIZE
    iretq

syscall_entry:
    swapgs
    mov gs:[104], rax // scratch_rax (nr)
    mov rax, rsp
    mov rsp, gs:[72] // exception_stack_ptr (kernel_rsp)
    
    push 0x23        // SS
    push rax         // RSP
    push r11         // RFLAGS
    push 0x2B        // CS
    push rcx         // RIP
    push rdi         // a0
    
    // 1. Allot result area (32 bytes) and stack arguments (16 bytes)
    sub rsp, 48
    
    // Position a4, a5 on stack for dispatch
    mov [rsp + 0], r8 // a4
    mov [rsp + 8], r9 // a5
    
    // 2. Prepare register arguments for dispatch
    // User registers: rdi=a0, rsi=a1, rdx=a2, r10=a3, r8=a4, r9=a5
    // a0 was pushed at offset 131, so it's at [rsp + 48]
    mov rcx, rsi      // a1 -> rcx (4th arg)
    mov r8, rdx       // a2 -> r8 (5th arg)
    mov rdx, [rsp + 48] // a0 -> rdx (3rd arg)
    mov rsi, gs:[104] // nr -> rsi (2nd arg)
    mov r9, r10       // a3 -> r9 (6th arg)
    
    // Result pointer (1st arg)
    lea rdi, [rsp + 16] // Put result at [rsp+16..47]
    
    sti
    cld
    call dispatch
    
    // 3. Unpack result into reconciled ABI return registers
    mov rax, [rsp + 16 + 0]  // result.status
    mov rdx, [rsp + 16 + 8]  // result.val0
    mov r8,  [rsp + 16 + 16] // result.val1
    
    add rsp, 48 // Clean up dispatch stack frame
    
    // 4. Restore user context
    pop rdi  // a0
    pop rcx  // RIP
    add rsp, 8 // CS
    pop r11  // RFLAGS
    pop r10  // User RSP (temporary)
    add rsp, 8 // SS
    
    mov rsp, r10 // Switch back to user stack
    cli
    swapgs
    sysretq

user_mode_trampoline:
    pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp; pop r8
    pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15
    DENORMALIZE
    test byte ptr [rsp + 8], 3
    jz .Luser_no_swap_t
    swapgs
.Luser_no_swap_t:
    iretq

x86_return_from_trap:
    mov rsp, rdi
    pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp; pop r8
    pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15
    DENORMALIZE
    test byte ptr [rsp + 8], 3
    jz .Ltrap_no_swap_t
    swapgs
.Ltrap_no_swap_t:
    iretq
