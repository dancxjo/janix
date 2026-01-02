
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
    mov rdi, rsp
    cld
    call sched_tick_asm_helper
    test rax, rax
    jz 2f
    mov rsp, rax
2:
    call timer_ack_asm_helper

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
    cld
    call keyboard_handler_asm_helper

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
    mov gs:[40], rax
    mov rax, rsp
    mov rsp, gs:[32]
    
    push 0x23
    push rax
    push r11
    push 0x2B
    push rcx
    push rdi
    
    mov rax, gs:[40]
    sti

    mov r9, r10; mov r8, rdx; mov rcx, rsi; mov rsi, rax; mov rdx, [rsp]
    sub rsp, 64
    mov rdi, rsp
    cld
    call dispatch
    
    mov rax, [rsp + 0]; mov rdx, [rsp + 8]; mov rsi, [rsp + 16]
    add rsp, 64
    
    pop rdi; pop rcx; add rsp, 8; pop r11; pop rax; add rsp, 8
    mov rsp, rax
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
