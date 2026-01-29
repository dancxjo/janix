// Dispatch module for blit operations
                                                              
pub fn blit_rgba8888_over(dst: &mut [u32], src: &[u32]) {
    crate::trace_counter!("raster.blit.backend.count", 1);
    stem::simd::blit_rgba8888_over(dst, src);
}

pub fn blit_a8_tinted_over(dst: &mut [u32], mask: &[u8], color: u32, tint_a: u8) {
    // Phase 3: Add SIMD dispatch for A8
    crate::trace_counter!("raster.blit.backend.scalar.count", 1);
    scalar::blit_a8_tinted_over_scalar_row(dst, mask, color, tint_a);
}
