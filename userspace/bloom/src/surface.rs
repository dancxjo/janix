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
//!
//! # Memory Initialization
//!
//! All surfaces should be explicitly initialized before use. Use:
//! - [`Surface::zeroed()`] for transparent surfaces
//! - [`Surface::poisoned()`] for debug mode (bright magenta)
//! - [`Surface::clear()`] to reset an existing surface

use abi::pixel::PixelFormat;

/// Debug poison color: bright magenta (0xFFCD00CD) - immediately visible.
/// Memory byte pattern 0xCD is chosen to match classic debug heap patterns.
pub const POISON_COLOR: u32 = 0xFFCD00CD;

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
    /// - **Memory is NOT initialized.** Call `clear()` or use `zeroed()` instead.
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

    /// Create a surface and zero-fill it (transparent black).
    ///
    /// This is the preferred constructor for drawable surfaces.
    ///
    /// # Safety
    ///
    /// - `ptr` must point to valid, writable memory of at least `len` bytes.
    /// - The memory must remain valid for the lifetime of the Surface.
    pub unsafe fn zeroed(
        ptr: *mut u8,
        len: usize,
        width: u32,
        height: u32,
        stride_bytes: u32,
    ) -> Self {
        core::ptr::write_bytes(ptr, 0, len);
        Self::new(ptr, len, width, height, stride_bytes)
    }

    /// Create a surface and fill with poison pattern (bright magenta).
    ///
    /// Use this in debug builds to detect uninitialized memory during rendering.
    /// Any visible magenta indicates a codepath that didn't properly paint its region.
    ///
    /// # Safety
    ///
    /// Same requirements as [`Self::new`].
    pub unsafe fn poisoned(
        ptr: *mut u8,
        len: usize,
        width: u32,
        height: u32,
        stride_bytes: u32,
    ) -> Self {
        // Fill with 0xCD byte pattern (like classic debug heaps)
        core::ptr::write_bytes(ptr, 0xCD, len);
        Self::new(ptr, len, width, height, stride_bytes)
    }

    /// Clear the entire surface to transparent black (0x00000000).
    ///
    /// This should be called before painting if the surface may contain
    /// stale data from previous frames.
    pub fn clear(&mut self) {
        unsafe {
            core::ptr::write_bytes(self.ptr, 0, self.len);
        }
    }

    /// Fill the entire surface with the debug poison color (bright magenta).
    ///
    /// Use this to detect areas that aren't being painted.
    #[allow(dead_code)]
    pub fn poison(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_px(x, y, POISON_COLOR);
            }
        }
    }

    /// Update the backing buffer of this surface.
    ///
    /// Used for swapchain buffer rotation where we reuse the Surface struct
    /// but point it to a different buffer.
    ///
    /// # Safety
    ///
    /// - `ptr` must point to valid, writable memory of at least `size` bytes.
    /// - The new memory must remain valid for the lifetime of the Surface.
    pub unsafe fn update_buffer(
        &mut self,
        ptr: *mut u8,
        size: usize,
        width: u32,
        height: u32,
        stride_bytes: u32,
    ) {
        self.ptr = ptr;
        self.len = size;
        self.width = width as i32;
        self.height = height as i32;
        self.stride_bytes = stride_bytes as usize;
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn surface_zeroed_is_transparent() {
        let mut buf = vec![0xFFu8; 100 * 100 * 4]; // Start with opaque white
        let surface = unsafe { Surface::zeroed(buf.as_mut_ptr(), buf.len(), 100, 100, 400) };

        // All pixels should be transparent black (0x00000000)
        for y in 0..100i32 {
            for x in 0..100i32 {
                assert_eq!(
                    surface.get_px(x, y),
                    0,
                    "pixel at ({}, {}) not transparent",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn surface_clear_resets_to_transparent() {
        let mut buf = vec![0xFFu8; 10 * 10 * 4];
        let mut surface = unsafe { Surface::new(buf.as_mut_ptr(), buf.len(), 10, 10, 40) };

        // Write some pixels
        surface.put_px(0, 0, 0xFFFF0000);
        surface.put_px(5, 5, 0xFF00FF00);

        // Clear
        surface.clear();

        // All should be transparent
        assert_eq!(surface.get_px(0, 0), 0);
        assert_eq!(surface.get_px(5, 5), 0);
    }

    #[test]
    fn surface_poisoned_has_poison_pattern() {
        let mut buf = vec![0u8; 10 * 10 * 4];
        let _surface = unsafe { Surface::poisoned(buf.as_mut_ptr(), buf.len(), 10, 10, 40) };

        // Memory should be filled with 0xCD bytes
        for byte in &buf {
            assert_eq!(*byte, 0xCD, "byte should be 0xCD poison pattern");
        }
    }
}
