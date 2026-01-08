//! Wallpaper Worker - Asynchronous wallpaper loading
//!
//! Loads wallpaper in a background thread so it doesn't block boot.
//! Main thread polls WALLPAPER_MBX for the result.

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

/// Spawn the wallpaper worker thread.
/// Worker loads wallpaper asynchronously and sends result via WALLPAPER_MBX.
pub fn spawn_worker() {
    let _worker_thread = thing_std::thread::thread_spawn(wallpaper_worker_entry, 0);
}

/// Wallpaper worker thread entry point
pub extern "C" fn wallpaper_worker_entry(_arg: u64) -> ! {
    WALLPAPER_WORKER_STARTED.store(true, core::sync::atomic::Ordering::Release);
    WALLPAPER_WORKER_PHASE.store(1, core::sync::atomic::Ordering::Release);
    log_info("BLOOM WALLPAPER: worker started");
    
    // Do the actual loading work
    if let Some(msg) = load_wallpaper_impl() {
        WALLPAPER_WORKER_PHASE.store(3, core::sync::atomic::Ordering::Release);
        log_info(&format!("BLOOM WALLPAPER: loaded {}x{}, sending to main", msg.width, msg.height));
        let _ = WALLPAPER_MBX.send(msg);
    } else {
        WALLPAPER_WORKER_PHASE.store(4, core::sync::atomic::Ordering::Release);
        log_info("BLOOM WALLPAPER: failed to load wallpaper");
    }
    
    // Worker done, just idle forever (thread will be cleaned up eventually)
    loop {
        thing_std::sched_yield();
    }
}

/// Load wallpaper from bytespace (called by worker thread)
fn load_wallpaper_impl() -> Option<WallpaperMsg> {
    WALLPAPER_WORKER_PHASE.store(2, core::sync::atomic::Ordering::Release);
    log_info("BLOOM WALLPAPER: finding bytespace");
    
    let bs_id = thing_find(::theme::current::WALLPAPER_BYTESPACE)?;
    
    let hint_va = 0x8500_0000u64;
    let map_size = 4 * 1024 * 1024u64;
    
    let actual_va = space_map(bs_id, hint_va, 0, map_size);
    if actual_va == 0 {
        log_info("BLOOM WALLPAPER: space_map failed");
        return None;
    }
    
    log_info(&format!("BLOOM WALLPAPER: mapped at {:#x}", actual_va));
    
    // Defensive length check
    if (map_size as usize) > isize::MAX as usize {
        log_info("BLOOM WALLPAPER: mapping size exceeds isize::MAX");
        return None;
    }
    let data = unsafe { 
        slice::from_raw_parts(actual_va as *const u8, map_size as usize) 
    };
    
    // Parse BMP
    let (width, height, pixels) = crate::assets::bitmap::parse_bmp_to_argb(data)?;
    
    log_info(&format!("BLOOM WALLPAPER: parsed {}x{}", width, height));
    
    let dominant = crate::assets::bitmap::compute_dominant_color(&pixels);
    
    Some(WallpaperMsg {
        width,
        height,
        pixels,
        dominant_color: dominant,
    })
}

/// Load wallpaper synchronously (legacy, kept for fallback/testing)
#[allow(dead_code)]
pub fn load_wallpaper_sync() -> Option<WallpaperMsg> {
    log_info("BLOOM: loading wallpaper sync");
    load_wallpaper_impl()
}
