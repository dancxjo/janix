use crate::draw_cmd::DrawCmd;
use crate::painter::{Painter, CpuPainter, Clip};
use crate::scene_cache::{BytespaceMappingCache, MapResult};
use crate::scene::{Rect, Point};
use crate::assets::bitmap::{Bitmap, BitmapStore};
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
    pub clip_changes: u32,
    pub clip_underflows: u32,
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
    pub fn add(&mut self, mut r: Rect, scene_rect: Rect) {
        if r.w == 0 || r.h == 0 { return; }
        // Clamp to scene to avoid outlandish damage reporting
        r = r.intersect(scene_rect);
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

struct ValidatedBlit<'a> {
    src: &'a [u32],
    stride: u32,
    w: u32,
    h: u32,
}

/// Execute a list of commands into a mutable buffer.
pub fn execute_cmds_into_scene(
    cmds: &[DrawCmd],
    scene_buffer: &mut [u32],
    width: u32,
    height: u32,
    mapping_cache: &mut BytespaceMappingCache,
    bitmap_store: &BitmapStore,
) -> ExecOutput {
    let mut stats = ExecStats::default();

    // 0. Safety Guard
    if scene_buffer.len() < (width as usize) * (height as usize) {
        stats.bad_cmds = cmds.len() as u32; // Mark all as bad/skipped
        log_info("BLOOM: Exec buffer too small for scene dimensions");
        return ExecOutput { stats, damage: Damage::default() };
    }

    let mut painter = CpuPainter::new(scene_buffer, width, height);
    let mut damage = Damage::default();
    
    // Initial clip is full screen
    let scene_rect = Rect { x: 0, y: 0, w: width, h: height };
    let mut current_clip = scene_rect; 
    let mut clip_stack: Vec<Rect> = Vec::with_capacity(4);

    // Initial painter clip logic
    painter.set_clip(Clip::from_rect(current_clip));

    for cmd in cmds {
        stats.cmds_total += 1;

        // 1. Compute bounds and cull
        let cmd_bounds = match cmd {
            _ => cmd.bounds(), // Now implemented for all, including Shadow
        };

        if let Some(bounds) = cmd_bounds {
            // Check if bounds intersect with current clip.
            // Note: `current_clip` is already intersected with `scene_rect` by our logic below.
            let visible_clip = current_clip; 
            
            // `bounds.intersect(visible_clip)` handles disjoint by returning empty rect.
            // `is_empty()` checks w==0 || h==0.
            if bounds.intersect(visible_clip).is_empty() {
                stats.cmds_skipped += 1;
                continue;
            }
        }

        match cmd {
            DrawCmd::FillRect { rect, color } => {
                painter.fill_rect(*rect, *color);
                damage.add(*rect, scene_rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::FillRectVGrad { rect, radius, top_color, bottom_color } => {
                painter.fill_rect_vgrad(*rect, *radius, *top_color, *bottom_color);
                damage.add(*rect, scene_rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::FillRoundedRect { rect, radius, color } => {
                painter.fill_rounded_rect(*rect, *radius, *color);
                damage.add(*rect, scene_rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::StrokeRoundedRect { rect, radius, thickness, color } => {
                painter.stroke_rounded_rect(*rect, *radius, *thickness, *color);
                damage.add(*rect, scene_rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::StrokeRoundedRectTop { rect, radius, thickness, color } => {
                painter.stroke_rounded_rect_top(*rect, *radius, *thickness, *color);
                damage.add(*rect, scene_rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::Clear { color } => {
                painter.clear(*color);
                damage.add(scene_rect, scene_rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::TextRun { x, y, text, color, font_size } => {
                crate::text::draw_text_on_painter(&mut painter, *x, *y, text, *color, *font_size);
                // Integer estimation for damage
                let avg_advance = (*font_size * 0.8) as u32; 
                let w_est = text.len() as u32 * avg_advance;
                let h_est = *font_size as u32;
                // Assuming (x,y) is top-left approx for damage purposes (aligned with bounds check?)
                damage.add(Rect { x: *x, y: *y, w: w_est, h: h_est }, scene_rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::BlitRgbaPremulBytespace { bytespace, src_rect, dst_x, dst_y, src_stride, src_len } => {
                 match validate_blit_buffer(mapping_cache, *bytespace, *src_len, *src_stride, src_rect) {
                     Ok((blit, was_new_map)) => {
                         if was_new_map {
                             stats.bytespace_maps += 1;
                         }
                         painter.blit_rgba_alpha_rect(
                            *dst_x, 
                            *dst_y, 
                            blit.src, 
                            blit.stride, 
                            blit.w, 
                            blit.h
                        );
                         damage.add(Rect { x: *dst_x, y: *dst_y, w: src_rect.w, h: src_rect.h }, scene_rect);
                         stats.cmds_drawn += 1;
                     }
                     Err(_) => {
                         stats.bad_cmds += 1;
                         stats.bytespace_map_fails += 1;
                     }
                 }
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
                 // Reuse bounds logic for damage
                 if let Some(r) = cmd.bounds() {
                     damage.add(r, scene_rect);
                 }
                 stats.cmds_drawn += 1;
            }
            DrawCmd::FillPanel { rect, radius, bg_rgba, title_bar_height } => {
                painter.fill_panel(*rect, *radius, *bg_rgba, *title_bar_height);
                damage.add(*rect, scene_rect);
                stats.cmds_drawn += 1;
            }
            DrawCmd::SetClip { rect } => {
                // Always clamp to scene
                current_clip = rect.intersect(scene_rect);
                painter.set_clip(Clip::from_rect(current_clip));
                stats.clip_changes += 1;
            }
            DrawCmd::PushClip { rect } => {
                clip_stack.push(current_clip);
                // "PushClip expects rect already intersected with scene or not" -> normalize:
                // intersect with current (which is intersected with scene) AND new rect.
                // intersect returns empty if disjoint.
                current_clip = current_clip.intersect(*rect);
                painter.set_clip(Clip::from_rect(current_clip));
                stats.clip_changes += 1;
            }
            DrawCmd::PopClip => {
                if let Some(prev) = clip_stack.pop() {
                    current_clip = prev;
                    painter.set_clip(Clip::from_rect(current_clip));
                    stats.clip_changes += 1;
                } else {
                    stats.clip_underflows += 1;
                    stats.bad_cmds += 1;
                    log_info("BLOOM: Clip underflow");
                }
            }
            DrawCmd::TileBitmap { dst, bitmap, bmp_w: _, bmp_h: _, origin, opacity: _ } => {
                if let Some(bmp) = bitmap_store.get(*bitmap) {
                    painter.draw_tiled_bitmap(*dst, bmp, *origin);
                    damage.add(*dst, scene_rect);
                    stats.cmds_drawn += 1;
                } else {
                    stats.bad_cmds += 1;
                }
            }
        }
    }
    
    ExecOutput { stats, damage }
}

#[cfg(test)]
mod executor_tests;

fn validate_blit_buffer<'a>(
    mapping_cache: &'a mut BytespaceMappingCache, 
    bytespace: abi::ids::ThingId,
    src_len: usize, 
    src_stride: u32, 
    src_rect: &Rect
) -> Result<(ValidatedBlit<'a>, bool), ExecErrorKind> {
    
    if src_len % 4 != 0 {
        return Err(ExecErrorKind::BadLen);
    }

    // Use specific invariants for Rect in blits
    if src_rect.x < 0 || src_rect.y < 0 {
        return Err(ExecErrorKind::OutOfBounds);
    }

    let map_res = mapping_cache.get_or_map_ro(bytespace, src_len)
        .ok_or(ExecErrorKind::MapFailed)?;
    
    let is_new = matches!(map_res, MapResult::Mapped(_));
    let buf = map_res.as_ref(); // deref to &[u8]

    if buf.len() < src_len {
        return Err(ExecErrorKind::OutOfBounds);
    }
    
    let bytes_per_px = 4usize;
    let stride_px = src_stride as usize;
    let h = src_rect.h as usize;
    let w = src_rect.w as usize;
    let sx = src_rect.x as usize;
    let sy = src_rect.y as usize; // safe cast due to checks above

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
        if w == 0 || h == 0 { 
            return Ok((ValidatedBlit { 
                src: &[], 
                stride: src_stride,
                w: 0,
                h: 0
            }, is_new)); 
        }
        return Err(ExecErrorKind::OutOfBounds);
    }

    Ok((ValidatedBlit {
        src: &u32_full[start_px..],
        stride: src_stride,
        w: src_rect.w,
        h: src_rect.h,
    }, is_new))
}
