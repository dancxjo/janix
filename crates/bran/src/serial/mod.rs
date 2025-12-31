//! Serial port drivers for all supported architectures
//!
//! Provides unified serial output across:
//! - x86_64: 16550 UART on COM1 (0x3F8)
//! - aarch64: PL011 at 0x0900_0000
//! - riscv64: 16550 at 0x1000_0000
//! - loongarch64: 16550 at 0x1000_0000

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "riscv64")]
mod riscv64;

#[cfg(target_arch = "loongarch64")]
mod loongarch64;

/// Write a single byte to the serial port
pub fn putc(c: u8) {
    #[cfg(target_arch = "x86_64")]
    x86_64::putc(c);
    
    #[cfg(target_arch = "aarch64")]
    aarch64::putc(c);
    
    #[cfg(target_arch = "riscv64")]
    riscv64::putc(c);
    
    #[cfg(target_arch = "loongarch64")]
    loongarch64::putc(c);
}

/// Write bytes to the serial port
pub fn write(bytes: &[u8]) {
    for &b in bytes {
        putc(b);
    }
}
