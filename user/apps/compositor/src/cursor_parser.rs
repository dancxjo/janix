// .ani and .cur parser
// Based on Microsoft RIFF and CUR/ICO specs

use thing_models::schema::bitmap::BitmapBody;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum CursorType {
    Static(BitmapBody),
    Animated(Vec<Frame>),
}

#[derive(Debug)]
pub struct Frame {
    pub bitmap: BitmapBody,
    pub duration_ms: u32,
}

pub fn parse(data: &[u8]) -> Option<CursorType> {
    if data.len() < 4 { return None; }

    // Check for RIFF (ANI)
    if &data[0..4] == b"RIFF" {
        return parse_ani(data);
    }

    // Check for CUR (Header: Reserved=0, Type=2, Count=...)
    if &data[0..4] == &[0, 0, 2, 0] {
        return parse_cur(data).map(CursorType::Static);
    }

    None
}

fn parse_cur(data: &[u8]) -> Option<BitmapBody> {
    // Header (6 bytes): Reserved(2), Type(2), Count(2)
    let count = u16::from_le_bytes(data[4..6].try_into().ok()?);
    if count == 0 { return None; }

    // Directory entries (16 bytes each)
    // We just take the first one for now, or find the best match.
    // Let's take the first one.
    let dir_entry_offset = 6;
    if data.len() < dir_entry_offset + 16 { return None; }

    let _w = data[dir_entry_offset];
    let _h = data[dir_entry_offset+1];
    // color count etc...
    let size = u32::from_le_bytes(data[dir_entry_offset+8..dir_entry_offset+12].try_into().ok()?);
    let offset = u32::from_le_bytes(data[dir_entry_offset+12..dir_entry_offset+16].try_into().ok()?);

    if data.len() < (offset + size) as usize { return None; }

    let img_data = &data[offset as usize .. (offset + size) as usize];

    // The image data is usually a BMP without the file header (BITMAPINFOHEADER + Bits)
    // Sometimes it's PNG.

    // Check for PNG signature
    if size > 8 && &img_data[0..8] == b"\x89PNG\r\n\x1a\n" {
        // Parsing PNG in no_std is hard without a library.
        // We might be stuck if they are PNG compressed.
        // But rw-designer cursors are usually old school BMP.
        return None;
    }

    // Parse BMP info header
    let header_size = u32::from_le_bytes(img_data[0..4].try_into().ok()?);
    let width = i32::from_le_bytes(img_data[4..8].try_into().ok()?) as u32;
    let height = i32::from_le_bytes(img_data[8..12].try_into().ok()?) as u32; // This is usually 2x height for XOR/AND masks
    let _planes = u16::from_le_bytes(img_data[12..14].try_into().ok()?);
    let bpp = u16::from_le_bytes(img_data[14..16].try_into().ok()?);

    // Only support 32bpp for simplicity
    if bpp != 32 {
        // We might need to handle other depths, but let's see.
        return None;
    }

    let real_height = height / 2; // XOR mask + AND mask
    let pixels_len = (width * real_height * 4) as usize;

    // Start of pixel data depends on header size.
    // Assuming BITMAPINFOHEADER (40 bytes).
    let pixel_start = header_size as usize;

    if img_data.len() < pixel_start + pixels_len { return None; }

    let mut pixels = alloc::vec![0u8; pixels_len];
    let src_pixels = &img_data[pixel_start..];

    // BMP is bottom-up
    for y in 0..real_height {
        let src_row = (real_height - 1 - y) as usize;
        let src_idx = src_row * (width as usize) * 4;
        let dst_idx = (y as usize) * (width as usize) * 4;

        if src_idx + ((width * 4) as usize) <= src_pixels.len() {
             pixels[dst_idx..dst_idx+(width*4) as usize].copy_from_slice(&src_pixels[src_idx..src_idx+(width*4) as usize]);
        }
    }

    Some(BitmapBody {
        width,
        height: real_height,
        format: 0,
        pixels
    })
}

fn parse_ani(data: &[u8]) -> Option<CursorType> {
    // RIFF header
    let _file_len = u32::from_le_bytes(data[4..8].try_into().ok()?);
    if &data[8..12] != b"ACON" { return None; } // Animated Cursor

    // Chunk parsing
    let mut offset = 12;
    let mut frames = Vec::new();
    let mut rates = Vec::new();
    let mut seq = Vec::new();

    while offset + 8 < data.len() {
        let chunk_id = &data[offset..offset+4];
        let chunk_size = u32::from_le_bytes(data[offset+4..offset+8].try_into().ok()?);
        let chunk_size_padded = (chunk_size + 1) & !1; // Word aligned

        let chunk_data_start = offset + 8;
        let chunk_data_end = chunk_data_start + chunk_size as usize;

        if chunk_data_end > data.len() { break; }

        let chunk_data = &data[chunk_data_start..chunk_data_end];

        if chunk_id == b"rate" {
             for i in 0..(chunk_size/4) {
                 let rate = u32::from_le_bytes(chunk_data[i as usize * 4 .. (i+1) as usize * 4].try_into().ok().unwrap_or([0;4]));
                 rates.push(rate);
             }
        } else if chunk_id == b"seq " {
             for i in 0..(chunk_size/4) {
                 let s = u32::from_le_bytes(chunk_data[i as usize * 4 .. (i+1) as usize * 4].try_into().ok().unwrap_or([0;4]));
                 seq.push(s);
             }
        } else if chunk_id == b"LIST" {
             // Subchunks (frames)
             if chunk_data.len() > 4 && &chunk_data[0..4] == b"fram" {
                 let mut sub_offset = 4;
                 while sub_offset + 8 < chunk_data.len() {
                     let sub_id = &chunk_data[sub_offset..sub_offset+4];
                     let sub_size = u32::from_le_bytes(chunk_data[sub_offset+4..sub_offset+8].try_into().ok()?);
                     let sub_padded = (sub_size + 1) & !1;

                     if sub_id == b"icon" {
                         let icon_data = &chunk_data[sub_offset+8 .. sub_offset+8+sub_size as usize];
                         if let Some(_bmp) = parse_cur(icon_data) { // .ani embeds .cur format inside 'icon' chunks usually? Or raw?
                             // Actually "icon" chunk in ANI usually contains a full CUR/ICO file or structure
                             // If it starts with 00 00 02 00 ...
                             if icon_data.len() > 4 && &icon_data[0..4] == &[0,0,2,0] {
                                  if let Some(b) = parse_cur(icon_data) {
                                      frames.push(b);
                                  }
                             } else {
                                  // Sometimes it's just the BITMAP info?
                                  // Let's assume standard ANI format which embeds ICO/CUR
                                  // If parse_cur fails, maybe try direct parsing?
                             }
                         }
                     }
                     sub_offset += 8 + sub_padded as usize;
                 }
             }
        }

        offset += 8 + chunk_size_padded as usize;
    }

    if frames.is_empty() { return None; }

    // Construct sequence
    let mut result_frames = Vec::new();

    // If no rate/seq, assume 1:1 with default rate?
    // Usually rate is Jiffies (1/60s).

    let loop_count = if !seq.is_empty() { seq.len() } else { frames.len() };

    for i in 0..loop_count {
        let frame_idx = if !seq.is_empty() { seq[i] as usize } else { i };
        if frame_idx >= frames.len() { continue; }

        let rate = if i < rates.len() { rates[i] } else if !rates.is_empty() { rates[0] } else { 10 }; // Default ?
        // rate is in jiffies (1/60th of a second)
        let duration_ms = (rate * 1000) / 60;

        result_frames.push(Frame {
            bitmap: frames[frame_idx].clone(), // We need clone for BitmapBody? It contains Vec, so cheap-ish? No, expensive.
            duration_ms
        });
    }

    if result_frames.is_empty() {
        // Fallback
        return Some(CursorType::Static(frames[0].clone()));
    }

    Some(CursorType::Animated(result_frames))
}
