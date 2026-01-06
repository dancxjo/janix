#[derive(Clone, Copy)]
pub struct Wallpaper {
    pub data_ptr: *const u8,
    pub width: u32,
    pub height: u32,
    pub row_stride: usize,
    pub bytes_per_pixel: usize,
    pub bottom_up: bool,
}

pub fn parse_bmp(buf: &[u8]) -> Option<Wallpaper> {
    if buf.len() < 54 || &buf[0..2] != b"BM" {
        return None;
    }
    let data_offset = u32::from_le_bytes([buf[10], buf[11], buf[12], buf[13]]) as usize;
    let width_i = i32::from_le_bytes([buf[18], buf[19], buf[20], buf[21]]);
    let height_i = i32::from_le_bytes([buf[22], buf[23], buf[24], buf[25]]);
    let bpp = u16::from_le_bytes([buf[28], buf[29]]);
    if width_i <= 0 {
        return None;
    }
    let bytes_per_pixel = ((bpp as usize) + 7) / 8;
    let row_stride = (((width_i as usize * bytes_per_pixel) + 3) / 4) * 4;
    
    // Safety check? buf bounds.
    let end_offset = data_offset + (height_i.abs() as usize * row_stride);
    if end_offset > buf.len() {
         return None;
    }

    let data_ptr = unsafe { buf.as_ptr().add(data_offset) };
    Some(Wallpaper {
        data_ptr,
        width: width_i.abs() as u32,
        height: height_i.abs() as u32,
        row_stride,
        bytes_per_pixel,
        bottom_up: height_i > 0,
    })
}

pub fn calculate_dominant_color(wallpaper: &Wallpaper) -> u32 {
    // 5-bit quantization per channel (32 levels) -> 15-bit index
    // 32*32*32 = 32768 buckets
    let mut histogram = alloc::vec![0u32; 32768];
    
    let w = wallpaper.width as usize;
    let h = wallpaper.height as usize;
    let stride = wallpaper.row_stride;
    let bpp = wallpaper.bytes_per_pixel;
    let data_ptr = wallpaper.data_ptr;

    // Sample every Nth pixel to save time? Or full scan?
    // Full scan for small wallpapers is fine. clouds.bmp is small.
    // 128x128 = 16k pixels. Fast enough.
    
    for y in 0..h {
        let row_offset = y * stride;
        for x in 0..w {
            let px_offset = row_offset + x * bpp;
            unsafe {
                let blue = *data_ptr.add(px_offset);
                let green = *data_ptr.add(px_offset + 1);
                let red = *data_ptr.add(px_offset + 2);
                
                let r5 = (red >> 3) as usize;
                let g5 = (green >> 3) as usize;
                let b5 = (blue >> 3) as usize;
                
                let index = (r5 << 10) | (g5 << 5) | b5;
                histogram[index] += 1;
            }
        }
    }
    
    // Find max
    let mut max_count = 0;
    let mut max_index = 0;
    for (i, &count) in histogram.iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_index = i;
        }
    }
    
    // De-quantize to 8-bit (center of bucket)
    // 5 bits: 0..31.  (val << 3) | 4
    let r = ((max_index >> 10) & 0x1F) as u8;
    let g = ((max_index >> 5) & 0x1F) as u8;
    let b = (max_index & 0x1F) as u8;
    
    let r8 = (r << 3) | 4;
    let g8 = (g << 3) | 4;
    let b8 = (b << 3) | 4;
    
    0xFF000000 | ((r8 as u32) << 16) | ((g8 as u32) << 8) | (b8 as u32)
}
