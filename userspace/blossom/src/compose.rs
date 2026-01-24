//! Simple CPU compositing helpers (RGBA8888).

use crate::surface::Surface;

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
    src: &Surface,
    dst: &mut Surface,
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
