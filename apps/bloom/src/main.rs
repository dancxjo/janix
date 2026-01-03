#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use thing_std::graph::*;

#[no_mangle]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    loop {
         if let Some(display_dev) = thing_find("device.display0") {
             log_info("BLOOM: found display0");
             
             // 1. Find Surface (PRED_PRIMARY)
             // For debugging, we assume standard naming "surface.display0" and "bytespace.display0"
             // as registered by boot.rs
             // The expect() calls will panic if not found, appearing in logs.
             
             let _surf = thing_find("surface.display0").expect("surface not found");
             
             // Map the bytespace
             let bs_id = thing_find("bytespace.display0").expect("bytespace not found");
             
             // Map at 2GB mark (userland safe)
             let fb_base = 0x8000_0000; 
             // 1920x1080x4 = ~8MB. 
             let fb_size = 1920 * 1080 * 4; 
             let _mapped = thing_std::memory::space_map(bs_id, fb_base, 0, fb_size);
             
             log_info("BLOOM: mapped framebuffer, painting...");
             
             let fb_ptr = fb_base as *mut u32;
             // Hardcoded resolution for now (matches standard QEMU default or what boot.rs sees)
             // boot.rs gets it from Multiboot/Limine. Usually 1024x768 or 800x600 if not set.
             // We'll write safely up to 1024x768. 
             let width = 1024; 
             let height = 768; 
             
             let mut frame = 0;
             loop {
                 // Simple gradient pattern
                 for y in 0..height {
                     for x in 0..width {
                         let offset = (y * width + x) as isize;
                         unsafe {
                             let r = ((x + frame) % 255) as u32;
                             let g = ((y + frame) % 255) as u32;
                             let b = 128; // Constant blue
                             // ARGB format (Subject to change based on BPP)
                             *fb_ptr.offset(offset) = 0xFF000000 | (r << 16) | (g << 8) | b;
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
