.intel_syntax noprefix

.section .text
.global syscall_entry
.extern sys_dispatch_trampoline

syscall_entry:
    swapgs
    mov gs:[0xC], rsp  # Save User RSP to percpu scratch
    mov rsp, gs:[0x4]  # Load Kernel RSP

    # Construct TrapFrame on stack to match struct layout using IRETQ frame style
    # Top of stack (Highest Addr) -> Bottom (Lowest Addr)
    
    # SS (User Data 0x18 | 3 = 0x1B)
    push 0x1B
    
    # RSP (User RSP)
    push qword ptr gs:[0xC]
    
    # RFLAGS (R11)
    push r11
    
    # CS (User Code 64 0x20 | 3 = 0x23)
    # 0x20 is User Code selector in our GDT (assumed)
    push 0x23
    
    # RIP (RCX)
    push rcx
    
    # Error Code (0)
    push 0
    # Trap Num (0)
    push 0
    
    # GPRs
    push rax
    push rbx
    push rcx # Pushed again as GPR
    push rdx
    push rbp
    push rdi
    push rsi
    push r8
    push r9
    push r10
    push r11 # Pushed again as GPR
    push r12
    push r13
    push r14
    push r15
    
    # Call Trampoline
    mov rdi, rsp # tf argument
    call sys_dispatch_trampoline
    
    # We should not return here usually, return_from_trap handles exit (via IRETQ)
    # But if we did return...
    
    cli
    hlt
