#![no_std]
extern crate alloc;

pub use abi;
#[cfg(feature = "rt")]
pub use stem_macros::main;

pub mod syscall;
pub mod errors;
pub mod console;
pub mod device;
#[cfg(feature = "panic-handler")]
pub mod panic;
pub mod rt;
pub mod time;
pub mod pci;
pub mod thread;
pub mod stack;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn log(s: &str) {
    let _ = syscall::log_write(s, 3);
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::console::log(1, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::console::log(2, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::console::log(3, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::console::log(4, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => ($crate::console::log(5, format_args!($($arg)*)));
}

pub fn yield_now() {
    thread::yield_now();
}

pub fn sleep(duration: core::time::Duration) {
    time::sleep(duration);
}

pub fn sleep_ms(ms: u64) {
    time::sleep_ms(ms);
}

pub fn monotonic_ns() -> u64 {
    time::monotonic_ns()
}

pub fn rtc_time() -> Result<abi::device::RtcTime, abi::errors::Errno> {
    let mut t = abi::device::RtcTime::default();
    syscall::rtc_read(&mut t).map(|_| t)
}
pub mod thing;
pub use thing_macros::*;
#[cfg(feature = "global-alloc")]
pub mod allocator;
