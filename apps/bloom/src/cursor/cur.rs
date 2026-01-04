//! .cur (Windows Cursor) file parser

extern crate alloc;
use alloc::vec::Vec;
use super::CursorFrame;

/// Parse a .cur file from raw bytes
pub fn load_cur(data: &[u8]) -> Option<CursorFrame> {
    if data.len() < 6 { return None; }

    let file_type = u16::from_le_bytes([data[2], data[3]]);
    let count = u16::from_le_bytes([data[4], data[5]]) as usize;

    if file_type != 2 && file_type != 1 { return None; }
    if count == 0 || data.len() < 6 + count * 16 { return None; }

    let mut best_idx = 0;
    let mut best_bpp = 0u16;
    let mut best_size = 0u32;

    for i in 0..count {
        let entry_offset = 6 + i * 16;
        let entry = &data[entry_offset..entry_offset + 16];
        let width = if entry[0] == 0 { 256u32 } else { entry[0] as u32 };
        let height = if entry[1] == 0 { 256u32 } else { entry[1] as u32 };
        let size = width * height;
        let img_offset = u32::from_le_bytes([entry[12], entry[13], entry[14], entry[15]]) as usize;
        let img_size = u32::from_le_bytes([entry[8], entry[9], entry[10], entry[11]]) as usize;
        let bpp = if img_offset + 16 <= data.len() {
            detect_bpp(&data[img_offset..data.len().min(img_offset + img_size)])
        } else { 0 };
        if bpp > best_bpp || (bpp == best_bpp && size > best_size) {
            best_bpp = bpp;
            best_size = size;
            best_idx = i;
        }
    }

    let entry_offset = 6 + best_idx * 16;
    let entry = &data[entry_offset..entry_offset + 16];
    let width = if entry[0] == 0 { 256u32 } else { entry[0] as u32 };
    let height = if entry[1] == 0 { 256u32 } else { entry[1] as u32 };
    let hotspot_x = u16::from_le_bytes([entry[4], entry[5]]) as i32;
    let hotspot_y = u16::from_le_bytes([entry[6], entry[7]]) as i32;
    let img_offset = u32::from_le_bytes([entry[12], entry[13], entry[14], entry[15]]) as usize;
    let img_size = u32::from_le_bytes([entry[8], entry[9], entry[10], entry[11]]) as usize;

    if img_offset + img_size > data.len() { return None; }
    let img_data = &data[img_offset..img_offset + img_size];

    if img_data.len() >= 8 && &img_data[0..8] == &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        return None;
    }

    let pixels = decode_dib(img_data, width, height)?;
    Some(CursorFrame::new(pixels, width, height, hotspot_x, hotspot_y))
}

fn detect_bpp(data: &[u8]) -> u16 {
    if data.len() >= 8 && &data[0..8] == &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        return 32;
    }
    if data.len() >= 16 { u16::from_le_bytes([data[14], data[15]]) } else { 0 }
}

fn decode_dib(data: &[u8], width: u32, height: u32) -> Option<Vec<u32>> {
    if data.len() < 40 { return None; }

    let header_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let dib_width = i32::from_le_bytes([data[4], data[5], data[6], data[7]]).abs() as u32;
    let dib_height = i32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    let bpp = u16::from_le_bytes([data[14], data[15]]);

    let actual_height = (dib_height.abs() as u32) / 2;
    let use_width = if dib_width > 0 { dib_width } else { width };
    let use_height = if actual_height > 0 { actual_height } else { height };

    let pixel_data_offset = header_size as usize;
    if pixel_data_offset >= data.len() { return None; }
    let pixel_data = &data[pixel_data_offset..];
    let mut pixels = Vec::with_capacity((use_width * use_height) as usize);

    match bpp {
        32 => {
            let row_stride = (use_width * 4) as usize;
            for y in 0..use_height {
                let src_y = use_height - 1 - y;
                let row_start = src_y as usize * row_stride;
                for x in 0..use_width {
                    let px_offset = row_start + (x as usize * 4);
                    if px_offset + 4 > pixel_data.len() {
                        pixels.push(0);
                        continue;
                    }
                    let b = pixel_data[px_offset];
                    let g = pixel_data[px_offset + 1];
                    let r = pixel_data[px_offset + 2];
                    let a = pixel_data[px_offset + 3];
                    let (pr, pg, pb) = premultiply(r, g, b, a);
                    pixels.push(((a as u32) << 24) | ((pr as u32) << 16) | ((pg as u32) << 8) | (pb as u32));
                }
            }
        }
        24 => {
            let row_stride = ((use_width * 3 + 3) / 4 * 4) as usize;
            let xor_size = row_stride * use_height as usize;
            let and_row_stride = ((use_width + 31) / 32 * 4) as usize;
            let and_offset = xor_size;
            for y in 0..use_height {
                let src_y = use_height - 1 - y;
                let xor_row = src_y as usize * row_stride;
                let and_row = and_offset + src_y as usize * and_row_stride;
                for x in 0..use_width {
                    let px_offset = xor_row + (x as usize * 3);
                    let and_byte_idx = and_row + (x as usize / 8);
                    let and_bit = 7 - (x % 8);
                    let is_transparent = if and_byte_idx < pixel_data.len() {
                        (pixel_data[and_byte_idx] >> and_bit) & 1 == 1
                    } else { false };
                    if is_transparent {
                        pixels.push(0);
                    } else if px_offset + 3 <= pixel_data.len() {
                        let b = pixel_data[px_offset];
                        let g = pixel_data[px_offset + 1];
                        let r = pixel_data[px_offset + 2];
                        pixels.push(0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
                    } else {
                        pixels.push(0);
                    }
                }
            }
        }
        _ => {
            for _ in 0..(use_width * use_height) { pixels.push(0); }
        }
    }
    Some(pixels)
}

#[inline]
fn premultiply(r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8) {
    if a == 255 { (r, g, b) }
    else if a == 0 { (0, 0, 0) }
    else {
        let a32 = a as u32;
        (((r as u32 * a32 + 127) / 255) as u8, ((g as u32 * a32 + 127) / 255) as u8, ((b as u32 * a32 + 127) / 255) as u8)
    }
}
