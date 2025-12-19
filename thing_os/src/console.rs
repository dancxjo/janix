use core::fmt;

use crate::sys::raw_syscall;

pub struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ptr = s.as_ptr() as u64;
        let len = s.len() as u64;
        unsafe {
            raw_syscall(abi::syscalls::SYSCALL_LOG, ptr, len, 0, 0, 0, 0);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use alloc::string::String;
    let mut buf = String::new();
    let _ = buf.write_fmt(args);
    let _ = Console.write_str(&buf);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::console::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}
