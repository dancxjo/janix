#[inline(always)]
fn scale_ch(c: u8, a: u8) -> u32 {
    let t = c as u32 * a as u32;
    (t + 1 + (t >> 8)) >> 8
}

pub fn blit_a8_tinted_over_scalar_row(dst: &mut [u32], src_a8: &[u8], color: u32, tint_a: u8) {
    if tint_a == 0 {
        return;
    }

    let len = dst.len().min(src_a8.len());
    let dst = &mut dst[0..len];
    let src = &src_a8[0..len];

    let tr = (color >> 16) & 0xFF;
    let tg = (color >> 8) & 0xFF;
    let tb = color & 0xFF;

    let mut i = 0;
    while i < len {
        let sa = src[i];
        if sa == 0 {
            i += 1;
            continue;
        }

        let ea = if tint_a == 255 {
            sa as u32
        } else {
            scale_ch(sa, tint_a)
        };

        if ea == 255 {
            dst[i] = (255 << 24) | (tr << 16) | (tg << 8) | tb;
        } else {
            let dv = dst[i];
            let da = (dv >> 24) & 0xFF;
            let dr = (dv >> 16) & 0xFF;
            let dg = (dv >> 8) & 0xFF;
            let db = dv & 0xFF;

            let out_a = ea + scale_ch(da as u8, (255 - ea) as u8);

            let blend_channel = |s: u32, d: u32, sa: u32| {
                let inv = 255 - sa;
                let t = s * sa + d * inv;
                (t + 1 + (t >> 8)) >> 8
            };

            let out_r = blend_channel(tr, dr, ea);
            let out_g = blend_channel(tg, dg, ea);
            let out_b = blend_channel(tb, db, ea);

            dst[i] = (out_a << 24) | (out_r << 16) | (out_g << 8) | out_b;
        }
        i += 1;
    }
}
