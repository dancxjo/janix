//! Chunked scene executor - processes draw commands in bounded chunks.
//!
//! This allows the main loop to present cursor updates between chunks,
//! keeping the UI responsive during long scene rebuilds.

use crate::draw_cmd::DrawCmd;
use crate::painter::{CpuPainter, Painter, Clip};
use crate::scene::Rect;
use crate::scene_cache::BytespaceMappingCache;
use crate::assets::bitmap::BitmapStore;
use crate::executor::{ExecStats, ExecOutput, Damage};
use alloc::vec::Vec;
use thing_std::log_info;

/// Chunked executor - processes commands in batches with budget control.
/// 
/// Note: This uses a "step and rebuild painter" approach to satisfy the borrow
/// checker. Each step() call creates a fresh CpuPainter, which is fine because
/// CpuPainter is stateless (just a view into the buffer).
pub struct ChunkedExecutor<'a> {
    cmds: &'a [DrawCmd],
    index: usize,
    
    // Scene state
    width: u32,
    height: u32,
    
    // Clip stack state (persists across chunks)
    scene_rect: Rect,
    current_clip: Rect,
    clip_stack: Vec<Rect>,
    
    // Execution state
    stats: ExecStats,
    damage: Damage,
    
    // Progress tracking
    last_progress_log: usize,
}

impl<'a> ChunkedExecutor<'a> {
    /// Create a new chunked executor.
    pub fn new(
        cmds: &'a [DrawCmd],
        width: u32,
        height: u32,
    ) -> Self {
        let scene_rect = Rect { x: 0, y: 0, w: width, h: height };
        Self {
            cmds,
            index: 0,
            width,
            height,
            scene_rect,
            current_clip: scene_rect,
            clip_stack: Vec::with_capacity(4),
            stats: ExecStats::default(),
            damage: Damage::default(),
            last_progress_log: 0,
        }
    }
    
    /// Execute up to `budget_cmds` commands.
    /// Returns `true` if all commands are complete, `false` if more work remains.
    pub fn step(
        &mut self,
        scene_buffer: &mut [u32],
        mapping_cache: &mut BytespaceMappingCache,
        bitmap_store: &BitmapStore,
        budget_cmds: usize,
    ) -> bool {
        if self.is_complete() {
            return true;
        }
        
        // Safety check
        if scene_buffer.len() < (self.width as usize) * (self.height as usize) {
            log_info("BLOOM: ChunkedExecutor buffer too small");
            return true;
        }
        
        let mut painter = CpuPainter::new(scene_buffer, self.width, self.height);
        painter.set_clip(Clip::from_rect(self.current_clip));
        
        let start_index = self.index;
        let end_index = (self.index + budget_cmds).min(self.cmds.len());
        
        for i in start_index..end_index {
            let cmd = &self.cmds[i];
            self.execute_single_cmd(&mut painter, cmd, mapping_cache, bitmap_store);
            self.index = i + 1;
        }
        
        // Progress logging every 2000 commands
        if self.index >= self.last_progress_log + 2000 {
            log_info(&alloc::format!(
                "BLOOM: exec progress {}/{} cmds",
                self.index, self.cmds.len()
            ));
            self.last_progress_log = self.index;
        }
        
        self.is_complete()
    }
    
    /// Check if all commands have been executed.
    pub fn is_complete(&self) -> bool {
        self.index >= self.cmds.len()
    }
    
    /// Get current execution index and total.
    pub fn progress(&self) -> (usize, usize) {
        (self.index, self.cmds.len())
    }
    
    /// Get accumulated stats.
    pub fn stats(&self) -> ExecStats {
        self.stats
    }
    
    /// Get accumulated damage.
    pub fn damage(&self) -> Damage {
        self.damage
    }
    
    /// Finalize and return output.
    pub fn finish(self) -> ExecOutput {
        ExecOutput {
            stats: self.stats,
            damage: self.damage,
        }
    }
    
    fn execute_single_cmd(
        &mut self, 
        painter: &mut CpuPainter, 
        cmd: &DrawCmd,
        _mapping_cache: &mut BytespaceMappingCache,
        bitmap_store: &BitmapStore,
    ) {
        self.stats.cmds_total += 1;
        
        // Bounds culling
        if let Some(bounds) = cmd.bounds() {
            if bounds.intersect(self.current_clip).is_empty() {
                self.stats.cmds_skipped += 1;
                return;
            }
        }
        
        match cmd {
            DrawCmd::FillRect { rect, color } => {
                painter.fill_rect(*rect, *color);
                self.damage.add(*rect, self.scene_rect);
                self.stats.cmds_drawn += 1;
            }
            DrawCmd::FillRectVGrad { rect, radius, top_color, bottom_color } => {
                painter.fill_rect_vgrad(*rect, *radius, *top_color, *bottom_color);
                self.damage.add(*rect, self.scene_rect);
                self.stats.cmds_drawn += 1;
            }
            DrawCmd::FillRoundedRect { rect, radius, color } => {
                painter.fill_rounded_rect(*rect, *radius, *color);
                self.damage.add(*rect, self.scene_rect);
                self.stats.cmds_drawn += 1;
            }
            DrawCmd::StrokeRoundedRect { rect, radius, thickness, color } => {
                painter.stroke_rounded_rect(*rect, *radius, *thickness, *color);
                self.damage.add(*rect, self.scene_rect);
                self.stats.cmds_drawn += 1;
            }
            DrawCmd::StrokeRoundedRectTop { rect, radius, thickness, color } => {
                painter.stroke_rounded_rect_top(*rect, *radius, *thickness, *color);
                self.damage.add(*rect, self.scene_rect);
                self.stats.cmds_drawn += 1;
            }
            DrawCmd::Clear { color } => {
                painter.clear(*color);
                self.damage.add(self.scene_rect, self.scene_rect);
                self.stats.cmds_drawn += 1;
            }
            DrawCmd::TextRun { x, y, text, color, font_size } => {
                crate::text::draw_text_on_painter(painter, *x, *y, text, *color, *font_size);
                let avg_advance = (*font_size * 0.8) as u32;
                let w_est = text.len() as u32 * avg_advance;
                let h_est = *font_size as u32;
                self.damage.add(Rect { x: *x, y: *y, w: w_est, h: h_est }, self.scene_rect);
                self.stats.cmds_drawn += 1;
            }
            DrawCmd::BlitRgbaPremulBytespace { src_rect, dst_x, dst_y, .. } => {
                // Simplified - full blit handling would require mapping_cache integration
                self.stats.cmds_drawn += 1;
                self.damage.add(Rect { x: *dst_x, y: *dst_y, w: src_rect.w, h: src_rect.h }, self.scene_rect);
            }
            DrawCmd::Shadow { x, y, width, height, radius, color, offset_x, offset_y, blur_radius, top_only } => {
                let mask = if *top_only {
                    crate::shadow::ShadowMask::RoundedRectTop { 
                        width: *width, 
                        height: *height, 
                        radius: *radius 
                    }
                } else {
                    crate::shadow::ShadowMask::RoundedRect { 
                        width: *width, 
                        height: *height, 
                        radius: *radius 
                    }
                };
                painter.draw_shadow_mask(
                    *x, *y,
                    mask,
                    crate::shadow::ShadowParams {
                        offset_x: *offset_x,
                        offset_y: *offset_y,
                        blur_radius: *blur_radius as u32,
                        color: *color,
                    }
                );
                if let Some(r) = cmd.bounds() {
                    self.damage.add(r, self.scene_rect);
                }
                self.stats.cmds_drawn += 1;
            }
            DrawCmd::FillPanel { rect, radius, bg_rgba, title_bar_height } => {
                painter.fill_panel(*rect, *radius, *bg_rgba, *title_bar_height);
                self.damage.add(*rect, self.scene_rect);
                self.stats.cmds_drawn += 1;
            }
            DrawCmd::SetClip { rect } => {
                self.current_clip = rect.intersect(self.scene_rect);
                painter.set_clip(Clip::from_rect(self.current_clip));
                self.stats.clip_changes += 1;
            }
            DrawCmd::PushClip { rect } => {
                self.clip_stack.push(self.current_clip);
                self.current_clip = self.current_clip.intersect(*rect);
                painter.set_clip(Clip::from_rect(self.current_clip));
                self.stats.clip_changes += 1;
            }
            DrawCmd::PopClip => {
                if let Some(prev) = self.clip_stack.pop() {
                    self.current_clip = prev;
                    painter.set_clip(Clip::from_rect(self.current_clip));
                    self.stats.clip_changes += 1;
                } else {
                    self.stats.clip_underflows += 1;
                    self.stats.bad_cmds += 1;
                }
            }
            DrawCmd::TileBitmap { dst, bitmap, origin, .. } => {
                if let Some(bmp) = bitmap_store.get(*bitmap) {
                    painter.draw_tiled_bitmap(*dst, bmp, *origin);
                    self.damage.add(*dst, self.scene_rect);
                    self.stats.cmds_drawn += 1;
                } else {
                    self.stats.bad_cmds += 1;
                }
            }
        }
    }
}
