#[inline(always)]
fn scale_ch(c: u8, a: u8) -> u32 {
    let t = c as u32 * a as u32;
    (t + 1 + (t >> 8)) >> 8
}

#[inline(always)]
fn blend_channel(s: u32, d: u32, sa: u32) -> u32 {
    let inv = 255 - sa;
    let t = s * sa + d * inv;
    (t + 1 + (t >> 8)) >> 8
}

/// Scalar reference implementation (canonical semantics).
pub fn blit_rgba8888_over_scalar(dst: &mut [u32], src: &[u32]) {
    let len = dst.len().min(src.len());
    let dst = &mut dst[0..len];
    let src = &src[0..len];

    for (d, &s) in dst.iter_mut().zip(src.iter()) {
        let sa = (s >> 24) & 0xFF;
        if sa == 0 {
            continue;
        }
        if sa == 255 {
            *d = s;
        } else {
            let sr = (s >> 16) & 0xFF;
            let sg = (s >> 8) & 0xFF;
            let sb = s & 0xFF;

            let dv = *d;
            let da = (dv >> 24) & 0xFF;
            let dr = (dv >> 16) & 0xFF;
            let dg = (dv >> 8) & 0xFF;
            let db = dv & 0xFF;

            let out_a = sa + scale_ch(da as u8, (255 - sa) as u8);
            let out_r = blend_channel(sr, dr, sa);
            let out_g = blend_channel(sg, dg, sa);
            let out_b = blend_channel(sb, db, sa);

            *d = (out_a << 24) | (out_r << 16) | (out_g << 8) | out_b;
        }
    }
}

/// Composite solid color with coverage mask (scalar reference).
///
/// Math contract (canonical):
/// - All inputs/outputs are premultiplied RGBA8888
/// - Coverage mask modulates the color's alpha: `color_a' = (color_a * mask + 127) / 255`
/// - RGB channels are modulated: `color_rgb' = (color_rgb * mask + 127) / 255`
/// - Over operator: `dst = color' + dst * (1 - color_a')`
/// - Rounding uses `(t + 1 + (t >> 8)) >> 8` for the over blend
/// - Mask values: 0 = no change, 255 = full color, intermediate = proportional blend
pub fn composite_solid_masked_over_scalar(
    dst: &mut [u32],
    dst_stride: usize,
    mask: &[u8],
    mask_stride: usize,
    rect_w: usize,
    rect_h: usize,
    color_premul: u32,
) {
    let ca = ((color_premul >> 24) & 0xFF) as u8;
    let cr = ((color_premul >> 16) & 0xFF) as u8;
    let cg = ((color_premul >> 8) & 0xFF) as u8;
    let cb = (color_premul & 0xFF) as u8;

    for y in 0..rect_h {
        let dst_row = &mut dst[y * dst_stride..];
        let mask_row = &mask[y * mask_stride..];

        for x in 0..rect_w {
            let m = mask_row[x];
            if m == 0 {
                continue;
            }

            // Modulate color by mask
            let sa = scale_ch(ca, m);
            let sr = scale_ch(cr, m);
            let sg = scale_ch(cg, m);
            let sb = scale_ch(cb, m);

            if sa == 255 {
                dst_row[x] = (sa << 24) | (sr << 16) | (sg << 8) | sb;
            } else if sa > 0 {
                let dv = dst_row[x];
                let da = (dv >> 24) & 0xFF;
                let dr = (dv >> 16) & 0xFF;
                let dg = (dv >> 8) & 0xFF;
                let db = dv & 0xFF;

                let out_a = sa + scale_ch(da as u8, (255 - sa) as u8);
                let out_r = blend_channel(sr, dr, sa);
                let out_g = blend_channel(sg, dg, sa);
                let out_b = blend_channel(sb, db, sa);

                dst_row[x] = (out_a << 24) | (out_r << 16) | (out_g << 8) | out_b;
            }
        }
    }
}

/// Composite source pixels with coverage mask (scalar reference).
///
/// Math contract (canonical):
/// - All inputs/outputs are premultiplied RGBA8888
/// - Mask modulates source alpha: `src_a' = (src_a * mask + 127) / 255`
/// - Mask modulates source RGB: `src_rgb' = (src_rgb * mask + 127) / 255`
/// - Over operator: `dst = src' + dst * (1 - src_a')`
/// - Rounding uses `(t + 1 + (t >> 8)) >> 8` for the over blend
pub fn composite_src_masked_over_scalar(
    dst: &mut [u32],
    dst_stride: usize,
    src: &[u32],
    src_stride: usize,
    mask: &[u8],
    mask_stride: usize,
    rect_w: usize,
    rect_h: usize,
) {
    for y in 0..rect_h {
        let dst_row = &mut dst[y * dst_stride..];
        let src_row = &src[y * src_stride..];
        let mask_row = &mask[y * mask_stride..];

        for x in 0..rect_w {
            let m = mask_row[x];
            if m == 0 {
                continue;
            }

            let s = src_row[x];
            let sa_orig = ((s >> 24) & 0xFF) as u8;
            let sr_orig = ((s >> 16) & 0xFF) as u8;
            let sg_orig = ((s >> 8) & 0xFF) as u8;
            let sb_orig = (s & 0xFF) as u8;

            // Modulate source by mask
            let sa = scale_ch(sa_orig, m);
            let sr = scale_ch(sr_orig, m);
            let sg = scale_ch(sg_orig, m);
            let sb = scale_ch(sb_orig, m);

            if sa == 255 {
                dst_row[x] = (sa << 24) | (sr << 16) | (sg << 8) | sb;
            } else if sa > 0 {
                let dv = dst_row[x];
                let da = (dv >> 24) & 0xFF;
                let dr = (dv >> 16) & 0xFF;
                let dg = (dv >> 8) & 0xFF;
                let db = dv & 0xFF;

                let out_a = sa + scale_ch(da as u8, (255 - sa) as u8);
                let out_r = blend_channel(sr, dr, sa);
                let out_g = blend_channel(sg, dg, sa);
                let out_b = blend_channel(sb, db, sa);

                dst_row[x] = (out_a << 24) | (out_r << 16) | (out_g << 8) | out_b;
            }
        }
    }
}
