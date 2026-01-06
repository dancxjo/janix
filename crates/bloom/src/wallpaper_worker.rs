//! Wallpaper worker thread - decodes wallpaper off the main render loop.

use alloc::vec::Vec;
use alloc::format;
use core::sync::atomic::{AtomicU32, Ordering};
use thing_std::{log_info, thread_exit};

use crate::mailbox::Mailbox;

pub struct WallpaperReady {
    pub width: u32,
    pub height: u32,
    pub row_stride: usize,
    pub dominant_rgb: u32,
    pub pixels: Vec<u32>,
}

pub static WALLPAPER_MBX: Mailbox<WallpaperReady> = Mailbox::new();
pub static WALLPAPER_WORKER_STARTED: AtomicU32 = AtomicU32::new(0);
pub static WALLPAPER_WORKER_PHASE: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn wallpaper_worker_entry(_arg: u64) -> ! {
    WALLPAPER_WORKER_STARTED.store(1, Ordering::Release);
    WALLPAPER_WORKER_PHASE.store(1, Ordering::Release);
    
    // Phase 2: before any log (test if log itself works)
    WALLPAPER_WORKER_PHASE.store(2, Ordering::Release);
    thing_std::debug::log("WP: phase2-log");
    
    // Phase 3: before decode_wallpaper
    WALLPAPER_WORKER_PHASE.store(3, Ordering::Release);
    
    match decode_wallpaper() {
        Ok(result) => {
            WALLPAPER_WORKER_PHASE.store(100, Ordering::Release);
            thing_std::debug::log("WP: phase100-success");
            if WALLPAPER_MBX.try_send(result).is_err() {
                thing_std::debug::log("WP: mailbox full");
            }
        }
        Err(reason) => {
            // Use simple log without formatting to ensure it works
            WALLPAPER_WORKER_PHASE.store(200, Ordering::Release);
            thing_std::debug::log("WP: phase200-decode-failed");
            // Now try to log the reason (this may fail)
            WALLPAPER_WORKER_PHASE.store(201, Ordering::Release);
            log_info(&format!("WP: reason: {}", reason));
        }
    }
    
    WALLPAPER_WORKER_PHASE.store(255, Ordering::Release);
    thing_std::debug::log("WP: exiting");
    thread_exit(0);
}

fn decode_wallpaper() -> Result<WallpaperReady, &'static str> {
    use thing_std::graph::thing_find;
    use thing_std::memory::space_map;
    
    // Phase 10: before thing_find
    WALLPAPER_WORKER_PHASE.store(10, Ordering::Release);
    thing_std::debug::log("WP: finding asset");
    
    let asset_name = "bytespace.asset.clouds.bmp";
    let bs_id = thing_find(asset_name).ok_or("wallpaper bytespace not found")?;
    
    // Phase 11: after thing_find succeeded
    WALLPAPER_WORKER_PHASE.store(11, Ordering::Release);
    thing_std::debug::log("WP: found asset");
    
    // Phase 12: before space_map
    WALLPAPER_WORKER_PHASE.store(12, Ordering::Release);
    let map_va = 0x8500_0000u64;
    let map_size = 4 * 1024 * 1024u64;
    
    let result = space_map(bs_id, map_va, 0, map_size);
    if result == 0 {
        return Err("space_map failed");
    }
    
    // Phase 13: after space_map succeeded
    WALLPAPER_WORKER_PHASE.store(13, Ordering::Release);
    thing_std::debug::log("WP: mapped");
    
    let data = unsafe { 
        core::slice::from_raw_parts(map_va as *const u8, map_size as usize) 
    };
    
    // Phase 14: before parse
    WALLPAPER_WORKER_PHASE.store(14, Ordering::Release);
    let (pixels, width, height, row_stride) = parse_bmp_to_argb(data)?;
    
    // Phase 15: after parse
    WALLPAPER_WORKER_PHASE.store(15, Ordering::Release);
    thing_std::debug::log("WP: parsed");
    
    let dominant = compute_dominant_color(&pixels);
    
    // Phase 16: done
    WALLPAPER_WORKER_PHASE.store(16, Ordering::Release);
    thing_std::debug::log("WP: decoded");
    
    Ok(WallpaperReady { width, height, row_stride, dominant_rgb: dominant, pixels })
}

fn parse_bmp_to_argb(data: &[u8]) -> Result<(Vec<u32>, u32, u32, usize), &'static str> {
    if data.len() < 54 { return Err("BMP too small"); }
    if data[0] != b'B' || data[1] != b'M' { return Err("not a BMP file"); }
    
    let data_offset = u32::from_le_bytes([data[10], data[11], data[12], data[13]]) as usize;
    let width = i32::from_le_bytes([data[18], data[19], data[20], data[21]]) as u32;
    let height_signed = i32::from_le_bytes([data[22], data[23], data[24], data[25]]);
    let height = height_signed.unsigned_abs();
    let top_down = height_signed < 0;
    let bits_per_pixel = u16::from_le_bytes([data[28], data[29]]) as usize;
    
    if bits_per_pixel != 24 && bits_per_pixel != 32 { return Err("unsupported BMP bit depth"); }
    
    let bytes_per_pixel = bits_per_pixel / 8;
    let row_stride = ((width as usize * bytes_per_pixel + 3) / 4) * 4;
    let pixel_data = &data[data_offset..];
    let mut pixels = Vec::with_capacity((width * height) as usize);
    
    for y in 0..height {
        let src_y = if top_down { y } else { height - 1 - y };
        let row_start = src_y as usize * row_stride;
        for x in 0..width {
            let px_start = row_start + (x as usize * bytes_per_pixel);
            if px_start + bytes_per_pixel > pixel_data.len() { break; }
            let b = pixel_data[px_start] as u32;
            let g = pixel_data[px_start + 1] as u32;
            let r = pixel_data[px_start + 2] as u32;
            let a = if bytes_per_pixel == 4 { pixel_data[px_start + 3] as u32 } else { 255 };
            pixels.push((a << 24) | (r << 16) | (g << 8) | b);
        }
    }
    
    Ok((pixels, width, height, width as usize))
}

fn compute_dominant_color(pixels: &[u32]) -> u32 {
    let mut histogram: alloc::boxed::Box<[u32; 32 * 32 * 32]> = 
        alloc::boxed::Box::new([0u32; 32 * 32 * 32]);
    
    for &px in pixels {
        let a = (px >> 24) & 0xFF;
        if a < 128 { continue; }
        let r = ((px >> 16) & 0xFF) >> 3;
        let g = ((px >> 8) & 0xFF) >> 3;
        let b = (px & 0xFF) >> 3;
        let idx = (r as usize * 32 * 32) + (g as usize * 32) + (b as usize);
        histogram[idx] += 1;
    }
    
    let mut max_idx = 0;
    let mut max_count = 0;
    for (idx, &count) in histogram.iter().enumerate() {
        if count > max_count { max_count = count; max_idx = idx; }
    }
    
    let r = ((max_idx / (32 * 32)) as u32) << 3;
    let g = (((max_idx / 32) % 32) as u32) << 3;
    let b = ((max_idx % 32) as u32) << 3;
    (r << 16) | (g << 8) | b
}
