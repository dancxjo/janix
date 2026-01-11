#![no_std]
// #![feature(asm_experimental_arch)]

pub use abi;

pub mod syscall;
// pub mod stream;
// pub mod event;
pub mod rt;
pub mod panic;
pub mod console;
// pub mod device;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}
