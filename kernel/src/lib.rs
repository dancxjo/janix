#![no_std]

pub mod logging;
pub mod time;

pub trait BootRuntime {
    fn putchar(&self, c: u8);
    fn halt(&self) -> !;

    /// Returns a strictly monotonic tick count since boot.
    ///
    /// This value must never decrease. If the underlying hardware timer
    /// wraps or is reset, the implementation must clamp the return value
    /// to the last largest value seen.
    fn mono_ticks(&self) -> u64 {
        0
    }

    /// Returns the frequency of the monotonic tick counter in Hz.
    ///
    /// Returns 0 if no monotonic timer is available.
    fn mono_freq_hz(&self) -> u64 {
        0
    }
}

pub fn start(runtime: &'static dyn BootRuntime) -> ! {
    unsafe {
        logging::init(runtime);
    }

    kinfo!("System booted");

    kinfo!("System halted");
    runtime.halt();
}
