//! CPU framebuffer implementation of the Painter trait.

use crate::assets::cursor::CursorFrame;
use crate::scene::{Rect, Point};
use crate::assets::bitmap::Bitmap;
use crate::shadow::{ShadowMask, ShadowParams};
use super::{Clip, Painter};
use alloc::vec;
use alloc::vec::Vec;

/// CPU-based painter that writes to a framebuffer slice.
pub struct CpuPainter<'a> {
    buf: &'a mut [u32],
    w: u32,
    h: u32,
    clip: Clip,
    clip_stack: Vec<Clip>,
    damage: Option<Rect>,
}

impl<'a> CpuPainter<'a> {
    /// Create a new CPU painter for the given framebuffer.
    pub fn new(buf: &'a mut [u32], w: u32, h: u32) -> Self {
        Self {
            buf,
            w,
            h,
            clip: Clip::full(w, h),
            clip_stack: Vec::new(),
            damage: None,
        }
    }
    
    /// Create a new CPU painter for scene buffer writing (debug-checked).
    /// 
    /// This constructor adds a debug assertion to ensure we're in the execute phase.
    /// Use this when creating a painter for scene_buffer in executors.
    /// Use `new()` for framebuffer or other non-scene uses.
    pub fn new_for_scene(buf: &'a mut [u32], w: u32, h: u32) -> Self {
        #[cfg(debug_assertions)]
        debug_assert!(
            crate::executor::is_executing(),
            "BLOOM DOCTRINE VIOLATION: CpuPainter::new_for_scene() called outside execute phase. \
             Scene buffer writes must only occur during execute_cmds_into_scene or ChunkedExecutor::step."
        );
        
        Self::new(buf, w, h)
    }

    /// Intersect a rect with the current clip.
    #[inline]
    fn clip_rect(&self, r: Rect) -> Rect {
        r.intersect(self.clip.rect)
    }

    /// Merge a rect into accumulated damage.
    #[inline]
    fn merge_damage(&mut self, r: Rect) {
        if r.w == 0 || r.h == 0 {
            return;
        }
        self.damage = Some(if let Some(d) = self.damage {
            Rect::union(d, r)
        } else {
            r
        });
    }

    /// Get pixel index, returns None if out of bounds.
    #[inline]
    fn pixel_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return None;
        }
        Some((y as u32 * self.w + x as u32) as usize)
    }

    /// Write a pixel at (x, y) with bounds checking.
    #[inline]
    fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        if let Some(idx) = self.pixel_idx(x, y) {
            self.buf[idx] = color;
        }
    }

    /// Blend a pixel at (x, y) using alpha blending.
    #[inline]
    fn blend_at(&mut self, x: i32, y: i32, src: u32) {
        if let Some(idx) = self.pixel_idx(x, y) {
            let dst = self.buf[idx];
            self.buf[idx] = blend_pixel(src, dst);
        }
    }

    /// Check if a point is inside a rounded rect corner.
    fn in_round(&self, x: i32, y: i32, w: u32, h: u32, radius: u16) -> bool {
        if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
            return false;
        }
        if radius == 0 {
            return true;
        }
        let r = radius as i32;
        let w_i = w as i32;
        let h_i = h as i32;
        let cx = if x < r {
            r - 1
        } else if x >= w_i - r {
            w_i - r
        } else {
            x
        };
        let cy = if y < r {
            r - 1
        } else if y >= h_i - r {
            h_i - r
        } else {
            y
        };
        let dx = x - cx;
        let dy = y - cy;
        dx * dx + dy * dy <= r * r
    }

    /// Check if a point is inside a rounded rect corner (top only).
    fn in_round_top(&self, x: i32, y: i32, w: u32, h: u32, radius: u16) -> bool {
        if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
            return false;
        }
        if radius == 0 {
            return true;
        }
        let r = radius as i32;

        // Bottom area is always valid (square corners)
        if y >= r {
            return true;
        }
        
        // Top corners check
        let w_i = w as i32;
        
        let cx = if x < r {
            r - 1
        } else if x >= w_i - r {
            w_i - r
        } else {
            // Middle top area
            return true;
        };
        
        let cy = r - 1;
        let dx = x - cx;
        let dy = y - cy;
        
        dx * dx + dy * dy <= r * r
    }

    /// Adjust a color by a brightness delta.
    fn adjust_color(&self, color: u32, delta: i16) -> u32 {
        let a = (color >> 24) & 0xFF;
        let r = (((color >> 16) & 0xFF) as i16 + delta).clamp(0, 255) as u32;
        let g = (((color >> 8) & 0xFF) as i16 + delta).clamp(0, 255) as u32;
        let b = ((color & 0xFF) as i16 + delta).clamp(0, 255) as u32;
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl<'a> Painter for CpuPainter<'a> {
    fn screen_size(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    fn set_clip(&mut self, clip: Clip) {
        self.clip = clip;
    }

    fn clip(&self) -> Clip {
        self.clip
    }

    fn push_clip(&mut self, rect: Rect) {
        self.clip_stack.push(self.clip);
        self.clip.rect = self.clip.rect.intersect(rect);
    }

    fn pop_clip(&mut self) {
        if let Some(prev) = self.clip_stack.pop() {
            self.clip = prev;
        }
    }

    fn damage(&mut self, rect: Rect) {
        self.merge_damage(rect);
    }

    fn take_damage(&mut self) -> Option<Rect> {
        self.damage.take()
    }

    fn clear(&mut self, color: u32) {
        let clip = self.clip_rect(Rect { x: 0, y: 0, w: self.w, h: self.h });
        for y in clip.y..(clip.y + clip.h as i32) {
            for x in clip.x..(clip.x + clip.w as i32) {
                self.put_pixel(x, y, color);
            }
        }
        self.merge_damage(clip);
    }

    fn fill_rect(&mut self, rect: Rect, color: u32) {
        let clip = self.clip_rect(rect);
        for y in clip.y..(clip.y + clip.h as i32) {
            for x in clip.x..(clip.x + clip.w as i32) {
                self.put_pixel(x, y, color);
            }
        }
        self.merge_damage(clip);
    }

    fn fill_rect_vgrad(&mut self, rect: Rect, radius: u16, top_color: u32, bottom_color: u32) {
        let clip = self.clip_rect(rect);
        let h = rect.h.max(1) as i32;

        let r1 = ((top_color >> 16) & 0xFF) as i32;
        let g1 = ((top_color >> 8) & 0xFF) as i32;
        let b1 = (top_color & 0xFF) as i32;
        let a1 = ((top_color >> 24) & 0xFF) as i32;

        let r2 = ((bottom_color >> 16) & 0xFF) as i32;
        let g2 = ((bottom_color >> 8) & 0xFF) as i32;
        let b2 = (bottom_color & 0xFF) as i32;
        let a2 = ((bottom_color >> 24) & 0xFF) as i32;
        
        // Precompute radius squared for corner checking
        let r_sq = (radius as i32) * (radius as i32);
        let r = radius as i32;
        let cx1 = rect.x + r;
        let cx2 = rect.x + rect.w as i32 - r;
        // let cy2 = rect.y + r; // same as cy1

        for y in clip.y..(clip.y + clip.h as i32) {
            let local_y = y - rect.y;

            // Check if we are in the rounded corner zone (top rows)
            // Optimization: only check corners if radius > 0 and we are in top r rows
            let check_corners = radius > 0 && local_y < r;

            let diff_y = if check_corners { r - local_y - 1 } else { 0 }; // distance from center y
            let diff_y_sq = diff_y * diff_y;

            // Linear interpolate based on Y position in rect
            let p = (local_y * 256) / h; // 0..256 fixed point
            let inv_p = 256 - p;

            let tr = (r1 * inv_p + r2 * p) / 256;
            let tg = (g1 * inv_p + g2 * p) / 256;
            let tb = (b1 * inv_p + b2 * p) / 256;
            let ta = (a1 * inv_p + a2 * p) / 256;
            
            let color = ((ta as u32) << 24) | ((tr as u32) << 16) | ((tg as u32) << 8) | (tb as u32);

            for x in clip.x..(clip.x + clip.w as i32) {
                if check_corners {
                    // Top Left
                    if x < cx1 {
                        let diff_x = cx1 - x - 1;
                        if diff_x * diff_x + diff_y_sq > r_sq {
                            continue;
                        }
                    }
                    // Top Right
                    else if x >= cx2 {
                        let diff_x = x - cx2;
                        if diff_x * diff_x + diff_y_sq > r_sq {
                            continue;
                        }
                    }
                }
                self.put_pixel(x, y, color);
            }
        }
        self.merge_damage(clip);
    }

    fn fill_rounded_rect(&mut self, rect: Rect, radius: u16, color: u32) {
        let clip = self.clip_rect(rect);
        for y in clip.y..(clip.y + clip.h as i32) {
            let local_y = y - rect.y;
            for x in clip.x..(clip.x + clip.w as i32) {
                let local_x = x - rect.x;
                if self.in_round(local_x, local_y, rect.w, rect.h, radius) {
                    self.put_pixel(x, y, color);
                }
            }
        }
        self.merge_damage(clip);
    }

    fn fill_panel(&mut self, rect: Rect, radius: u16, base: u32, stripe_height: Option<i32>) {
        let clip = self.clip_rect(rect);
        let light = self.adjust_color(base, 6);
        let dark = self.adjust_color(base, -8);
        let h = rect.h.max(1) as i32;

        // Main fill with gradient
        for dy in 0..clip.h as i32 {
            let y = clip.y + dy;
            let local_y = y - rect.y;
            let dist = (local_y - h / 2).abs();
            let grad = ((h / 2 - dist) * 4 / (h / 2).max(1)).clamp(0, 4) as i16 - 2;
            let row_color = self.adjust_color(base, grad);

            for dx in 0..clip.w as i32 {
                let x = clip.x + dx;
                let local_x = x - rect.x;
                if self.in_round(local_x, local_y, rect.w, rect.h, radius) {
                    self.put_pixel(x, y, row_color);
                }
            }
        }

        // Bevel highlights
        for dx in 0..clip.w as i32 {
            let x = clip.x + dx;
            let local_x = x - rect.x;

            let top_y = rect.y;
            if top_y >= clip.y && top_y < (clip.y + clip.h as i32) {
                if self.in_round(local_x, 0, rect.w, rect.h, radius) {
                    self.put_pixel(x, top_y, light);
                }
            }

            let bottom_y = rect.y + rect.h as i32 - 1;
            if bottom_y >= clip.y && bottom_y < (clip.y + clip.h as i32) {
                if self.in_round(local_x, rect.h as i32 - 1, rect.w, rect.h, radius) {
                    self.put_pixel(x, bottom_y, dark);
                }
            }
        }

        // Title stripe
        if let Some(hs) = stripe_height {
            let title_h = hs.min(rect.h as i32).max(0);
            let stripe_a = self.adjust_color(base, -3);
            let y0 = rect.y.max(clip.y);
            let y1 = (rect.y + title_h).min(clip.y + clip.h as i32);
            for y in y0..y1 {
                let local_y = y - rect.y;
                if local_y % 2 != 0 {
                    continue;
                }
                for x in clip.x..(clip.x + clip.w as i32) {
                    let local_x = x - rect.x;
                    if self.in_round(local_x, local_y, rect.w, rect.h, radius) {
                        self.put_pixel(x, y, stripe_a);
                    }
                }
            }
        }

        self.merge_damage(clip);
    }

    fn stroke_rounded_rect(&mut self, rect: Rect, radius: u16, thickness: u16, color: u32) {
        let clip = self.clip_rect(rect);
        let outer_r = radius;
        let inner_r = radius.saturating_sub(thickness);
        let thick = thickness as i32;

        for y in clip.y..(clip.y + clip.h as i32) {
            let local_y = y - rect.y;
            for x in clip.x..(clip.x + clip.w as i32) {
                let local_x = x - rect.x;

                if !self.in_round(local_x, local_y, rect.w, rect.h, outer_r) {
                    continue;
                }

                let inner_x = local_x - thick;
                let inner_y = local_y - thick;
                let inner_w = rect.w.saturating_sub((thick * 2) as u32);
                let inner_h = rect.h.saturating_sub((thick * 2) as u32);

                if inner_x >= 0
                    && inner_y >= 0
                    && inner_x < inner_w as i32
                    && inner_y < inner_h as i32
                    && self.in_round(inner_x, inner_y, inner_w, inner_h, inner_r)
                {
                    continue;
                }

                self.put_pixel(x, y, color);
            }
        }
        self.merge_damage(clip);
    }
    
    fn stroke_rounded_rect_top(&mut self, rect: Rect, radius: u16, thickness: u16, color: u32) {
        let clip = self.clip_rect(rect);
        let outer_r = radius;
        let inner_r = radius.saturating_sub(thickness);
        let thick = thickness as i32;
        
        let inner_w = rect.w.saturating_sub((thick * 2) as u32);
        let inner_h = rect.h.saturating_sub((thick * 2) as u32);

        for y in clip.y..(clip.y + clip.h as i32) {
            let local_y = y - rect.y;
            for x in clip.x..(clip.x + clip.w as i32) {
                let local_x = x - rect.x;

                // Must be inside outer top-rounded shape
                if !self.in_round_top(local_x, local_y, rect.w, rect.h, outer_r) {
                    continue;
                }

                // If strictly inside inner shape, skip (hollow)
                // Use in_round_top logic for inner too
                let inner_x = local_x - thick;
                let inner_y = local_y - thick;
                
                if inner_x >= 0
                    && inner_y >= 0
                    && inner_x < inner_w as i32
                    && inner_y < inner_h as i32
                    && self.in_round_top(inner_x, inner_y, inner_w, inner_h, inner_r)
                {
                    continue;
                }

                self.put_pixel(x, y, color);
            }
        }
        self.merge_damage(clip);
    }

    fn blit_rgba(&mut self, dst_x: i32, dst_y: i32, src: &[u32], src_w: u32, src_h: u32) {
        let dst_rect = Rect { x: dst_x, y: dst_y, w: src_w, h: src_h };
        let clip = self.clip_rect(dst_rect);

        for y in clip.y..(clip.y + clip.h as i32) {
            let src_y = (y - dst_y) as u32;
            for x in clip.x..(clip.x + clip.w as i32) {
                let src_x = (x - dst_x) as u32;
                let src_idx = (src_y * src_w + src_x) as usize;
                if src_idx < src.len() {
                    self.put_pixel(x, y, src[src_idx]);
                }
            }
        }
        self.merge_damage(clip);
    }

    fn blit_rgba_alpha(&mut self, dst_x: i32, dst_y: i32, src: &[u32], src_w: u32, src_h: u32) {
        let dst_rect = Rect { x: dst_x, y: dst_y, w: src_w, h: src_h };
        let clip = self.clip_rect(dst_rect);

        for y in clip.y..(clip.y + clip.h as i32) {
            let src_y = (y - dst_y) as u32;
            for x in clip.x..(clip.x + clip.w as i32) {
                let src_x = (x - dst_x) as u32;
                let src_idx = (src_y * src_w + src_x) as usize;
                if src_idx < src.len() {
                    let src_px = src[src_idx];
                    let alpha = (src_px >> 24) & 0xFF;
                    if alpha > 0 {
                        self.blend_at(x, y, src_px);
                    }
                }
            }
        }
        self.merge_damage(clip);
    }

    fn blit_rgba_alpha_rect(&mut self, dst_x: i32, dst_y: i32, src: &[u32], src_stride: u32, w: u32, h: u32) {
        let dst_rect = Rect { x: dst_x, y: dst_y, w, h };
        let clip = self.clip_rect(dst_rect);

        for y in clip.y..(clip.y + clip.h as i32) {
            let src_y = (y - dst_y) as u32;
            for x in clip.x..(clip.x + clip.w as i32) {
                let src_x = (x - dst_x) as u32;
                let src_idx = (src_y * src_stride + src_x) as usize;
                if src_idx < src.len() {
                    let src_px = src[src_idx];
                    let alpha = (src_px >> 24) & 0xFF;
                    if alpha > 0 {
                        self.blend_at(x, y, src_px);
                    }
                }
            }
        }
        self.merge_damage(clip);
    }

    fn blit_asset(&mut self, dst_x: i32, dst_y: i32, id: abi::ids::ThingId, src_w: u32, src_h: u32, _src_stride: u32, src_len: usize, cache: &mut crate::scene_cache::BytespaceMappingCache) {
        // Immediate mode: resolve mapping and blit
        if let Some(buf) = cache.get_or_map_ro(id, src_len) { let buf: crate::scene_cache::MapResult = buf; {
            // Check alignment before creating u32 slice
            let ptr = buf.as_ptr();
            if (ptr as usize) % 4 != 0 {
                thing_std::log_info(&alloc::format!("BLOOM: blit_asset: unaligned ptr={:#x} for id={}", ptr as usize, id.low()));
                return;
            }
            if buf.len() < 4 {
                return;
            }
            let u32_buf = unsafe {
                 core::slice::from_raw_parts(ptr as *const u32, buf.len() / 4)
            };
            self.blit_rgba_alpha(dst_x, dst_y, u32_buf, src_w, src_h);
        }}
    }

    fn draw_text(&mut self, x: i32, y: i32, text: &str, color: u32, size_px: f32) {
        crate::text::draw_text_on_painter(self, x, y, text, color, size_px);
    }

    fn copy_region(&mut self, src: &[u32], src_w: u32, region: Rect) {
        let clip = self.clip_rect(region);
        let x1 = clip.x.max(0) as u32;
        let y1 = clip.y.max(0) as u32;
        let x2 = ((clip.x + clip.w as i32) as u32).min(self.w);
        let y2 = ((clip.y + clip.h as i32) as u32).min(self.h);

        for y in y1..y2 {
            let row_start = (y * src_w + x1) as usize;
            let row_len = (x2 - x1) as usize;
            let dst_start = (y * self.w + x1) as usize;
            if row_start + row_len <= src.len() && dst_start + row_len <= self.buf.len() {
                self.buf[dst_start..dst_start + row_len]
                    .copy_from_slice(&src[row_start..row_start + row_len]);
            }
        }
        self.merge_damage(clip);
    }

    fn draw_shadow_mask(&mut self, origin_x: i32, origin_y: i32, mask: ShadowMask<'_>, params: ShadowParams) {
        let (mask_w, mask_h) = mask.dimensions();
        let blur = params.blur_radius as i32;
        
        if mask_w == 0 || mask_h == 0 { return; }
        
        // Use precomputed shadow cache for massive speedup
        if let Some(cache) = crate::shadow_cache::get_shadow_cache() {
            let total_w = mask_w as i32 + blur * 2;
            let total_h = mask_h as i32 + blur * 2;
            
            let shadow_x = origin_x + params.offset_x - blur;
            let shadow_y = origin_y + params.offset_y - blur;
            
            let shadow_rect = Rect {
                x: shadow_x,
                y: shadow_y,
                w: total_w as u32,
                h: total_h as u32,
            };
            let clip = self.clip_rect(shadow_rect);
            
            let base_a = ((params.color >> 24) & 0xFF) as u32;
            let tint = params.color & 0x00FF_FFFF;
            
            for screen_y in clip.y..(clip.y + clip.h as i32) {
                let local_y = screen_y - shadow_y;
                for screen_x in clip.x..(clip.x + clip.w as i32) {
                    let local_x = screen_x - shadow_x;
                    let alpha = cache.alpha_at(local_x, local_y, mask_w, mask_h);
                    if alpha == 0 { continue; }
                    
                    let final_a = ((alpha as u32) * base_a / 255).min(255);
                    if final_a == 0 { continue; }
                    
                    let src = (final_a << 24) | tint;
                    self.blend_at(screen_x, screen_y, src);
                }
            }
            self.merge_damage(clip);
            return;
        }
        
        // Fallback to original slow implementation if cache not initialized

        if mask_w == 0 || mask_h == 0 {
            return;
        }

        let buf_w = mask_w as i32 + blur * 2;
        let buf_h = mask_h as i32 + blur * 2;
        if buf_w <= 0 || buf_h <= 0 {
            return;
        }
        let buf_w = buf_w as usize;
        let buf_h = buf_h as usize;

        // Rasterize mask into alpha buffer
        let mut alpha_buf: Vec<u16> = vec![0u16; buf_w * buf_h];
        for my in 0..buf_h {
            let mask_y = my as i32 - blur;
            for mx in 0..buf_w {
                let mask_x = mx as i32 - blur;
                let a = mask.alpha_at(mask_x, mask_y) as u16;
                alpha_buf[my * buf_w + mx] = a;
            }
        }

        // Horizontal box blur
        if blur > 0 {
            let mut row_tmp: Vec<u16> = vec![0u16; buf_w];
            for y in 0..buf_h {
                let row_start = y * buf_w;
                for x in 0..buf_w {
                    let left = (x as i32 - blur).max(0) as usize;
                    let right = (x as i32 + blur).min(buf_w as i32 - 1) as usize;
                    let count = (right - left + 1) as u32;
                    let mut acc: u32 = 0;
                    for bx in left..=right {
                        acc += alpha_buf[row_start + bx] as u32;
                    }
                    row_tmp[x] = (acc / count.max(1)) as u16;
                }
                for x in 0..buf_w {
                    alpha_buf[row_start + x] = row_tmp[x];
                }
            }

            // Vertical box blur
            let mut col_tmp: Vec<u16> = vec![0u16; buf_h];
            for x in 0..buf_w {
                for y in 0..buf_h {
                    let top = (y as i32 - blur).max(0) as usize;
                    let bottom = (y as i32 + blur).min(buf_h as i32 - 1) as usize;
                    let count = (bottom - top + 1) as u32;
                    let mut acc: u32 = 0;
                    for by in top..=bottom {
                        acc += alpha_buf[by * buf_w + x] as u32;
                    }
                    col_tmp[y] = (acc / count.max(1)) as u16;
                }
                for y in 0..buf_h {
                    alpha_buf[y * buf_w + x] = col_tmp[y];
                }
            }
        }

        // Blit with shadow color
        let shadow_x = origin_x + params.offset_x - blur;
        let shadow_y = origin_y + params.offset_y - blur;
        let base_a = ((params.color >> 24) & 0xFF) as u32;
        let tint = params.color & 0x00FF_FFFF;

        let shadow_rect = Rect {
            x: shadow_x,
            y: shadow_y,
            w: buf_w as u32,
            h: buf_h as u32,
        };
        let clip = self.clip_rect(shadow_rect);

        for screen_y in clip.y..(clip.y + clip.h as i32) {
            let buf_y = (screen_y - shadow_y) as usize;
            for screen_x in clip.x..(clip.x + clip.w as i32) {
                let buf_x = (screen_x - shadow_x) as usize;
                let avg_alpha = alpha_buf[buf_y * buf_w + buf_x].min(255) as u8;
                if avg_alpha == 0 {
                    continue;
                }
                let final_a = ((avg_alpha as u32) * base_a / 255).min(255) as u8;
                if final_a == 0 {
                    continue;
                }
                let src = (final_a as u32) << 24 | tint;
                self.blend_at(screen_x, screen_y, src);
            }
        }
        self.merge_damage(clip);
    }

    fn draw_cursor_frame(&mut self, frame: &CursorFrame, px: i32, py: i32) {
        let cx = px - frame.hotspot_x;
        let cy = py - frame.hotspot_y;

        let cursor_rect = Rect {
            x: cx,
            y: cy,
            w: frame.width,
            h: frame.height,
        };
        let clip = self.clip_rect(cursor_rect);

        for row in 0..frame.height {
            let screen_y = cy + row as i32;
            if screen_y < clip.y || screen_y >= clip.y + clip.h as i32 {
                continue;
            }
            for col in 0..frame.width {
                let screen_x = cx + col as i32;
                if screen_x < clip.x || screen_x >= clip.x + clip.w as i32 {
                    continue;
                }
                let idx = (row * frame.width + col) as usize;
                let src_px = frame.pixels[idx];
                let alpha = (src_px >> 24) & 0xFF;
                if alpha == 0 {
                    continue;
                }
                if alpha == 255 {
                    self.put_pixel(screen_x, screen_y, src_px);
                } else {
                    self.blend_at(screen_x, screen_y, src_px);
                }
            }
        }
        self.merge_damage(clip);
    }

    fn draw_fallback_cursor(&mut self, px: i32, py: i32) {
        // Shadow
        for dy in 0..8i32 {
            for dx in 0..8i32 {
                let x = px + dx + 2;
                let y = py + dy + 3;
                self.blend_at(x, y, 0x66000000);
            }
        }
        // Cursor body
        for dy in 0..8i32 {
            for dx in 0..8i32 {
                let x = px + dx;
                let y = py + dy;
                let pixel = if dx == 0 || dy == 0 || dx == 7 || dy == 7 {
                    0xFF000000
                } else {
                    0xFFFFFFFF
                };
                self.put_pixel(x, y, pixel);
            }
        }
        let cursor_rect = Rect { x: px, y: py, w: 10, h: 11 };
        self.merge_damage(cursor_rect);
    }
}

impl<'a> CpuPainter<'a> {
    pub fn draw_tiled_bitmap(&mut self, dst: Rect, bmp: &Bitmap, origin: Point) {
        if bmp.w == 0 || bmp.h == 0 { return; }
        let clip = self.clip_rect(dst);
        
        if clip.w == 0 || clip.h == 0 { return; }

        let bmp_w = bmp.w as i32;
        let bmp_h = bmp.h as i32;
        let bmp_pixels = &bmp.pixels;

        let start_x = clip.x;
        let start_y = clip.y;
        let end_x = clip.x + clip.w as i32;
        let end_y = clip.y + clip.h as i32;

        for y in start_y..end_y {
            // src_y = mod_floor(y - origin.y, bmp_h)
            let mut src_y = (y - origin.y) % bmp_h;
            if src_y < 0 { src_y += bmp_h; }
            
            // Optimize: calculate src_x0 for start of row
            let mut src_x = (start_x - origin.x) % bmp_w;
            if src_x < 0 { src_x += bmp_w; }

            let target_row_offset = (y * self.w as i32 + start_x) as usize;
            let mut target_idx = target_row_offset;
            let mut x = start_x;
            
            let bmp_row_offset = (src_y * bmp_w) as usize;
            // Bound check for safety, though width should be consistent
            let bmp_row = &bmp_pixels[bmp_row_offset..bmp_row_offset + bmp_w as usize];

            while x < end_x {
                let remaining_row = end_x - x;
                let can_copy = (bmp_w - src_x).min(remaining_row);
                
                // Do copy
                let src_start = src_x as usize;
                let src_end = src_start + can_copy as usize;
                let dst_end_idx = target_idx + can_copy as usize;
                
                if dst_end_idx <= self.buf.len() {
                    self.buf[target_idx..dst_end_idx].copy_from_slice(&bmp_row[src_start..src_end]);
                }
                
                target_idx += can_copy as usize;
                x += can_copy;
                src_x = 0; // Wrap around for next chunk
            }
        }
        self.merge_damage(clip);
    }
}

/// Alpha blend two pixels (src over dst).
#[inline]
pub fn blend_pixel(src: u32, dst: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    let inv_a = 255 - sa;
    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;
    let or = sr + ((dr * inv_a + 127) / 255);
    let og = sg + ((dg * inv_a + 127) / 255);
    let ob = sb + ((db * inv_a + 127) / 255);
    0xFF000000 | (or.min(255) << 16) | (og.min(255) << 8) | ob.min(255)
}
