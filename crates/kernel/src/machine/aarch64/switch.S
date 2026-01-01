.global aarch64_switch_to
.type aarch64_switch_to, %function
aarch64_switch_to:
    // x0 = &old_sp, x1 = &new_sp

    // Save callee-saved regs on current stack
    // AAPCS64: x19-x29 are callee-saved. x30 is LR. 
    // We save pairs to keep stack aligned. 6 pairs = 12 regs = 96 bytes.
    stp x19, x20, [sp, #-16]!
    stp x21, x22, [sp, #-16]!
    stp x23, x24, [sp, #-16]!
    stp x25, x26, [sp, #-16]!
    stp x27, x28, [sp, #-16]!
    stp x29, x30, [sp, #-16]!

    // Save old SP
    mov x2, sp
    str x2, [x0]

    // Load new SP
    ldr x2, [x1]
    mov sp, x2

    // Restore callee-saved regs from new stack
    ldp x29, x30, [sp], #16
    ldp x27, x28, [sp], #16
    ldp x25, x26, [sp], #16
    ldp x23, x24, [sp], #16
    ldp x21, x22, [sp], #16
    ldp x19, x20, [sp], #16

    ret
.size aarch64_switch_to, .-aarch64_switch_to

.global aarch64_task_entry_stub
.type aarch64_task_entry_stub, %function
aarch64_task_entry_stub:
    // Stack layout at entry (top to bottom):
    // [sp+0]  dispatch_ptr (x0)
    // [sp+8]  entry_point  (x1)
    
    // Load args
    ldr x0, [sp], #8     // x0 = dispatch_ptr
    ldr x1, [sp], #8     // x1 = entry_point

    // Call into Rust dispatch trampoline
    bl task_dispatch
    
    // Should never return
1:  wfi
    b 1b
.size aarch64_task_entry_stub, .-aarch64_task_entry_stub
