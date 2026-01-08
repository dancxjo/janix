#![no_std]

pub trait BootRuntime {
    fn putchar(&self, c: u8);
}

pub fn start(runtime: impl BootRuntime) -> ! {
    let msg = b"System booted\n";
    for &c in msg {
        runtime.putchar(c);
    }

    loop {}
}
