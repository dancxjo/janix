// Dispatch module for blit operations
mod scalar;
pub fn blit_rgba8888_over(dst: &mut [u32], src: &[u32]) {
    crate::trace_counter!("raster.blit.rgba.count", 1);
    stem::simd::blit_rgba8888_over(dst, src);
}

pub fn blit_a8_tinted_over(dst: &mut [u32], mask: &[u8], color: u32, tint_a: u8) {
    if tint_a == 0 {
        return;
    }

    // Compute effective color by modulating with tint_a
    // Note: stem doesn't export scale_ch, so we use the same formula here
    let effective_color = if tint_a == 255 {
        color
    } else {
        let ca = ((color >> 24) & 0xFF) as u8;
        let cr = ((color >> 16) & 0xFF) as u8;
        let cg = ((color >> 8) & 0xFF) as u8;
        let cb = (color & 0xFF) as u8;

        // Fast approximation of (c * a) / 255 with exact rounding
        let scale_ch = |c: u8, a: u8| -> u32 {
            let t = c as u32 * a as u32;
            (t + 1 + (t >> 8)) >> 8
        };

        let ea = scale_ch(ca, tint_a);
        let er = scale_ch(cr, tint_a);
        let eg = scale_ch(cg, tint_a);
        let eb = scale_ch(cb, tint_a);

        (ea << 24) | (er << 16) | (eg << 8) | eb
    };

    crate::trace_counter!("raster.blit.a8_masked.count", 1);

    // Use the new SIMD masked composite (1 row at a time, contiguous)
    let len = dst.len().min(mask.len());
    stem::simd::composite_solid_masked_over(dst, len, mask, len, len, 1, effective_color);
}
