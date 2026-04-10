#![feature(restricted_std)]
#![no_main]

#[stem::main]
fn main(_arg: usize) -> ! {
    stem::syscall::exit(1)
}
