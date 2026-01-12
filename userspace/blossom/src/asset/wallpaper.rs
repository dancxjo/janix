extern crate alloc;

use alloc::vec::Vec;

#[derive(Debug)]
pub enum DecodeError {
    ShortFile,
    BadSignature,
    UnsupportedHeader,
    UnsupportedPlanes(u16),
    UnsupportedCompression(u32),
    UnsupportedBpp(u16),
    EmptyDimensions,
    OutOfBounds,
    Oom,
}

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

    pub fn error_fallback() -> Self {
        let width = 64u32;
        let height = 64u32;
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                let dark = ((x / 8 + y / 8) & 1) == 0;
                let mut color = if dark { 0x002A2A2A } else { 0x00404040 };
                if x == y || x + y == width - 1 {
                    color = 0x00B03030;
                }
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

pub fn decode_bmp(bytes: &[u8]) -> Result<WallpaperSurface, DecodeError> {
    if bytes.len() < 54 {
        return Err(DecodeError::ShortFile);
    }
    if &bytes[0..2] != b"BM" {
        return Err(DecodeError::BadSignature);
    }
    let data_offset = u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]) as usize;
    let header_size = u32::from_le_bytes([bytes[14], bytes[15], bytes[16], bytes[17]]);
    if header_size < 40 {
        return Err(DecodeError::UnsupportedHeader);
    }

    let width = i32::from_le_bytes([bytes[18], bytes[19], bytes[20], bytes[21]]);
    let height = i32::from_le_bytes([bytes[22], bytes[23], bytes[24], bytes[25]]);
    let planes = u16::from_le_bytes([bytes[26], bytes[27]]);
    let bpp = u16::from_le_bytes([bytes[28], bytes[29]]);
    let compression = u32::from_le_bytes([bytes[30], bytes[31], bytes[32], bytes[33]]);

    if planes != 1 {
        return Err(DecodeError::UnsupportedPlanes(planes));
    }
    if compression != 0 {
        return Err(DecodeError::UnsupportedCompression(compression));
    }
    if bpp != 24 && bpp != 32 {
        return Err(DecodeError::UnsupportedBpp(bpp));
    }

    let width_u = if width < 0 { (-width) as u32 } else { width as u32 };
    let height_u = if height < 0 { (-height) as u32 } else { height as u32 };
    if width_u == 0 || height_u == 0 {
        return Err(DecodeError::EmptyDimensions);
    }

    let row_stride = ((width_u * bpp as u32 + 31) / 32 * 4) as usize;
    if data_offset >= bytes.len() {
        return Err(DecodeError::OutOfBounds);
    }
    let pixel_data = &bytes[data_offset..];
    let target_w = width_u.min(256);
    let target_h = height_u.min(256);
    let mut pixels = Vec::new();
    let pixel_len = (target_w as usize).saturating_mul(target_h as usize);
    if pixels.try_reserve_exact(pixel_len).is_err() {
        return Err(DecodeError::Oom);
    }

    let bottom_up = height > 0;
    let bytes_per_px = (bpp as usize / 8).max(3);
    for y in 0..target_h {
        let src_y = (y as u64 * height_u as u64 / target_h as u64) as u32;
        let src_row = if bottom_up {
            height_u - 1 - src_y
        } else {
            src_y
        } as usize;
        let row_start = src_row * row_stride;
        for x in 0..target_w {
            let src_x = (x as u64 * width_u as u64 / target_w as u64) as usize;
            let offset = row_start + src_x * bytes_per_px;
            if offset + 3 > pixel_data.len() {
                pixels.push(0);
                continue;
            }
            let b = pixel_data[offset];
            let g = pixel_data[offset + 1];
            let r = pixel_data[offset + 2];
            pixels.push(((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
        }
    }

    Ok(WallpaperSurface {
        width: target_w,
        height: target_h,
        pixels,
    })
}
