//! Simple CPU compositing helpers (RGBA8888).
//!
//! # Examples
//! ```
//! use blossom::compose::blend_rgba;
//! assert_eq!(blend_rgba(0xFFFFFFFF, 0x00000000), 0xFFFFFFFF);
//! ```

use crate::surface::MappedSurface;

/// Blend a source RGBA pixel over a destination RGBA pixel.
pub fn blend_rgba(src: u32, dst: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    if sa == 0 {
        return dst;
    }
    if sa == 0xFF {
        return src | 0xFF00_0000;
    }
    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;

    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;

    let inv = 255 - sa;
    let r = (sr * sa + dr * inv) / 255;
    let g = (sg * sa + dg * inv) / 255;
    let b = (sb * sa + db * inv) / 255;
    0xFF00_0000 | (r << 16) | (g << 8) | b
}

/// Blit a source surface into a destination surface with alpha blending.
pub fn blit(
    src: &MappedSurface,
    dst: &mut MappedSurface,
    src_x: i32,
    src_y: i32,
    width: i32,
    height: i32,
    dst_x: i32,
    dst_y: i32,
) {
    for y in 0..height {
        for x in 0..width {
            let sx = src_x + x;
            let sy = src_y + y;
            let dx = dst_x + x;
            let dy = dst_y + y;
            let s = src.get_px(sx, sy);
            if (s >> 24) & 0xFF == 0 {
                continue;
            }
            let d = dst.get_px(dx, dy);
            let out = blend_rgba(s, d);
            dst.put_px(dx, dy, out);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::surface::{MappedSurface, SurfaceSpec};

    #[test]
    fn blend_half_alpha() {
        let src = 0x800000FF;
        let dst = 0xFF000000;
        let blended = blend_rgba(src, dst);
        assert_eq!(blended & 0xFF, 0x7F);
    }

    #[test]
    fn blit_copies_pixels() {
        let mut src_buf = [0u8; 16];
        let mut dst_buf = [0u8; 16];
        let spec = SurfaceSpec {
            width: 2,
            height: 2,
            stride_bytes: 8,
        };
        let mut src = unsafe { MappedSurface::from_parts(src_buf.as_mut_ptr(), src_buf.len(), spec) };
        let mut dst = unsafe { MappedSurface::from_parts(dst_buf.as_mut_ptr(), dst_buf.len(), spec) };
        src.clear(0xFFFF0000);
        blit(&src, &mut dst, 0, 0, 2, 2, 0, 0);
        assert_eq!(dst.get_px(1, 1), 0xFFFF0000);
    }
}
