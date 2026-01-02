
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

// =============================================================================
// Interrupt Normalization (Landmark 6.1)
// =============================================================================
// Same-privilege interrupts only push RIP, CS, RFLAGS.
// Different-privilege interrupts push SS, RSP, RFLAGS, CS, RIP.
// To satisfy 'iretq' and 'TrapFrame' struct, we normalize all to 5 words.

.macro NORMALIZE_INTERRUPT name
    test byte ptr [rsp + 8], 3  // Check CS RPL
    jnz .L\name\()_norm_done    // User Mode: Hardware already pushed 5 words.
    
    // Kernel Mode: 3 words pushed. We must shift up and synthesize SS/RSP.
    sub rsp, 16                 // Room for 2 extra quadwords
    push rax                    // Save scratch
    mov rax, [rsp + 24]         // Original RIP
    mov [rsp + 8], rax          // Target RIP
    mov rax, [rsp + 32]         // Original CS
    mov [rsp + 16], rax         // Target CS
    mov rax, [rsp + 40]         // Original RFLAGS
    mov [rsp + 24], rax         // Target RFLAGS
    
    mov qword ptr [rsp + 40], 0x10 // SS (Kernel Data)
    lea rax, [rsp + 48]            // Original RSP
    mov [rsp + 32], rax            // RSP
    pop rax                        // Restore scratch, RSP now at normalized RIP
.L\name\()_norm_done:
.endm

// =============================================================================
// CS Selector Hardening (Landmark 6.3)
// =============================================================================
// Reconstructs CS if corrupted (0). High half RIP -> 8, Low half -> 0x2B.

.macro REPAIR_CS name
    cmp qword ptr [rsp + 8], 0
    jne .L\name\()_iret
    test byte ptr [rsp + 7], 0x80
    jz 11f
    mov qword ptr [rsp + 8], 8      // Kernel Code
    jmp .L\name\()_iret
11: 
    mov qword ptr [rsp + 8], 0x2B   // User Code 64 (index 5 | RPL 3)
.L\name\()_iret:
.endm

// =============================================================================
// Timer Interrupt
// =============================================================================
.align 16
timer_interrupt_trampoline:
    NORMALIZE_INTERRUPT timer
    
    // Save GPRs (Matches TrapFrame)
    push r15; push r14; push r13; push r12; push r11; push r10; push r9; push r8
    push rbp; push rdi; push rsi; push rdx; push rcx; push rbx; push rax
    
    // swapgs if from user
    test byte ptr [rsp + 160], 3 // CS at bottom of 160-byte frame? 
                                 // Wait. 15 regs = 120. 5 words = 40. Total 160.
                                 // [RSP+0..119] GPRs. [RSP+120] RIP. [RSP+128] CS.
    test byte ptr [rsp + 128], 3
    jz .Ltimer_skip_swapgs_entry
    swapgs
.Ltimer_skip_swapgs_entry:
    
    mov rdi, rsp
    cld
    call sched_tick_asm_helper
    
    test rax, rax
    jz .Ltimer_no_switch
    mov rsp, rax
.Ltimer_no_switch:
    
    call timer_ack_asm_helper
    
    // Restore GPRs
    pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp; pop r8
    pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15
    
    test byte ptr [rsp + 8], 3
    jz .Ltimer_skip_swapgs_exit
    swapgs
.Ltimer_skip_swapgs_exit:
    
    REPAIR_CS timer
    iretq

// =============================================================================
// Keyboard Interrupt
// =============================================================================
.align 16
keyboard_interrupt_trampoline:
    NORMALIZE_INTERRUPT kbd
    
    push r15; push r14; push r13; push r12; push r11; push r10; push r9; push r8
    push rbp; push rdi; push rsi; push rdx; push rcx; push rbx; push rax
    
    test byte ptr [rsp + 128], 3
    jz .Lkbd_skip_swapgs_entry
    swapgs
.Lkbd_skip_swapgs_entry:
    
    cld
    call keyboard_handler_asm_helper
    
    pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp; pop r8
    pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15
    
    test byte ptr [rsp + 8], 3
    jz .Lkbd_skip_swapgs_exit
    swapgs
.Lkbd_skip_swapgs_exit:
    
    REPAIR_CS kbd
    iretq

// =============================================================================
// System Call Transition
// =============================================================================
syscall_entry:
    swapgs
    mov gs:[40], rax // scratch
    mov rax, rsp
    mov rsp, gs:[32] // Kernel Stack Top
    
    // Save minimal context for sysretq
    push rax // User RSP [TOP-8]
    push rdi // User RDI [TOP-16]
    push r11 // User RFLAGS [TOP-24]
    push rcx // User RIP [TOP-32]
    
    mov rax, gs:[40]
    sti

    // Shuffle registers to Syscall ABI (Linux-ish)
    // Nr(RAX), A0(RDI), A1(RSI), A2(RDX), A3(R10)
    // Targets for C: RSI(Nr), RDX(A0), RCX(A1), R8(A2), R9(A3)
    mov r9, r10
    mov r8, rdx
    mov rcx, rsi
    mov rsi, rax
    
    // Result Buffer (64 bytes)
    sub rsp, 64
    mov rdi, rsp
    
    // A0 was pushed at [TOP-16]. Offset from current RSP:
    // [RSP+0..63] Buffer
    // [RSP+64] User RIP
    // [RSP+72] User RFLAGS
    // [RSP+80] User RDI <-- A0
    mov rdx, [rsp + 80]
    
    cld
    call dispatch
    
    mov rax, [rsp + 0]
    mov rdx, [rsp + 8]
    mov rsi, [rsp + 16]
    add rsp, 64
    
    pop rcx     // User RIP
    pop r11     // User RFLAGS
    pop rdi     // Cleanup
    pop rsp     // User RSP
    
    cli
    swapgs
    sysretq

// =============================================================================
// Task Entry & Context Switching
// =============================================================================
user_mode_trampoline:
    pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp; pop r8
    pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15
    
    test byte ptr [rsp + 8], 3
    jz .Luser_no_swap
    swapgs
.Luser_no_swap:
    REPAIR_CS user
    iretq

x86_return_from_trap:
    mov rsp, rdi
    pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp; pop r8
    pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15
    
    test byte ptr [rsp + 8], 3
    jz .Ltrap_no_swap
    swapgs
.Ltrap_no_swap:
    REPAIR_CS trap
    iretq
