#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn blit_rgba8888_over_neon(dst: &mut [u32], src: &[u32]) {
    let len = dst.len().min(src.len());
    let mut i = 0;

    // Process 4 pixels at a time
    while i + 4 <= len {
        let src_ptr = src.as_ptr().add(i);
        let dst_ptr = dst.as_mut_ptr().add(i);

        let s = vld1q_u32(src_ptr);

        // Alpha Check
        let alphas = vshrq_n_u32(s, 24); // 000000AA

        // Zero skip
        // vmaxvq_u32 checks max across vector. If max is 0, all are 0.
        if vmaxvq_u32(alphas) == 0 {
            i += 4;
            continue;
        }

        // Opaque copy
        // vminvq_u32 checks min. If min is 255, all are 255.
        if vminvq_u32(alphas) == 255 {
            vst1q_u32(dst_ptr, s);
            crate::trace_counter!("raster.blit.rgba.fastpath_opaque_px_total", 4);
            i += 4;
            continue;
        }

        let d = vld1q_u32(dst_ptr);

        // Broadcast alpha: s = [A R G B] (u32, little endian in memory)
        // alphas (u32) = 000000AA
        // vsliq inserts shifted bits.
        // t = 0000AA00 | 000000AA = 0000AAAA
        let a_step1 = vsliq_n_u32(alphas, alphas, 8);
        // t2 = AAAA0000 | 0000AAAA = AAAAAAAA
        let a_step2 = vsliq_n_u32(a_step1, a_step1, 16);
        
        let a_u8 = vreinterpretq_u8_u32(a_step2);
        
        // 255 - a
        let v255 = vdupq_n_u8(255);
        let inv_a_u8 = vsubq_u8(v255, a_u8);
        
        // Unpack High/Low
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
        
        // Multiply Long: u8 * u8 -> u16
        // s * a
        let s_mul_lo = vmull_u8(s_lo, a_lo);
        let s_mul_hi = vmull_u8(s_hi, a_hi);
        
        // d * (255 - a)
        // vmlal_u8 accumulates: res = a + b * c
        // We want s_mul + d * inv_a
        let sum_lo = vmlal_u8(s_mul_lo, d_lo, inv_a_lo);
        let sum_hi = vmlal_u8(s_mul_hi, d_hi, inv_a_hi);
        
        // Div 255: (x + 1 + (x >> 8)) >> 8
        let one = vdupq_n_u16(1);
        
        // LO
        let t_lo = sum_lo;
        let t_shr_lo = vshrq_n_u16(t_lo, 8);
        let t_plus_1_lo = vaddq_u16(t_lo, one);
        let res_lo_u16 = vshrq_n_u16(vaddq_u16(t_plus_1_lo, t_shr_lo), 8);
        
        // HI
        let t_hi = sum_hi;
        let t_shr_hi = vshrq_n_u16(t_hi, 8);
        let t_plus_1_hi = vaddq_u16(t_hi, one);
        let res_hi_u16 = vshrq_n_u16(vaddq_u16(t_plus_1_hi, t_shr_hi), 8);
        
        // Narrow back to u8: vmovn_u16 -> u8x8
        let res_lo_u8 = vmovn_u16(res_lo_u16);
        let res_hi_u8 = vmovn_u16(res_hi_u16);
        
        // Combine to u8x16
        let result = vcombine_u8(res_lo_u8, res_hi_u8);
        let result_u32 = vreinterpretq_u32_u8(result);
        
        vst1q_u32(dst_ptr, result_u32);

        crate::trace_counter!("raster.blit.rgba.blend_px_total", 4);
        i += 4;
    }

    if i < len {
         crate::blit::scalar::blit_rgba8888_over_scalar_row(&mut dst[i..], &src[i..]);
    }
}
