use core::panic::PanicInfo;

use crate::sys::raw_syscall;

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log_raw("PANIC\n");
    if let Some(loc) = info.location() {
        log_raw(" at ");
        log_raw(loc.file());
        log_raw(":");
        log_num(loc.line() as u64);
        log_raw("\n");
    }

    // Exit thread
    unsafe {
        raw_syscall(abi::syscalls::SYSCALL_EXIT_THREAD, 0, 0, 0, 0, 0, 0);
    }
    loop {}
}

fn log_raw(s: &str) {
    let ptr = s.as_ptr() as u64;
    let len = s.len() as u64;
    unsafe {
        raw_syscall(abi::syscalls::SYSCALL_LOG, ptr, len, 0, 0, 0, 0);
    }
}

fn log_num(mut n: u64) {
    // Decimal print without fmt to keep panic path minimal.
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    if n == 0 {
        buf[i - 1] = b'0';
        i -= 1;
    } else {
        while n > 0 && i > 0 {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }
    }
    let s = unsafe { core::str::from_utf8_unchecked(&buf[i..]) };
    log_raw(s);
}
