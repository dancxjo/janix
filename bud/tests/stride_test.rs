
use bud::framebuffer::{FramebufferTarget, FramebufferInfo, PixelFormat};
use bud::display::BootUpDisplay;
use std::cell::RefCell;
use std::rc::Rc;

struct MockFb {
    width: u32,
    height: u32,
    stride: u32,
    buffer: Rc<RefCell<Vec<u8>>>,
}

impl FramebufferTarget for MockFb {
    fn info(&self) -> FramebufferInfo {
        FramebufferInfo {
            width: self.width,
            height: self.height,
            stride: self.stride,
            format: PixelFormat::Bgrx8888,
        }
    }

    fn buffer_mut(&mut self) -> &mut [u8] {
        let mut borrow = self.buffer.borrow_mut();
        let ptr = borrow.as_mut_ptr();
        let len = borrow.len();
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    }

    fn clear(&mut self, color: u32) {
        // Handle BGRX color packing
        // color is u32 (0xAARRGGBB in LE? No, depends on caller)
        // bud passes: u32::from_le_bytes([b, g, r, 0])
        // So color is 0x00RRGGBB.
        let bytes = color.to_le_bytes(); // [B, G, R, 0]

        let mut buf = self.buffer.borrow_mut();
        for chunk in buf.chunks_exact_mut(4) {
            chunk[0] = bytes[0];
            chunk[1] = bytes[1];
            chunk[2] = bytes[2];
            chunk[3] = bytes[3];
        }
    }
}

#[test]
fn test_render_log_line_respects_stride() {
    let width = 200;
    let height = 100;
    let bpp = 4;
    // Stride is larger than width * bpp.
    // width * 4 = 800. Stride = 900.
    let stride = 900;

    let buffer_size = (stride * height) as usize;
    let buffer = Rc::new(RefCell::new(vec![0u8; buffer_size]));

    let fb = MockFb {
        width,
        height,
        stride,
        buffer: buffer.clone(),
    };

    let mut display = BootUpDisplay::new(fb);

    // Draw a log line
    display.render_log_line("Test");

    // "Test" starts at MARGIN + TEXT_PAD.
    // MARGIN=20, TEXT_PAD=5. Start X = 25.
    // Start Y = 25.

    // The first character 'T' is at (25, 25).
    // Row 1 (gy=1) of 'T' has 0x7E.
    // x = 26, y = 26.

    // Offset should be: y * stride + x * bpp
    // 26 * 900 + 26 * 4 = 23400 + 104 = 23504.

    // Wrong offset (width based):
    // 26 * 800 + 26 * 4 = 20800 + 104 = 20904.

    let buf = buffer.borrow();

    let b = buf[23504];
    let g = buf[23504 + 1];
    let r = buf[23504 + 2];

    // Text is White (255, 255, 255).
    assert_eq!(b, 255, "Blue channel should be 255 at correct stride");
    assert_eq!(g, 255, "Green channel should be 255 at correct stride");
    assert_eq!(r, 255, "Red channel should be 255 at correct stride");

    // Check pixel at WRONG stride.
    // It should be background color (Blue: 128, 0, 0).
    // Note: MockFb.clear implements filling with Blue.

    let b_wrong = buf[20904];
    let g_wrong = buf[20904 + 1];
    let r_wrong = buf[20904 + 2];

    assert_eq!(b_wrong, 128, "Blue channel should be 128 (BG) at wrong stride");
    assert_eq!(g_wrong, 0, "Green channel should be 0 (BG) at wrong stride");
    assert_eq!(r_wrong, 0, "Red channel should be 0 (BG) at wrong stride");
}

#[test]
fn test_stride_zero_fallback() {
    let width = 100;
    let height = 100;
    let stride = 0; // Invalid stride, should be treated as width * 4 = 400
    let buffer_size = (width * 4 * height) as usize;
    let buffer = Rc::new(RefCell::new(vec![0u8; buffer_size]));

    let fb = MockFb {
        width,
        height,
        stride,
        buffer: buffer.clone(),
    };

    let mut display = BootUpDisplay::new(fb);

    // Draw text "A".
    display.render_log_line("A");

    // If stride was 0, y*stride would be 0.
    // Text at y=25 would be drawn at offset 0 + x*4.
    // x starts at 25.
    // So it would overwrite the first line.

    // If stride is corrected (400), it draws at y=25 -> offset 25*400 = 10000.

    let buf = buffer.borrow();

    // 'A' at x=25, y=25.
    // 'A' row 3 (y=28) is 0x3C (00111100).
    // x = 25+2 = 27.

    // Correct offset: 28 * 400 + 27 * 4 = 11200 + 108 = 11308.

    // Zero-stride offset: 28 * 0 + 27 * 4 = 108.

    let b_correct = buf[11308];
    assert_eq!(b_correct, 255, "Should have text pixel at corrected stride");

    let b_zero = buf[108];
    // If stride was 0, text would be here.
    assert_ne!(b_zero, 255, "Should NOT have text pixel at zero stride offset");
}
