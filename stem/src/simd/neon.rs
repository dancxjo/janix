#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[inline(always)]
fn scale_ch(c: u8, a: u8) -> u32 {
    let t = c as u32 * a as u32;
    (t + 1 + (t >> 8)) >> 8
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn blit_rgba8888_over_neon(dst: &mut [u32], src: &[u32]) {
    let len = dst.len().min(src.len());
    let mut i = 0;

    while i + 4 <= len {
        let src_ptr = src.as_ptr().add(i);
        let dst_ptr = dst.as_mut_ptr().add(i);

        let s = vld1q_u32(src_ptr);
        let alphas = vshrq_n_u32(s, 24);

        if vmaxvq_u32(alphas) == 0 {
            i += 4;
            continue;
        }

        if vminvq_u32(alphas) == 255 {
            vst1q_u32(dst_ptr, s);
            i += 4;
            continue;
        }

        let d = vld1q_u32(dst_ptr);

        let a_step1 = vsliq_n_u32(alphas, alphas, 8);
        let a_step2 = vsliq_n_u32(a_step1, a_step1, 16);
        let a_u8 = vreinterpretq_u8_u32(a_step2);

        let v255 = vdupq_n_u8(255);
        let inv_a_u8 = vsubq_u8(v255, a_u8);

        let s_u8 = vreinterpretq_u8_u32(s);
        let d_u8 = vreinterpretq_u8_u32(d);

        let s_lo = vget_low_u8(s_u8);
        let s_hi = vget_high_u8(s_u8);
        let d_lo = vget_low_u8(d_u8);
        let d_hi = vget_high_u8(d_u8);

        let a_lo = vget_low_u8(a_u8);
        let a_hi = vget_high_u8(a_u8);
        let inv_a_lo = vget_low_u8(inv_a_u8);
        let inv_a_hi = vget_high_u8(inv_a_u8);

        let s_mul_lo = vmull_u8(s_lo, a_lo);
        let s_mul_hi = vmull_u8(s_hi, a_hi);
        let sum_lo = vmlal_u8(s_mul_lo, d_lo, inv_a_lo);
        let sum_hi = vmlal_u8(s_mul_hi, d_hi, inv_a_hi);

        let one = vdupq_n_u16(1);
        let t_shr_lo = vshrq_n_u16(sum_lo, 8);
        let t_shr_hi = vshrq_n_u16(sum_hi, 8);
        let res_lo_u16 = vshrq_n_u16(vaddq_u16(vaddq_u16(sum_lo, one), t_shr_lo), 8);
        let res_hi_u16 = vshrq_n_u16(vaddq_u16(vaddq_u16(sum_hi, one), t_shr_hi), 8);

        let res_lo_u8 = vmovn_u16(res_lo_u16);
        let res_hi_u8 = vmovn_u16(res_hi_u16);
        let result = vcombine_u8(res_lo_u8, res_hi_u8);
        let result_u32 = vreinterpretq_u32_u8(result);

        // Fix alpha per-lane to match scalar semantics
        let mut rgb_out: [u32; 4] = core::mem::transmute(result_u32);
        let dst_in: [u32; 4] = core::mem::transmute(d);
        let src_in: [u32; 4] = core::mem::transmute(s);
        for k in 0..4 {
            let s_px = src_in[k];
            let sa = (s_px >> 24) & 0xFF;
            if sa == 0 {
                continue;
            }
            if sa == 255 {
                rgb_out[k] = s_px;
                continue;
            }
            let d_px = dst_in[k];
            let da = (d_px >> 24) & 0xFF;
            let out_a = sa + scale_ch(da as u8, (255 - sa) as u8);
            rgb_out[k] = (out_a << 24) | (rgb_out[k] & 0x00FF_FFFF);
        }

        vst1q_u32(dst_ptr, core::mem::transmute(rgb_out));
        i += 4;
    }

    if i < len {
        crate::simd::scalar::blit_rgba8888_over_scalar(&mut dst[i..], &src[i..]);
    }
}
