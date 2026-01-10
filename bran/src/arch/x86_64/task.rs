use core::arch::global_asm;
use kernel::UserTaskSpec;
use super::paging::X86_64AddressSpace;

#[derive(Clone, Copy, Default)]
pub struct X86_64Context(pub [usize; 1]);

unsafe extern "C" {
    pub fn context_switch(old: *mut usize, new: *const usize);
}

// Trampolines
unsafe extern "C" {
    fn kernel_trampoline();
    fn user_trampoline();
}

global_asm!(r#"
.section .text
.global context_switch
context_switch:
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    mov [rdi], rsp
    mov rsp, [rsi]
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    ret

.global kernel_trampoline
kernel_trampoline:
    mov rdi, r13
    call r12
    ud2

.global user_trampoline
user_trampoline:
    // r12 = user_entry, r13 = user_stack, r14 = aspace.0 (cr3), r15 = arg
    mov cr3, r14
    
    push 0x23 // User SS
    push r13  // User RSP
    push 0x202 // RFLAGS (IF=1)
    push 0x1B // User CS
    push r12  // User RIP
    
    mov rdi, r15
    xor rax, rax
    xor rbx, rbx
    xor rcx, rcx
    xor rdx, rdx
    xor rsi, rsi
    xor rbp, rbp
    xor r8, r8
    xor r9, r9
    xor r10, r10
    xor r11, r11
    
    iretq
"#);

pub fn init_kernel_context(
    entry: extern "C" fn(arg: usize) -> !,
    kstack_top: u64,
    arg: usize,
) -> X86_64Context {
    let mut sp = kstack_top;
    let mut push = |val: u64| {
        sp -= 8;
        unsafe { (sp as *mut u64).write(val) };
    };

    push(kernel_trampoline as usize as u64);
    push(0); // rbx
    push(0); // rbp
    push(entry as usize as u64); // r12
    push(arg as u64); // r13
    push(0); // r14
    push(0); // r15
    
    X86_64Context([sp as usize])
}

pub fn init_user_context(spec: UserTaskSpec<X86_64AddressSpace>, kstack_top: u64) -> X86_64Context {
    let mut sp = kstack_top;
    let mut push = |val: u64| {
        sp -= 8;
        unsafe { (sp as *mut u64).write(val) };
    };

    push(user_trampoline as usize as u64);
    push(0); // rbx
    push(0); // rbp
    push(spec.entry); // r12
    push(spec.stack_top); // r13
    push(spec.aspace.0); // r14
    push(spec.arg as u64); // r15
    
    X86_64Context([sp as usize])
}

pub unsafe fn switch(from: &mut X86_64Context, to: &X86_64Context) {
    unsafe {
        context_switch(from.0.as_mut_ptr(), to.0.as_ptr());
    }
}
