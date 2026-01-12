/// Pixel formats supported by the software presenter.
extern crate alloc;

use alloc::vec::Vec;

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

pub struct BackBuffer {
    width: usize,
    height: usize,
    stride_bytes: usize,
    format: PixelFormat,
    buf: Option<Vec<u8>>,
    len: usize,
}

impl BackBuffer {
    pub fn new(width: usize, height: usize, stride_bytes: usize, format: PixelFormat) -> Self {
        let len = stride_bytes.saturating_mul(height);
        let mut buf = Vec::new();
        let owned = if buf.try_reserve_exact(len).is_ok() {
            buf.resize(len, 0);
            Some(buf)
        } else {
            None
        };
        Self {
            width,
            height,
            stride_bytes,
            format,
            buf: owned,
            len,
        }
    }

    pub fn surface_mut_with_fallback(&mut self, fallback: *mut u8) -> Surface<'_> {
        if let Some(buf) = self.buf.as_mut() {
            Surface {
                width: self.width,
                height: self.height,
                stride_bytes: self.stride_bytes,
                format: self.format,
                buf,
            }
        } else {
            let slice = unsafe { core::slice::from_raw_parts_mut(fallback, self.len) };
            Surface {
                width: self.width,
                height: self.height,
                stride_bytes: self.stride_bytes,
                format: self.format,
                buf: slice,
            }
        }
    }

    pub fn buf(&self) -> Option<&[u8]> {
        self.buf.as_deref()
    }

    pub fn has_buffer(&self) -> bool {
        self.buf.is_some()
    }
}
