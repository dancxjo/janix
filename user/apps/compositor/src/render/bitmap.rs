use thing_models::schema::bitmap::BitmapBody;

pub fn draw_bitmap(
    fb_ptr: *mut u32,
    pitch: u32,
    fb_w: u32,
    fb_h: u32,
    x: i32,
    y: i32,
    bmp: &BitmapBody
) {
    unsafe {
        for dy in 0..bmp.height {
             let sy = y + dy as i32;
             if sy < 0 || sy >= fb_h as i32 { continue; }

             for dx in 0..bmp.width {
                 let sx = x + dx as i32;
                 if sx < 0 || sx >= fb_w as i32 { continue; }

                 let i = ((dy * bmp.width + dx) * 4) as usize;
                 if i + 3 >= bmp.pixels.len() { break; }

                 let r = bmp.pixels[i] as u32;
                 let g = bmp.pixels[i+1] as u32;
                 let b = bmp.pixels[i+2] as u32;
                 let a = bmp.pixels[i+3] as u32;

                 if a == 0 { continue; }

                 let offset = (sy as u32 * pitch / 4 + sx as u32) as isize;
                 let dst_val = *fb_ptr.offset(offset);

                 // Blend BGRA output
                 let dr = (dst_val >> 16) & 0xFF;
                 let dg = (dst_val >> 8) & 0xFF;
                 let db = dst_val & 0xFF;

                 let inv_a = 255 - a;

                 let r_out = (r * a + dr * inv_a) / 255;
                 let g_out = (g * a + dg * inv_a) / 255;
                 let b_out = (b * a + db * inv_a) / 255;

                 *fb_ptr.offset(offset) = (0xFF << 24) | (r_out << 16) | (g_out << 8) | b_out;
             }
        }
    }
}
