use crate::draw_cmd::DrawCmd;
use crate::painter::{Painter, CpuPainter, Clip};
use crate::scene_cache::BytespaceMappingCache;
use crate::scene::Rect;
use alloc::vec::Vec;
use thing_std::log_info;

#[derive(Default, Debug, Clone, Copy)]
pub struct ExecStats {
    pub cmds_total: u32,
    pub cmds_drawn: u32,
    pub cmds_skipped: u32,
    pub bytespace_maps: u32,
    pub bytespace_map_fails: u32,
    pub bad_cmds: u32,
    pub clip_pushes: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum ExecErrorKind {
    OutOfBounds,
    BadStride,
    BadLen,
    Unaligned,
    MapFailed,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Damage {
    pub rect: Option<Rect>,
}

impl Damage {
    pub fn add(&mut self, r: Rect) {
        if r.w == 0 || r.h == 0 { return; }
        self.rect = Some(match self.rect {
            Some(existing) => Rect::union(existing, r),
            None => r,
        });
    }
}

pub struct ExecOutput {
    pub stats: ExecStats,
    pub damage: Damage,
}

/// Execute a list of commands into a mutable buffer.
pub fn execute_cmds_into_scene(
    cmds: &[DrawCmd],
    scene_buffer: &mut [u32],
    width: u32,
    height: u32,
    mapping_cache: &mut BytespaceMappingCache,
) -> ExecOutput {
    let mut painter = CpuPainter::new(scene_buffer, width, height);
    let mut stats = ExecStats::default();
    let mut damage = Damage::default();
    
    // Initial clip is full screen
    let scene_rect = Rect { x: 0, y: 0, w: width, h: height };
    let mut current_clip = scene_rect; 
    let mut clip_stack: Vec<Rect> = Vec::with_capacity(4);

    // Initial painter clip logic is implicitly full screen, but let's be explicit
    painter.set_clip(Clip::from_rect(current_clip));

    for cmd in cmds {
        stats.cmds_total += 1;

        // 1. Compute bounds and cull
        let cmd_bounds = match cmd {
            DrawCmd::FillRect { rect, .. } => Some(*rect),
            DrawCmd::FillRectVGrad { rect, .. } => Some(*rect),
            DrawCmd::FillRoundedRect { rect, .. } => Some(*rect),
            DrawCmd::StrokeRoundedRect { rect, .. } => Some(*rect),
            DrawCmd::FillPanel { rect, .. } => Some(*rect),
            DrawCmd::BlitRgbaPremulBytespace { dst_x, dst_y, src_rect, .. } => {
                Some(Rect { x: *dst_x, y: *dst_y, w: src_rect.w, h: src_rect.h })
            }
            DrawCmd::TextRun { .. } => {
                 // For safety, let's not cull Text aggressively yet.
                 None
            }
            DrawCmd::Shadow { x, y, width, height, offset_x, offset_y, blur_radius, .. } => {
                let blur = *blur_radius as i32;
                let min_x = x + offset_x - blur;
                let min_y = y + offset_y - blur;
                let max_x = x + *width as i32 + offset_x + blur;
                let max_y = y + *height as i32 + offset_y + blur;
                 Some(Rect { 
                     x: min_x, 
                     y: min_y, 
                     w: (max_x - min_x).max(0) as u32, 
                     h: (max_y - min_y).max(0) as u32 
                 })
            }
            DrawCmd::Clear { .. } => Some(scene_rect),
            DrawCmd::SetClip { .. } => None, 
            DrawCmd::PushClip { .. } => None,
            DrawCmd::PopClip => None,
        };

        if let Some(bounds) = cmd_bounds {
            // Check if bounds intersect with current clip AND scene rect.
            let visible_clip = current_clip.intersect(scene_rect);
            if bounds.intersect(visible_clip).is_empty() {
                stats.cmds_skipped += 1;
                continue;
            }
        }

        match cmd {
            DrawCmd::FillRect { rect, color } => {
                painter.fill_rect(*rect, *color);
                damage.add(*rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::FillRectVGrad { rect, radius, top_color, bottom_color } => {
                painter.fill_rect_vgrad(*rect, *radius, *top_color, *bottom_color);
                damage.add(*rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::FillRoundedRect { rect, radius, color } => {
                painter.fill_rounded_rect(*rect, *radius, *color);
                damage.add(*rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::StrokeRoundedRect { rect, radius, thickness, color } => {
                painter.stroke_rounded_rect(*rect, *radius, *thickness, *color);
                damage.add(*rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::Clear { color } => {
                painter.clear(*color);
                damage.add(scene_rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::TextRun { x, y, text, color, font_size } => {
                crate::text::draw_text_on_painter(&mut painter, *x, *y, text, *color, *font_size);
                let w_est = (text.len() as f32 * font_size * 0.8) as u32; 
                damage.add(Rect { x: *x, y: *y, w: w_est, h: *font_size as u32 });
                stats.cmds_drawn += 1;
            }
            DrawCmd::BlitRgbaPremulBytespace { bytespace, src_rect, dst_x, dst_y, src_stride, src_len } => {
                 match validate_blit_buffer(mapping_cache, *bytespace, *src_len, *src_stride, src_rect) {
                     Ok(u32_buf) => {
                         painter.blit_rgba_alpha_rect(*dst_x, *dst_y, u32_buf, *src_stride, src_rect.w, src_rect.h);
                         damage.add(Rect { x: *dst_x, y: *dst_y, w: src_rect.w, h: src_rect.h });
                         stats.cmds_drawn += 1;
                     }
                     Err(_) => {
                         stats.bad_cmds += 1;
                         stats.bytespace_map_fails += 1;
                     }
                 }
            }
            DrawCmd::Shadow { x, y, width, height, radius, color, offset_x, offset_y, blur_radius } => {
                 painter.draw_shadow_mask(
                    *x, *y,
                     crate::shadow::ShadowMask::RoundedRect { 
                        width: *width, 
                        height: *height, 
                        radius: *radius 
                     },
                     crate::shadow::ShadowParams {
                         offset_x: *offset_x,
                         offset_y: *offset_y,
                         blur_radius: *blur_radius as u32,
                         color: *color,
                     }
                 );
                 let blur = *blur_radius as i32;
                 let min_x = x + offset_x - blur;
                 let min_y = y + offset_y - blur;
                 let max_x = x + *width as i32 + offset_x + blur;
                 let max_y = y + *height as i32 + offset_y + blur;
                 damage.add(Rect { 
                     x: min_x, 
                     y: min_y, 
                     w: (max_x - min_x).max(0) as u32, 
                     h: (max_y - min_y).max(0) as u32 
                 });
                 stats.cmds_drawn += 1;
            }
            DrawCmd::FillPanel { rect, radius, bg_rgba, title_bar_height } => {
                painter.fill_panel(*rect, *radius, *bg_rgba, *title_bar_height);
                damage.add(*rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::SetClip { rect } => {
                current_clip = *rect;
                painter.set_clip(Clip::from_rect(current_clip));
                stats.clip_pushes += 1;
            }
            DrawCmd::PushClip { rect } => {
                clip_stack.push(current_clip);
                current_clip = current_clip.intersect(*rect);
                painter.set_clip(Clip::from_rect(current_clip));
                stats.clip_pushes += 1;
            }
            DrawCmd::PopClip => {
                if let Some(prev) = clip_stack.pop() {
                    current_clip = prev;
                    painter.set_clip(Clip::from_rect(current_clip));
                }
            }
        }
    }
    
    ExecOutput { stats, damage }
}

fn validate_blit_buffer<'a>(
    mapping_cache: &'a mut BytespaceMappingCache, 
    bytespace: abi::ids::ThingId,
    src_len: usize, 
    src_stride: u32, 
    src_rect: &Rect
) -> Result<&'a [u32], ExecErrorKind> {
    
    if src_len % 4 != 0 {
        return Err(ExecErrorKind::BadLen);
    }

    let buf = mapping_cache.get_or_map_ro(bytespace, src_len)
        .ok_or(ExecErrorKind::MapFailed)?;
    
    if buf.len() < src_len {
        return Err(ExecErrorKind::OutOfBounds);
    }
    
    let bytes_per_px = 4usize;
    let stride_px = src_stride as usize;
    let h = src_rect.h as usize;
    let w = src_rect.w as usize;
    let sx = src_rect.x as usize;
    let sy = src_rect.y as usize;

    if stride_px < w {
        return Err(ExecErrorKind::BadStride);
    }

    let last_row = sy.checked_add(h.saturating_sub(1))
        .ok_or(ExecErrorKind::OutOfBounds)?;
    let last_px_index = last_row.checked_mul(stride_px)
        .and_then(|idx| idx.checked_add(sx))
        .and_then(|idx| idx.checked_add(w))
        .ok_or(ExecErrorKind::OutOfBounds)?;
    
    let required_bytes = last_px_index.checked_mul(bytes_per_px)
        .ok_or(ExecErrorKind::OutOfBounds)?;

    if required_bytes > buf.len() {
        return Err(ExecErrorKind::OutOfBounds);
    }
    
    if (buf.as_ptr() as usize) % 4 != 0 {
        return Err(ExecErrorKind::Unaligned);
    }

    let start_px = sy * stride_px + sx;
    
    let u32_full = unsafe {
         core::slice::from_raw_parts(buf.as_ptr() as *const u32, buf.len() / 4)
    };
    
    if start_px >= u32_full.len() {
        if w == 0 || h == 0 { return Ok(&[]); }
        return Err(ExecErrorKind::OutOfBounds);
    }

    Ok(&u32_full[start_px..])
}
