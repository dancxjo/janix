use abi::cpu::CpuFeaturesWire;
use abi::syscall::err;
use abi::wire::SyscallResult;
use crate::syscall::user_mem;

pub fn sys_cpu_features(out_ptr: u64, out_len: u64) -> SyscallResult {
    let required = core::mem::size_of::<CpuFeaturesWire>();
    let out_len = match usize::try_from(out_len) {
        Ok(len) => len,
        Err(_) => return SyscallResult::new(err::EINVAL, required as u64, 0),
    };
    if out_ptr == 0 || out_len < required {
        return SyscallResult::new(err::EINVAL, required as u64, 0);
    }

    let simd = crate::machine::simd();
    let avail = simd.available_families();
    let enabled = simd.enabled_families();

    let mut mask_avail = 0u64;
    for f in avail {
        mask_avail |= 1 << (f as u64);
    }

    let mut mask_enabled = 0u64;
    for f in enabled {
        mask_enabled |= 1 << (f as u64);
    }

    let wire = CpuFeaturesWire {
        vendor: [0; 12], // TODO: Implement vendor detection
        model: 0,
        features: 0,
        simd_available_mask: mask_avail,
        simd_enabled_mask: mask_enabled,
        simd_policy: simd.save_policy() as u32,
        _pad: 0,
    };

    let bytes = unsafe {
        core::slice::from_raw_parts(&wire as *const CpuFeaturesWire as *const u8, required)
    };
    if let Err(code) = user_mem::copy_to_user(out_ptr, bytes, required) {
        return SyscallResult::new(code, 0, 0);
    }

    SyscallResult::new(0, required as u64, 0)
}
