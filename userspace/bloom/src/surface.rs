//! Bloom Surface - Framebuffer pixel manipulation.
//!
//! # Pixel Format
//!
//! The canonical pixel format is **BGRA8888** (see [`abi::pixel::PixelFormat`]).
//!
//! Memory layout for a 32-bit pixel: `[B, G, R, A]` (4 bytes, little-endian).
//! When read as a `u32`: `0xAARRGGBB`.
//!
//! - Alpha 255 = fully opaque
//! - Alpha 0 = fully transparent
//!
//! All color values passed to `put_px` should be in this format.
//! Use [`abi::pixel::Color`] for safe color construction.

use abi::pixel::PixelFormat;

/// A surface backed by a raw pixel buffer.
pub struct Surface {
    pub ptr: *mut u8,
    pub len: usize,
    pub width: i32,
    pub height: i32,
    pub stride_bytes: usize,
    /// Pixel format of this surface. Default: Bgrx8888 (no alpha).
    pub format: PixelFormat,
}

impl Surface {
    /// Create a new surface from raw framebuffer memory.
    ///
    /// # Safety
    ///
    /// - `ptr` must point to valid, writable memory of at least `len` bytes.
    /// - The memory must remain valid for the lifetime of the Surface.
    pub unsafe fn new(
        ptr: *mut u8,
        len: usize,
        width: u32,
        height: u32,
        stride_bytes: u32,
    ) -> Self {
        Self {
            ptr,
            len,
            width: width as i32,
            height: height as i32,
            stride_bytes: stride_bytes as usize,
            format: PixelFormat::Bgrx8888, // Default to no-alpha format
        }
    }

    /// Create a surface with an explicit pixel format.
    ///
    /// # Safety
    ///
    /// Same requirements as [`Self::new`].
    pub unsafe fn with_format(
        ptr: *mut u8,
        len: usize,
        width: u32,
        height: u32,
        stride_bytes: u32,
        format: PixelFormat,
    ) -> Self {
        Self {
            ptr,
            len,
            width: width as i32,
            height: height as i32,
            stride_bytes: stride_bytes as usize,
            format,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    #[allow(dead_code)]
    pub fn stride_bytes(&self) -> usize {
        self.stride_bytes
    }

    /// Write a pixel in 0xAARRGGBB format.
    ///
    /// The value is written as little-endian bytes `[B, G, R, A]` matching
    /// the canonical BGRA8888 memory layout.
    #[inline]
    pub fn put_px(&mut self, x: i32, y: i32, argb: u32) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        let offset = y as usize * self.stride_bytes + x as usize * 4;
        if offset + 4 > self.len {
            return;
        }
        let bytes = argb.to_le_bytes();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.add(offset), 4);
        }
    }

    /// Read a pixel as 0xAARRGGBB.
    ///
    /// Returns 0 if coordinates are out of bounds.
    #[inline]
    pub fn get_px(&self, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return 0;
        }
        let offset = y as usize * self.stride_bytes + x as usize * 4;
        if offset + 4 > self.len {
            return 0;
        }
        unsafe {
            let mut bytes = [0u8; 4];
            core::ptr::copy_nonoverlapping(self.ptr.add(offset), bytes.as_mut_ptr(), 4);
            u32::from_le_bytes(bytes)
        }
    }
}

