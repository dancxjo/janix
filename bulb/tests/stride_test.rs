
use bulb::framebuffer::{FramebufferTarget, FramebufferInfo, PixelFormat};
use bulb::display::BootUpDisplay;
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
    let width = 800;
    let height = 600;
    // Stride is larger than width * bpp.
    // 800 * 4 = 3200. Stride = 3300.
    let stride = 3300;

    let buffer_size = (stride * height) as usize;
    let buffer = Rc::new(RefCell::new(vec![0u8; buffer_size]));

    let fb = MockFb {
        width,
        height,
        stride,
        buffer: buffer.clone(),
    };

    let mut display = BootUpDisplay::new(fb);

    // Draw a log line "Test"
    display.render_log_line("Test");

    // Calculation of position:
    // Center X = 400. Center Y = 300.
    // Text "Test": 4 chars * 8 = 32 width.
    // Start X = 400 - 16 = 384.
    // Lines = 1. Height = 13.
    // Start Y = 300 - 6 = 294.

    // Character 'T' is at (384, 294).
    // b'T' => [0x00, 0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00]
    // Row 1 is 0x7E (01111110).
    // x offsets: 1 to 6.
    // x = 384 + 1 = 385.
    // y = 294 + 1 = 295.

    let target_x = 385;
    let target_y = 295;
    let bpp = 4;

    // Offset = y * stride + x * bpp
    let offset = (target_y as usize * stride as usize) + (target_x as usize * bpp);

    // Wrong offset (width based):
    let wrong_stride = width * 4;
    let wrong_offset = (target_y as usize * wrong_stride as usize) + (target_x as usize * bpp);

    let buf = buffer.borrow();

    let b = buf[offset];
    let g = buf[offset + 1];
    let r = buf[offset + 2];

    // Text is White (255, 255, 255).
    assert_eq!(b, 255, "Blue channel should be 255 at correct stride");
    assert_eq!(g, 255, "Green channel should be 255 at correct stride");
    assert_eq!(r, 255, "Red channel should be 255 at correct stride");

    // Check pixel at WRONG stride.
    // It should be background color (Black: 0, 0, 0).

    let b_wrong = buf[wrong_offset];
    let g_wrong = buf[wrong_offset + 1];
    let r_wrong = buf[wrong_offset + 2];

    assert_eq!(b_wrong, 0, "Blue channel should be 0 (BG) at wrong stride");
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

    // Center X = 50, Center Y = 50.
    // "A" width 8. Start X = 46.
    // Height 13. Start Y = 44.

    // 'A' row 3 (idx 3, 4th byte).
    // b'A' => [0x00, 0x18, 0x3C, 0x66, ...]
    // Row 2 is 0x3C (00111100).
    // x offsets: 2,3,4,5.
    // x = 46 + 2 = 48.
    // y = 44 + 2 = 46.

    let target_x = 48;
    let target_y = 46;
    let bpp = 4;
    let corrected_stride = width * 4;

    let offset = (target_y as usize * corrected_stride as usize) + (target_x as usize * bpp);

    // Zero-stride offset: y*0 + x*4
    let zero_offset = target_x as usize * bpp;

    let buf = buffer.borrow();

    let b_correct = buf[offset];
    assert_eq!(b_correct, 255, "Should have text pixel at corrected stride");

    let b_zero = buf[zero_offset];
    // If stride was 0, text would be here.
    assert_ne!(b_zero, 255, "Should NOT have text pixel at zero stride offset");
}

#[test]
fn test_put_pixel_respects_stride_in_pixels() {
    let width = 200;
    let height = 100;
    // Reported stride is 100 pixels, which is LESS than width (200).
    // This is weird but handled by calc_stride_bytes(200, 4, 100) -> 800 bytes.
    // If the system misinterprets it as bytes, it will be 100 bytes (INVALID).
    // If the system treats it as pixels: 100 * 4 = 400 bytes (STILL < 800).
    // calc_stride_bytes should fall back to 800 bytes.
    let reported_stride = 100;
    let bpp = 4;
    let expected_stride = width * bpp; // 800

    let buffer_size = (expected_stride * height) as usize;
    let buffer = Rc::new(RefCell::new(vec![0u8; buffer_size]));

    let fb = MockFb {
        width,
        height,
        stride: reported_stride,
        buffer: buffer.clone(),
    };

    let mut display = BootUpDisplay::new(fb);

    // Draw a single pixel at (10, 10)
    // We need to access the inner FbDrawer or use a public method.
    // render_log_line uses put_pixel.
    display.render_log_line("X");

    // Check a small area around the expected text position.
    // If stride was wrong, the text would be shifted by many pixels (at least (800-400)*44 = 17600 bytes, which is ~22 rows).
    let buf = buffer.borrow();
    let mut found = false;
    for y in 40..60 {
        for x in 90..110 {
            let off = (y as usize * expected_stride as usize) + (x * bpp as usize);
            if off + 2 < buf.len() && buf[off] == 255 && buf[off + 1] == 255 && buf[off + 2] == 255 {
                found = true;
                break;
            }
        }
        if found { break; }
    }
    assert!(found, "Should have rendered text at correct row with normalized stride");
}
