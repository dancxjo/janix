use core::arch::global_asm;

#[repr(C)]
#[derive(Debug, Default)]
pub struct ArchContext {
    pub sp: u64,
}

unsafe extern "C" {
    pub fn context_switch(old: *mut ArchContext, new: *const ArchContext);
}

unsafe extern "C" {
    fn trampoline();
}

global_asm!(r#"
.section .text
.global context_switch
context_switch:
    // x0 = old, x1 = new
    
    // Save callee-saved registers (x19-x29, x30/LR)
    // We push pairs. Stack must remain 16-byte aligned.
    // 12 regs: x19-x30. 6 pairs.
    
    stp x19, x20, [sp, #-16]!
    stp x21, x22, [sp, #-16]!
    stp x23, x24, [sp, #-16]!
    stp x25, x26, [sp, #-16]!
    stp x27, x28, [sp, #-16]!
    stp x29, x30, [sp, #-16]!
    
    // Save stack pointer
    mov x9, sp
    str x9, [x0]
    
    // Load new stack pointer
    ldr x9, [x1]
    mov sp, x9
    
    // Restore registers
    ldp x29, x30, [sp], #16
    ldp x27, x28, [sp], #16
    ldp x25, x26, [sp], #16
    ldp x23, x24, [sp], #16
    ldp x21, x22, [sp], #16
    ldp x19, x20, [sp], #16
    
    ret

.global trampoline
trampoline:
    // x19 = entry, x20 = arg
    mov x0, x20
    blr x19
    // Should not return
    brk #0
"#);

pub fn context_init(
    ctx: &mut ArchContext,
    kstack_top: u64,
    entry: extern "C" fn(arg: usize) -> !,
    arg: usize,
) {
    let mut sp = kstack_top;
    
    // Helper to push a pair
    let mut push_pair = |a: u64, b: u64| {
        sp -= 16;
        let ptr = sp as *mut u64;
        unsafe {
            ptr.offset(0).write(a);
            ptr.offset(1).write(b);
        }
    };
    
    // Stack layout mirrors push order:
    // (High Addr)
    // Initial:
    // x19, x20
    // x21, x22
    // x23, x24
    // x25, x26
    // x27, x28
    // x29, x30
    // (Low Addr)
    
    // Wait.
    // `stp x19, x20, [sp, #-16]!` (pre-index)
    // SP moves down 16. [SP] = x19, [SP+8] = x20.
    // So the Last Pushed Pair is at the Top (Lowest Address).
    
    // Order of pushes in ASM:
    // 1. x19, x20
    // ...
    // 6. x29, x30
    
    // Order of pops (ldp post-index) needs to reverse the stack movement but match the data.
    // `ldp x29, x30, [sp], #16`
    // Loads x29 from [SP], x30 from [SP+8], then SP += 16.
    
    // So the TOP of the stack (lowest addr) must contain x29, x30.
    // The BOTTOM of the saved frame (highest addr) must contain x19, x20.
    
    // So we push in reverse order of ASM pushes?
    // Asm pushes x19/x20 FIRST (highest address).
    // Asm pushes x29/x30 LAST (lowest address).
    
    // So to simulate:
    // We push x19/x20.
    // We push x21/x22.
    // ...
    // We push x29/x30.
    
    // x19 = entry, x20 = arg
    push_pair(entry as usize as u64, arg as u64); // x19, x20
    push_pair(0, 0); // x21, x22
    push_pair(0, 0); // x23, x24
    push_pair(0, 0); // x25, x26
    push_pair(0, 0); // x27, x28
    
    // x29 (FP), x30 (LR -> trampoline)
    push_pair(0, trampoline as usize as u64); 
    
    ctx.sp = sp;
}
