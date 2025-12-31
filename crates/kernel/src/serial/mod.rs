//! Kernel serial output

#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::serial::Serial;

#[cfg(target_arch = "aarch64")]
use crate::arch::aarch64::serial::Serial;

#[cfg(target_arch = "riscv64")]
use crate::arch::riscv64::serial::Serial;

#[cfg(target_arch = "loongarch64")]
use crate::arch::loongarch64::serial::Serial;

static SERIAL: Serial = Serial::new();

pub fn putc(c: u8) {
    SERIAL.putc(c);
}

pub fn init(hhdm_offset: u64) {
    #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
    SERIAL.init(hhdm_offset);
}

pub fn write(bytes: &[u8]) {
    SERIAL.write(bytes);
}
