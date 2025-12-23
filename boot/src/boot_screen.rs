use crate::console;
use core::sync::atomic::{AtomicUsize, Ordering};

// Total expected steps.
const TOTAL_STEPS: usize = 16;

static CURRENT_STEP: AtomicUsize = AtomicUsize::new(0);

pub fn step(message: &str) {
    let step = CURRENT_STEP.fetch_add(1, Ordering::SeqCst) + 1;
    let progress = (step as f32 / TOTAL_STEPS as f32).min(1.0);

    // Calculate background color (Black -> Target boot blue #2E80D1)
    let target_r: u32 = 0x2E;
    let target_g: u32 = 0x80;
    let target_b: u32 = 0xD1;

    let r = (target_r as f32 * progress) as u32;
    let g = (target_g as f32 * progress) as u32;
    let b = (target_b as f32 * progress) as u32;

    // ARGB
    let bg_color: u32 = 0xFF000000 | (r << 16) | (g << 8) | b;

    // Switch text to black if background is too bright
    let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u32;
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
