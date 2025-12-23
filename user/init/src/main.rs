#![no_std]
#![no_main]

#[cfg(target_os = "none")]
extern crate alloc;

// Panic handler is provided by thing_os::panic


#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        let msg = "INIT: MANUAL MAIN START\n";
        thing_os::sys::raw_syscall(
            thing_os::abi::syscalls::SYSCALL_LOG,
            msg.as_ptr() as u64,
            msg.len() as u64,
            0, 0, 0, 0
        );
    }

    thing_os::heap::init_user_heap();

    unsafe {
        let msg = "INIT: HEAP INIT DONE\n";
        thing_os::sys::raw_syscall(
            thing_os::abi::syscalls::SYSCALL_LOG,
            msg.as_ptr() as u64,
            msg.len() as u64,
            0, 0, 0, 0
        );
    }

    init::init_main();
}

