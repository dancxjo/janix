//! CPU capabilities and SIMD types.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SimdFamily {
    None = 0,
    X86_SSE = 1,
    X86_AVX = 2,
    X86_AVX512 = 3,
    ARM_NEON = 10,
    RISCV_V = 20,
    LOONG_LSX = 30,
    LOONG_LASX = 31,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SimdSavePolicy {
    None = 0,
    Eager = 1,
    Lazy = 2,
}

/// Wire format for CPU features (POD).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CpuFeaturesWire {
    pub vendor: [u8; 12],
    pub model: u32,
    pub features: u64,
    pub simd_available_mask: u64, // Bitmask of 1 << SimdFamily
    pub simd_enabled_mask: u64,
    pub simd_policy: u32, // SimdSavePolicy
    pub _pad: u32,
}
