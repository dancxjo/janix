//! Wallpaper worker thread - decodes wallpaper off the main render loop.
//!
//! The worker:
//! 1. Finds the wallpaper bytespace/module
//! 2. Maps it at a worker-chosen VA (not fixed 0x8600_0000)
//! 3. Decodes BMP into Vec<u32> pixels
//! 4. Computes dominant color
//! 5. Sends result via mailbox
//! 6. Exits

use alloc::vec::Vec;
use alloc::format;
use core::sync::atomic::{AtomicU32, Ordering};
use thing_std::{log_info, thread_exit, trace_fn};

use crate::mailbox::Mailbox;

/// Wallpaper decode result - sent from worker to main
pub struct WallpaperReady {
    pub width: u32,
    pub height: u32,
    pub row_stride: usize,
    pub dominant_rgb: u32,  // Packed 0x00RRGGBB
    pub pixels: Vec<u32>,   // ARGB pixels, owned
}

/// Global mailbox for wallpaper worker → main communication
pub static WALLPAPER_MBX: Mailbox<WallpaperReady> = Mailbox::new();

/// Diagnostics: set to 1 by worker at first instruction (bypasses logging)
pub static WALLPAPER_WORKER_STARTED: AtomicU32 = AtomicU32::new(0);

/// Worker thread entry point.
/// 
/// Takes no arguments (arg is unused). Decodes wallpaper and sends via mailbox.
#[unsafe(no_mangle)]
pub extern "C" fn wallpaper_worker_entry(_arg: u64) -> ! {
    // FIRST INSTRUCTION: set started flag (bypasses any logging issues)
    WALLPAPER_WORKER_STARTED.store(1, Ordering::Release);
    
    log_info("WALLPAPER_WORKER: starting");
    
    let start = thing_std::time::monotonic_now();
    
    match decode_wallpaper() {
        Ok(result) => {
            let elapsed_ns = thing_std::time::monotonic_now() - start;
            let elapsed_ms = elapsed_ns / 1_000_000;
            log_info(&format!(
                "WALLPAPER_WORKER: decoded {}x{} ({} pixels) in {}ms",
                result.width, result.height, result.pixels.len(), elapsed_ms
            ));
            
            if WALLPAPER_MBX.try_send(result).is_err() {
                log_info("WALLPAPER_WORKER: mailbox already full (unexpected)");
            }
        }
        Err(reason) => {
            log_info(&format!("WALLPAPER_WORKER: failed: {}", reason));
        }
    }
    
    log_info("WALLPAPER_WORKER: exiting");
    thread_exit(0);
}

/// Find, map, and decode the wallpaper bytespace.
fn decode_wallpaper() -> Result<WallpaperReady, &'static str> {
    use thing_std::graph::thing_find;
    use thing_std::memory::space_map;
    
    // 1. Find the wallpaper asset
    let asset_name = "bytespace.asset.clouds.bmp";
    let bs_id = thing_find(asset_name).ok_or("wallpaper bytespace not found")?;
    log_info(&format!("WALLPAPER_WORKER: found {} id={}", asset_name, bs_id.low()));
    
    // 2. Map at a worker-chosen VA (not 0x8600_0000 to avoid conflicts)
    // Use 0x8500_0000 for the worker's wallpaper mapping
    let map_va = 0x8500_0000u64;
    let map_size = 4 * 1024 * 1024u64; // 4MB, same as existing code
    
    let result = space_map(bs_id, map_va, 0, map_size);
    if result == 0 {
        return Err("space_map failed");
    }
    log_info(&format!("WALLPAPER_WORKER: mapped at {:#x} len={}", map_va, map_size));
    
    // 3. Parse BMP and decode pixels
    let data = unsafe { 
        core::slice::from_raw_parts(map_va as *const u8, map_size as usize) 
    };
    
    let (pixels, width, height, row_stride) = parse_bmp_to_argb(data)?;
    
    // 4. Compute dominant color
    let dominant = compute_dominant_color(&pixels);
    
    Ok(WallpaperReady {
        width,
        height,
        row_stride,
        dominant_rgb: dominant,
        pixels,
    })
}

/// Parse a BMP file into ARGB pixels.
fn parse_bmp_to_argb(data: &[u8]) -> Result<(Vec<u32>, u32, u32, usize), &'static str> {
    // BMP header validation
    if data.len() < 54 {
        return Err("BMP too small");
    }
    if data[0] != b'B' || data[1] != b'M' {
        return Err("not a BMP file");
    }
    
    // Read header fields
    let data_offset = u32::from_le_bytes([data[10], data[11], data[12], data[13]]) as usize;
    let width = i32::from_le_bytes([data[18], data[19], data[20], data[21]]) as u32;
    let height_signed = i32::from_le_bytes([data[22], data[23], data[24], data[25]]);
    let height = height_signed.unsigned_abs();
    let top_down = height_signed < 0;
    let bits_per_pixel = u16::from_le_bytes([data[28], data[29]]) as usize;
    
    if bits_per_pixel != 24 && bits_per_pixel != 32 {
        return Err("unsupported BMP bit depth");
    }
    
    let bytes_per_pixel = bits_per_pixel / 8;
    let row_stride = ((width as usize * bytes_per_pixel + 3) / 4) * 4; // BMP rows are 4-byte aligned
    
    let pixel_data = &data[data_offset..];
    let mut pixels = Vec::with_capacity((width * height) as usize);
    
    for y in 0..height {
        let src_y = if top_down { y } else { height - 1 - y };
        let row_start = src_y as usize * row_stride;
        
        for x in 0..width {
            let px_start = row_start + (x as usize * bytes_per_pixel);
            if px_start + bytes_per_pixel > pixel_data.len() {
                break;
            }
            
            let b = pixel_data[px_start] as u32;
            let g = pixel_data[px_start + 1] as u32;
            let r = pixel_data[px_start + 2] as u32;
            let a = if bytes_per_pixel == 4 { pixel_data[px_start + 3] as u32 } else { 255 };
            
            // Pack as ARGB
            pixels.push((a << 24) | (r << 16) | (g << 8) | b);
        }
    }
    
    Ok((pixels, width, height, width as usize))
}

/// Compute dominant color using histogram with 5-bit quantization.
fn compute_dominant_color(pixels: &[u32]) -> u32 {
    // 32x32x32 = 32768 buckets
    let mut histogram = [0u32; 32 * 32 * 32];
    
    for &px in pixels {
        // Skip transparent pixels
        let a = (px >> 24) & 0xFF;
        if a < 128 {
            continue;
        }
        
        let r = ((px >> 16) & 0xFF) >> 3;
        let g = ((px >> 8) & 0xFF) >> 3;
        let b = (px & 0xFF) >> 3;
        
        let idx = (r as usize * 32 * 32) + (g as usize * 32) + (b as usize);
        histogram[idx] += 1;
    }
    
    // Find max bucket
    let mut max_idx = 0;
    let mut max_count = 0;
    for (idx, &count) in histogram.iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_idx = idx;
        }
    }
    
    // Convert back to RGB
    let r = ((max_idx / (32 * 32)) as u32) << 3;
    let g = (((max_idx / 32) % 32) as u32) << 3;
    let b = ((max_idx % 32) as u32) << 3;
    
    (r << 16) | (g << 8) | b
}
