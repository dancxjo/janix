#![no_std]
#![no_main]
#![feature(portable_simd)]

extern crate alloc;

use core::simd::u32x4;
use models::DisplayDevice;
use thing_std::*;

/// SIMD 4-wide fill - fastest path for solid color
#[inline(always)]
fn fill_simd(fb: &mut [u32], color: u32) {
    let color_vec = u32x4::splat(color);
    let mut chunks = fb.chunks_exact_mut(4);
    for chunk in chunks.by_ref() {
        let arr: &mut [u32; 4] = chunk.try_into().unwrap();
        *arr = color_vec.to_array();
    }
    // Handle remainder (0-3 pixels)
    for pixel in chunks.into_remainder() {
        *pixel = color;
    }
}

#[no_mangle]
pub fn main() {
    thing_std::init(0);
    log_info("SPARK: starting");

    loop {
        if let Some(display_id) = thing_find("device.display0") {
            log_info("SPARK: found display0");

            let (width, height) = if let Ok(d) = DisplayDevice::read(&SyscallGraphClient, display_id) {
                (d.width.clamp(1, 4096), d.height.clamp(1, 4096))
            } else {
                (1280u32, 720u32)
            };

            let Some(bs_id) = thing_find("bytespace.display0") else {
                sleep_ms(10);
                continue;
            };

            let fb_vaddr = 0xB000_0000u64; // Different from bloom's 0xA000_0000
            let fb_size = (width as u64) * (height as u64) * 4;
            memory::space_map(bs_id, fb_vaddr, 0, fb_size);
            
            log_info(&alloc::format!(
                "SPARK: {}x{} ({} bytes) at {:#x}",
                width, height, fb_size, fb_vaddr
            ));

            // Safety: fb_vaddr is 4-byte aligned, size is width*height pixels
            let fb = unsafe {
                core::slice::from_raw_parts_mut(fb_vaddr as *mut u32, (width * height) as usize)
            };

            let period_ns: u64 = 500_000_000; // 500ms full cycle (black -> white)
            let mut frame: u64 = 0;

            loop {
                let now = monotonic_now();
                let phase = (now % period_ns) as f32 / period_ns as f32;
                let intensity = (phase * 255.0) as u8;
                // ARGB format: 0xAARRGGBB
                let color = 0xFF000000 | (intensity as u32) * 0x010101;

                fill_simd(fb, color);

                frame += 1;
                if frame % 60 == 0 {
                    log_info(&alloc::format!("SPARK: f{} c={:#x}", frame, color));
                }

                // ~60 FPS cadence
                sleep_ms(16);
            }
        }
        sched_yield();
    }
}
