#[cfg(all(target_arch = "x86_64", target_os = "thingos"))]
mod x86_64;
#[cfg(all(target_arch = "x86_64", target_os = "thingos"))]
pub use x86_64::*;

#[cfg(all(target_arch = "aarch64", target_os = "thingos"))]
mod aarch64;
#[cfg(all(target_arch = "aarch64", target_os = "thingos"))]
pub use aarch64::*;

#[cfg(all(target_arch = "riscv64", target_os = "thingos"))]
mod riscv64;
#[cfg(all(target_arch = "riscv64", target_os = "thingos"))]
pub use riscv64::*;

#[cfg(all(target_arch = "loongarch64", target_os = "thingos"))]
mod loongarch64;
#[cfg(all(target_arch = "loongarch64", target_os = "thingos"))]
pub use loongarch64::*;

// Host fallback
#[cfg(not(target_os = "thingos"))]
pub unsafe fn raw_syscall6(_n: u32, _a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    use abi::errors::Errno;
    -(Errno::NotSupported as isize)
}
