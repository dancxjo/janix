#![no_std]
#![no_main]

extern crate alloc;
use thing_std::graph::*;
use thing_std::*;

#[no_mangle]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    loop {
        if let Some(display_dev) = thing_find("device.display0") {
            log_info("BLOOM: found display0");

            let _surf = thing_find("surface.display0").expect("surface not found");
            let bs_id = thing_find("bytespace.display0").expect("bytespace not found");

            let fb_base = 0xA000_0000;
            let fb_size = 1920 * 1080 * 4;
            let _mapped = thing_std::memory::space_map(bs_id, fb_base, 0, fb_size);

            log_info("BLOOM: mapped framebuffer");

            let fb_ptr = fb_base as *mut u32;
            let width = 1024;
            let height = 768;

            let mut frame = 0;
            let mut current_color: Option<u32> = None;

            loop {
                // Check for input to switch colors (for BDD tests)
                let mut input_buf = [0u8; 1];
                if thing_std::input::read(&mut input_buf) > 0 {
                    match input_buf[0] {
                        b'c' => { // Cornflower Blue (0x64, 0x95, 0xED)
                            current_color = Some(0xFF6495ED);
                            log_info("BLOOM: color=cornflower");
                        }
                        b'g' => { // Gradient (default)
                            current_color = None;
                            log_info("BLOOM: color=gradient");
                        }
                        _ => {}
                    }
                }

                if let Some(color) = current_color {
                    for i in 0..(width * height) as isize {
                        unsafe { *fb_ptr.offset(i) = color; }
                    }
                } else {
                    // Simple gradient pattern
                    for y in 0..height {
                        for x in 0..width {
                            let offset = (y * width + x) as isize;
                            unsafe {
                                let r = ((x + frame) % 255) as u32;
                                let g = ((y + frame) % 255) as u32;
                                let b = 128; // Constant blue
                                *fb_ptr.offset(offset) = 0xFF000000 | (r << 16) | (g << 8) | b;
                            }
                        }
                    }
                }
                sched_yield();
                frame += 1;
            }
        }
        sched_yield();
    }
}
