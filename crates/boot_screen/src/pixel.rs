#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    Xrgb8888,
    Argb8888,
    Abgr8888,
    Bgra8888,
}

impl PixelFormat {
    /// Convert 0xAARRGGBB to the target format u32.
    #[inline(always)]
    pub fn convert(&self, argb: u32) -> u32 {
        let a = (argb >> 24) & 0xFF;
        let r = (argb >> 16) & 0xFF;
        let g = (argb >> 8) & 0xFF;
        let b = argb & 0xFF;

        match self {
            PixelFormat::Xrgb8888 | PixelFormat::Argb8888 => argb,
            PixelFormat::Abgr8888 => (a << 24) | (b << 16) | (g << 8) | r,
            PixelFormat::Bgra8888 => (b << 24) | (g << 16) | (r << 8) | a,
        }
    }
}
