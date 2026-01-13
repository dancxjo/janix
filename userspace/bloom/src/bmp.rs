extern crate alloc;
use alloc::vec::Vec;
use alloc::sync::Arc;
// use crate::asset::Image;

pub struct BmpImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

#[derive(Debug)]
pub enum BmpError {
    InvalidHeader,
    UnsupportedDepth(u16),
    UnsupportedCompression(u32),
    InvalidSize,
}

// Minimal BITMAPINFOHEADER struct layout helper
// OFFSET 0x00: "BM"
// OFFSET 0x0A: Data Offset
// OFFSET 0x0E: InfoHeader Size (usually 40)
// OFFSET 0x12: Width (i32)
// OFFSET 0x16: Height (i32)
// OFFSET 0x1A: Planes (u16)
// OFFSET 0x1C: BitCount (u16)
// OFFSET 0x1E: Compression (u32)
// OFFSET 0x22: ImageSize (u32)

pub fn decode(bytes: &[u8]) -> Result<BmpImage, BmpError> {
    if bytes.len() < 54 {
        return Err(BmpError::InvalidHeader);
    }

    if &bytes[0..2] != b"BM" {
        return Err(BmpError::InvalidHeader);
    }

    let data_offset = u32::from_le_bytes(bytes[10..14].try_into().unwrap()) as usize;
    let header_size = u32::from_le_bytes(bytes[14..18].try_into().unwrap());
    
    if header_size < 40 {
        return Err(BmpError::InvalidHeader);
    }

    let width = i32::from_le_bytes(bytes[18..22].try_into().unwrap());
    let height = i32::from_le_bytes(bytes[22..26].try_into().unwrap());
    let planes = u16::from_le_bytes(bytes[26..28].try_into().unwrap());
    let bit_count = u16::from_le_bytes(bytes[28..30].try_into().unwrap());
    let compression = u32::from_le_bytes(bytes[30..34].try_into().unwrap());

    if planes != 1 { return Err(BmpError::InvalidHeader); }
    if compression != 0 { return Err(BmpError::UnsupportedCompression(compression)); } // BI_RGB only

    let w = width.abs() as usize;
    let h = height.abs() as usize;
    let top_down = height < 0;

    let bytes_per_pixel = match bit_count {
        24 => 3,
        32 => 4,
        d => return Err(BmpError::UnsupportedDepth(d)),
    };

    let row_stride = (w * bytes_per_pixel + 3) & !3; // Align to 4 bytes
    let pixel_data_len = row_stride * h;
    
    if bytes.len() < data_offset + pixel_data_len {
        return Err(BmpError::InvalidSize);
    }

    let mut pixels = Vec::with_capacity(w * h);

    for y in 0..h {
        let src_y = if top_down { y } else { h - 1 - y };
        let offset = data_offset + src_y * row_stride;
        let row_data = &bytes[offset..offset + w * bytes_per_pixel];

        match bit_count {
            32 => {
                 // BGRA -> XRGB (Assuming alpha is ignored or simple)
                 // Windows BMP 32-bit usually BGRA or BGRX
                 for chunk in row_data.chunks_exact(4) {
                     let b = chunk[0] as u32;
                     let g = chunk[1] as u32;
                     let r = chunk[2] as u32;
                     // let a = chunk[3] as u32;
                     pixels.push(0xFF000000 | (r << 16) | (g << 8) | b);
                 }
            },
            24 => {
                 // BGR -> XRGB
                 for chunk in row_data.chunks_exact(3) {
                     let b = chunk[0] as u32;
                     let g = chunk[1] as u32;
                     let r = chunk[2] as u32;
                     pixels.push(0xFF000000 | (r << 16) | (g << 8) | b);
                 }
            },
            _ => unreachable!(),
        }
    }

    Ok(BmpImage {
        width: w as u32,
        height: h as u32,
        pixels,
    })
}
