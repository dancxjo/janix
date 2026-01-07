//! Cursor overlay presenter - composites cursor sprite over scene buffer.
//!
//! ## THE ONLY IMMEDIATE-MODE EXCEPTION
//!
//! The cursor overlay is the **ONLY** code in Bloom that is allowed to write
//! pixels immediately (outside the record-execute pipeline). This is necessary
//! for cursor responsiveness during long scene rebuilds.
//!
//! ## The Classic Sprite Trick
//!
//! 1. Restore old cursor region (copy from scene_buffer → framebuffer)
//! 2. Draw cursor sprite at new position into framebuffer
//! 3. Track last bounds for next restore
//!
//! This decouples cursor rendering from scene execution, keeping the cursor
//! responsive even during 35-second scene rebuilds.
//!
//! ## Important Constraints
//!
//! - Must NOT invalidate the scene command list
//! - Must NOT write to scene_buffer (only framebuffer)
//! - Must track bounds accurately for clean restore

use crate::scene::Rect;
use crate::painter::{CpuPainter, Painter, Clip};
use crate::assets::cursor::CursorFrame;

/// Cursor overlay state - tracks last presented position for dirty rect restore.
/// 
/// ## Invariant: Must track prev_bounds internally
/// 
/// This struct MUST maintain `last_bounds` to restore the previous cursor region
/// from scene_buffer before drawing the new cursor. Without this, cursor trails
/// would appear as the old cursor sprite wouldn't be erased.
pub struct CursorOverlay {
    last_pos: Option<(i32, i32)>,
    last_bounds: Option<Rect>,
    screen_w: u32,
    screen_h: u32,
}

impl CursorOverlay {
    pub fn new(screen_w: u32, screen_h: u32) -> Self {
        Self {
            last_pos: None,
            last_bounds: None,
            screen_w,
            screen_h,
        }
    }

    /// Present cursor at new position, restoring old region from scene_buffer.
    ///
    /// Returns the dirty rect that was modified (union of old + new bounds).
    pub fn present(
        &mut self,
        scene_buffer: &[u32],
        framebuffer: &mut [u32],
        new_x: i32,
        new_y: i32,
        frame: Option<&CursorFrame>,
    ) -> Option<Rect> {
        let new_bounds = self.cursor_bounds(frame, new_x, new_y);
        
        // Compute dirty region (union of old and new bounds)
        let dirty = match self.last_bounds {
            Some(old) => Some(Rect::union(old, new_bounds)),
            None => Some(new_bounds),
        };
        
        // 1. Restore old cursor region from scene_buffer
        if let Some(old_bounds) = self.last_bounds {
            self.copy_region(scene_buffer, framebuffer, old_bounds);
        }
        
        // 2. Draw cursor sprite at new position
        {
            let mut painter = CpuPainter::new(framebuffer, self.screen_w, self.screen_h);
            painter.set_clip(Clip::full(self.screen_w, self.screen_h));
            
            if let Some(f) = frame {
                // Draw shadow first (if enabled)
                #[cfg(feature = "shadows")]
                painter.blit_rgba_alpha(
                    new_x - f.hotspot_x + f.shadow_offset_x,
                    new_y - f.hotspot_y + f.shadow_offset_y,
                    &f.shadow_pixels,
                    f.width,
                    f.height,
                );
                painter.draw_cursor_frame(f, new_x, new_y);
            } else {
                painter.draw_fallback_cursor(new_x, new_y);
            }
        }
        
        // 3. Update tracking state
        self.last_pos = Some((new_x, new_y));
        self.last_bounds = Some(new_bounds);
        
        dirty
    }

    /// Copy a region from scene_buffer to framebuffer.
    fn copy_region(&self, src: &[u32], dst: &mut [u32], rect: Rect) {
        // Clip to screen bounds
        let x0 = rect.x.max(0) as u32;
        let y0 = rect.y.max(0) as u32;
        let x1 = ((rect.x + rect.w as i32) as u32).min(self.screen_w);
        let y1 = ((rect.y + rect.h as i32) as u32).min(self.screen_h);
        
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        
        let stride = self.screen_w as usize;
        for row in y0..y1 {
            let row_start = row as usize * stride + x0 as usize;
            let row_end = row as usize * stride + x1 as usize;
            if row_end <= src.len() && row_end <= dst.len() {
                dst[row_start..row_end].copy_from_slice(&src[row_start..row_end]);
            }
        }
    }

    /// Compute cursor bounds including shadow.
    fn cursor_bounds(&self, frame: Option<&CursorFrame>, px: i32, py: i32) -> Rect {
        match frame {
            Some(f) => {
                let sprite_x = px - f.hotspot_x;
                let sprite_y = py - f.hotspot_y;
                let shadow_x = sprite_x + f.shadow_offset_x;
                let shadow_y = sprite_y + f.shadow_offset_y;
                let min_x = sprite_x.min(shadow_x);
                let min_y = sprite_y.min(shadow_y);
                let max_x = (sprite_x + f.width as i32).max(shadow_x + f.width as i32);
                let max_y = (sprite_y + f.height as i32).max(shadow_y + f.height as i32);
                Rect { x: min_x, y: min_y, w: (max_x - min_x) as u32, h: (max_y - min_y) as u32 }
            }
            None => Rect { x: px, y: py, w: 10, h: 11 },
        }
    }

    /// Get last presented bounds (for external damage tracking).
    pub fn last_bounds(&self) -> Option<Rect> {
        self.last_bounds
    }
}
