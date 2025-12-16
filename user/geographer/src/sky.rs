use alloc::vec::Vec;
use thing_os::prelude::*;

pub struct Sky {
    width: u32,
    height: u32,
    texture: Vec<u32>, // BGRA8888
}

impl Sky {
    pub fn new() -> Self {
        let width = 256;
        let height = 256;
        let mut texture = Vec::with_capacity((width * height) as usize);
        
        // Simple procedural noise generation
        // We want soft clouds (white/blue-ish) on a blue sky.
        
        for y in 0..height {
            for x in 0..width {
                // Generate some noise
                // We use a simple sum of sines for "good enough" clouds without heavy dependencies
                let fx = x as f32 / width as f32;
                let fy = y as f32 / height as f32;
                
                let n1 = multisine(fx * 4.0, fy * 4.0);
                let n2 = multisine(fx * 8.0 + 0.5, fy * 8.0 + 0.5) * 0.5;
                let n3 = multisine(fx * 16.0 + 0.2, fy * 16.0 + 0.2) * 0.25;
                
                let noise = (n1 + n2 + n3) / 1.75; // Normalize roughly to -1..1 range
                let val = (noise + 1.0) / 2.0; // 0..1
                
                // Color mapping
                // Sky Blue: 0x87CEEB (R=135, G=206, B=235)
                // Cloud White: 0xFFFFFF
                
                // Lerp
                let r = lerp(135.0, 255.0, val) as u32;
                let g = lerp(206.0, 255.0, val) as u32;
                let b = lerp(235.0, 255.0, val) as u32;
                
                // BGRA
                let pixel = 0xFF000000 | (r << 16) | (g << 8) | b;
                texture.push(pixel);
            }
        }
        
        Self { width, height, texture }
    }
    
    pub fn draw(&self, buffer: &mut [u8], stride: u32, view_w: u32, view_h: u32, time_ns: u64) {
        // Scroll speed: 1 pixel per 50ms?
        // time_ns is monotonic. 
        // x_offset = time / SPEED
        let speed_div = 20_000_000; // 20ms per tick
        let offset_x = (time_ns / speed_div) as u32;
        let offset_y = (time_ns / (speed_div * 2)) as u32; // Slower vertical drift
        
        let mut b_ptr = buffer.as_mut_ptr() as *mut u32;
        let stride_u32 = stride / 4;
        
        for y in 0..view_h {
             // Calculate texture Y
             let ty = (y + offset_y) % self.height;
             let row_start = ty * self.width;
             
             // Optimization: We could copy in chunks if view_w > tex_w, but pixel-by-pixel is simplest for wrapping
             // For a 256x256 texture, we just repeat.
             
             // To speed this up, we can pre-calculate the row pointer in texture
             // But we need to handle x wrapping.
             
             // x_start in texture
             let tx_start = offset_x % self.width;
             
             // We can do two memcpy calls effectively per row if we wanted, but let's just loop for now.
             // It's the background, it runs every frame.
             
             /*
             for x in 0..view_w {
                 let tx = (x + offset_x) % self.width;
                 unsafe {
                     *b_ptr.add((y * stride_u32 + x) as usize) = self.texture[(row_start + tx) as usize];
                 }
             }
             */
             
             // Semi-optimized loop
             let dest_row = unsafe { b_ptr.add((y * stride_u32) as usize) };
             let mut tx = tx_start;
             
             for x in 0..view_w {
                 unsafe {
                     *dest_row.add(x as usize) = self.texture[(row_start + tx) as usize];
                 }
                 tx += 1;
                 if tx == self.width { tx = 0; }
             }
        }
    }
}

fn multisine(x: f32, y: f32) -> f32 {
    libm::sinf(x) * libm::cosf(y)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
