//! Wallpaper loader - synchronous for now until worker thread heap issues are resolved.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use thing_std::graph::thing_find;
use thing_std::memory::space_map;
use crate::mailbox::Mailbox;

pub struct WallpaperMsg {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
    pub dominant_color: u32,
}

pub static WALLPAPER_MBX: Mailbox<WallpaperMsg> = Mailbox::new();
pub static WALLPAPER_WORKER_STARTED: AtomicU32 = AtomicU32::new(0);
pub static WALLPAPER_WORKER_PHASE: AtomicU32 = AtomicU32::new(0);

/// Load wallpaper synchronously in main thread (worker thread heap is too slow)
pub fn load_wallpaper_sync() -> Option<WallpaperMsg> {
    thing_std::log_info("BLOOM: loading wallpaper sync");
    
    let bs_id = thing_find(theme::current::WALLPAPER_BYTESPACE)?;
    
    let hint_va = 0x8500_0000u64;
    let map_size = 4 * 1024 * 1024u64;
    
    let actual_va = space_map(bs_id, hint_va, 0, map_size);
    if actual_va == 0 {
        thing_std::log_info("BLOOM: wallpaper space_map failed");
        return None;
    }
    
    thing_std::log_info(&alloc::format!("BLOOM: wallpaper mapped at {:#x}", actual_va));
    
    let data = unsafe { 
        core::slice::from_raw_parts(actual_va as *const u8, map_size as usize) 
    };
    
    // Parse BMP
    let (pixels, width, height, _) = parse_bmp_to_argb(data).ok()?;
    
    thing_std::log_info(&alloc::format!("BLOOM: wallpaper parsed {}x{}", width, height));
    
    let dominant = compute_dominant_color(&pixels);
    
    Some(WallpaperMsg {
        width,
        height,
        pixels,
        dominant_color: dominant,
    })
}

/// Wallpaper worker entry point - disabled for now
#[unsafe(no_mangle)]
pub extern "C" fn wallpaper_worker_entry(_arg: u64) -> ! {
    WALLPAPER_WORKER_STARTED.store(1, Ordering::Release);
    WALLPAPER_WORKER_PHASE.store(255, Ordering::Release); // Mark as done immediately
    thing_std::process::exit(0);
}

fn parse_bmp_to_argb(data: &[u8]) -> Result<(Vec<u32>, u32, u32, u32), &'static str> {
    if data.len() < 54 {
        return Err("BMP too small");
    }
    if data[0] != b'B' || data[1] != b'M' {
        return Err("Invalid BMP magic");
    }
    
    let width = u32::from_le_bytes([data[18], data[19], data[20], data[21]]);
    let height_raw = i32::from_le_bytes([data[22], data[23], data[24], data[25]]);
    let height = height_raw.unsigned_abs();
    let bottom_up = height_raw > 0;
    
    let bits_per_pixel = u16::from_le_bytes([data[28], data[29]]);
    let data_offset = u32::from_le_bytes([data[10], data[11], data[12], data[13]]) as usize;
    
    if bits_per_pixel != 24 && bits_per_pixel != 32 {
        return Err("Unsupported BMP bit depth");
    }
    
    let bytes_per_pixel = (bits_per_pixel / 8) as usize;
    let row_stride = ((width as usize * bytes_per_pixel + 3) / 4) * 4;
    
    thing_std::log_info(&alloc::format!("BLOOM: allocating {}x{} pixels", width, height));
    let mut pixels = alloc::vec![0u32; (width * height) as usize];
    thing_std::log_info("BLOOM: allocation done, parsing rows");
    
    for y in 0..height {
        let src_y = if bottom_up { height - 1 - y } else { y };
        let row_start = data_offset + (src_y as usize) * row_stride;
        
        for x in 0..width {
            let px_start = row_start + (x as usize) * bytes_per_pixel;
            if px_start + bytes_per_pixel > data.len() {
                continue;
            }
            
            let b = data[px_start] as u32;
            let g = data[px_start + 1] as u32;
            let r = data[px_start + 2] as u32;
            let a = if bytes_per_pixel == 4 { data[px_start + 3] as u32 } else { 255 };
            
            let dst_idx = (y * width + x) as usize;
            pixels[dst_idx] = (a << 24) | (r << 16) | (g << 8) | b;
        }
    }
    
    Ok((pixels, width, height, row_stride as u32))
}

fn compute_dominant_color(pixels: &[u32]) -> u32 {
    let mut histogram = [0u32; 64];
    
    for &px in pixels.iter() {
        let r = ((px >> 16) & 0xFF) as usize;
        let g = ((px >> 8) & 0xFF) as usize;
        let b = (px & 0xFF) as usize;
        let idx = ((r >> 6) << 4) | ((g >> 6) << 2) | (b >> 6);
        histogram[idx] += 1;
    }
    
    let mut max_idx = 0;
    let mut max_count = 0;
    for (idx, &count) in histogram.iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_idx = idx;
        }
    }
    
    let r = (((max_idx >> 4) & 0x3) << 6) + 32;
    let g = (((max_idx >> 2) & 0x3) << 6) + 32;
    let b = ((max_idx & 0x3) << 6) + 32;
    
    0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
