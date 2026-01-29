#[cfg(test)]
mod tests {
    use crate::blit::scalar;
    use crate::blit::blit_rgba8888_over;
    use alloc::vec::Vec;

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
        scalar::blit_rgba8888_over_scalar_row(&mut dst_scalar, &src);
        
        // Dispatched Run (SSE2/NEON if available)
        let mut dst_simd = dst_orig.clone();
        blit_rgba8888_over(&mut dst_simd, &src);
        
        // Verify
        for i in 0..len {
            if dst_scalar[i] != dst_simd[i] {
                panic!("Mismatch at index {}: scalar={:08X}, simd={:08X}, src={:08X}", 
                       i, dst_scalar[i], dst_simd[i], src[i]);
            }
        }
    }

    struct XorShift32 {
        state: u32,
    }
    impl XorShift32 {
        fn new(seed: u32) -> Self { Self { state: seed } }
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
        scalar::blit_rgba8888_over_scalar_row(&mut dst_scalar, &src);
        
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
}
