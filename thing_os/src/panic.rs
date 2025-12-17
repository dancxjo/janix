use core::panic::PanicInfo;
use abi::SyscallNumber;
use crate::sys::raw_syscall;

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let _ = crate::println!("Panic: {}", info);
    
    // Exit thread
    unsafe {
        raw_syscall(SyscallNumber::ExitThread, 0, 0, 0, 0, 0, 0);
    }
    loop {}
}
