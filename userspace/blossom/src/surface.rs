//! Surface helpers for Blossom's CPU painter.
//!
//! These helpers provide safe-ish wrappers over mapped bytespaces.
//!
//! # Examples
//! ```
//! use blossom::surface::{MappedSurface, SurfaceSpec};
//!
//! let mut buf = [0u8; 16];
//! let spec = SurfaceSpec { width: 2, height: 2, stride_bytes: 8 };
//! let mut surface = unsafe { MappedSurface::from_parts(buf.as_mut_ptr(), buf.len(), spec) };
//! surface.clear(0xFF000000);
//! ```

use core::ptr::NonNull;

/// Width/height/stride metadata for a pixel surface.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceSpec {
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
}

/// A mapped bytespace surface (RGBA8888).
#[derive(Debug)]
pub struct MappedSurface {
    ptr: NonNull<u8>,
    len: usize,
    spec: SurfaceSpec,
}

impl MappedSurface {
    /// Create a surface from raw parts.
    ///
    /// # Safety
    /// Caller must ensure the pointer and length are valid for the duration
    /// of the surface usage.
    pub unsafe fn from_parts(ptr: *mut u8, len: usize, spec: SurfaceSpec) -> Self {
        let ptr = NonNull::new(ptr).unwrap_or_else(|| NonNull::dangling());
        Self { ptr, len, spec }
    }

    pub fn spec(&self) -> SurfaceSpec {
        self.spec
    }

    pub fn ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    pub fn clear(&mut self, rgba: u32) {
        let bytes = rgba.to_le_bytes();
        for y in 0..self.spec.height as usize {
            let row = y * self.spec.stride_bytes as usize;
            for x in 0..self.spec.width as usize {
                let offset = row + x * 4;
                if offset + 4 <= self.len {
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            bytes.as_ptr(),
                            self.ptr.as_ptr().add(offset),
                            4,
                        );
                    }
                }
            }
        }
    }

    pub fn put_px(&mut self, x: i32, y: i32, rgba: u32) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.spec.width as usize || y >= self.spec.height as usize {
            return;
        }
        let offset = y * self.spec.stride_bytes as usize + x * 4;
        if offset + 4 > self.len {
            return;
        }
        let bytes = rgba.to_le_bytes();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.as_ptr().add(offset), 4);
        }
    }

    pub fn get_px(&self, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 {
            return 0;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.spec.width as usize || y >= self.spec.height as usize {
            return 0;
        }
        let offset = y * self.spec.stride_bytes as usize + x * 4;
        if offset + 4 > self.len {
            return 0;
        }
        unsafe {
            let mut bytes = [0u8; 4];
            core::ptr::copy_nonoverlapping(self.ptr.as_ptr().add(offset), bytes.as_mut_ptr(), 4);
            u32::from_le_bytes(bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    #[test]
    fn clear_sets_pixels() {
        let mut buf = [0u8; 16];
        let spec = SurfaceSpec {
            width: 2,
            height: 2,
            stride_bytes: 8,
        };
        let mut surface = unsafe { MappedSurface::from_parts(buf.as_mut_ptr(), buf.len(), spec) };
        surface.clear(0xFF112233);
        assert_eq!(surface.get_px(0, 0), 0xFF112233);
        assert_eq!(surface.get_px(1, 1), 0xFF112233);
    }
}
