use core::fmt;
use crate::console::console_putchar;

/// A simple writer that pushes characters to the global console sink.
pub struct ConsoleWriter;

impl fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            console_putchar(c);
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    let mut writer = ConsoleWriter;
    let _ = writer.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Kernel-style level macros, mapped to println for now (or decorated later)

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => ($crate::println!("[INFO] {}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kwarn {
    ($($arg:tt)*) => ($crate::println!("[WARN] {}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kerr {
    ($($arg:tt)*) => ($crate::println!("[ERR]  {}", format_args!($($arg)*)));
}
