use core::arch::global_asm;

#[repr(C)]
#[derive(Debug, Default)]
pub struct ArchContext {
    pub rsp: u64,
}

unsafe extern "C" {
    pub fn context_switch(old: *mut ArchContext, new: *const ArchContext);
}

// Trampoline for new threads
// We expect: r12 = entry (fn), r13 = arg
unsafe extern "C" {
    fn trampoline();
}

global_asm!(r#"
.section .text
.global context_switch
context_switch:
    // rdi = old (ptr to ArchContext), rsi = new (ptr to ArchContext)
    
    // Save callee-saved registers
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    
    // Save old stack pointer
    mov [rdi], rsp
    
    // Load new stack pointer
    mov rsp, [rsi]
    
    // Restore callee-saved registers
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    
    ret

.global trampoline
trampoline:
    // Move arg (r13) to first argument register (rdi)
    mov rdi, r13
    // Call entry point (r12)
    call r12
    // Should not return
    ud2
"#);

pub fn context_init(
    ctx: &mut ArchContext,
    kstack_top: u64,
    entry: extern "C" fn(arg: usize) -> !,
    arg: usize,
) {
    let mut sp = kstack_top;
    
    // Helper to push a u64 onto the stack
    let mut push = |val: u64| {
        sp -= 8;
        unsafe { (sp as *mut u64).write(val) };
    };

    // Construct the initial stack frame to match what `context_switch` expects
    // when it pops registers.
    
    // 1. Return address (popped by `ret`) -> trampoline
    push(trampoline as usize as u64);
    
    // 2. Callee-saved registers (popped in reverse order of push)
    // Order of pop: r15, r14, r13, r12, rbp, rbx
    // So we push: rbx, rbp, r12, r13, r14, r15 (Wait, stack grows down)
    // context_switch pushes: rbx, rbp, ... r15
    // Stack top -> [r15] [r14] ... [rbx] -> [saved rsp]
    // Pop order: r15 (top), r14, ... rbx.
    
    // So we need to push in order:
    // rbx, rbp, r12, r13, r14, r15? No.
    // Logic:
    // SP points to last pushed item.
    // POP r15 reads from SP.
    // So r15 must be at the lowest address (top of stack).
    // So we push r15 LAST.
    
    // Stack layout (high to low):
    // [Trampoline Addr] (ret will pop this)
    // [rbx]
    // [rbp]
    // [r12]
    // [r13]
    // [r14]
    // [r15]  <-- SP
    
    // Correct? 
    // context_switch:
    // push rbx
    // ...
    // push r15
    // mov [rdi], rsp
    
    // So stack has r15 at the "top" (lowest address).
    // Pop r15 does: val = *sp; sp += 8.
    
    // So yes:
    // We push rbx first (highest addr below ret), then rbp, ... r15 last.
    
    push(0); // rbx
    push(0); // rbp
    push(entry as usize as u64); // r12
    push(arg as u64); // r13
    push(0); // r14
    push(0); // r15
    
    ctx.rsp = sp;
}
