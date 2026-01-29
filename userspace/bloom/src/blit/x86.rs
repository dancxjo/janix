// x86 SIMD Backend

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[target_feature(enable = "sse2")]
pub unsafe fn blit_rgba8888_over_sse2(dst: &mut [u32], src: &[u32]) {
    let len = dst.len().min(src.len());
    let mut i = 0;

    // TODO: Handle unaligned start if necessary, but u32 slices are usually 4-byte aligned.
    // Ideally we want 16-byte alignment for SSE, but unaligned load/store is okay-ish on modern x86.
    
    // Process 4 pixels at a time
    while i + 4 <= len {
        let src_ptr = src.as_ptr().add(i) as *const __m128i;
        let dst_ptr = dst.as_mut_ptr().add(i) as *mut __m128i;
        
        let s = _mm_loadu_si128(src_ptr);
        
        // Extract Alpha: 
        // s contains 4 pixels: ABGR ABGR ABGR ABGR (little endian u32)
        // Alpha is the high byte of each u32.
        // We want to check if all alphas are 0 or 255.
        // Shift right logically by 24 to get alphas in low bytes?
        // Or mask.
        // Let's use _mm_srli_epi32
        
        let alphas = _mm_srli_epi32(s, 24); // 000000AA 000000AA ...
        
        // Check zero (Skip)
        // _mm_cmpeq_epi32 returns 0xFFFFFFFF for equal, 0 for not.
        // If all are zero, cmpeq returns all ones. movemask extracts sign bits.
        // Wait, alphas has garbage in upper bytes? No, srli shifts in zeros.
        // So alphas is strictly 000000AA.
        
        let zero = _mm_setzero_si128();
        let cmp_zero = _mm_cmpeq_epi32(alphas, zero);
        let mask_zero = _mm_movemask_epi8(cmp_zero);
        if mask_zero == 0xFFFF {
             i += 4;
             continue;
        }

        // Check opaque (Copy)
        let alpha_255 = _mm_set1_epi32(255);
        let cmp_255 = _mm_cmpeq_epi32(alphas, alpha_255);
        let mask_255 = _mm_movemask_epi8(cmp_255);
        if mask_255 == 0xFFFF {
            _mm_storeu_si128(dst_ptr, s);
            crate::trace_counter!("raster.blit.rgba.fastpath_opaque_px_total", 4);
            i += 4;
            continue;
        }
        
        // Blend
        let d = _mm_loadu_si128(dst_ptr);

        // Unpack to 16-bit
        // s: P3 P2 P1 P0 (u32) -> u8 components
        // We need separate LO and HI unpacking for 16-bit math.
        
        // unpack_lo_epi8(s, zero) -> interleaves s and 0 -> u16 components
        // This effectively upgrades u8 to u16.
        let s_lo = _mm_unpacklo_epi8(s, zero);
        let s_hi = _mm_unpackhi_epi8(s, zero);
        let d_lo = _mm_unpacklo_epi8(d, zero);
        let d_hi = _mm_unpackhi_epi8(d, zero);
        
        // Alpha is in the high byte of each u16 pixel component?
        // No, unpacklo_epi8 interacts byte-wise.
        // s = [A3 R3 G3 B3] ... [A0 R0 G0 B0] (Little Endian u32: BB GG RR AA in reg?)
        // x86 is LE. u32 0xAARRGGBB -> memory BB GG RR AA.
        // _mm_unpacklo_epi8 -> B0 00 G0 00 R0 00 A0 00 ...
        
        // We need to broadcast alpha to all channels for multiplication.
        // Shuffle is tricky with SSE2 for defaults.
        // But we have the alphas in `alphas` (32-bit: 00 00 00 AA).
        
        // Let's create a 16-bit alpha vector: 00AA 00AA 00AA 00AA
        // s_lo has 2 pixels. s_hi has 2 pixels.
        
        // Extract alphas for LO:
        // s_lo: B0 . G0 . R0 . A0 . B1 . G1 . R1 . A1 .
        // We want A0 . A0 . A0 . A0 . A1 . A1 . A1 . A1 .
        
        // Shuffle high byte (A) to all positions?
        // SSE2 shuffle is limited.
        // Alternative:
        // s (u8): B G R A
        // shuffle: A A A A
        // unpack to u16.
        
        // Shuffle mask to broadcast alpha:
        // P0 has alpha at byte 3. P1 at 7. P2 at 11. P3 at 15.
        // We want 3 3 3 3 7 7 7 7 ...
        // _mm_shuffle_epi8 is SSSE3. We only have SSE2.
        
        // SSE2 fallback for broadcasting alpha:
        // shift and OR?
        // alphas (32-bit): 00 00 00 A0 | 00 00 00 A1 ...
        // Shift left by 16: 00 A0 00 00
        // Or together: 00 A0 00 A0
        // Shift by 8?
        
        // Better:
        // alpha_32 = 00 00 00 AA
        // a_lo = unpacklo_epi16(alpha_32, alpha_32) -> 00 00 00 A0 | 00 00 00 A0
        // This duplicates the 32-bit lanes.
        // Then unpacklo_epi8?
        
        // Let's rely on the fact that `scale_ch(c, a)` is `(c * a + 255) >> 8` (approx).
        // Actually the scalar code uses `(t + 1 + (t >> 8)) >> 8`.
        // For SSE2, simpler might be `(s * a + d * (255 - a)) / 255`.
        // `(x + 128) / 255` is approximately `((x + 128) + ((x + 128) >> 8)) >> 8`.
        
        // Let's prepare alpha vectors for s_lo and s_hi.
        // High shuffle (copies 32-bit chunks):
        // pshufd loop is good.
        let alpha_32 = alphas; // 00 00 00 AA
        
        // Broadcast alpha to all 4 bytes of the 32-bit lane.
        // Standard SSE2 trick:
        // a1 = slli(a, 16) | a
        // a2 = slli(a1, 8) | a1? No, alpha is at bits 0-7 now (from srli 24).
        // So a has 00 00 00 AA.
        // t1 = slli(a, 8) -> 00 00 AA 00
        // t2 = or(a, t1) -> 00 00 AA AA
        // t3 = slli(t2, 16) -> AA AA 00 00
        // a_vec = or(t2, t3) -> AA AA AA AA
        
        let t1 = _mm_slli_epi32(alphas, 8);
        let t2 = _mm_or_si128(alphas, t1);
        let t3 = _mm_slli_epi32(t2, 16);
        let a_vec = _mm_or_si128(t2, t3); // All channels have alpha
        
        let a_lo = _mm_unpacklo_epi8(a_vec, zero);
        let a_hi = _mm_unpackhi_epi8(a_vec, zero);
        
        // Invert alpha for dst
        let v255 = _mm_set1_epi16(255);
        let inv_a_lo = _mm_sub_epi16(v255, a_lo);
        let inv_a_hi = _mm_sub_epi16(v255, a_hi);
        
        // Multiply
        let s_mul_lo = _mm_mullo_epi16(s_lo, a_lo);
        let s_mul_hi = _mm_mullo_epi16(s_hi, a_hi);
        let d_mul_lo = _mm_mullo_epi16(d_lo, inv_a_lo);
        let d_mul_hi = _mm_mullo_epi16(d_hi, inv_a_hi);
        
        // Add
        let sum_lo = _mm_add_epi16(s_mul_lo, d_mul_lo);
        let sum_hi = _mm_add_epi16(s_mul_hi, d_mul_hi);
        
        // Divide by 255.
        // Using common approximation: (x + 128 + (x >> 8)) >> 8
        // Or strictly matching scalar: (x + 1 + (x >> 8)) >> 8
        // Scalar: (t + 1 + (t >> 8)) >> 8
        
        let one = _mm_set1_epi16(1);
        
        // lo
        let t_lo = sum_lo;
        let t_shr_lo = _mm_srli_epi16(t_lo, 8);
        let t_plus_1_lo = _mm_add_epi16(t_lo, one);
        let res_lo = _mm_srli_epi16(_mm_add_epi16(t_plus_1_lo, t_shr_lo), 8);

        // hi
        let t_hi = sum_hi;
        let t_shr_hi = _mm_srli_epi16(t_hi, 8);
        let t_plus_1_hi = _mm_add_epi16(t_hi, one);
        let res_hi = _mm_srli_epi16(_mm_add_epi16(t_plus_1_hi, t_shr_hi), 8);
        
        // Pack back to u8
        let result = _mm_packus_epi16(res_lo, res_hi);
        
        _mm_storeu_si128(dst_ptr, result);

        crate::trace_counter!("raster.blit.rgba.blend_px_total", 4);
        i += 4;
    }
    
    // Tail
    if i < len {
        crate::blit::scalar::blit_rgba8888_over_scalar_row(&mut dst[i..], &src[i..]);
    }
}
