//! Simple SIMD abstraction for pixel blits.
//! Provides scalar reference and architecture backends with identical math.
//! The scalar implementation is the canonical truth; SIMD backends must match it exactly.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(target_arch = "aarch64")]
mod neon;
mod scalar;

/// Blend src over dst (premultiplied RGBA8888).
pub fn blit_rgba8888_over(dst: &mut [u32], src: &[u32]) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[cfg(target_feature = "sse2")]
    unsafe {
        x86::blit_rgba8888_over_sse2(dst, src);
        return;
    }

    #[cfg(target_arch = "aarch64")]
    #[cfg(target_feature = "neon")]
    unsafe {
        neon::blit_rgba8888_over_neon(dst, src);
        return;
    }

    scalar::blit_rgba8888_over_scalar(dst, src);
}

#[cfg(test)]
mod tests;
