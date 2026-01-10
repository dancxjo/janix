use core::arch::global_asm;

unsafe extern "C" {
    pub fn context_switch(old_handle_ptr: *mut u64, new_handle: u64);
}

// Trampoline for new threads
unsafe extern "C" {
    fn trampoline();
}

global_asm!(r#"
.section .text
.global context_switch
context_switch:
    // x0 = old_handle_ptr (*mut u64)
    // x1 = new_handle (u64) - This is the new SP

    // Save callee-saved registers (x19-x29, x30/LR)
    // We push pairs to keep stack aligned to 16 bytes
    sub sp, sp, #96
    stp x19, x20, [sp, #0]
    stp x21, x22, [sp, #16]
    stp x23, x24, [sp, #32]
    stp x25, x26, [sp, #48]
    stp x27, x28, [sp, #64]
    stp x29, x30, [sp, #80]

    // Save old SP to *old_handle_ptr
    mov x9, sp
    str x9, [x0]

    // Load new SP from new_handle
    mov sp, x1

    // Restore callee-saved registers
    ldp x29, x30, [sp, #80]
    ldp x27, x28, [sp, #64]
    ldp x25, x26, [sp, #48]
    ldp x23, x24, [sp, #32]
    ldp x21, x22, [sp, #16]
    ldp x19, x20, [sp, #0]
    add sp, sp, #96

    ret

.global trampoline
trampoline:
    // We expect:
    // x19 = arg (usize) -> Move to x0
    // x20 = entry (fn) -> Call
    
    mov x0, x19
    blr x20
    
    // Should not return
    brk #0
"#);

pub fn context_init(
    kstack_top: u64,
    entry: extern "C" fn(usize) -> !,
    arg: usize,
) -> usize {
    // Stack must be 16-byte aligned.
    // We push 96 bytes (6 pairs of 8 bytes).
    let stack_size = 96;
    let sp = kstack_top - stack_size;
    
    // Write initial values to the stack frame
    // Frame layout matches `context_switch` save/restore order.
    // [sp+0]  = x19, x20
    // ...
    // [sp+80] = x29, x30 (LR)
    
    let ptr = sp as *mut u64;
    unsafe {
        // x19 = arg, x20 = entry (for trampoline)
        ptr.offset(0).write(arg as u64);      // x19
        ptr.offset(1).write(entry as u64 as u64); // x20
        
        // x21..x28 = 0
        ptr.offset(2).write(0); // x21
        ptr.offset(3).write(0); // x22
        ptr.offset(4).write(0); // x23
        ptr.offset(5).write(0); // x24
        ptr.offset(6).write(0); // x25
        ptr.offset(7).write(0); // x26
        ptr.offset(8).write(0); // x27
        ptr.offset(9).write(0); // x28
        
        // x29(FP) = 0, x30(LR) = trampoline
        ptr.offset(10).write(0); // x29
        ptr.offset(11).write(trampoline as usize as u64); // x30 (LR)
    }
    
    sp as usize
}
