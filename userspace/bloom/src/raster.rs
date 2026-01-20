//! CPU Rasterizer - executes LowLevelOps on a framebuffer Surface
//!
//! Supports both full-frame and damage-aware rendering.
//! strict adherence to Portable Render ISA.
//! Text rendering delegates to fontd via font_client.

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;

use crate::damage::{Damage, Rect as DamageRect};
use crate::drawlist::DrawList;
use crate::lowered::{lower, LowLevelOp, LoweredDraw};
use crate::surface::Surface;
use crate::asset::Image;
use crate::isa::{BlendMode, FilterMode, Transform2D, Rect, EdgeAA};
use stem::thing::ThingId;
// use crate::font_client; // No longer needed
use crate::font_graph::{self, FontStyle};
use crate::log;
use fontdue::layout::GlyphRasterConfig;

/// Execution Context maintaining state stacks
struct RasterContext<'a> {
    surface: &'a mut Surface,
    clip_stack: Vec<Rect>,
    transform_stack: Vec<Transform2D>,
    current_clip: Rect,
    current_transform: Transform2D,
}

impl<'a> RasterContext<'a> {
    fn new(surface: &'a mut Surface) -> Self {
        let full_rect = Rect::new(0, 0, surface.width(), surface.height());
        Self {
            surface,
            clip_stack: Vec::with_capacity(4),
            transform_stack: Vec::with_capacity(4),
            current_clip: full_rect,
            current_transform: Transform2D::identity(),
        }
    }

    fn push_clip(&mut self, rect: Rect) {
        self.clip_stack.push(self.current_clip);
        // Intersect new clip with current clip
        // Note: transform applies to the clip rect too?
        // Usually, set_clip(rect) means "set clip to intersection of current clip and transformed rect"
        let transformed_rect = self.current_transform.transform_rect(rect);
        
        if let Some(intersection) = self.current_clip.intersection(&transformed_rect) {
            self.current_clip = intersection;
        } else {
            // Empty intersection - clip to nothing
            self.current_clip = Rect::new(0, 0, 0, 0);
        }
    }

    fn pop_clip(&mut self) {
        if let Some(prev) = self.clip_stack.pop() {
            self.current_clip = prev;
        }
    }

    fn push_transform(&mut self, t: Transform2D) {
        self.transform_stack.push(self.current_transform);
        self.current_transform = self.current_transform.combine(&t);
    }

    fn pop_transform(&mut self) {
        if let Some(prev) = self.transform_stack.pop() {
            self.current_transform = prev;
        }
    }

    // Helper to check if a rect is visible within current clip
    #[allow(dead_code)]
    fn is_visible(&self, r: Rect) -> bool {
        self.current_clip.intersection(&r).is_some()
    }
}

/// Execute a DrawList on a CPU surface (convenience wrapper)
pub fn execute(surface: &mut Surface, list: &DrawList) {
    let lowered = lower(list);
    execute_lowered(surface, &lowered);
}

/// Execute a DrawList respecting damage regions
pub fn execute_with_damage(surface: &mut Surface, list: &DrawList, damage: &Damage) {
    if damage.is_full {
        execute(surface, list);
        return;
    }
    let lowered = lower(list);
    execute_lowered_with_damage(surface, &lowered, damage);
}

/// Execute lowered ops directly
pub fn execute_lowered(surface: &mut Surface, lowered: &LoweredDraw) {
    let mut ctx = RasterContext::new(surface);

    for op in lowered.ops.iter() {
        match op {
            LowLevelOp::Clear { color } => clear(ctx.surface, color.to_u32()),
            
            LowLevelOp::PushClip { rect } => ctx.push_clip(*rect),
            LowLevelOp::PopClip => ctx.pop_clip(),
            
            LowLevelOp::PushTransform { t } => ctx.push_transform(*t),
            LowLevelOp::PopTransform => ctx.pop_transform(),

            LowLevelOp::FillRect { rect, color, aa } => {
                let t_rect = ctx.current_transform.transform_rect(*rect);
                if let Some(clipped) = ctx.current_clip.intersection(&t_rect) {
                     fill_rect_blend(ctx.surface, clipped.x(), clipped.y(), clipped.width(), clipped.height(), color.to_u32());
                     let _ = aa; 
                }
            },

            LowLevelOp::StrokeRect { rect, color, width } => {
                 let t_rect = ctx.current_transform.transform_rect(*rect);
                 stroke_rect_clipped_blend(ctx.surface, &t_rect, *width, color.to_u32(), &ctx.current_clip);
            },
            
            LowLevelOp::Line { from, to, color, width: _ } => {
                let p0 = ctx.current_transform.transform_point(*from);
                let p1 = ctx.current_transform.transform_point(*to);
                line(ctx.surface, p0.x, p0.y, p1.x, p1.y, color.to_u32());
            },

            LowLevelOp::FillCircle { center, radius, color } => {
                let c = ctx.current_transform.transform_point(*center);
                fill_circle_blend(ctx.surface, c.x, c.y, *radius, color.to_u32());
            },

            LowLevelOp::FillArc { center, radius, start_angle, end_angle, color, aa } => {
                let c = ctx.current_transform.transform_point(*center);
                fill_arc_clipped_blend(ctx.surface, c.x, c.y, *radius, *start_angle, *end_angle, color.to_u32(), *aa, &ctx.current_clip);
            },

            LowLevelOp::BlitOpaque { image, src, dst, filter } => {
                let t_dst = ctx.current_transform.transform_rect(*dst);
                if let Some(clipped_dst) = ctx.current_clip.intersection(&t_dst) {
                    blit_opaque(ctx.surface, image, src, &t_dst, &clipped_dst, *filter);
                }
            },

            LowLevelOp::BlitAlpha { image, src, dst, filter, blend, const_alpha } => {
                let t_dst = ctx.current_transform.transform_rect(*dst);
                if let Some(clipped_dst) = ctx.current_clip.intersection(&t_dst) {
                    blit_alpha(ctx.surface, image, src, &t_dst, &clipped_dst, *filter, *blend, *const_alpha);
                }
            },

            LowLevelOp::TextSpan { text, pos, size, color, font_name, font_debug } => {
                let p = ctx.current_transform.transform_point(*pos);
                rasterize_text_locally(
                    ctx.surface, 
                    text, 
                    p.x, 
                    p.y, 
                    *size, 
                    color.to_u32(), 
                    &ctx.current_clip,
                    font_name.as_deref(),
                    *font_debug
                );
            }
        }
    }
}

/// Execute lowered ops with damage tracking optimization
/// This basically applies an *additional* clip (the damage rect) on top of the op stream.
/// However, since damage is a set of rects, we might run the ops multiple times or union the clip.
/// For simplicity in v0: We iterate damage rects and set the initial clip to the damage rect.
pub fn execute_lowered_with_damage(surface: &mut Surface, lowered: &LoweredDraw, damage: &Damage) {
    let mut damage_rects = [DamageRect::default(); 8];
    let mut damage_count = 0;
    for rect in damage.iter() {
        if damage_count < 8 {
            damage_rects[damage_count] = rect;
            damage_count += 1;
        }
    }

    let start = stem::monotonic_ns();

    for i in 0..damage_count {
        let d = damage_rects[i];
        // Create a context where the initial clip is the damage rect
        let mut ctx = RasterContext::new(surface);
        // Override initial clip
        ctx.current_clip = Rect::new(d.x, d.y, d.w, d.h);

        execute_lowered_on_context(&mut ctx, lowered);
    }

    let elapsed = stem::monotonic_ns().saturating_sub(start);
    if elapsed > 10_000_000 { // 10ms threshold
        log!("[bloom::raster] WARN: slow rasterize ({} rects) = {:.1}ms", damage_count, elapsed as f64 / 1_000_000.0);
    }
}

// Helper to run ops on an existing context (used by damage loop)
fn execute_lowered_on_context(ctx: &mut RasterContext, lowered: &LoweredDraw) {
    for op in lowered.ops.iter() {
        match op {
            LowLevelOp::Clear { color } => {
                fill_rect_copy(ctx.surface, ctx.current_clip.x(), ctx.current_clip.y(), ctx.current_clip.width(), ctx.current_clip.height(), color.to_u32());
            },
            
            LowLevelOp::PushClip { rect } => ctx.push_clip(*rect),
            LowLevelOp::PopClip => ctx.pop_clip(),
            
            LowLevelOp::PushTransform { t } => ctx.push_transform(*t),
            LowLevelOp::PopTransform => ctx.pop_transform(),
            
            LowLevelOp::FillRect { rect, color, aa: _ } => {
                let t_rect = ctx.current_transform.transform_rect(*rect);
                if let Some(clipped) = ctx.current_clip.intersection(&t_rect) {
                     let c = color.to_u32();
                     if (c >> 24) == 255 {
                         fill_rect_copy(ctx.surface, clipped.x(), clipped.y(), clipped.width(), clipped.height(), c);
                     } else {
                         fill_rect_blend(ctx.surface, clipped.x(), clipped.y(), clipped.width(), clipped.height(), c);
                     }
                }
            },

            LowLevelOp::StrokeRect { rect, color, width } => {
                 let t_rect = ctx.current_transform.transform_rect(*rect);
                 stroke_rect_clipped_blend(ctx.surface, &t_rect, *width, color.to_u32(), &ctx.current_clip);
            },
            
            LowLevelOp::Line { from, to, color, width: _ } => {
                let p0 = ctx.current_transform.transform_point(*from);
                let p1 = ctx.current_transform.transform_point(*to);
                line(ctx.surface, p0.x, p0.y, p1.x, p1.y, color.to_u32());
            },

            LowLevelOp::FillCircle { center, radius, color } => {
                let c = ctx.current_transform.transform_point(*center);
                fill_circle_blend(ctx.surface, c.x, c.y, *radius, color.to_u32());
            },

            LowLevelOp::FillArc { center, radius, start_angle, end_angle, color, aa } => {
                let c = ctx.current_transform.transform_point(*center);
                fill_arc_clipped_blend(ctx.surface, c.x, c.y, *radius, *start_angle, *end_angle, color.to_u32(), *aa, &ctx.current_clip);
            },

            LowLevelOp::BlitOpaque { image, src, dst, filter } => {
                let t_dst = ctx.current_transform.transform_rect(*dst);
                if let Some(clipped_dst) = ctx.current_clip.intersection(&t_dst) {
                    blit_opaque(ctx.surface, image, src, &t_dst, &clipped_dst, *filter);
                }
            },

            LowLevelOp::BlitAlpha { image, src, dst, filter, blend, const_alpha } => {
                let t_dst = ctx.current_transform.transform_rect(*dst);
                if let Some(clipped_dst) = ctx.current_clip.intersection(&t_dst) {
                    blit_alpha(ctx.surface, image, src, &t_dst, &clipped_dst, *filter, *blend, *const_alpha);
                }
            },
            
            LowLevelOp::TextSpan { text, pos, size, color, font_name, font_debug } => {
                let p = ctx.current_transform.transform_point(*pos);
                rasterize_text_locally(
                    ctx.surface, 
                    text, 
                    p.x, 
                    p.y, 
                    *size, 
                    color.to_u32(), 
                    &ctx.current_clip,
                    font_name.as_deref(),
                    *font_debug
                );
            }
        }
    }
}

// --- Primitives ---

pub fn clear(surface: &mut Surface, xrgb: u32) {
    fill_rect_copy(surface, 0, 0, surface.width(), surface.height(), xrgb);
}

pub fn fill_rect_copy(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, color: u32) {
    if w <= 0 || h <= 0 { return; }
    let mut x0 = x; let mut y0 = y;
    let mut x1 = x + w; let mut y1 = y + h;

    if x0 < 0 { x0 = 0; }
    if y0 < 0 { y0 = 0; }
    if x1 > surface.width() { x1 = surface.width(); }
    if y1 > surface.height() { y1 = surface.height(); }

    // Optimization: if stride matched width/color, could use memset?
    // For now simple loop
    for yy in y0..y1 {
        for xx in x0..x1 {
            surface.put_px(xx, yy, color);
        }
    }
}

pub fn fill_rect_blend(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, color: u32) {
    let a = ((color >> 24) & 0xFF) as u8;
    if a == 255 {
        fill_rect_copy(surface, x, y, w, h, color);
        return;
    }
    if a == 0 { return; }

    if w <= 0 || h <= 0 { return; }
    let mut x0 = x; let mut y0 = y;
    let mut x1 = x + w; let mut y1 = y + h;

    if x0 < 0 { x0 = 0; }
    if y0 < 0 { y0 = 0; }
    if x1 > surface.width() { x1 = surface.width(); }
    if y1 > surface.height() { y1 = surface.height(); }
    
    let sr = ((color >> 16) & 0xFF) as u8;
    let sg = ((color >> 8) & 0xFF) as u8;
    let sb = (color & 0xFF) as u8;
    let inv_a = 255 - a;

    let stride = surface.stride_bytes;
    let base_ptr = surface.ptr as *mut u32;

    for yy in y0..y1 {
        for xx in x0..x1 {
            let offset = (stride / 4) * (yy as usize) + (xx as usize);
            unsafe {
                let dst_ptr = base_ptr.add(offset);
                let dst = *dst_ptr;
                let dr = ((dst >> 16) & 0xFF) as u8;
                let dg = ((dst >> 8) & 0xFF) as u8;
                let db = (dst & 0xFF) as u8;
                
                let out_r = ((sr as u32 * a as u32) + (dr as u32 * inv_a as u32)) / 255;
                let out_g = ((sg as u32 * a as u32) + (dg as u32 * inv_a as u32)) / 255;
                let out_b = ((sb as u32 * a as u32) + (db as u32 * inv_a as u32)) / 255;
                
                *dst_ptr = (out_r << 16) | (out_g << 8) | out_b;
            }
        }
    }
}

fn stroke_rect_clipped_blend(surface: &mut Surface, rect: &Rect, width: i32, color: u32, clip: &Rect) {
    // 4 fill_rects, each clipped
    let t = Rect::new(rect.x(), rect.y(), rect.width(), width);
    let b = Rect::new(rect.x(), rect.y() + rect.height() - width, rect.width(), width);
    let l = Rect::new(rect.x(), rect.y() + width, width, rect.height() - 2*width);
    let r = Rect::new(rect.x() + rect.width() - width, rect.y() + width, width, rect.height() - 2*width);

    for r_part in [t, b, l, r] {
        if let Some(c) = r_part.intersection(clip) {
            fill_rect_blend(surface, c.x(), c.y(), c.width(), c.height(), color);
        }
    }
}

pub fn fill_circle_blend(surface: &mut Surface, cx: i32, cy: i32, r: i32, color: u32) {
    let a = ((color >> 24) & 0xFF) as u8;
    if a == 0 { return; }
    
    // Bounds
    let x0 = (cx - r).max(0);
    let y0 = (cy - r).max(0);
    let x1 = (cx + r).min(surface.width());
    let y1 = (cy + r).min(surface.height());
    let r2 = r * r;

    let sr = ((color >> 16) & 0xFF) as u8;
    let sg = ((color >> 8) & 0xFF) as u8;
    let sb = (color & 0xFF) as u8;
    let inv_a = 255 - a;

    let stride = surface.stride_bytes;
    let base_ptr = surface.ptr as *mut u32;

    for y in y0..y1 {
        for x in x0..x1 {
            let dx = x - cx; let dy = y - cy;
            if dx*dx + dy*dy <= r2 {
                if a == 255 {
                    surface.put_px(x, y, color);
                } else {
                    let offset = (stride / 4) * (y as usize) + (x as usize);
                    unsafe {
                        let dst_ptr = base_ptr.add(offset);
                        let dst = *dst_ptr;
                        let dr = ((dst >> 16) & 0xFF) as u8;
                        let dg = ((dst >> 8) & 0xFF) as u8;
                        let db = (dst & 0xFF) as u8;
                        
                        let out_r = ((sr as u32 * a as u32) + (dr as u32 * inv_a as u32)) / 255;
                        let out_g = ((sg as u32 * a as u32) + (dg as u32 * inv_a as u32)) / 255;
                        let out_b = ((sb as u32 * a as u32) + (db as u32 * inv_a as u32)) / 255;
                        
                        *dst_ptr = (out_r << 16) | (out_g << 8) | out_b;
                    }
                }
            }
        }
    }
}

pub fn fill_arc_clipped_blend(surface: &mut Surface, cx: i32, cy: i32, r: i32, start_deg: f32, end_deg: f32, color: u32, aa: EdgeAA, clip: &Rect) {
    let sa = ((color >> 24) & 0xFF) as u8;
    if sa == 0 { return; }
    
    // Bounds: expanded slightly for AA
    let margin = if aa != EdgeAA::None { 1 } else { 0 };
    let x0 = (cx - r - margin).max(clip.x()).max(0);
    let y0 = (cy - r - margin).max(clip.y()).max(0);
    let x1 = (cx + r + margin).min(clip.x() + clip.width()).min(surface.width());
    let y1 = (cy + r + margin).min(clip.y() + clip.height()).min(surface.height());
    
    let r_f = r as f32;
    let r2 = r_f * r_f;

    let sr = ((color >> 16) & 0xFF) as u8;
    let sg = ((color >> 8) & 0xFF) as u8;
    let sb = (color & 0xFF) as u8;

    // Normalizing angles for easier comparison
    let mut s = start_deg;
    let mut e = end_deg;
    while s < 0.0 { s += 360.0; }
    while s >= 360.0 { s -= 360.0; }
    while e < s { e += 360.0; }

    let do_aa = aa != EdgeAA::None;

    for y in y0..y1 {
        for x in x0..x1 {
            let dx = (x as f32 + 0.5) - cx as f32;
            let dy = (y as f32 + 0.5) - cy as f32;
            let dist_sq = dx*dx + dy*dy;
            
            if dist_sq <= (r_f + 1.0) * (r_f + 1.0) {
                // Angle check
                let mut angle = libm::atan2f(dy, dx) * 180.0 / 3.14159265;
                while angle < s { angle += 360.0; }
                
                if angle <= e {
                    let mut coverage = 1.0;
                    if do_aa {
                        let dist = libm::sqrtf(dist_sq);
                        coverage = (r_f - dist + 0.5).clamp(0.0, 1.0);
                    } else if dist_sq > r2 {
                        coverage = 0.0;
                    }
                    
                    if coverage > 0.0 {
                        let final_a = (sa as f32 * coverage) as u8;
                        if final_a > 0 {
                            blend_pixel(surface, x, y, sr, sg, sb, final_a);
                        }
                    }
                }
            }
        }
    }
}

pub fn line(surface: &mut Surface, mut x0: i32, mut y0: i32, x1: i32, y1: i32, xrgb: u32) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        // Bounds check (primitive clipping)
        if x0 >= 0 && y0 >= 0 && x0 < surface.width() && y0 < surface.height() {
            surface.put_px(x0, y0, xrgb);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = err * 2;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

// --- Images ---

fn blit_opaque(surface: &mut Surface, image: &Image, src: &Rect, full_dst: &Rect, clipped_dst: &Rect, _filter: FilterMode) {
    // Calculate mapping: for a pixel (dx, dy) in clipped_dst, what is (sx, sy) in src?
    // Scale factors based on FULL dst
    let scale_x = src.width() as f32 / full_dst.width() as f32;
    let scale_y = src.height() as f32 / full_dst.height() as f32;

    let cx0 = clipped_dst.x();
    let cy0 = clipped_dst.y();
    let cx1 = cx0 + clipped_dst.width();
    let cy1 = cy0 + clipped_dst.height();

    for dy in cy0..cy1 {
        for dx in cx0..cx1 {
            // Nearest neighbor sample
            let sx_f = (dx - full_dst.x()) as f32 * scale_x;
            let sy_f = (dy - full_dst.y()) as f32 * scale_y;
            let sx = (src.x() as f32 + sx_f) as i32;
            let sy = (src.y() as f32 + sy_f) as i32;

            if sx >= 0 && sy >= 0 && sx < image.width as i32 && sy < image.height as i32 {
                let idx = (sy as usize) * (image.width as usize) + (sx as usize);
                // Safety check
                if idx < image.pixels.len() {
                    let px = image.pixels[idx];
                    surface.put_px(dx, dy, px);
                }
            }
        }
    }
}

fn blit_alpha(
    surface: &mut Surface, 
    image: &Image, 
    src: &Rect, 
    full_dst: &Rect, 
    clipped_dst: &Rect, 
    _filter: FilterMode, 
    blend: BlendMode, 
    const_alpha: Option<u8>
) {
    let ca = const_alpha.unwrap_or(255) as u32;
    
    // Fast path: 1:1 scale
    let is_1to1 = src.width() == full_dst.width() && src.height() == full_dst.height();
    
    if is_1to1 {
        let dx_offset = clipped_dst.x() - full_dst.x();
        let dy_offset = clipped_dst.y() - full_dst.y();
        let sx_start = src.x() + dx_offset;
        let sy_start = src.y() + dy_offset;

        for dy in 0..clipped_dst.height() {
            let sy = sy_start + dy;
            let dst_y = clipped_dst.y() + dy;
            
            for dx in 0..clipped_dst.width() {
                let sx = sx_start + dx;
                let dst_x = clipped_dst.x() + dx;
                
                if sx >= 0 && sy >= 0 && sx < image.width as i32 && sy < image.height as i32 {
                    let idx = (sy as usize) * (image.width as usize) + (sx as usize);
                    let src_px = image.pixels[idx];
                    let mut a = (src_px >> 24) & 0xFF;
                    
                    if ca != 255 { a = (a * ca) / 255; }
                    if a == 0 { continue; }

                    if blend == BlendMode::Src || a == 255 {
                        surface.put_px(dst_x, dst_y, src_px);
                        continue;
                    }

                    blend_pixel(surface, dst_x, dst_y, 
                        ((src_px >> 16) & 0xFF) as u8, 
                        ((src_px >> 8) & 0xFF) as u8, 
                        (src_px & 0xFF) as u8, a as u8);
                }
            }
        }
        return;
    }

    // Slow path: Scaled blit (existing logic)
    let scale_x = src.width() as f32 / full_dst.width() as f32;
    let scale_y = src.height() as f32 / full_dst.height() as f32;

    let cx0 = clipped_dst.x();
    let cy0 = clipped_dst.y();
    let cx1 = cx0 + clipped_dst.width();
    let cy1 = cy0 + clipped_dst.height();

    for dy in cy0..cy1 {
        for dx in cx0..cx1 {
            let sx_f = (dx - full_dst.x()) as f32 * scale_x;
            let sy_f = (dy - full_dst.y()) as f32 * scale_y;
            let sx = (src.x() as f32 + sx_f) as i32;
            let sy = (src.y() as f32 + sy_f) as i32;

            if sx >= 0 && sy >= 0 && sx < image.width as i32 && sy < image.height as i32 {
                let idx = (sy as usize) * (image.width as usize) + (sx as usize);
                if idx < image.pixels.len() {
                    let src_px = image.pixels[idx];
                    let mut a = (src_px >> 24) & 0xFF;
                    if ca != 255 { a = (a * ca) / 255; }
                    if a == 0 { continue; }

                    if blend == BlendMode::Src || a == 255 {
                         surface.put_px(dx, dy, src_px);
                         continue;
                    }

                    blend_pixel(surface, dx, dy, 
                        ((src_px >> 16) & 0xFF) as u8, 
                        ((src_px >> 8) & 0xFF) as u8, 
                        (src_px & 0xFF) as u8, a as u8);
                }
            }
        }
    }
}

/// Render text using local fontdue rasterization and a simple glyph cache.
fn rasterize_text_locally(
    surface: &mut Surface,
    text: &str,
    x: i32,
    y: i32,
    size: f32,
    color: u32,
    clip: &Rect,
    requested_font: Option<&str>,
    font_debug: bool,
) {
    let ca = ((color >> 24) & 0xFF) as u8;
    if ca == 0 {
        return;
    }

    let mut debug_lines: Vec<String> = Vec::new();

    let mut handled = false;
    font_graph::with_graph(|graph| {
        let style = FontStyle::default();
        let stack = graph.resolve_stack(requested_font);
        if stack.is_empty() {
            return;
        }

        let primary_face_id = stack
            .iter()
            .find_map(|family| graph.select_face_for_family(*family, style));
        let primary_face_id = match primary_face_id {
            Some(id) => id,
            None => return,
        };
        let primary_font = match graph.font_for_face(primary_face_id) {
            Some(font) => font,
            None => return,
        };

        handled = true; // Graph is usable

        let line_metrics = primary_font.font.horizontal_line_metrics(size);
        let (line_height, ascent) = if let Some(metrics) = line_metrics {
            (metrics.new_line_size.max(size * 1.1), metrics.ascent)
        } else {
            (size * 1.2, size * 0.8)
        };
        let baseline = y as f32 + ascent;

        let mut pen_x = x as f32;
        let mut pen_y = baseline;

        for ch in text.chars() {
            if ch == '\n' {
                pen_x = x as f32;
                pen_y += line_height;
                continue;
            }
            if ch == '\r' {
                continue;
            }

            let codepoint = ch as u32;
            let resolved = graph
                .resolve_face_for_glyph(&stack, style, codepoint)
                .or_else(|| graph.resolved_face_by_id(primary_face_id));
            let resolved = match resolved {
                Some(r) => r,
                None => {
                    pen_x += size * 0.4;
                    continue;
                }
            };

            let font = match graph.font_for_face(resolved.face_id) {
                Some(font) => font,
                None => continue,
            };
            let glyph_index = font.font.lookup_glyph_index(ch);
            if glyph_index == 0 {
                pen_x += size * 0.4;
                continue;
            }

            let config = GlyphRasterConfig {
                glyph_index,
                px: size,
                font_hash: font.font.file_hash(),
            };
            let (metrics, bitmap) = font.get_glyph(config);

            let gx = (pen_x + metrics.xmin as f32) as i32;
            let gy = pen_y as i32 - metrics.height as i32 - metrics.ymin;

            let glyph_color = if font_debug {
                family_color(resolved.family_id)
            } else {
                color
            };
            let gr = ((glyph_color >> 16) & 0xFF) as u8;
            let gg = ((glyph_color >> 8) & 0xFF) as u8;
            let gb = (glyph_color & 0xFF) as u8;

            for row in 0..metrics.height {
                for col in 0..metrics.width {
                    let px = gx + col as i32;
                    let py = gy + row as i32;

                    if px < clip.x()
                        || px >= clip.x() + clip.width()
                        || py < clip.y()
                        || py >= clip.y() + clip.height()
                    {
                        continue;
                    }

                    let alpha = bitmap[row * metrics.width + col];
                    if alpha == 0 {
                        continue;
                    }

                    let final_alpha = ((alpha as u32 * ca as u32) / 255) as u8;
                    blend_pixel(surface, px, py, gr, gg, gb, final_alpha);
                }
            }

            if font_debug {
                let label = format!(
                    "U+{:04X} '{}' -> {} / {}",
                    codepoint,
                    if ch.is_control() { ' ' } else { ch },
                    resolved.family_name,
                    resolved.style_name
                );
                debug_lines.push(label);
            }

            pen_x += metrics.advance_width;
        }
    });

    if !handled {
        rasterize_text_fallback(surface, text, x, y, size, color, clip, requested_font, font_debug);
    }
    
    if handled && font_debug && !debug_lines.is_empty() {
        render_debug_overlay(surface, &debug_lines, clip, requested_font);
    }
}

fn rasterize_text_fallback(
    surface: &mut Surface,
    text: &str,
    x: i32,
    y: i32,
    size: f32,
    color: u32,
    clip: &Rect,
    requested_font: Option<&str>,
    font_debug: bool,
) {
    let fonts = crate::ASSETS.get_fonts();
    if fonts.is_empty() {
        return;
    }

    // Select font: try requested, else default to first
    let font = if let Some(req) = requested_font {
        fonts.iter().find(|f| f.name.contains(req))
            .or_else(|| fonts.iter().find(|f| f.name.contains("NotoSans-Regular")))
            .unwrap_or(&fonts[0])
    } else {
        fonts.iter().find(|f| f.name.contains("NotoSans-Regular"))
            .unwrap_or(&fonts[0])
    };

    let ca = ((color >> 24) & 0xFF) as u8;
    let sr = ((color >> 16) & 0xFF) as u8;
    let sg = ((color >> 8) & 0xFF) as u8;
    let sb = (color & 0xFF) as u8;

    let line_metrics = font.font.horizontal_line_metrics(size);
    let (line_height, ascent) = if let Some(metrics) = line_metrics {
        (metrics.new_line_size.max(size * 1.1), metrics.ascent)
    } else {
        (size * 1.2, size * 0.8)
    };
    let baseline = y as f32 + ascent;

    let mut pen_x = x as f32;
    let mut pen_y = baseline;

    for ch in text.chars() {
        if ch == '\n' {
            pen_x = x as f32;
            pen_y += line_height;
            continue;
        }
        if ch == '\r' { continue; }

        let glyph_index = font.font.lookup_glyph_index(ch);
        if glyph_index == 0 {
             pen_x += size * 0.4;
             continue;
        }

        let config = GlyphRasterConfig {
            glyph_index,
            px: size,
            font_hash: font.font.file_hash(),
        };
        let (metrics, bitmap) = font.get_glyph(config);

        let gx = (pen_x + metrics.xmin as f32) as i32;
        let gy = pen_y as i32 - metrics.height as i32 - metrics.ymin;

        for row in 0..metrics.height {
            for col in 0..metrics.width {
                let px = gx + col as i32;
                let py = gy + row as i32;

                if px < clip.x() || px >= clip.x() + clip.width() || 
                   py < clip.y() || py >= clip.y() + clip.height() {
                    continue;
                }

                let alpha = bitmap[row * metrics.width + col];
                if alpha == 0 { continue; }

                let final_alpha = ((alpha as u32 * ca as u32) / 255) as u8;
                blend_pixel(surface, px, py, sr, sg, sb, final_alpha);
            }
        }
        pen_x += metrics.advance_width;
    }

    if font_debug {
        // Simple legacy overlay (avoid recursion)
        // ... or simple logs?
        // Let's just draw a red box or something? 
        // Or recursively call ourselves but force debug=false
        // Guard against recursion: render_debug_overlay calls rasterize_text_locally with false.
        let msg = format!("Legacy: used '{}'", font.name);
        let debug_lines = alloc::vec![msg];
        render_debug_overlay(surface, &debug_lines, clip, None);
    }
}

fn render_debug_overlay(
    surface: &mut Surface,
    lines: &[String],
    clip: &Rect,
    requested_font: Option<&str>,
) {
    let max_lines = 24usize;
    let font_size = 12.0;
    let start_x = clip.x() + 8;
    let mut cursor_y = clip.y() + 8;

    for line in lines.iter().take(max_lines) {
        rasterize_text_locally(
            surface,
            line,
            start_x,
            cursor_y,
            font_size,
            0xFFFFFFFF,
            clip,
            requested_font,
            false,
        );
        cursor_y += (font_size as i32) + 2;
    }
}

fn family_color(family_id: ThingId) -> u32 {
    const PALETTE: [u32; 6] = [
        0xFFE76F51,
        0xFFF4A261,
        0xFF2A9D8F,
        0xFF264653,
        0xFF3A86FF,
        0xFF06D6A0,
    ];
    let idx = (family_id.to_u64_lossy() as usize) % PALETTE.len();
    PALETTE[idx]
}


// Helper for single pixel blending
fn blend_pixel(surface: &mut Surface, x: i32, y: i32, sr: u8, sg: u8, sb: u8, sa: u8) {
    let offset = (surface.stride_bytes / 4) * (y as usize) + (x as usize);
    let ptr = surface.ptr as *mut u32;
    unsafe {
        let dst_ptr = ptr.add(offset);
        let dst = *dst_ptr;
        
        let dr = ((dst >> 16) & 0xFF) as u8;
        let dg = ((dst >> 8) & 0xFF) as u8;
        let db = (dst & 0xFF) as u8;
        
        let inv_a = 255 - sa;
        
        let out_r = ((sr as u32 * sa as u32) + (dr as u32 * inv_a as u32)) / 255;
        let out_g = ((sg as u32 * sa as u32) + (dg as u32 * inv_a as u32)) / 255;
        let out_b = ((sb as u32 * sa as u32) + (db as u32 * inv_a as u32)) / 255;
        
        *dst_ptr = (out_r << 16) | (out_g << 8) | out_b;
    }
}
