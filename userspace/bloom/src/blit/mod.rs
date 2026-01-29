// Dispatch module for blit operations
mod scalar;
#[cfg(test)]
mod tests;

pub fn blit_rgba8888_over(dst: &mut [u32], src: &[u32]) {
    let len = dst.len().min(src.len());
    crate::trace_counter!("raster.blit.rgba.count", 1);
    crate::trace_counter!("raster.blit.rgba.pixels", len);
    stem::simd::blit_rgba8888_over(dst, src);
}

/// Blit an alpha mask with tinted color over destination.
///
/// Applies a coverage mask with a tinted solid color using the over operator.
/// This function includes several fast-path optimizations:
/// - Early exit when `tint_a == 0` (no contribution)
/// - Skip pre-modulation when `tint_a == 255` (no tint adjustment needed)
/// - For small spans (≤64 pixels):
///   - Skip when mask is all zeros (no coverage)
///   - Use solid fill when mask is all 255s with opaque color
///
/// Trace counters track optimization hits for performance analysis.
///
/// Note: This function processes one row at a time. For multi-row batching,
/// callers could be refactored to call the underlying SIMD function directly
/// with rect_h > 1, but that would require API changes.
pub fn blit_a8_tinted_over(dst: &mut [u32], mask: &[u8], color: u32, tint_a: u8) {
    if tint_a == 0 {
        crate::trace_counter!("raster.blit.a8_masked.early_exit_tint_zero", 1);
        return;
    }

    let len = dst.len().min(mask.len());
    if len == 0 {
        return;
    }

    // Compute effective color by modulating with tint_a
    // Note: stem doesn't export scale_ch, so we use the same formula here
    let effective_color = if tint_a == 255 {
        crate::trace_counter!("raster.blit.a8_masked.skip_modulation", 1);
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
    crate::trace_counter!("raster.blit.a8_masked.pixels", len);

    // Fast path: check for common span patterns (only worth it for small spans)
    if len <= 64 {
        // Check if mask is all zeros (common for anti-aliased edges outside the glyph)
        let all_zero = mask[..len].iter().all(|&m| m == 0);
        if all_zero {
            crate::trace_counter!("raster.blit.a8_masked.fast_all_zero", 1);
            crate::trace_counter!("raster.blit.a8_masked.fast_all_zero_pixels", len);
            return;
        }

        // Check if mask is all 255s with opaque color (common for solid text)
        if tint_a == 255 {
            let color_alpha = ((color >> 24) & 0xFF) as u8;
            if color_alpha == 255 {
                let all_255 = mask[..len].iter().all(|&m| m == 255);
                if all_255 {
                    crate::trace_counter!("raster.blit.a8_masked.fast_solid_opaque", 1);
                    crate::trace_counter!("raster.blit.a8_masked.fast_solid_opaque_pixels", len);
                    // Opaque solid fill - just copy the color
                    dst[..len].fill(effective_color);
                    return;
                }
            }
        }
    }

    // Use the new SIMD masked composite (1 row at a time, contiguous)
    stem::simd::composite_solid_masked_over(dst, len, mask, len, len, 1, effective_color);
}
