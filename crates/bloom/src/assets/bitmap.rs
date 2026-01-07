use alloc::sync::Arc;
use alloc::collections::BTreeMap;
use serde::{Serialize, Deserialize};

pub struct Bitmap {
    pub w: u32,
    pub h: u32,
    pub pixels: Arc<[u32]>, // ARGB or premultiplied ARGB
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BitmapHandle(pub u64);

pub struct BitmapStore {
    bitmaps: BTreeMap<u64, Bitmap>,
    next_id: u64,
}

impl BitmapStore {
    pub fn new() -> Self {
        Self {
            bitmaps: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, bitmap: Bitmap) -> BitmapHandle {
        let id = self.next_id;
        self.next_id += 1;
        self.bitmaps.insert(id, bitmap);
        BitmapHandle(id)
    }

    pub fn get(&self, handle: BitmapHandle) -> Option<&Bitmap> {
        self.bitmaps.get(&handle.0)
    }
    
    pub fn remove(&mut self, handle: BitmapHandle) {
        self.bitmaps.remove(&handle.0);
    }
}

/// Simple BMP parser for ARGB32.
/// Assumes uncompressed 24/32-bit BMP.
pub fn parse_bmp_to_argb(data: &[u8]) -> Option<(u32, u32, Arc<[u32]>)> {
    if data.len() < 54 || &data[0..2] != b"BM" {
        return None;
    }
    
    let offset = u32::from_le_bytes(data[10..14].try_into().ok()?) as usize;
    let width = u32::from_le_bytes(data[18..22].try_into().ok()?) as u32;
    let height = i32::from_le_bytes(data[22..26].try_into().ok()?) as i32;
    let bpp = u16::from_le_bytes(data[28..30].try_into().ok()?) as u16;
    
    let abs_height = height.abs() as u32;
    let mut pixels = alloc::vec![0u32; (width * abs_height) as usize];
    
    if bpp == 24 {
        let padding = (4 - (width * 3) % 4) % 4;
        for y in 0..abs_height {
            let src_y = if height > 0 { abs_height - 1 - y } else { y };
            let row_start = offset + (src_y * (width * 3 + padding as u32)) as usize;
            for x in 0..width {
                let pixel_start = row_start + (x * 3) as usize;
                let b = data[pixel_start];
                let g = data[pixel_start + 1];
                let r = data[pixel_start + 2];
                pixels[(y * width + x) as usize] = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    } else if bpp == 32 {
        for y in 0..abs_height {
            let src_y = if height > 0 { abs_height - 1 - y } else { y };
            let row_start = offset + (src_y * width * 4) as usize;
            for x in 0..width {
                let pixel_start = row_start + (x * 4) as usize;
                let b = data[pixel_start];
                let g = data[pixel_start + 1];
                let r = data[pixel_start + 2];
                let a = data[pixel_start + 3];
                pixels[(y * width + x) as usize] = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    } else {
        return None;
    }
    
    Some((width, abs_height, Arc::from(pixels)))
}

pub fn compute_dominant_color(pixels: &[u32]) -> u32 {
    if pixels.is_empty() {
        return 0;
    }
    
    let mut r_sum = 0u64;
    let mut g_sum = 0u64;
    let mut b_sum = 0u64;
    
    // Sample every 4th pixel for speed
    let step = 4;
    let mut count = 0;
    for i in (0..pixels.len()).step_by(step) {
        let p = pixels[i];
        r_sum += ((p >> 16) & 0xFF) as u64;
        g_sum += ((p >> 8) & 0xFF) as u64;
        b_sum += (p & 0xFF) as u64;
        count += 1;
    }
    
    if count == 0 { return 0; }
    
    let r = (r_sum / count as u64) as u32;
    let g = (g_sum / count as u64) as u32;
    let b = (b_sum / count as u64) as u32;
    
    0xFF000000 | (r << 16) | (g << 8) | b
}
