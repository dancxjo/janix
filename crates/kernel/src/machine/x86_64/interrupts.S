
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

// Precise SAVE_CONTEXT macro
.macro SAVE_ALL
    push r15; push r14; push r13; push r12; push r11; push r10; push r9; push r8
    push rbp; push rdi; push rsi; push rdx; push rcx; push rbx; push rax
    
    // Check entry mode (CS at +128)
    test byte ptr [rsp + 128], 3
    jz 10f
    swapgs
10:
.endm

// Precise RESTORE_CONTEXT macro
.macro RESTORE_ALL
    // Check return mode (CS at +128)
    test byte ptr [rsp + 128], 3
    jz 20f
    swapgs
20:
    pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp; pop r8
    pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15
.endm

.macro REPAIR_SEGMENTS
    cmp qword ptr [rsp + 8], 0
    jne 100f
    test byte ptr [rsp + 7], 0x80
    jz 11f
    mov qword ptr [rsp + 8], 8
    jmp 100f
11: 
    mov qword ptr [rsp + 8], 0x2B
100:
.endm

.align 16
timer_interrupt_trampoline:
    // 0. Manual Normalization (Mandatory 5-word fix)
    test byte ptr [rsp + 8], 3
    jnz .Ltimer_norm_done
    
    // Ring 0 -> Ring 0: Shift 3 words [RIP, CS, FL] down by 16 bytes.
    // Synthesize [RSP, SS].
    sub rsp, 16
    push rax
    mov rax, [rsp + 24]; mov [rsp + 8], rax  // RIP
    mov rax, [rsp + 32]; mov [rsp + 16], rax // CS
    mov rax, [rsp + 40]; mov [rsp + 24], rax // FLAGS
    lea rax, [rsp + 48]; mov [rsp + 32], rax // RSP
    mov qword ptr [rsp + 40], 0x10           // SS
    pop rax
.Ltimer_norm_done:

    SAVE_ALL
    mov rdi, rsp
    cld
    call sched_tick_asm_helper
    test rax, rax
    jz .Ltimer_no_switch
    mov rsp, rax
.Ltimer_no_switch:
    call timer_ack_asm_helper
    RESTORE_ALL
    REPAIR_SEGMENTS
    iretq

.align 16
keyboard_interrupt_trampoline:
    test byte ptr [rsp + 8], 3
    jnz .Lkbd_norm_done
    sub rsp, 16
    push rax
    mov rax, [rsp + 24]; mov [rsp + 8], rax
    mov rax, [rsp + 32]; mov [rsp + 16], rax
    mov rax, [rsp + 40]; mov [rsp + 24], rax
    lea rax, [rsp + 48]; mov [rsp + 32], rax
    mov qword ptr [rsp + 40], 0x10
    pop rax
.Lkbd_norm_done:

    SAVE_ALL
    cld
    call keyboard_handler_asm_helper
    RESTORE_ALL
    REPAIR_SEGMENTS
    iretq

syscall_entry:
    swapgs
    mov gs:[40], rax
    mov rax, rsp
    mov rsp, gs:[32] 
    
    // Create 5-word frame for Preemption consistency
    push 0x23        // SS
    push rax         // RSP
    push r11         // RFLAGS
    push 0x2B        // CS
    push rcx         // RIP
    
    // Save A0
    push rdi
    
    mov rax, gs:[40]
    // sti

    mov r9, r10; mov r8, rdx; mov rcx, rsi; mov rsi, rax; mov rdx, [rsp]
    sub rsp, 64
    mov rdi, rsp
    cld
    call dispatch
    
    mov rax, [rsp + 0]
    mov rdx, [rsp + 8]
    mov rsi, [rsp + 16]
    add rsp, 64
    
    pop rdi          // Cleanup A0
    pop rcx          // RIP
    add rsp, 8       // Skip CS
    pop r11          // RFLAGS
    pop rax          // RSP
    add rsp, 8       // Skip SS
    
    mov rsp, rax     // Restore User RSP
    cli
    swapgs
    sysretq

user_mode_trampoline:
    RESTORE_ALL
    REPAIR_SEGMENTS
    iretq

x86_return_from_trap:
    mov rsp, rdi
    RESTORE_ALL
    REPAIR_SEGMENTS
    iretq
