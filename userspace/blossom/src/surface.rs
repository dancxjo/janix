/// Pixel formats supported by the software presenter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PixelFormat {
    Xrgb8888,
    Argb8888,
}

/// A raw pixel buffer surface to render into.
pub struct Surface<'a> {
    pub width: usize,
    pub height: usize,
    pub stride_bytes: usize,
    pub format: PixelFormat,
    pub buf: &'a mut [u8],
}

impl<'a> Surface<'a> {
    pub fn required_len(&self) -> usize {
        self.stride_bytes.saturating_mul(self.height)
    }
}
