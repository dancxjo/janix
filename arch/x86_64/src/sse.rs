use core::arch::x86_64::*;

/// SSE-optimized blit for 32bpp framebuffers.
/// Fallbacks to `rep movsb` if scale is 256 (opaque).
///
/// Safety: dst and src must be valid for len bytes. len must be a multiple of 4.
#[target_feature(enable = "sse2")]
pub unsafe fn sse_blit(dst: *mut u8, src: *const u8, len: usize, scale: u32) {
    if scale >= 256 {
        // Fast path: Opaque copy using ERMS (rep movsb)
        core::arch::asm!(
            "rep movsb",
            inout("rcx") len => _,
            inout("rdi") dst => _,
            inout("rsi") src => _,
            options(nostack, preserves_flags)
        );
        return;
    }

    if scale == 0 {
        // Transparent: do nothing (or clear dst, but this blit is src-over-nothing effectively)
        return;
    }

    // SSE Alpha blending
    // (color * scale) >> 8
    // We process 4 pixels (16 bytes) at a time
    let mut i = 0;
    let v_scale = _mm_set1_epi16(scale as i16);
    let v_zero = _mm_setzero_si128();

    while i + 16 <= len {
        let s = _mm_load_si128(src.add(i) as *const __m128i);

        // Unpack bytes to words to multiply
        // Low 8 bytes -> [b0, g0, r0, a0, b1, g1, r1, a1]
        let lo = _mm_unpacklo_epi8(s, v_zero);
        let hi = _mm_unpackhi_epi8(s, v_zero);

        // multiply by scale
        let res_lo = _mm_mullo_epi16(lo, v_scale);
        let res_hi = _mm_mullo_epi16(hi, v_scale);

        // shift right 8
        let res_lo = _mm_srli_epi16(res_lo, 8);
        let res_hi = _mm_srli_epi16(res_hi, 8);

        // pack back to bytes
        let res = _mm_packus_epi16(res_lo, res_hi);

        _mm_store_si128(dst.add(i) as *mut __m128i, res);

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
