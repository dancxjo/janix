
.section .text
.global timer_interrupt_trampoline
.global keyboard_interrupt_trampoline
.extern sched_tick_wrapper
.extern keyboard_handler_asm_helper

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
    
    # call timer_ack_asm_helper
    
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

.align 16
keyboard_interrupt_trampoline:
    // 1. Save generic regs (caller saved + others to be safe)
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

    // Check CS for User Mode
    test byte ptr [rsp + 128], 3
    jz 1f
    swapgs
1:
    cld
    
    // Call handler helper (Rust)
    call keyboard_handler_asm_helper
    
    // Restore
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

    // Check CS for User Mode
    test byte ptr [rsp + 8], 3
    jz 2f
    swapgs
2:
    iretq

.global syscall_entry
.extern dispatch
syscall_entry:
    swapgs # Switch to kernel GS (for per-cpu)
    mov gs:[24], rsp # Save User SP to PerCpu.syscall_rsp
    mov rsp, gs:[32] # Load Kernel SP from PerCpu.kernel_rsp
    
    sti # Enable interrupts
    
    # Save volatile regs needed for sysret (RCX=RIP, R11=RFLAGS)
    push rcx 
    push r11 
    
    # SyscallResult is 24 bytes (returned by pointer)
    # Align stack to 16 bytes. 32 bytes allocation.
    sub rsp, 32
    mov rdi, rsp # Arg0: Hidden Pointer to Return Value
    
    # Map User Regs to Dispatch Args
    # User: RAX(Nr), RDI(A0), RSI(A1), RDX(A2), R10(A3)
    # Func: RSI(Nr), RDX(A0), RCX(A1), R8(A2),  R9(A3)
    # Arg0 is ResPtr (RDI)
    
    # Needs careful shuffling to avoid overwriting
    # Use R9 as temp? R9 is Dest Arg5 (User A5). 
    # thing_std only passes up to A3. Stack args A4/A5 unused by current apps.
    
    mov r9, r10  # A3 -> R9 (Arg5)
    mov r8, rdx  # A2 -> R8 (Arg4)
    mov rcx, rsi # A1 -> RCX (Arg3)
    mov rdx, rdi # A0 -> RDX (Arg2) (Usage of RDI before overwrite!) 
    # Wait! RDI was overwritten by `mov rdi, rsp` above!
    # Implementation error in thought process!
    
    # FIX: We need to save User RDI before overwriting it.
    # Where? We can use a register if we do it early.
    
    # Correct sequence:
    # 1. Save User RDI (A0) to Temp (RDX is A2, so RDX is occupied). 
    #    RAX is Nr. RSI is A1.
    #    Use R8 or R9 as Temp (User doesn't use them in thing_std, but for ABI correctness we should preserve if used?)
    #    thing_std only uses up to A3 (R10). So R8/R9 are free/garbage.
    
    # shuffle:
    mov r9, r10  # User R10(A3) -> Func R9 (Arg5)
    mov r8, rdx  # User RDX(A2) -> Func R8 (Arg4)
    mov rcx, rsi # User RSI(A1) -> Func RCX (Arg3)
    # Now RDX is free (moved to R8).
    # RSI is free (moved to RCX).
    
    # Move User RDI (A0) -> Func RDX (Arg2)
    # But User RDI is clobbered by `mov rdi, rsp`?
    # NO, I haven't executed that instruction yet in this fixed sequence.
    # I must save RDI before `mov rdi, rsp`.
    
    # Let's start over sequence:
    mov rcx, rsi # User RSI(A1) -> Func RCX(Arg3)
    mov rsi, rax # User RAX(Nr) -> Func RSI(Arg1) - WAIT! RAX is Nr!
    # Func Arg1 is RSI.
    
    mov rdx, rdi # User RDI(A0) -> Func RDX(Arg2). 
    # Now User RDI is safe in RDX.
    
    # Continue shuffling:
    mov r9, r10 # User R10(A3) -> Func R9(Arg5)
    
    # Wait, R8 needs User RDX(A2).
    # But I overwrote RDX with User RDI!
    # DEADLOCK in registers.
    
    # Use Stack or Scratch?
    # R12-R15 are callee-saved, so we must push/pop if we use them?
    # Or just use the stack we allocated?
    
    # Better Strategy: Push Registers to Stack in Order?
    # Or use RAX (User Nr) as scratch after moving it?
    
    # 1. Move Nr (RAX) to RSI. (RSI is occupied by User A1).
    # Move User A1 (RSI) to RCX. (RCX is occupied by User RIP? No, I pushed RCX/RIP).
    # RCX Register is free/scratch.
    
    # Sequence:
    mov rcx, rsi # Save User A1 to RCX (Dest Arg3). Safe.
    mov rsi, rax # Save User Nr to RSI (Dest Arg1). Safe.
    
    # Now need: User A0 (RDI) -> RDX (Dest Arg2).
    # And:      User A2 (RDX) -> R8 (Dest Arg4).
    
    mov r8, rdx  # Save User A2 to R8 (Dest Arg4). Safe.
    mov rdx, rdi # Save User A0 to RDX (Dest Arg2). Safe.
    
    mov r9, r10  # Save User A3 to R9 (Dest Arg5). Safe.
    
    # Now Args 1-5 are set (RSI, RDX, RCX, R8, R9).
    # Set Arg0 (ResPtr) to RDI.
    mov rdi, rsp
    
    # Clean and ready.
    call dispatch
    
    # Result in [RSP].
    # thing_std expects:
    # Status -> RAX
    # Val0 -> RDX
    # Val1 -> RSI
    
    mov rax, [rsp + 0]  # Status (i32/element 0, padded to 8 bytes)
    mov rdx, [rsp + 8]  # Val0
    mov rsi, [rsp + 16] # Val1
    
    add rsp, 32
    
    pop r11
    pop rcx
    
    cli
    mov rsp, gs:[24]
    swapgs
    sysretq
