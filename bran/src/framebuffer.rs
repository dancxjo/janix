use embedded_graphics::{
    prelude::*,
    Pixel,
    pixelcolor::Rgb888,
    primitives::Rectangle,
};

pub struct Framebuffer {
    buffer: &'static mut [u32],
    width: usize,
    height: usize,
    pitch: usize, // in bytes
}

impl Framebuffer {
    pub fn new(framebuffer: &limine::framebuffer::Framebuffer) -> Self {
        let width = framebuffer.width() as usize;
        let height = framebuffer.height() as usize;
        let pitch = framebuffer.pitch() as usize;
        let bpp = framebuffer.bpp() as usize;
        
        // Enforce 32bpp for now as per design requirements
        assert!(bpp == 32, "Bran only supports 32bpp framebuffers");
        
        let buffer = unsafe {
            core::slice::from_raw_parts_mut(
                framebuffer.addr().cast::<u32>(),
                (pitch * height) / 4,
            )
        };
        
        Self {
            buffer,
            width,
            height,
            pitch,
        }
    }

    /// Fast clear of the screen with a solid packed color (0x00RRGGBB).
    pub fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            let row_start = (y * self.pitch) / 4;
            // width is in pixels. row_end is row_start + width
            // Be careful with pitch vs width. pitch is bytes.
             let row_end = row_start + self.width;
             
             if row_end <= self.buffer.len() {
                 self.buffer[row_start..row_end].fill(color);
             }
        }
    }
}

impl OriginDimensions for Framebuffer {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl DrawTarget for Framebuffer {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32 {
                 let x = point.x as usize;
                 let y = point.y as usize;
                 let offset = (y * self.pitch + x * 4) / 4;
                 let color_u32 = ((color.r() as u32) << 16) | ((color.g() as u32) << 8) | (color.b() as u32);
                 self.buffer[offset] = color_u32;
            }
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let intersection = area.intersection(&self.bounding_box());
        
        if intersection.size == Size::zero() {
            return Ok(());
        }

        let color_u32 = ((color.r() as u32) << 16) | ((color.g() as u32) << 8) | (color.b() as u32);

        let min_x = intersection.top_left.x as usize;
        let min_y = intersection.top_left.y as usize;
        let max_x = (intersection.top_left.x as usize) + (intersection.size.width as usize);
        let max_y = (intersection.top_left.y as usize) + (intersection.size.height as usize);

        for y in min_y..max_y {
            let row_start = (y * self.pitch + min_x * 4) / 4;
            let row_end = (y * self.pitch + max_x * 4) / 4;
            self.buffer[row_start..row_end].fill(color_u32);
        }
        
        Ok(())
    }
}
