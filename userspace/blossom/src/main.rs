#![no_std]
#![no_main]

use blossom::BlossomRuntime;

#[stem::main]
fn main(arg: usize) -> ! {
    BlossomRuntime::new(arg).run()
}
