#![no_std]
// #![feature(asm_experimental_arch)]
// Actually loongarch64 asm is stable in recent nightlies, but let's see. 
// We might need feature(naked_functions) later for _start.

pub use abi;

pub mod syscall;
pub mod stream;
pub mod event;
pub mod rt;
pub mod panic;
