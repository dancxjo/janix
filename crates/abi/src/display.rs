#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Argb8888 = 0,
    Rgba8888 = 1,
    Bgra8888 = 2,
    Rgb888 = 3,
    XRGB8888 = 4, // Added
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DisplayInfo {
    pub bytespace: crate::ids::ThingId,
    pub byte_len: u64,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub format: u32, // PixelFormat discriminant
}
