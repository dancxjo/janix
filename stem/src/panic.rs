use core::panic::PanicInfo;
use standard::kerr;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kerr!("PANIC in userspace!");
    if let Some(location) = info.location() {
        kerr!("  at {}:{}:{}", location.file(), location.line(), location.column());
    }
    kerr!("  {}", info.message());
    
    // Exit with error
    unsafe {
        core::arch::asm!(
            "mov rax, {syscall_exit}",
            "mov rdi, 1", // exit code 1
            "syscall",
            syscall_exit = const abi::SYSCALL_EXIT,
            options(noreturn)
        );
    }
}
