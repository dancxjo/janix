#[cfg(test)]
mod tests {
    use crate::simd::blit_rgba8888_over;
    use crate::simd::scalar::{
        blit_rgba8888_over_scalar, composite_solid_masked_over_scalar,
        composite_src_masked_over_scalar,
    };
    use alloc::vec;
    use alloc::vec::Vec;

    // Simple deterministic pattern
    fn make_pattern(len: usize) -> (Vec<u32>, Vec<u32>) {
        let mut dst = Vec::with_capacity(len);
        let mut src = Vec::with_capacity(len);
        for i in 0..len {
            dst.push(0xFF000000 | (i as u32)); // Opaque base
            let m = i % 10;
            let val = if m == 0 {
                0
            } else if m == 1 {
                0xFFFFFFFF
            } else {
                let a = (i * 17) as u8;
                let r = (i * 5) as u8;
                let g = (i * 7) as u8;
                let b = (i * 11) as u8;
                ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            };
            src.push(val);
        }
        (dst, src)
    }

    #[test]
    fn test_rgba_blit_correctness() {
        let len = 128;
        let (dst_orig, src) = make_pattern(len);

        let mut dst_scalar = dst_orig.clone();
        blit_rgba8888_over_scalar(&mut dst_scalar, &src);

        let mut dst_simd = dst_orig.clone();
        blit_rgba8888_over(&mut dst_simd, &src);

        for i in 0..len {
            assert_eq!(dst_scalar[i], dst_simd[i], "idx {}", i);
        }
    }

    struct XorShift32 {
        state: u32,
    }
    impl XorShift32 {
        fn new(seed: u32) -> Self {
            Self { state: seed }
        }
        fn next(&mut self) -> u32 {
            let mut x = self.state;
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            self.state = x;
            x
        }
    }

    #[test]
    fn test_rgba_blit_fuzz() {
        let len = 1024;
        let mut rng = XorShift32::new(0x1234_5678);

        let mut src = Vec::with_capacity(len);
        let mut dst_orig = Vec::with_capacity(len);
        for _ in 0..len {
            src.push(rng.next());
            dst_orig.push(rng.next() | 0xFF00_0000);
        }

        let mut dst_scalar = dst_orig.clone();
        blit_rgba8888_over_scalar(&mut dst_scalar, &src);

        let mut dst_simd = dst_orig.clone();
        blit_rgba8888_over(&mut dst_simd, &src);

        for i in 0..len {
            assert_eq!(dst_scalar[i], dst_simd[i], "fuzz idx {}", i);
        }
    }

    // Tests for masked compositing
    #[test]
    fn test_solid_masked_edge_cases() {
        // Test critical mask and alpha values
        let mask_values = [0u8, 1, 2, 127, 128, 254, 255];
        let alpha_values = [0u8, 1, 2, 127, 128, 254, 255];

        for &mask in &mask_values {
            for &alpha in &alpha_values {
                let color = (alpha as u32) << 24 | 0x80_40_20; // premul color
                let mut dst = vec![0xFF_C0_80_40u32; 8];
                let mask_buf = vec![mask; 8];

                composite_solid_masked_over_scalar(&mut dst, 8, &mask_buf, 8, 8, 1, color);

                // Verify first pixel manually
                let expected = compute_solid_masked_pixel(0xFF_C0_80_40, color, mask);
                assert_eq!(
                    dst[0], expected,
                    "solid masked: mask={} alpha={} got {:08x} expected {:08x}",
                    mask, alpha, dst[0], expected
                );
            }
        }
    }

    fn compute_solid_masked_pixel(dst: u32, color: u32, mask: u8) -> u32 {
        if mask == 0 {
            return dst;
        }

        let ca = ((color >> 24) & 0xFF) as u8;
        let cr = ((color >> 16) & 0xFF) as u8;
        let cg = ((color >> 8) & 0xFF) as u8;
        let cb = (color & 0xFF) as u8;

        let scale_ch = |c: u8, a: u8| -> u32 {
            let t = c as u32 * a as u32;
            (t + 1 + (t >> 8)) >> 8
        };

        let sa = scale_ch(ca, mask);
        let sr = scale_ch(cr, mask);
        let sg = scale_ch(cg, mask);
        let sb = scale_ch(cb, mask);

        if sa == 255 {
            return (sa << 24) | (sr << 16) | (sg << 8) | sb;
        } else if sa == 0 {
            return dst;
        }

        let da = (dst >> 24) & 0xFF;
        let dr = (dst >> 16) & 0xFF;
        let dg = (dst >> 8) & 0xFF;
        let db = dst & 0xFF;

        let blend_channel = |s: u32, d: u32, sa: u32| -> u32 {
            let inv = 255 - sa;
            let t = s * sa + d * inv;
            (t + 1 + (t >> 8)) >> 8
        };

        let out_a = sa + scale_ch(da as u8, (255 - sa) as u8);
        let out_r = blend_channel(sr, dr, sa);
        let out_g = blend_channel(sg, dg, sa);
        let out_b = blend_channel(sb, db, sa);

        (out_a << 24) | (out_r << 16) | (out_g << 8) | out_b
    }

    #[test]
    fn test_solid_masked_fuzz() {
        let w = 32;
        let h = 16;
        let mut rng = XorShift32::new(0xABCD_1234);

        for _ in 0..10 {
            let color = rng.next();
            let mut dst = Vec::with_capacity(w * h);
            let mut mask = Vec::with_capacity(w * h);

            for _ in 0..(w * h) {
                dst.push(rng.next() | 0xFF00_0000); // opaque dst
                mask.push((rng.next() & 0xFF) as u8);
            }

            let mut result = dst.clone();
            composite_solid_masked_over_scalar(&mut result, w, &mask, w, w, h, color);

            // Validate against reference
            for y in 0..h {
                for x in 0..w {
                    let idx = y * w + x;
                    let expected = compute_solid_masked_pixel(dst[idx], color, mask[idx]);
                    assert_eq!(
                        result[idx], expected,
                        "fuzz solid masked at ({},{}) mask={}",
                        x, y, mask[idx]
                    );
                }
            }
        }
    }

    #[test]
    fn test_src_masked_edge_cases() {
        let mask_values = [0u8, 1, 2, 127, 128, 254, 255];
        let alpha_values = [0u8, 1, 2, 127, 128, 254, 255];

        for &mask in &mask_values {
            for &alpha in &alpha_values {
                let src_color = (alpha as u32) << 24 | 0x80_40_20;
                let src = vec![src_color; 8];
                let mut dst = vec![0xFF_C0_80_40u32; 8];
                let mask_buf = vec![mask; 8];

                composite_src_masked_over_scalar(&mut dst, 8, &src, 8, &mask_buf, 8, 8, 1);

                let expected = compute_solid_masked_pixel(0xFF_C0_80_40, src_color, mask);
                assert_eq!(
                    dst[0], expected,
                    "src masked: mask={} alpha={} got {:08x} expected {:08x}",
                    mask, alpha, dst[0], expected
                );
            }
        }
    }

    #[test]
    fn test_src_masked_fuzz() {
        let w = 32;
        let h = 16;
        let mut rng = XorShift32::new(0x9876_5432);

        for _ in 0..10 {
            let mut src = Vec::with_capacity(w * h);
            let mut dst = Vec::with_capacity(w * h);
            let mut mask = Vec::with_capacity(w * h);

            for _ in 0..(w * h) {
                src.push(rng.next());
                dst.push(rng.next() | 0xFF00_0000);
                mask.push((rng.next() & 0xFF) as u8);
            }

            let mut result = dst.clone();
            composite_src_masked_over_scalar(&mut result, w, &src, w, &mask, w, w, h);

            for y in 0..h {
                for x in 0..w {
                    let idx = y * w + x;
                    let expected = compute_solid_masked_pixel(dst[idx], src[idx], mask[idx]);
                    assert_eq!(
                        result[idx], expected,
                        "fuzz src masked at ({},{}) mask={}",
                        x, y, mask[idx]
                    );
                }
            }
        }
    }

    #[test]
    fn test_masked_stride() {
        // Test non-contiguous stride access
        let w = 4;
        let h = 3;
        let dst_stride = 8;
        let mask_stride = 6;

        let mut dst = vec![0xFF_80_80_80u32; dst_stride * h];
        let mut mask = vec![0u8; mask_stride * h];

        // Fill mask with pattern
        for y in 0..h {
            for x in 0..w {
                mask[y * mask_stride + x] = ((y * w + x) * 32) as u8;
            }
        }

        let color = 0x80_FF_00_FFu32;
        composite_solid_masked_over_scalar(&mut dst, dst_stride, &mask, mask_stride, w, h, color);

        // Verify pixels in active region
        for y in 0..h {
            for x in 0..w {
                let m = mask[y * mask_stride + x];
                let expected = compute_solid_masked_pixel(0xFF_80_80_80, color, m);
                assert_eq!(
                    dst[y * dst_stride + x],
                    expected,
                    "stride test at ({},{}) mask={}",
                    x,
                    y,
                    m
                );
            }
            // Verify pixels beyond rect_w are untouched
            for x in w..dst_stride {
                assert_eq!(dst[y * dst_stride + x], 0xFF_80_80_80, "untouched at ({},{})", x, y);
            }
        }
    }
}
