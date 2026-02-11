use crate::display::BootUpDisplay;
use crate::framebuffer::{FramebufferInfo, FramebufferTarget, PixelFormat};
use std::vec;
use std::vec::Vec;

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
fn test_centered_rendering() {
    let width = 800;
    let height = 600;
    let fb = MockFb::new(width, height);
    let mut display = BootUpDisplay::new(fb);

    // Render a line with timestamp, level, source, message
    let line = "[12345] [INFO] [kernel] Hello World";
    display.render_log_line(line);

    let fb = display.into_inner();

    // Center of screen
    let cx = width / 2;
    let cy = height / 2;

    // Check center pixel. Should be colored (part of Source or Message).
    // Timestamp is top, Source is middle, Message is bottom.
    // Source should be around cy.
    // Source color is Yellow (R=255, G=255, B=0).

    // We check a small area around center to find ANY non-black pixel.
    let mut found_yellow = false;
    for y in (cy - 10)..(cy + 10) {
        for x in (cx - 50)..(cx + 50) {
            let (r, g, b) = fb.get_pixel(x, y);
            if r > 200 && g > 200 && b < 50 {
                found_yellow = true;
                break;
            }
        }
    }
    assert!(
        found_yellow,
        "Should have rendered yellow text (Source) near center"
    );

    // Re-create display to test clearing
    let mut display = BootUpDisplay::new(fb);

    // Render a NEW line with different content/color.
    // "[99999] [ERROR] [newsrc] New Message" -> level is ERROR (RED).
    display.render_log_line("[99999] [ERROR] [newsrc] New Message");

    let fb = display.into_inner();

    // Check for RED pixel (Message) at bottom.
    // And verify previous text is gone (or overwritten).

    let mut found_red = false;
    // Message is below center.
    for y in cy..(cy + 50) {
        for x in (cx - 100)..(cx + 100) {
            let (r, g, b) = fb.get_pixel(x, y);
            if r > 200 && g < 50 && b < 50 {
                found_red = true;
                break;
            }
        }
    }
    assert!(
        found_red,
        "Should have rendered red text (Error Message) below center"
    );
}
