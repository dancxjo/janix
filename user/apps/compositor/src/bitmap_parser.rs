// use alloc::vec::Vec;
use thing_models::schema::bitmap::BitmapBody;

pub fn parse_bmp(data: &[u8]) -> Option<BitmapBody> {
    if data.len() < 54 || &data[0..2] != b"BM" { return None; }
    let pixel_offset = u32::from_le_bytes(data[10..14].try_into().ok()?) as usize;
    let width = i32::from_le_bytes(data[18..22].try_into().ok()?) as u32;
    let height = i32::from_le_bytes(data[22..26].try_into().ok()?);
    let bpp = u16::from_le_bytes(data[28..30].try_into().ok()?);

    if bpp != 32 { return None; }

    let height_abs = height.abs() as u32;
    let row_size = (width * 4) as usize;
    let pixels_len = (width * height_abs * 4) as usize;

    if data.len() < pixel_offset + pixels_len { return None; }

    let mut pixels = alloc::vec![0u8; pixels_len];
    let src_pixels = &data[pixel_offset..];

    for y in 0..height_abs {
        let src_row_idx = if height > 0 {
            (height_abs - 1 - y) as usize
        } else {
            y as usize
        };

        let src_start = src_row_idx * row_size;
        let dst_start = (y as usize) * row_size;

        if src_start + row_size <= src_pixels.len() && dst_start + row_size <= pixels.len() {
             pixels[dst_start..dst_start+row_size].copy_from_slice(&src_pixels[src_start..src_start+row_size]);
        }
    }

    Some(BitmapBody {
        width,
        height: height_abs,
        format: 0,
        pixels
    })
}
