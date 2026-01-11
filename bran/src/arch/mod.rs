#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "loongarch64")]
pub mod loongarch64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::hcf;
#[cfg(target_arch = "aarch64")]
pub use aarch64::hcf;
#[cfg(target_arch = "riscv64")]
pub use riscv64::hcf;
#[cfg(target_arch = "loongarch64")]
pub use loongarch64::hcf;

#[cfg(target_arch = "x86_64")]
pub type CurrentRuntime = crate::runtime::Runtime<x86_64::X86_64Runtime>;
#[cfg(target_arch = "aarch64")]
pub type CurrentRuntime = crate::runtime::Runtime<aarch64::AArch64Runtime>;
#[cfg(target_arch = "riscv64")]
pub type CurrentRuntime = crate::runtime::Runtime<riscv64::RISCV64Runtime>;
#[cfg(target_arch = "loongarch64")]
pub type CurrentRuntime = crate::runtime::Runtime<loongarch64::LoongArch64Runtime>;

pub const fn create_runtime() -> CurrentRuntime {
    crate::runtime::Runtime {
        #[cfg(target_arch = "x86_64")]
        arch: x86_64::X86_64Runtime::new(),
        #[cfg(target_arch = "aarch64")]
        arch: aarch64::AArch64Runtime::new(),
        #[cfg(target_arch = "riscv64")]
        arch: riscv64::RISCV64Runtime::new(),
        #[cfg(target_arch = "loongarch64")]
        arch: loongarch64::LoongArch64Runtime::new(),
        limine: crate::runtime::LimineRuntimeData::new(),
    }
}

pub fn init_paging() {
    let offset = crate::requests::HHDM_REQUEST.get_response().map(|r| r.offset()).unwrap_or(0);
    #[cfg(target_arch = "x86_64")]
    x86_64::paging::init(offset);
    #[cfg(target_arch = "aarch64")]
    aarch64::paging::init(offset);
    #[cfg(target_arch = "riscv64")]
    riscv64::paging::init(offset);
    #[cfg(target_arch = "loongarch64")]
    loongarch64::paging::init(offset);
}

pub unsafe fn init_interrupts() {
    #[cfg(target_arch = "aarch64")]
    unsafe { aarch64::vector::init(); }
    
    // x86_64, riscv64, loongarch64 interrupt init to be added if needed/when identified.
    // Assuming they are either handled elsewhere or not currently critical for this phase.
}
