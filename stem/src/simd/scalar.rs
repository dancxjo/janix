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
