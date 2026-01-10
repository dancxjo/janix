.section .text
.global yield_handler
.extern kernel_trap_handler

// Yield Handler (Int 0x81)
// Stack on entry (Kernel -> Kernel yield):
// [SS], [RSP], RFLAGS, CS, RIP  (SS/RSP optional if no privilege change, but int instruction always pushes them? No.
// INT from Ring 0 to Ring 0: Pushes RFLAGS, CS, RIP. Does NOT switch stack.
// INT from Ring 3 to Ring 0: Pushes SS, RSP, RFLAGS, CS, RIP. Switches stack.

// Yield can be called from Ring 0 (kernel threads) or Ring 3 (user threads implicit yield?).
// yield_trap() uses `int 0x81`.
// If called from Kernel:
// Stack: [RFLAGS, CS, RIP]
// If called from User:
// Stack: [SS, RSP, RFLAGS, CS, RIP]

// If we want a uniform TrapFrame, we need to handle the missing SS/RSP for kernel-kernel interrupts?
// BUT `TrapFrame` struct has SS/RSP fields.
// If the CPU didn't push them, our struct layout will be misaligned with the stack if we assume they are there.

// This is the classic "Kernel Interrupt Stack Frame" issue.
// If we interrupt kernel, we don't get SS/RSP pushed.
// But `TrapFrame` expects them.

// Option 1: Fake them.
// If CS & 3 == 0 (Kernel), we manually push current RSP and SS=0x10.
// Option 2: Use an interrupt gate that always switches stack (IST)? NO, IST switches stack but frame layout depends on CPL change?
// Actually, x86_64 IST mechanism:
// "If the handler is running at a CPL of 0... the processor pushes the old SS and RSP... ONLY if a privilege level change occurred."
// Actually, older docs say so. But 64-bit mode ALWAYS pushes SS/RSP.
// Let's verify: Intel SDM Vol 3A 6-13 "Exception and Interrupt Handling in 64-bit Mode":
// "In 64-bit mode... the processor saves the SS, RSP, RFLAGS, CS, and RIP... onto the stack. If a stack switch occurs... the processor saves the old calling stack... If no stack switch occurs, the processor saves the SS and RSP... onto the current stack."
// WAIT. "If no stack switch occurs... pushes... onto the current stack."
// Does it push SS/RSP even for Ring 0 -> Ring 0?
// "In 64-bit mode, the size of the interrupt stack frame is fixed at 40 bytes... (SS, RSP, RFLAGS, CS, RIP)."
// YES. 64-bit mode ALWAYS pushes SS and RSP unconditionally.
// Unlike 32-bit mode.
// EXCELLENT.
// So the layout is consistent. [SS, RSP, RFLAGS, CS, RIP].

yield_handler:
    // 1. Push Error Code (dummy) and Trap Num
    push 0      // Error Code
    push 0x81   // Trap Num
    
    // 2. Push GPRs
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
    
    // Stack is now &TrapFrame (RSP points to R15)
    
    // 3. Check for User Mode entry to do SwapGS
    // [RSP + 120 + 24] = CS. (R15..RAX=120, TrapNum=8, Err=8 -> 136 total offset to RIP? No)
    // R15..RAX (15*8 = 120 bytes). Offset 0 to 119.
    // TrapNum (8). Offset 120.
    // ErrCode (8). Offset 128.
    // RIP (8). Offset 136.
    // CS (8). Offset 144.
    
    test byte ptr [rsp + 144], 3
    jz 1f
    swapgs
1:
    
    // 4. Call kernel trap handler
    // Arg 1 (RDI) = &TrapFrame (Current RSP)
    mov rdi, rsp
    cld
    call kernel_trap_handler
    
    // 5. Restore SwapGS if returning to user
    test byte ptr [rsp + 144], 3
    jz 2f
    swapgs
2:

    // 6. Pop GPRs
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
    
    // 7. Skip TrapNum and ErrorCode
    add rsp, 16
    
    // 8. Return
    iretq
