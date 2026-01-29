// x86 SSE2 backend
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[inline(always)]
fn scale_ch(c: u8, a: u8) -> u32 {
    let t = c as u32 * a as u32;
    (t + 1 + (t >> 8)) >> 8
}

#[target_feature(enable = "sse2")]
pub unsafe fn blit_rgba8888_over_sse2(dst: &mut [u32], src: &[u32]) {
    let len = dst.len().min(src.len());
    let mut i = 0;

    while i + 4 <= len {
        let src_ptr = src.as_ptr().add(i) as *const __m128i;
        let dst_ptr = dst.as_mut_ptr().add(i) as *mut __m128i;

        let s = _mm_loadu_si128(src_ptr);
        let alphas = _mm_srli_epi32(s, 24);

        let zero = _mm_setzero_si128();
        let cmp_zero = _mm_cmpeq_epi32(alphas, zero);
        if _mm_movemask_epi8(cmp_zero) == 0xFFFF {
            i += 4;
            continue;
        }

        let alpha_255 = _mm_set1_epi32(255);
        let cmp_255 = _mm_cmpeq_epi32(alphas, alpha_255);
        if _mm_movemask_epi8(cmp_255) == 0xFFFF {
            _mm_storeu_si128(dst_ptr, s);
            i += 4;
            continue;
        }

        let d = _mm_loadu_si128(dst_ptr);

        let s_lo = _mm_unpacklo_epi8(s, zero);
        let s_hi = _mm_unpackhi_epi8(s, zero);
        let d_lo = _mm_unpacklo_epi8(d, zero);
        let d_hi = _mm_unpackhi_epi8(d, zero);

        let t1 = _mm_slli_epi32(alphas, 8);
        let t2 = _mm_or_si128(alphas, t1);
        let t3 = _mm_slli_epi32(t2, 16);
        let a_vec = _mm_or_si128(t2, t3);

        let a_lo = _mm_unpacklo_epi8(a_vec, zero);
        let a_hi = _mm_unpackhi_epi8(a_vec, zero);

        let v255 = _mm_set1_epi16(255);
        let inv_a_lo = _mm_sub_epi16(v255, a_lo);
        let inv_a_hi = _mm_sub_epi16(v255, a_hi);

        let s_mul_lo = _mm_mullo_epi16(s_lo, a_lo);
        let s_mul_hi = _mm_mullo_epi16(s_hi, a_hi);
        let d_mul_lo = _mm_mullo_epi16(d_lo, inv_a_lo);
        let d_mul_hi = _mm_mullo_epi16(d_hi, inv_a_hi);

        let sum_lo = _mm_add_epi16(s_mul_lo, d_mul_lo);
        let sum_hi = _mm_add_epi16(s_mul_hi, d_mul_hi);

        let one = _mm_set1_epi16(1);
        let t_shr_lo = _mm_srli_epi16(sum_lo, 8);
        let t_shr_hi = _mm_srli_epi16(sum_hi, 8);
        let t_lo = _mm_add_epi16(_mm_add_epi16(sum_lo, one), t_shr_lo);
        let t_hi = _mm_add_epi16(_mm_add_epi16(sum_hi, one), t_shr_hi);
        let res_lo = _mm_srli_epi16(t_lo, 8);
        let res_hi = _mm_srli_epi16(t_hi, 8);

        let result = _mm_packus_epi16(res_lo, res_hi);

        #[repr(align(16))]
        struct Buf([u32; 4]);
        let mut rgb_out = Buf([0; 4]);
        let mut dst_in = Buf([0; 4]);
        _mm_storeu_si128(rgb_out.0.as_mut_ptr() as *mut __m128i, result);
        _mm_storeu_si128(dst_in.0.as_mut_ptr() as *mut __m128i, d);
        let src_ptr_u32 = src.as_ptr().add(i);

        for k in 0..4 {
            let s_px = *src_ptr_u32.add(k);
            let sa = (s_px >> 24) & 0xFF;
            if sa == 0 {
                continue;
            }
            if sa == 255 {
                rgb_out.0[k] = s_px;
                continue;
            }
            let d_px = dst_in.0[k];
            let da = (d_px >> 24) & 0xFF;
            let out_a = sa + scale_ch(da as u8, (255 - sa) as u8);
            rgb_out.0[k] = (out_a << 24) | (rgb_out.0[k] & 0x00FF_FFFF);
        }

        _mm_storeu_si128(dst_ptr, rgb_out.0.as_ptr() as *const __m128i);
        i += 4;
    }

    if i < len {
        crate::simd::scalar::blit_rgba8888_over_scalar(&mut dst[i..], &src[i..]);
    }
}
