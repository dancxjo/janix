
use crate::display::BootUpDisplay;
use crate::framebuffer::{FramebufferTarget, FramebufferInfo, PixelFormat};
use std::vec;
use std::vec::Vec;
use std::format;
use std::println;

struct MockFb {
    buffer: Vec<u8>,
    width: u32,
    height: u32,
}

impl MockFb {
    fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: vec![0; (width * height * 4) as usize],
            width,
            height,
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let offset = (y * self.width * 4 + x * 4) as usize;
        // Bgrx8888
        let b = self.buffer[offset];
        let g = self.buffer[offset + 1];
        let r = self.buffer[offset + 2];
        (r, g, b)
    }
}

impl FramebufferTarget for MockFb {
    fn info(&self) -> FramebufferInfo {
        FramebufferInfo {
            width: self.width,
            height: self.height,
            stride: self.width * 4,
            format: PixelFormat::Bgrx8888,
        }
    }

    fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn clear(&mut self, color: u32) {
        for chunk in self.buffer.chunks_mut(4) {
            let bytes = color.to_le_bytes();
            chunk.copy_from_slice(&bytes);
        }
    }
}

#[test]
fn test_console_wrapping_and_dimming() {
    let fb = MockFb::new(800, 600);
    // Fill background with BLUE (0, 0, 128)
    // BootUpDisplay::new calls clear.
    let mut display = BootUpDisplay::new(fb);

    // Fill 32 lines. 0..31.
    for i in 0..32 {
        display.render_log_line(&format!("Line {}", i));
    }

    // Add 33rd line. Wraps to index 0.
    display.render_log_line("Line 32 (Newest)");

    let fb = display.into_inner();

    // Helper to get max red in a row (assuming text is White/Red/Yellow, so R component is present).
    // Background is (0, 0, 128). R=0.
    let get_row_max_r = |row_idx: usize| -> u8 {
        let y_start = 25 + (row_idx * 13) as u32;
        let mut max_r = 0;
        for y in y_start..y_start+13 {
            // Check first 200 pixels
            for x in 25..200 {
                let (r, _, _) = fb.get_pixel(x, y);
                if r > max_r { max_r = r; }
            }
        }
        max_r
    };

    let r_row0 = get_row_max_r(0);
    let r_row1 = get_row_max_r(1);
    let r_row31 = get_row_max_r(31);

    println!("R Row 0: {}", r_row0);
    println!("R Row 1: {}", r_row1);
    println!("R Row 31: {}", r_row31);

    // Assertions for NEW behavior.

    // Row 0 (Active) should be bright.
    assert!(r_row0 > 200, "Row 0 (Active) should be bright. Got {}", r_row0);

    // Row 1 (Oldest) should be dim.
    assert!(r_row1 < 100, "Row 1 (Oldest) should be dim. Got {}", r_row1);

    // Row 31 (Recent) should be bright.
    assert!(r_row31 > 150, "Row 31 (Recent) should be relatively bright. Got {}", r_row31);

    // Row 31 should be slightly dimmer than Row 0.
    assert!(r_row31 < r_row0, "Row 31 should be slightly dimmer than Row 0. Got {} vs {}", r_row31, r_row0);
}
