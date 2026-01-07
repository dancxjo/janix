//! Boot fade animation thread - updates boot progress until wallpaper ready

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub static WALLPAPER_READY: AtomicBool = AtomicBool::new(false);
pub static CURRENT_BG_COLOR: AtomicU32 = AtomicU32::new(0xFF174069);

// Screen dimensions (for legacy compat, not used now)
pub static SCREEN_WIDTH: AtomicU32 = AtomicU32::new(1280);
pub static SCREEN_HEIGHT: AtomicU32 = AtomicU32::new(720);

pub fn configure_display(_vaddr: u64, width: u32, height: u32, _stride: u32) {
    SCREEN_WIDTH.store(width, Ordering::Release);
    SCREEN_HEIGHT.store(height, Ordering::Release);
}

/// Animation steps for userspace boot fade (lerps from half to full color)
const FADE_MAX_STEP: u32 = 60;

/// Thread entry: animate from half to full color until WALLPAPER_READY
pub extern "C" fn boot_fade_entry(_arg: u64) -> ! {
    thing_std::log_info("BOOT_FADE: starting animation");
    
    let mut step: u32 = 0;
    let mut direction: i32 = 1; // 1 = up, -1 = down
    
    while !WALLPAPER_READY.load(Ordering::Acquire) {
        // Call syscall - kernel handles color calculation and screen fill
        thing_std::boot_progress(step, FADE_MAX_STEP);
        
        // Update step for pulsing effect
        if direction > 0 {
            step = step.saturating_add(1);
            if step >= FADE_MAX_STEP {
                direction = -1;
            }
        } else {
            step = step.saturating_sub(1);
            if step == 0 {
                direction = 1;
            }
        }
        
        thing_std::sched_yield();
    }
    
    // Set to full color before exiting
    thing_std::boot_progress(FADE_MAX_STEP, FADE_MAX_STEP);
    
    thing_std::log_info("BOOT_FADE: wallpaper ready, exiting");
    thing_std::thread::thread_exit(0);
}
