use crate::console;
use core::sync::atomic::{AtomicUsize, Ordering};

// Total expected steps.
const TOTAL_STEPS: usize = 16;

static CURRENT_STEP: AtomicUsize = AtomicUsize::new(0);

pub fn step(message: &str) {
    let step = CURRENT_STEP.fetch_add(1, Ordering::SeqCst) + 1;
    let progress = (step as f32 / TOTAL_STEPS as f32).min(1.0);

    // Calculate background color (Black -> Light Gray)
    // We go from 0x00 to 0xAA.
    let max_brightness: u32 = 0xAA;
    let brightness = (max_brightness as f32 * progress) as u32;
    // ARGB
    let bg_color: u32 = 0xFF000000 | (brightness << 16) | (brightness << 8) | brightness;

    // Switch text to black if background is too bright
    let fg_color: u32 = if brightness > 0x80 {
        0xFF000000
    } else {
        0xFFFFFFFF
    };

    // Log the message to kernel log
    kernel::log(message);

    // Update visual
    // console::update_theme(bg_color, fg_color);
    // console::draw_progress(progress);
}
