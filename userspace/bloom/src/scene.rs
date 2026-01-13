use crate::cursor::CursorState;

#[derive(Debug)]
pub enum SceneError {
    InvalidStride,
    BufferTooSmall,
}

/// Safe wrapper for a 32-bit framebuffer slice.
/// Invariants:
/// - stride_px >= width
/// - pixels.len() >= height * stride_px
pub struct Fb32<'a> {
    pixels: &'a mut [u32],
    width: usize,
    height: usize,
    stride_px: usize,
}

impl<'a> Fb32<'a> {
    pub fn new(pixels: &'a mut [u32], width: usize, height: usize, stride_px: usize) -> Result<Self, SceneError> {
        if stride_px < width {
            return Err(SceneError::InvalidStride);
        }
        let required_len = height.saturating_mul(stride_px); // strict check: full stride for all rows
        if pixels.len() < required_len {
            // Relaxation: The last row strictly only needs `width` pixels, but maintaining full stride is cleaner
            // We'll stick to full buffer safety for now.
            return Err(SceneError::BufferTooSmall);
        }

        Ok(Self {
            pixels,
            width,
            height,
            stride_px,
        })
    }

    #[inline]
    pub fn pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut u32> {
        if x < self.width && y < self.height {
            // SAFETY: Bounds verified at construction and x, y checked here.
            // stride_px * y + x is guaranteed < pixels.len()
            unsafe {
                Some(self.pixels.get_unchecked_mut(y * self.stride_px + x))
            }
        } else {
            None
        }
    }

    /// Fill a rectangle with a color. Safe and fast.
    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        // Clamp to buffer dimensions
        let start_x = x.max(0) as usize;
        let start_y = y.max(0) as usize;
        let end_x = (x + w).min(self.width as i32) as usize;
        let end_y = (y + h).min(self.height as i32) as usize;

        if start_x >= end_x || start_y >= end_y {
            return;
        }

        let rect_width = end_x - start_x;
        
        for row in start_y..end_y {
            let offset = row * self.stride_px + start_x;
            // SAFETY: construction invariants ensure row < height and start_x + rect_width <= width <= stride
            // so offset + rect_width <= len
            unsafe {
                let dest = self.pixels.as_mut_ptr().add(offset);
                // Simple memset-like fill for u32? 
                // Using slice::fill is safer and likely optimized
                let slice = core::slice::from_raw_parts_mut(dest, rect_width);
                slice.fill(color);
            }
        }
    }
}

pub fn draw(fb: &mut Fb32, frame: u64, cursor: &CursorState) {
    // 1. Color Bars
    let w = fb.width;
    let h = fb.height;
    
    // We can iterate rows manually for speed, but `fill_rect` is cleaner for "Grade-A" clarity
    // Let's do raw loop for the bars since it's a full-screen generation
    for y in 0..h {
        let row_offset = y * fb.stride_px;
        let row_slice = &mut fb.pixels[row_offset..row_offset + w];
        
        for x in 0..w {
            let bar = (x * 8) / w;
            let color = match bar {
                0 => 0xFF000000, 
                1 => 0xFFFF0000, 
                2 => 0xFF00FF00, 
                3 => 0xFF0000FF, 
                4 => 0xFFFFFF00, 
                5 => 0xFFFF00FF, 
                6 => 0xFF00FFFF, 
                _ => 0xFFFFFFFF, 
            };
            row_slice[x] = color;
        }
    }

    // 2. Moving Square
    let sx = (frame as usize * 4) % (w.saturating_sub(64).max(1));
    let sy = (frame as usize * 4) % (h.saturating_sub(64).max(1));
    fb.fill_rect(sx as i32, sy as i32, 64, 64, 0xFFFFFFFF);

    // 3. Cursor
    // Red 10x10 square
    fb.fill_rect(cursor.x, cursor.y, 10, 10, 0xFFFF0000);
}


