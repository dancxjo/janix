#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

/// Initialize SIMD (NEON) on AArch64.
/// On AArch64, NEON is typically enabled by default, but this function
/// exists for API symmetry with x86_64.
pub unsafe fn init_simd() {
    // NEON is enabled by default on AArch64 when CPACR_EL1 is configured.
    // The _start function already sets CPACR_EL1.FPEN = 0b11.
}

/// NEON-optimized blit for 32bpp framebuffers on AArch64.
///
/// Safety: dst and src must be valid for len bytes. len must be a multiple of 4.
#[cfg(target_arch = "aarch64")]
pub unsafe fn neon_blit(dst: *mut u8, src: *const u8, len: usize, scale: u32) {
    if scale >= 256 {
        // Fast path: Opaque copy
        core::ptr::copy_nonoverlapping(src, dst, len);
        return;
    }

    if scale == 0 {
        return;
    }

    // NEON Alpha blending: (color * scale) >> 8
    // Process 4 pixels (16 bytes) at a time
    let mut i = 0;

    // vdupq_n_u16 creates a vector of 8x 16-bit values
    let v_scale = vdupq_n_u16(scale as u16);

    while i + 16 <= len {
        // Load 16 bytes (4 pixels)
        let s = vld1q_u8(src.add(i));

        // Unpack to 16-bit
        // vget_low_u8/vget_high_u8 get 8 bytes each
        let lo = vmovl_u8(vget_low_u8(s));
        let hi = vmovl_u8(vget_high_u8(s));

        // Multiply by scale
        let res_lo = vmulq_u16(lo, v_scale);
        let res_hi = vmulq_u16(hi, v_scale);

        // Shift right 8 and narrow back to u8
        // vshrn_n_u16 narrows 8x u16 to 8x u8
        let res_lo_u8 = vshrn_n_u16::<8>(res_lo);
        let res_hi_u8 = vshrn_n_u16::<8>(res_hi);

        // Combine back to 16 bytes
        let res = vcombine_u8(res_lo_u8, res_hi_u8);

        // Store
        vst1q_u8(dst.add(i), res);

        i += 16;
    }

    // Residual pixels (if any)
    while i < len {
        let b = *src.add(i);
        let g = *src.add(i + 1);
        let r = *src.add(i + 2);
        let a = *src.add(i + 3);

        *dst.add(i) = ((b as u32 * scale) >> 8) as u8;
        *dst.add(i + 1) = ((g as u32 * scale) >> 8) as u8;
        *dst.add(i + 2) = ((r as u32 * scale) >> 8) as u8;
        *dst.add(i + 3) = ((a as u32 * scale) >> 8) as u8;

        i += 4;
    }
}

/// Fallback neon_blit for non-aarch64 builds (e.g. host compile checks).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe fn neon_blit(dst: *mut u8, src: *const u8, len: usize, scale: u32) {
    if scale >= 256 {
        core::ptr::copy_nonoverlapping(src, dst, len);
        return;
    }
    if scale == 0 {
        return;
    }
    let mut i = 0;
    while i < len {
        let val = *src.add(i);
        *dst.add(i) = ((val as u32 * scale) >> 8) as u8;
        i += 1;
    }
}
