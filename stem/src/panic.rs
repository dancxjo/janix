use core::panic::PanicInfo;
use crate::sys_debug_putchar;
use abi::syscall::SYSCALL_EXIT;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Simplest v0 panic: Just print "PANIC" and exit.
    
    unsafe {
        for b in b"PANIC\n" {
            sys_debug_putchar(*b);
        }
    
        if let Some(location) = info.location() {
             // We can tries to print file name if we had formatted output
             // For now just newline
        }
        
        crate::sys_exit(1);
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
