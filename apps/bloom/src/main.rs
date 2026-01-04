#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::vec::Vec;
use thing_std::graph::*;
use thing_std::*;

// Double buffer for smooth rendering
static mut BACK_BUFFER: Option<Vec<u32>> = None;

#[no_mangle]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    loop {
        if let Some(_) = thing_find("device.display0") {
            log_info("BLOOM: found display");

            let fb_base = 0xA000_0000u64;
            let width = 1280u32; 
            let height = 720u32;
            let fb_size: u64 = (width as u64) * (height as u64) * 4;
            
            let bs_id = thing_find("bytespace.display0").expect("bytespace not found");
            let _mapped = thing_std::memory::space_map(bs_id, fb_base, 0, fb_size);
            log_info("BLOOM: mapped framebuffer");

            // Allocate double buffer
            let buffer_size = (width * height) as usize;
            unsafe {
                BACK_BUFFER = Some(alloc::vec![0u32; buffer_size]);
            }
            log_info("BLOOM: allocated double buffer");

            // Load wallpaper
            let mut wallpaper: Option<Wallpaper> = None;
            
            if let Some(asset_bs_id) = thing_find("bytespace.asset.clouds.bmp") {
                log_info("BLOOM: loading clouds.bmp");
                if let Some(wp) = load_bmp(asset_bs_id, 0x8100_0000) {
                    let msg = format!("BLOOM: BMP loaded {}x{} bpp={}", wp.width, wp.height, wp.bpp);
                    log_info(&msg);
                    wallpaper = Some(wp);
                }
            }
            
            let fb_ptr = fb_base as *mut u32;
            
            // Render to back buffer
            unsafe {
                if let Some(ref buf) = BACK_BUFFER {
                    let back = buf.as_ptr() as *mut u32;
                    
                    if let Some(ref wp) = wallpaper {
                        log_info("BLOOM: rendering wallpaper to back buffer");
                        render_wallpaper_tiled(back, width, height, wp);
                        log_info("BLOOM: wallpaper rendered");
                    } else {
                        // Fallback: solid gradient
                        log_info("BLOOM: rendering gradient fallback");
                        for y in 0..height {
                            for x in 0..width {
                                let r = ((x * 255) / width) as u32;
                                let b = ((y * 255) / height) as u32;
                                *back.add((y * width + x) as usize) = 0xFF000000 | (r << 16) | b;
                            }
                        }
                    }
                    
                    // Blit back buffer to front buffer using fast copy
                    log_info("BLOOM: blitting to screen");
                    fast_blit(fb_ptr, back, buffer_size);
                    log_info("BLOOM: blit complete");
                }
            }
            
            // Main loop - just yield
            loop {
                sched_yield();
            }
        }
        sched_yield();
    }
}

struct Wallpaper {
    data_ptr: *const u8,
    width: u32,
    height: u32,
    bpp: u16,
    row_stride: usize,
    bottom_up: bool,
}

fn load_bmp(bs_id: ThingId, vaddr: u64) -> Option<Wallpaper> {
    let len: u64 = 4 * 1024 * 1024; // 4MB
    thing_std::memory::space_map(bs_id, vaddr, 0, len);
    let buf = unsafe { core::slice::from_raw_parts(vaddr as *const u8, len as usize) };
    
    // Manual BMP parsing for reliability
    if buf.len() < 54 || &buf[0..2] != b"BM" {
        log_info("BLOOM: not a valid BMP");
        return None;
    }
    
    let data_offset = u32::from_le_bytes([buf[10], buf[11], buf[12], buf[13]]) as usize;
    let width = i32::from_le_bytes([buf[18], buf[19], buf[20], buf[21]]);
    let height = i32::from_le_bytes([buf[22], buf[23], buf[24], buf[25]]);
    let bpp = u16::from_le_bytes([buf[28], buf[29]]);
    
    let msg = format!("BLOOM: BMP header: {}x{} bpp={} offset={}", width, height, bpp, data_offset);
    log_info(&msg);
    
    if width <= 0 {
        return None;
    }
    
    // Calculate row stride (each row is padded to 4-byte boundary)
    let bytes_per_pixel = (bpp as usize + 7) / 8;
    let row_stride = ((width as usize * bytes_per_pixel + 3) / 4) * 4;
    
    let data_ptr = unsafe { (vaddr as *const u8).add(data_offset) };
    
    Some(Wallpaper {
        data_ptr,
        width: width.abs() as u32,
        height: height.abs() as u32,
        bpp,
        row_stride,
        bottom_up: height > 0, // Positive height = bottom-up
    })
}

fn render_wallpaper_tiled(dest: *mut u32, dest_w: u32, dest_h: u32, wp: &Wallpaper) {
    let bytes_per_pixel = (wp.bpp as usize + 7) / 8;
    
    for y in 0..dest_h {
        let src_y = y % wp.height;
        // Handle bottom-up BMPs
        let actual_src_y = if wp.bottom_up {
            wp.height - 1 - src_y
        } else {
            src_y
        };
        
        let row_ptr = unsafe { wp.data_ptr.add(actual_src_y as usize * wp.row_stride) };
        
        for x in 0..dest_w {
            let src_x = x % wp.width;
            let pixel_ptr = unsafe { row_ptr.add(src_x as usize * bytes_per_pixel) };
            
            let color = unsafe {
                match bytes_per_pixel {
                    3 => {
                        // BGR -> ARGB
                        let b = *pixel_ptr as u32;
                        let g = *pixel_ptr.add(1) as u32;
                        let r = *pixel_ptr.add(2) as u32;
                        0xFF000000 | (r << 16) | (g << 8) | b
                    }
                    4 => {
                        // BGRA -> ARGB
                        let b = *pixel_ptr as u32;
                        let g = *pixel_ptr.add(1) as u32;
                        let r = *pixel_ptr.add(2) as u32;
                        let a = *pixel_ptr.add(3) as u32;
                        (a << 24) | (r << 16) | (g << 8) | b
                    }
                    _ => 0xFF888888, // Gray for unsupported
                }
            };
            
            unsafe {
                *dest.add((y * dest_w + x) as usize) = color;
            }
        }
    }
}

/// Fast blit using larger writes when possible
#[inline(never)]
fn fast_blit(dest: *mut u32, src: *const u32, count: usize) {
    // Use 64-bit writes for 2x throughput on 64-bit systems
    let dest64 = dest as *mut u64;
    let src64 = src as *const u64;
    let count64 = count / 2;
    
    for i in 0..count64 {
        unsafe {
            core::ptr::write_volatile(dest64.add(i), *src64.add(i));
        }
    }
    
    // Handle odd count
    if count % 2 == 1 {
        unsafe {
            core::ptr::write_volatile(dest.add(count - 1), *src.add(count - 1));
        }
    }
}
