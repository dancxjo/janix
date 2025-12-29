#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    Xrgb8888,
    Argb8888,
    Abgr8888,
    Bgra8888,
}

impl PixelFormat {
    /// Input: 0xAARRGGBB
    /// Output: u32 whose LE bytes are in framebuffer byte order
    #[inline(always)]
    pub fn pack_le_bytes(self, argb: u32) -> u32 {
        let a = ((argb >> 24) & 0xFF) as u8;
        let r = ((argb >> 16) & 0xFF) as u8;
        let g = ((argb >> 8) & 0xFF) as u8;
        let b = (argb & 0xFF) as u8;

        match self {
            // Memory: [B, G, R, X] or [B, G, R, A]
            PixelFormat::Xrgb8888 => u32::from_le_bytes([b, g, r, 0xFF]),
            PixelFormat::Argb8888 => u32::from_le_bytes([b, g, r, a]),

            // Memory: [R, G, B, A]
            PixelFormat::Abgr8888 => u32::from_le_bytes([r, g, b, a]),

            // Memory: [B, G, R, A]
            PixelFormat::Bgra8888 => u32::from_le_bytes([b, g, r, a]),
        }
    }
}
