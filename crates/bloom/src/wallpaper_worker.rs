use thing_std::memory::space_map;
use thing_std::graph::thing_find;
use thing_std::log_info;
use alloc::vec::Vec;
use alloc::format;
use core::slice;
use alloc::sync::Arc;

pub struct WallpaperMsg {
    pub width: u32,
    pub height: u32,
    pub pixels: Arc<[u32]>,
    pub dominant_color: u32,
}

pub static WALLPAPER_WORKER_STARTED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
pub static WALLPAPER_WORKER_PHASE: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);
pub static WALLPAPER_MBX: thing_std::process::OneShotMailbox<WallpaperMsg> = thing_std::process::OneShotMailbox::new();

pub fn spawn_worker() {
    // For now, loading wallpaper synchronously in the main thread is safer 
    // due to heap performance issues in worker threads.
}

pub fn load_wallpaper_sync() -> Option<WallpaperMsg> {
    log_info("BLOOM: loading wallpaper sync");
    
    let bs_id = thing_find(::theme::current::WALLPAPER_BYTESPACE)?;
    
    let hint_va = 0x8500_0000u64;
    let map_size = 4 * 1024 * 1024u64;
    
    let actual_va = space_map(bs_id, hint_va, 0, map_size);
    if actual_va == 0 {
        log_info("BLOOM: wallpaper space_map failed");
        return None;
    }
    
    log_info(&format!("BLOOM: wallpaper mapped at {:#x}", actual_va));
    
    // --- BLOOM PANIC TRAP ---
    if actual_va == 0 {
        log_info("BLOOM BUG: load_wallpaper_sync: actual_va is 0");
        return None;
    }

    let data = unsafe { 
        slice::from_raw_parts(actual_va as *const u8, map_size as usize) 
    };
    
    // Parse BMP
    let (width, height, pixels) = crate::assets::bitmap::parse_bmp_to_argb(data)?;
    
    log_info(&format!("BLOOM: wallpaper parsed {}x{}", width, height));
    
    let dominant = crate::assets::bitmap::compute_dominant_color(&pixels);
    
    Some(WallpaperMsg {
        width,
        height,
        pixels,
        dominant_color: dominant,
    })
}
