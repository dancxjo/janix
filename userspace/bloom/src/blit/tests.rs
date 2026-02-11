#[cfg(test)]
mod tests {
    use crate::blit::{blit_a8_tinted_over, blit_rgba8888_over};
    use alloc::vec::Vec;
    use stem::simd::scalar;

    // Helper to create a pattern
    fn make_pattern(len: usize) -> (Vec<u32>, Vec<u32>) {
        let mut dst = Vec::with_capacity(len);
        let mut src = Vec::with_capacity(len);
        for i in 0..len {
            dst.push(0xFF000000 | (i as u32)); // Opaque black + noise

            // Mix of alpha: 0, 255, and encoded
            let m = i % 10;
            let val = if m == 0 {
                0 // alpha 0
            } else if m == 1 {
                0xFFFFFFFF // alpha 255
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
        let len = 100;
        let (dst_orig, src) = make_pattern(len);

        // Scalar Run
        let mut dst_scalar = dst_orig.clone();
        scalar::blit_rgba8888_over_scalar(&mut dst_scalar, &src);

        // Dispatched Run (SSE2/NEON if available)
        let mut dst_simd = dst_orig.clone();
        blit_rgba8888_over(&mut dst_simd, &src);

        // Verify
        for i in 0..len {
            if dst_scalar[i] != dst_simd[i] {
                panic!(
                    "Mismatch at index {}: scalar={:08X}, simd={:08X}, src={:08X}",
                    i, dst_scalar[i], dst_simd[i], src[i]
                );
            }
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
        let len = 1000;
        let mut rng = XorShift32::new(0x12345678);

        let mut src = Vec::with_capacity(len);
        let mut dst_orig = Vec::with_capacity(len);

        for _ in 0..len {
            src.push(rng.next());
            dst_orig.push(rng.next() | 0xFF000000); // Ensure dst is opaque-ish usually
        }

        // Scalar
        let mut dst_scalar = dst_orig.clone();
        scalar::blit_rgba8888_over_scalar(&mut dst_scalar, &src);

        // SIMD
        let mut dst_simd = dst_orig.clone();
        blit_rgba8888_over(&mut dst_simd, &src);

        // Verify
        for i in 0..len {
            if dst_scalar[i] != dst_simd[i] {
                panic!("FUZZ Mismatch at index {}: scalar={:08X}, simd={:08X}, src={:08X}, dst_orig={:08X}", 
                       i, dst_scalar[i], dst_simd[i], src[i], dst_orig[i]);
            }
        }
    }

    #[test]
    fn test_a8_tinted_all_zero_mask() {
        let len = 32; // Small span to trigger fast path
        let color = 0xFFFF0000; // Opaque red
        let tint_a = 255;

        let mut dst = vec![0xFFFFFFFF; len];
        let dst_orig = dst.clone();
        let mask = vec![0u8; len]; // All zeros

        blit_a8_tinted_over(&mut dst, &mask, color, tint_a);

        // Should be unchanged
        assert_eq!(dst, dst_orig, "All-zero mask should not modify destination");
    }

    #[test]
    fn test_a8_tinted_all_255_mask_opaque() {
        let len = 32; // Small span to trigger fast path
        let color = 0xFFFF0000; // Opaque red
        let tint_a = 255;

        let mut dst = vec![0xFF000000; len];
        let mask = vec![255u8; len]; // All 255s

        blit_a8_tinted_over(&mut dst, &mask, color, tint_a);

        // Should all be red
        for &pixel in &dst {
            assert_eq!(
                pixel, color,
                "All-255 mask with opaque color should fill with color"
            );
        }
    }

    #[test]
    fn test_a8_tinted_zero_tint() {
        let len = 50;
        let color = 0xFFFF0000; // Opaque red
        let tint_a = 0;

        let mut dst = vec![0xFF000000; len];
        let dst_orig = dst.clone();
        let mask = vec![128u8; len];

        blit_a8_tinted_over(&mut dst, &mask, color, tint_a);

        // Should be unchanged
        assert_eq!(dst, dst_orig, "Zero tint_a should not modify destination");
    }

    #[test]
    fn test_a8_tinted_vs_scalar() {
        let len = 100;
        let color = 0xFFFF8800; // Opaque orange
        let tint_a = 255;

        let mut rng = XorShift32::new(0xABCDEF);
        let mut mask = Vec::with_capacity(len);
        let mut dst_orig = Vec::with_capacity(len);

        for _ in 0..len {
            mask.push((rng.next() & 0xFF) as u8);
            dst_orig.push(rng.next() | 0xFF000000);
        }

        // Reference (scalar)
        let mut dst_scalar = dst_orig.clone();
        stem::simd::scalar::composite_solid_masked_over_scalar(
            &mut dst_scalar,
            len,
            &mask,
            len,
            len,
            1,
            color,
        );

        // Optimized
        let mut dst_optimized = dst_orig.clone();
        blit_a8_tinted_over(&mut dst_optimized, &mask, color, tint_a);

        // Verify
        for i in 0..len {
            assert_eq!(
                dst_scalar[i], dst_optimized[i],
                "Mismatch at index {}: scalar={:08X}, optimized={:08X}, mask={}, color={:08X}",
                i, dst_scalar[i], dst_optimized[i], mask[i], color
            );
        }
    }

    #[test]
    fn test_a8_tinted_with_modulation() {
        let len = 100;
        let color = 0xFFFF8800; // Opaque orange
        let tint_a = 128; // Half alpha

        let mut rng = XorShift32::new(0xBEEF);
        let mut mask = Vec::with_capacity(len);
        let mut dst_orig = Vec::with_capacity(len);

        for _ in 0..len {
            mask.push((rng.next() & 0xFF) as u8);
            dst_orig.push(rng.next() | 0xFF000000);
        }

        // Compute effective color with tint_a modulation
        let scale_ch = |c: u8, a: u8| -> u32 {
            let t = c as u32 * a as u32;
            (t + 1 + (t >> 8)) >> 8
        };
        let ca = ((color >> 24) & 0xFF) as u8;
        let cr = ((color >> 16) & 0xFF) as u8;
        let cg = ((color >> 8) & 0xFF) as u8;
        let cb = (color & 0xFF) as u8;
        let ea = scale_ch(ca, tint_a);
        let er = scale_ch(cr, tint_a);
        let eg = scale_ch(cg, tint_a);
        let eb = scale_ch(cb, tint_a);
        let effective_color = (ea << 24) | (er << 16) | (eg << 8) | eb;

        // Reference (scalar)
        let mut dst_scalar = dst_orig.clone();
        stem::simd::scalar::composite_solid_masked_over_scalar(
            &mut dst_scalar,
            len,
            &mask,
            len,
            len,
            1,
            effective_color,
        );

        // Optimized
        let mut dst_optimized = dst_orig.clone();
        blit_a8_tinted_over(&mut dst_optimized, &mask, color, tint_a);

        // Verify
        for i in 0..len {
            assert_eq!(
                dst_scalar[i], dst_optimized[i],
                "Mismatch at index {}: scalar={:08X}, optimized={:08X}, mask={}, tint_a={}",
                i, dst_scalar[i], dst_optimized[i], mask[i], tint_a
            );
        }
    }

    #[test]
    fn test_a8_tinted_small_span_all_zero() {
        // Test the fast path for small spans with all-zero mask
        let len = 32; // Small span
        let color = 0xFFFF0000;
        let tint_a = 255;

        let mut dst = vec![0xFFFFFFFF; len];
        let dst_orig = dst.clone();
        let mask = vec![0u8; len];

        blit_a8_tinted_over(&mut dst, &mask, color, tint_a);

        assert_eq!(dst, dst_orig, "Small span all-zero should use fast path");
    }

    #[test]
    fn test_a8_tinted_small_span_all_255() {
        // Test the fast path for small spans with all-255 mask
        let len = 32; // Small span
        let color = 0xFFFF0000; // Opaque red
        let tint_a = 255;

        let mut dst = vec![0xFF000000; len];
        let mask = vec![255u8; len];

        blit_a8_tinted_over(&mut dst, &mask, color, tint_a);

        for &pixel in &dst {
            assert_eq!(
                pixel, color,
                "Small span all-255 should use fast path and fill with color"
            );
        }
    }

    #[test]
    fn test_a8_tinted_large_span_all_zero() {
        // Test that large spans don't use the fast path
        let len = 128; // > 64, so no fast path
        let color = 0xFFFF0000;
        let tint_a = 255;

        let mut dst = vec![0xFFFFFFFF; len];
        let dst_orig = dst.clone();
        let mask = vec![0u8; len];

        blit_a8_tinted_over(&mut dst, &mask, color, tint_a);

        // Should still be unchanged (SIMD handles zero mask correctly)
        assert_eq!(
            dst, dst_orig,
            "Large span all-zero should still produce correct output"
        );
    }
}
