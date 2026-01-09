#![no_std]

pub mod logging;

pub trait BootRuntime {
    fn putchar(&self, c: u8);
    fn halt(&self) -> !;
}

pub fn start(runtime: &'static dyn BootRuntime) -> ! {
    unsafe {
        logging::init(runtime);
    }

    kinfo!("System booted");

    kinfo!("System halted");
    runtime.halt();
}
