use core::fmt;
use crate::syscall::debug_write;

pub struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = debug_write(s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut cons = Console;
    let _ = cons.write_fmt(args);
}
