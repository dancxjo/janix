extern crate alloc;

use alloc::vec::Vec;

#[derive(Debug)]
pub struct WallpaperSurface {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

impl WallpaperSurface {
    pub fn checkerboard() -> Self {
        let width = 32u32;
        let height = 32u32;
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                let dark = ((x / 8 + y / 8) & 1) == 0;
                let color = if dark { 0x0022262C } else { 0x00323943 };
                pixels.push(color);
            }
        }
        Self {
            width,
            height,
            pixels,
        }
    }
}

pub fn decode_bmp(bytes: &[u8]) -> Option<WallpaperSurface> {
    if bytes.len() < 54 {
        return None;
    }
    if &bytes[0..2] != b"BM" {
        return None;
    }
    let data_offset = u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]) as usize;
    let header_size = u32::from_le_bytes([bytes[14], bytes[15], bytes[16], bytes[17]]);
    if header_size < 40 {
        return None;
    }

    let width = i32::from_le_bytes([bytes[18], bytes[19], bytes[20], bytes[21]]);
    let height = i32::from_le_bytes([bytes[22], bytes[23], bytes[24], bytes[25]]);
    let planes = u16::from_le_bytes([bytes[26], bytes[27]]);
    let bpp = u16::from_le_bytes([bytes[28], bytes[29]]);
    let compression = u32::from_le_bytes([bytes[30], bytes[31], bytes[32], bytes[33]]);

    if planes != 1 || compression != 0 {
        return None;
    }
    if bpp != 24 && bpp != 32 {
        return None;
    }

    let width_u = if width < 0 { (-width) as u32 } else { width as u32 };
    let height_u = if height < 0 { (-height) as u32 } else { height as u32 };
    if width_u == 0 || height_u == 0 {
        return None;
    }

    let row_stride = ((width_u * bpp as u32 + 31) / 32 * 4) as usize;
    if data_offset >= bytes.len() {
        return None;
    }
    let pixel_data = &bytes[data_offset..];
    let mut pixels = Vec::new();
    let pixel_len = (width_u as usize).saturating_mul(height_u as usize);
    if pixels.try_reserve_exact(pixel_len).is_err() {
        return None;
    }

    let bottom_up = height > 0;
    for row in 0..height_u {
        let src_row = if bottom_up {
            height_u - 1 - row
        } else {
            row
        } as usize;
        let row_start = src_row * row_stride;
        for col in 0..width_u as usize {
            let offset = row_start + col * (bpp as usize / 8);
            if offset + (bpp as usize / 8) > pixel_data.len() {
                pixels.push(0);
                continue;
            }
            let b = pixel_data[offset];
            let g = pixel_data[offset + 1];
            let r = pixel_data[offset + 2];
            pixels.push(((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
        }
    }

    Some(WallpaperSurface {
        width: width_u,
        height: height_u,
        pixels,
    })
}
