
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
    
    // Now SP points to start of TrapFrame (rax).
    
    // 2. Call sched::tick
    // fn tick(current_sp: u64) -> Option<u64> (returns new SP or 0/None)
    // Argument 1 (rdi) = current SP
    mov rdi, rsp
    
    // We need to adhere to SysV ABI (align stack?)
    // Call might clobber volatile regs, but we saved them.
    // Direction Flag? cld.
    cld
    
    call sched_tick_asm_helper
    
    // Result in rax. If 0, no switch. If != 0, new SP.
    test rax, rax
    jz 1f
    
    // Switch stack!
    mov rsp, rax
    
1:
    // 3. Ack Interrupt? 
    // Ideally ack before switch or after? 
    // If we switch, the old task context is saved on old stack.
    // The new task context is on new stack.
    // We restore from RSP.
    // We need to ACK the timer before IRET?
    // If we switched, we are now "returning" into the new task.
    // Who acks?
    // Either `tick` called ack, or we do it.
    // Let's assume `tick` handles logic or we assume EOI irrelevant for switch mechanics (Edge triggered?).
    // LAPIC Timer needs EOI.
    // We should call `ack_timer` logic here?
    // Safe to call Rust `timer::ack()`? Yes.
    
    call timer_ack_asm_helper
    
    // 4. Restore
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
    
    iretq
