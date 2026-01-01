
.section .text
.global timer_interrupt_trampoline
.extern sched_tick_wrapper

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
    // 1. Save GPRs
    // Layout must match `TrapFrame` struct in Rust.
    // We push in reverse order of struct fields?
    // Let's define uniform order: 
    // rax, rbx, rcx, rdx, rsi, rdi, rbp, r8, r9, r10, r11, r12, r13, r14, r15.
    
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
    
    # Check CS (RSP+128) for User Mode (3)
    test byte ptr [rsp + 128], 3
    jz 1f
    swapgs
1:
    
    # 2. Call sched::tick
    # fn tick(current_sp: u64) -> Option<u64> (returns new SP or 0/None)
    # Argument 1 (rdi) = current SP
    mov rdi, rsp
    
    # We need to adhere to SysV ABI (align stack?)
    # Call might clobber volatile regs, but we saved them.
    # Direction Flag? cld.
    cld
    
    call sched_tick_asm_helper
    
    # Result in rax. If 0, no switch. If != 0, new SP.
    test rax, rax
    jz 2f
    
    # Switch stack!
    mov rsp, rax
    
2:
    # 3. Ack Interrupt? 
    # Ideally ack before switch or after? 
    # If we switch, the old task context is saved on old stack.
    # The new task context is on new stack.
    # We restore from RSP.
    # We need to ACK the timer before IRET?
    # If we switched, we are now "returning" into the new task.
    # Who acks?
    # Either `tick` called ack, or we do it.
    # Let's assume `tick` handles logic or we assume EOI irrelevant for switch mechanics (Edge triggered?).
    # LAPIC Timer needs EOI.
    # We should call `ack_timer` logic here?
    # Safe to call Rust `timer::ack()`? Yes.
    
    call timer_ack_asm_helper
    
    # 4. Restore
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
    
    # Check CS (RSP+8) for User Mode
    test byte ptr [rsp + 8], 3
    jz 3f
    swapgs
3:
    iretq

.global syscall_entry
.extern dispatch
syscall_entry:
    swapgs # Switch to kernel GS (for per-cpu)
    mov gs:[24], rsp # Save User SP to PerCpu.syscall_rsp
    mov rsp, gs:[32] # Load Kernel SP from PerCpu.kernel_rsp
    
    sti # Enable interrupts (optional, but standard for preemptible kernel)
    
    # Save volatile regs needed for sysret (RCX=RIP, R11=RFLAGS)
    push rcx 
    push r11 
    
    # Move 4th arg (R10) to ABI slot (RCX)
    mov rcx, r10 
    
    sub rsp, 32
    mov rdi, rsp # RetPtr
    
    mov rsi, rax # NR -> RSI
    
    push r9 # Push A5
    push r8 # Push A4
    
    mov r9, r10
    mov r8, rdx
    mov rcx, rsi
    mov rdx, rdi
    
    call dispatch
    
    # Cleanup stack args (16 bytes)
    add rsp, 16 
    
    pop rax # Error
    pop rdx # Val1
    pop rsi # Val2
    
    # Restore stack (alignment padding 8 bytes)
    add rsp, 8
    
    pop r11 # RFLAGS
    pop rcx # RIP
    
    cli 
    
    # Restore User SP
    mov rsp, gs:[24]
    swapgs
    
    sysretq
