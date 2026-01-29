// Dispatch module for blit operations
                                                              
pub(crate) mod scalar;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;

#[cfg(target_arch = "aarch64")]
mod arm;

mod tests;

pub fn blit_rgba8888_over(dst: &mut [u32], src: &[u32]) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[cfg(target_feature = "sse2")]
    unsafe {
        crate::trace_counter!("raster.blit.backend.sse2.count", 1);
        x86::blit_rgba8888_over_sse2(dst, src);
        return;
    }

    #[cfg(target_arch = "aarch64")]
    #[cfg(target_feature = "neon")]
    unsafe {
        crate::trace_counter!("raster.blit.backend.neon.count", 1);
        arm::blit_rgba8888_over_neon(dst, src);
        return;
    }

    crate::trace_counter!("raster.blit.backend.scalar.count", 1);
    scalar::blit_rgba8888_over_scalar_row(dst, src);
}

pub fn blit_a8_tinted_over(dst: &mut [u32], mask: &[u8], color: u32, tint_a: u8) {
    // Phase 3: Add SIMD dispatch for A8
    
    crate::trace_counter!("raster.blit.backend.scalar.count", 1);
    scalar::blit_a8_tinted_over_scalar_row(dst, mask, color, tint_a);
}
