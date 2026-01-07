//! Boot fade animation thread - updates boot progress until wallpaper ready

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub static WALLPAPER_READY: AtomicBool = AtomicBool::new(false);
pub static CURRENT_BG_COLOR: AtomicU32 = AtomicU32::new(0xFF174069);

// Screen dimensions (for legacy compat, not used now)
pub static SCREEN_WIDTH: AtomicU32 = AtomicU32::new(1280);
pub static SCREEN_HEIGHT: AtomicU32 = AtomicU32::new(720);

// Minimal boot fade atoms (only used for signalling WALLPAPER_READY now)
pub fn configure_display(_vaddr: u64, _width: u32, _height: u32, _stride: u32) {}
pub extern "C" fn boot_fade_entry(_arg: u64) -> ! { loop { thing_std::sched_yield() } }
