#![no_std]
// #![feature(asm_experimental_arch)]

pub use abi;

pub mod syscall;
pub mod errors;
// pub mod stream;
// pub mod event;
pub mod console;
pub mod device;
pub mod panic;
pub mod rt;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

pub fn log(s: &str) {
    let _ = syscall::log_write(s);
}

pub fn yield_now() {
    syscall::yield_now();
}

pub fn sleep(duration: core::time::Duration) {
    syscall::sleep_ns(duration.as_nanos() as u64);
}

pub fn monotonic_ns() -> u64 {
    syscall::monotonic_ns()
}

pub fn rtc_time() -> Result<abi::device::RtcTime, abi::errors::Errno> {
    let mut t = abi::device::RtcTime::default();
    syscall::rtc_read(&mut t).map(|_| t)
}
pub mod thing;
pub use thing_macros::*;
