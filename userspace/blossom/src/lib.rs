#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod present;
mod surface;

pub use present::render;
pub use surface::{PixelFormat, Surface};

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::{render, PixelFormat, Surface};

    #[test]
    fn demo_drawlist_changes_center_pixel() {
        let width = 64usize;
        let height = 48usize;
        let stride = width * 4;
        let mut buf = vec![0u8; stride * height];

        let list = bloom::demo_drawlist(width as i32, height as i32);
        let mut surface = Surface {
            width,
            height,
            stride_bytes: stride,
            format: PixelFormat::Xrgb8888,
            buf: &mut buf,
        };

        render(&mut surface, &list);

        let center = (height / 2) * stride + (width / 2) * 4;
        let pixel = u32::from_le_bytes([
            surface.buf[center],
            surface.buf[center + 1],
            surface.buf[center + 2],
            surface.buf[center + 3],
        ]);

        assert_ne!(pixel, 0x00101018);
    }
}
