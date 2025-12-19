use core::panic::PanicInfo;

use crate::sys::raw_syscall;

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let _ = crate::println!("Panic: {}", info);
    
    // Exit thread
    unsafe {
        raw_syscall(abi::syscalls::SYSCALL_EXIT_THREAD, 0, 0, 0, 0, 0, 0);
    }
    loop {}
}
