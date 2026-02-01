#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb888,
    Bgr888,
    Bgrx8888, // 32-bit BGR with padding (Blue is at offset 0)
    Rgbx8888, // 32-bit RGB with padding (Red is at offset 0)
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct FramebufferInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32, // bytes per row (pitch)
    pub format: PixelFormat,
}

pub trait FramebufferTarget {
    fn info(&self) -> FramebufferInfo;
    fn buffer_mut(&mut self) -> &mut [u8];
    fn clear(&mut self, color: u32);
}
