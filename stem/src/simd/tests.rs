#[cfg(test)]
mod tests {
    use crate::simd::{
        blit_rgba8888_over, composite_solid_masked_over, composite_src_masked_over,
    };
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
                ((a as u32) << 24)
                    | ((r as u32) << 16)
                    | ((g as u32) << 8)
                    | (b as u32)
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

    // --- masked compositing reference ---
    fn compute_solid_masked_pixel(dst: u32, color: u32, mask: u8) -> u32 {
        if mask == 0 {
            return dst;
        }

        let ca = ((color >> 24) & 0xFF) as u8;
        let cr = ((color >> 16) & 0xFF) as u8;
        let cg = ((color >> 8) & 0xFF) as u8;
        let cb = (color & 0xFF) as u8;

        let scale = |c: u8, a: u8| -> u32 {
            let t = c as u32 * a as u32;
            (t + 1 + (t >> 8)) >> 8
        };

        let sa = scale(ca, mask);
        let sr = scale(cr, mask);
        let sg = scale(cg, mask);
        let sb = scale(cb, mask);

        if sa == 255 {
            return (sa << 24) | (sr << 16) | (sg << 8) | sb;
        } else if sa == 0 {
            return dst;
        }

        let da = (dst >> 24) & 0xFF;
        let dr = (dst >> 16) & 0xFF;
        let dg = (dst >> 8) & 0xFF;
        let db = dst & 0xFF;

        let blend = |s: u32, d: u32, sa: u32| -> u32 {
            let inv = 255 - sa;
            let t = s * sa + d * inv;
            (t + 1 + (t >> 8)) >> 8
        };

        let out_a = sa + scale(da as u8, (255 - sa) as u8);
        let out_r = blend(sr, dr, sa);
        let out_g = blend(sg, dg, sa);
        let out_b = blend(sb, db, sa);

        (out_a << 24) | (out_r << 16) | (out_g << 8) | out_b
    }

    // --- SIMD correctness vs scalar ---

    #[test]
    fn test_solid_masked_simd_correctness() {
        let w = 32;
        let h = 8;
        let mut rng = XorShift32::new(0x5555_AAAA);

        let color = rng.next();
        let mut dst = vec![0u32; w * h];
        let mut mask = vec![0u8; w * h];

        for i in 0..(w * h) {
            dst[i] = rng.next() | 0xFF00_0000;
            mask[i] = (rng.next() & 0xFF) as u8;
        }

        let mut dst_scalar = dst.clone();
        composite_solid_masked_over_scalar(&mut dst_scalar, w, &mask, w, w, h, color);

        let mut dst_simd = dst.clone();
        composite_solid_masked_over(&mut dst_simd, w, &mask, w, w, h, color);

        assert_eq!(dst_scalar, dst_simd);
    }

    #[test]
    fn test_src_masked_simd_correctness() {
        let w = 32;
        let h = 8;
        let mut rng = XorShift32::new(0xAAAA_5555);

        let mut src = vec![0u32; w * h];
        let mut dst = vec![0u32; w * h];
        let mut mask = vec![0u8; w * h];

        for i in 0..(w * h) {
            src[i] = rng.next();
            dst[i] = rng.next() | 0xFF00_0000;
            mask[i] = (rng.next() & 0xFF) as u8;
        }

        let mut dst_scalar = dst.clone();
        composite_src_masked_over_scalar(&mut dst_scalar, w, &src, w, &mask, w, w, h);

        let mut dst_simd = dst.clone();
        composite_src_masked_over(&mut dst_simd, w, &src, w, &mask, w, w, h);

        assert_eq!(dst_scalar, dst_simd);
    }

    // --- SSE2 sanity checks ---

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    fn test_sse2_solid_masked_correctness() {
        let w = 16;
        let h = 4;
        let mut dst = vec![0xFF_80_80_80u32; w * h];
        let mask = vec![128u8; w * h];
        let color = 0x80_FF_00_FFu32;

        composite_solid_masked_over(&mut dst, w, &mask, w, w, h, color);

        assert!(dst.iter().any(|&p| p != 0xFF_80_80_80));
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    fn test_sse2_src_masked_correctness() {
        let w = 16;
        let h = 4;
        let mut dst = vec![0xFF_80_80_80u32; w * h];
        let src = vec![0x80_FF_00_FFu32; w * h];
        let mask = vec![128u8; w * h];

        composite_src_masked_over(&mut dst, w, &src, w, &mask, w, w, h);

        assert!(dst.iter().any(|&p| p != 0xFF_80_80_80));
    }

    // --- AVX2 bit-exact tests ---

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_avx2_solid_masked_correctness() {
        use crate::simd::x86::is_avx2_available;
        if !is_avx2_available() {
            return;
        }

        let w = 24;
        let h = 4;
        let mut dst = vec![0xFF_80_80_80u32; w * h];
        let mask = vec![128u8; w * h];
        let color = 0x80_FF_00_FFu32;

        let mut scalar = dst.clone();
        composite_solid_masked_over_scalar(&mut scalar, w, &mask, w, w, h, color);
        composite_solid_masked_over(&mut dst, w, &mask, w, w, h, color);

        assert_eq!(dst, scalar);
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_avx2_src_masked_correctness() {
        use crate::simd::x86::is_avx2_available;
        if !is_avx2_available() {
            return;
        }

        let w = 24;
        let h = 4;
        let mut dst = vec![0xFF_80_80_80u32; w * h];
        let src = vec![0x80_FF_00_FFu32; w * h];
        let mask = vec![128u8; w * h];

        let mut scalar = dst.clone();
        composite_src_masked_over_scalar(&mut scalar, w, &src, w, &mask, w, w, h);
        composite_src_masked_over(&mut dst, w, &src, w, &mask, w, w, h);

        assert_eq!(dst, scalar);
    }

    // --- NEON sanity checks ---

    #[test]
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    fn test_neon_solid_masked_correctness() {
        let w = 16;
        let h = 4;
        let mut dst = vec![0xFF_80_80_80u32; w * h];
        let mask = vec![128u8; w * h];
        let color = 0x80_FF_00_FFu32;

        composite_solid_masked_over(&mut dst, w, &mask, w, w, h, color);

        assert!(dst.iter().any(|&p| p != 0xFF_80_80_80));
    }

    #[test]
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    fn test_neon_src_masked_correctness() {
        let w = 16;
        let h = 4;
        let mut dst = vec![0xFF_80_80_80u32; w * h];
        let src = vec![0x80_FF_00_FFu32; w * h];
        let mask = vec![128u8; w * h];

        composite_src_masked_over(&mut dst, w, &src, w, &mask, w, w, h);

        assert!(dst.iter().any(|&p| p != 0xFF_80_80_80));
    }
}
