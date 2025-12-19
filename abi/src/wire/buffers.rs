#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PixelFormat {
    Rgba8888 = 0,
    Bgra8888 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SharedBufferInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: PixelFormat,
}
