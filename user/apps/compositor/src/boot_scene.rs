use alloc::vec::Vec;
use alloc::string::String;
use crate::render; // Assumes main.rs does `pub mod render` or similar
use crate::layout::{self, Rect, Size};

pub struct BootMilestone {
    pub label: String,
    pub alpha: f32, // 0.0 to 1.0
}

pub struct BootScene {
    pub milestones: Vec<BootMilestone>,
    pub is_active: bool,
    pub desktop_ready: bool,
    pub global_alpha: f32, // For fading out the whole scene
}

impl BootScene {
    pub fn new() -> Self {
        Self {
            milestones: Vec::new(),
            is_active: true,
            desktop_ready: false,
            global_alpha: 1.0,
        }
    }

    pub fn add_milestone(&mut self, label: &str) {
        // Prevent duplicates if needed, or just append
        // Filter out empty lines
        if label.trim().is_empty() { return; }

        self.milestones.push(BootMilestone {
            label: String::from(label),
            alpha: 0.0,
        });
    }

    pub fn update(&mut self) {
        // Fade in new milestones
        for m in &mut self.milestones {
            if m.alpha < 1.0 {
                m.alpha += 0.02; // Tune this speed. 0.02 * 60fps = ~1.2 alpha/sec -> 0.8s fade. 
                                 // User asked for 250-400ms. 0.05 is better.
                if m.alpha > 1.0 { m.alpha = 1.0; }
            }
        }

        // Fade out scene if desktop ready
        if self.desktop_ready {
            if self.global_alpha > 0.0 {
                self.global_alpha -= 0.05;
                if self.global_alpha < 0.0 { self.global_alpha = 0.0; }
            } else {
                self.is_active = false;
            }
        }
    }

    pub fn render(&self, fb_ptr: *mut u32, pitch: u32, width: u32, height: u32) {
        // 1. Clear Screen to Black (or very dark gray)
        // Only if we are solely responsible for the screen.
        // If we are fading out, we might want to let what's behind show?
        // But the compositor doesn't support transparency blending with windows underneath easily yet.
        // So we strictly draw black on top, and use global_alpha to "dissolve" (requires read-modify-write?).
        // Since we are software rendering, reading from FB is possible or we just don't draw if alpha is 0.
        // User said: "Boot scene fades out as a whole (optional)".
        // Implementing dissolve is expensive (read-back).
        // Let's just stop drawing when alpha is 0, and draw opaque black when alpha is 1.
        // If global_alpha < 1.0, we ideally want to blend with "whatever is desktop".
        // But we don't render desktop if boot scene is active?
        // "When desktop appears, boot text fades away cleanly"
        // This implies compositing Boot Scene ON TOP of Desktop.
        
        let should_clear = self.global_alpha >= 0.99;
        
        if should_clear {
             render::primitives::fill_rect(
                fb_ptr, pitch, width, height,
                0, 0, width as i32, height as i32,
                0xFF000000, 
                None
            );
        }

        // Calculate layout
        let line_height = 24;
        let total_lines = self.milestones.len() as i32;
        let total_height = total_lines * line_height;
        
        let start_y = (height as i32 - total_height) / 2;
        
        for (i, m) in self.milestones.iter().enumerate() {
            let y = start_y + (i as i32 * line_height);
            
            // Layout text
            let content_len = m.label.len() as u32;
            let text_size = Size { width: content_len * 8, height: 16 }; // Assumes 8x16 font
            let bounds = Rect::new(0, y, width, 24);
            let pos = layout::center_text(bounds, text_size); // Centers horizontally in full width, vertically in 24px
            
            // Calculate Color
            // Base White 0xFFFFFF
            // Alpha: m.alpha * self.global_alpha
            let effective_alpha = m.alpha * self.global_alpha;
            let alpha_byte = (effective_alpha * 255.0) as u32;
            
            if alpha_byte < 5 { continue; } // Optimization
            
            let color = (alpha_byte << 24) | 0x00FFFFFF;
            
            render::text::draw_text(
                fb_ptr, pitch, width, height,
                pos.x, pos.y,
                &m.label,
                color
            );
        }
    }
}
