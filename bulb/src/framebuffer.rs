pub use fb_common::PixelFormat;

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
