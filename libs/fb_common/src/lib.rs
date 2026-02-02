#![no_std]

/// Simple pixel format list shared between boot-time drawing code and
/// framebuffer helpers. Byte order is little-endian in memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb888,     // [R, G, B]
    Bgr888,     // [B, G, R]
    Bgrx8888,   // [B, G, R, X]
    Rgbx8888,   // [R, G, B, X]
    Rgb565,     // 16-bit 5:6:5, little-endian
    Unknown,
}

impl PixelFormat {
    #[inline]
    pub const fn bytes_per_pixel(self) -> u32 {
        match self {
            PixelFormat::Bgrx8888 | PixelFormat::Rgbx8888 => 4,
            PixelFormat::Rgb888 | PixelFormat::Bgr888 => 3,
            PixelFormat::Rgb565 => 2,
            PixelFormat::Unknown => 0,
        }
    }
}

/// Normalize a reported stride (pitch).
///
/// - `reported_stride` is assumed to be bytes if >= width * bpp.
/// - If smaller, treat it as pixels-per-row and scale by bpp.
/// - Fallback to tight packing if 0 or still smaller than the row payload.
#[inline]
pub const fn calc_stride_bytes(width: u32, bpp: u32, reported_stride: u32) -> u32 {
    let row_bytes = width.saturating_mul(bpp);

    if reported_stride == 0 {
        return row_bytes;
    }

    if reported_stride >= row_bytes {
        return reported_stride;
    }

    let scaled = reported_stride.saturating_mul(bpp);
    if scaled >= row_bytes {
        scaled
    } else {
        row_bytes
    }
}
