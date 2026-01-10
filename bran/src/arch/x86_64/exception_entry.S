.section .text
.extern exception_handler

// Macro for exceptions with error code
.macro exception_err vector
.global exception_\vector
exception_\vector:
    // Stack: [SS, RSP, RFLAGS, CS, RIP, ERR, ...]
    // Push vector
    push \vector
    jmp exception_common
.endm

// Macro for exceptions without error code
.macro exception_no_err vector
.global exception_\vector
exception_\vector:
    // Stack: [SS, RSP, RFLAGS, CS, RIP, ...]
    push 0 // Dummy error code
    push \vector
    jmp exception_common
.endm

exception_common:
    // Stack: [SS, RSP, RFLAGS, CS, RIP, ERR, VECTOR]
    
    // Save registers? For panic, maybe not strictly needed, but let's be safe + consistent
    // We can just follow C ABI.
    // Argument 1: Stack Frame Pointer (RSP + 16) -> Points to RIP
    // Argument 2: Vector (RSP + 0)
    // Argument 3: Error Code (RSP + 8)
    
    // RSP currently points to VECTOR.
    // RSP+8 is ERR.
    // RSP+16 is RIP (start of stack frame).
    
    mov rdi, rsp
    add rdi, 16 // Arg1: &ExceptionStackFrame
    
    mov rsi, [rsp] // Arg2: Vector
    mov rdx, [rsp + 8] // Arg3: Error Code
    
    // Align stack?
    // We pushed 2 u64s (Err, Vector). Stack is 16-byte aligned if interrupt frame was aligned.
    // Interrupt frame pushes 5 u64s (40 bytes).
    // So if RSP was 16-aligned before interrupt:
    // -40 -> Not aligned (ending in 8).
    // Push Err (if present) -> -48. Aligned.
    // If we pushed dummy err -> -48. Aligned.
    // Push Vector -> -56. Not aligned.
    // Call -> -64. Aligned.
    
    // So we should adhere to alignment.
    // But exception_handler panics anyway.
    
    cld
    call exception_handler
    
    // Verify stack state... we enable interrupts for hcf?
    cli
4:
    hlt
    jmp 4b

// Instantiate commonly used exceptions
exception_no_err 0  // DIV ZERO
exception_no_err 3  // BP
exception_no_err 6  // UD
exception_err    8  // DF
exception_err    10 // TS
exception_err    11 // NP
exception_err    12 // SS
exception_err    13 // GPF
exception_err    14 // PF
