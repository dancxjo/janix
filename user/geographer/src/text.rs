include!(concat!(env!("OUT_DIR"), "/unifont.rs"));

/// Draw a string of text to the buffer.
/// Color is expected to be 0xAABBGGRR (little endian u32) = R, G, B, A in byte order.
/// e.g. 0xFF00FF00 -> R=00, G=FF, B=00, A=FF (Green)
/// e.g. 0xFFFFFFFF -> R=FF, G=FF, B=FF, A=FF (White)
pub fn draw_text_simple(
    buffer: &mut [u8],
    stride: u32,
    surface_width: u32,
    surface_height: u32,
    x: i32,
    y: i32,
    text: &str,
    color: u32,
) {
    let mut curr_x = x;
    let mut curr_y = y;

    for ch in text.chars() {
        if ch == '\n' {
            curr_x = x;
            curr_y += GLYPH_HEIGHT as i32;
            continue;
        }

        if let Some(glyph) = lookup_glyph(ch) {
            draw_glyph(
                buffer,
                stride,
                surface_width,
                surface_height,
                curr_x,
                curr_y,
                glyph,
                color,
            );
            curr_x += GLYPH_WIDTH as i32;
        } else {
            // Draw box or space for missing glyph?
            curr_x += GLYPH_WIDTH as i32;
        }
    }
}

fn draw_glyph(
    buffer: &mut [u8],
    stride: u32,
    surface_width: u32,
    surface_height: u32,
    x: i32,
    y: i32,
    glyph: &[u8],
    color: u32,
) {
    if x < 0 || x >= surface_width as i32 || y < 0 || y >= surface_height as i32 {
        return; // Clipping simplistic
    }

    // Decompose color for alpha blending if we wanted (but here simple overwrite)
    // Actually, let's treat 0 as transparent in the font
    
    // Convert u32 color to bytes
    let b_r = (color & 0xFF) as u8;
    let b_g = ((color >> 8) & 0xFF) as u8;
    let b_b = ((color >> 16) & 0xFF) as u8;
    let b_a = ((color >> 24) & 0xFF) as u8;

    for row in 0..GLYPH_HEIGHT {
        let py = y + row as i32;
        if py < 0 || py >= surface_height as i32 {
            continue;
        }

        let bits = glyph[row as usize];
        for col in 0..GLYPH_WIDTH {
            let px = x + col as i32;
            if px < 0 || px >= surface_width as i32 {
                continue;
            }

            // Check bit
            if (bits & (0x80 >> col)) != 0 {
                // Set pixel
                let offset = (py as u32 * stride + px as u32 * 4) as usize;
                if offset + 4 <= buffer.len() {
                    buffer[offset] = b_r;
                    buffer[offset + 1] = b_g;
                    buffer[offset + 2] = b_b;
                    buffer[offset + 3] = b_a;
                }
            }
        }
    }
}
