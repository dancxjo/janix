use core::panic::PanicInfo;
use crate::sys_debug_putchar;
use abi::syscall::SYSCALL_EXIT;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let _ = mysprintf("PANIC in userspace!");
    if let Some(location) = info.location() {
        let _ = mysprintf("  at file/line"); 
        // We don't have a formatter without standard/core::fmt linked properly with a sink? 
        // Actually core::fmt::Write needs a struct.
        // We can just print the message string if possible.
    }
    // info.message() is not directly a string slice usually? It is typically Arguments.
    // We need fmt.
    
    // Simplest v0 panic: Just print "PANIC" and exit.
    // Or minimal implementation.
    
    unsafe {
        // "PANIC"
        for b in b"PANIC\n" {
            sys_debug_putchar(*b);
        }
        
        core::arch::asm!(
            "mov rax, {syscall_exit}",
            "mov rdi, 1", // exit code 1
            "syscall",
            syscall_exit = const SYSCALL_EXIT,
            options(noreturn)
        );
    }
}

fn mysprintf(s: &str) {
    unsafe {
        for b in s.bytes() {
            sys_debug_putchar(b);
        }
        sys_debug_putchar(b'\n');
    }
}
