#![no_std]
#![no_main]

extern crate alloc;
use abi::cpu::CpuFeaturesWire;
use abi::syscall::nr::SYS_CPU_FEATURES;
use alloc::format;
use thing_std::{log_info, process, syscall};

#[no_mangle]
pub extern "C" fn main() {
    log_info("SIMD Check Starting");

    let mut features = CpuFeaturesWire {
        vendor: [0; 12],
        model: 0,
        features: 0,
        simd_available_mask: 0,
        simd_enabled_mask: 0,
        simd_policy: 0,
        _pad: 0,
    };

    let ptr = &mut features as *mut _ as u64;
    let len = core::mem::size_of::<CpuFeaturesWire>() as u64;

    let res = unsafe { syscall(SYS_CPU_FEATURES, ptr, len, 0, 0, 0, 0) };

    if res.status == 0 {
        log_info("Syscall Success");
        log_info(&format!("Policy: {}", features.simd_policy));
        log_info(&format!("Available: {:x}", features.simd_available_mask));
        log_info(&format!("Enabled: {:x}", features.simd_enabled_mask));
    } else {
        log_info(&format!("Syscall Failed: {}", res.status));
    }
    process::exit(0);
}
