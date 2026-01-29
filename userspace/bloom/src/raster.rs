use crate::asset::Image;
use crate::damage::Damage;
use crate::geometry::Rect;
type DamageRect = Rect;
use crate::drawlist::DrawList;
use crate::font_client;
use crate::font_graph::{self, FontStyle};
use crate::isa::{BlendMode, Color, EdgeAA, FilterMode, Transform2D};
use crate::lowered::{lower, LowLevelOp, LoweredDraw};
use crate::surface::Surface;
use alloc::vec;
use alloc::vec::Vec;
use fontdue::layout::GlyphRasterConfig;

use alloc::collections::BTreeMap;
use stem::thing::{HandleId, ThingId};

struct MappedBytespace {
    ptr: *mut u8,
    len: usize,
}

struct BytespaceMapCache {
    entries: BTreeMap<ThingId, MappedBytespace>,
}

impl BytespaceMapCache {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    fn get_or_map(&mut self, bs: ThingId, stride: u32, height: u32) -> Option<*mut u8> {
        if let Some(entry) = self.entries.get(&bs) {
            return Some(entry.ptr);
        }

        let start = stem::monotonic_ns();
        match stem::thing::sys::bytespace_map(bs) {
            Ok(ptr) => {
                let dt = stem::monotonic_ns().saturating_sub(start);
                crate::trace_counter!("raster.bytespace_map.count", 1);
                crate::trace_counter!("raster.bytespace_map.ns_total", dt);

                let len = (stride * height) as usize;
                self.entries.insert(
                    bs,
                    MappedBytespace {
                        ptr: ptr as *mut u8,
                        len,
                    },
                );
                Some(ptr as *mut u8)
            }
            Err(_) => None,
        }
    }
}

impl Drop for BytespaceMapCache {
    fn drop(&mut self) {
        for (bs, entry) in self.entries.iter() {
            let _ = stem::thing::sys::bytespace_unmap(*bs, entry.ptr);
            crate::trace_counter!("raster.bytespace_unmap.count", 1);
        }
    }
}

struct RasterContext<'a> {
    surface: &'a mut Surface,
    cache: &'a mut BytespaceMapCache,
    clip_stack: Vec<Rect>,
    transform_stack: Vec<Transform2D>,
    current_clip: Rect,
    current_transform: Transform2D,
    solid_text: bool,
}

impl<'a> RasterContext<'a> {
    fn new(surface: &'a mut Surface, cache: &'a mut BytespaceMapCache, solid_text: bool) -> Self {
        let fr = Rect::new(0, 0, surface.width(), surface.height());
        Self {
            surface,
            cache,
            clip_stack: Vec::with_capacity(4),
            transform_stack: Vec::with_capacity(4),
            current_clip: fr,
            current_transform: Transform2D::identity(),
            solid_text,
        }
    }
    fn push_clip(&mut self, rect: Rect) {
        self.clip_stack.push(self.current_clip);
        let tr = self.current_transform.transform_rect(rect);
        if let Some(i) = self.current_clip.intersection(&tr) {
            self.current_clip = i;
        } else {
            self.current_clip = Rect::new(0, 0, 0, 0);
        }
    }
    fn pop_clip(&mut self) {
        if let Some(p) = self.clip_stack.pop() {
            self.current_clip = p;
        }
    }
    fn push_transform(&mut self, t: Transform2D) {
        self.transform_stack.push(self.current_transform);
        self.current_transform = self.current_transform.combine(&t);
    }
    fn pop_transform(&mut self) {
        if let Some(p) = self.transform_stack.pop() {
            self.current_transform = p;
        }
    }
}

pub fn execute(surface: &mut Surface, list: &DrawList, solid_text: bool) {
    let lowered = {
        crate::trace_span!("raster.lower");
        lower(list)
    };

    let mut cache = BytespaceMapCache::new();
    let mut ctx = RasterContext::new(surface, &mut cache, solid_text);
    execute_lowered_on_context(&mut ctx, &lowered);
}

pub fn execute_with_damage(
    surface: &mut Surface,
    list: &DrawList,
    damage: &Damage,
    solid_text: bool,
) {
    if damage.is_full {
        execute(surface, list, solid_text);
        return;
    }
    let lowered = {
        crate::trace_span!("raster.lower");
        lower(list)
    };
    execute_lowered_with_damage(surface, &lowered, damage, solid_text);
}

pub fn execute_lowered_with_damage(
    surface: &mut Surface,
    lowered: &LoweredDraw,
    damage: &Damage,
    solid_text: bool,
) {
    let mut dr = [DamageRect::default(); 8];
    let mut count = 0;
    for r in damage.iter() {
        if count < 8 {
            dr[count] = r;
            count += 1;
        }
    }
    let mut cache = BytespaceMapCache::new();
    for i in 0..count {
        let d = dr[i];
        crate::trace_span!("raster.rect.total");
        let mut ctx = RasterContext::new(surface, &mut cache, solid_text);
        ctx.current_clip = Rect::new(d.x(), d.y(), d.width(), d.height());
        execute_lowered_on_context(&mut ctx, lowered);
    }
}

fn execute_lowered_on_context(ctx: &mut RasterContext, lowered: &LoweredDraw) {
    let start = stem::monotonic_ns();
    crate::trace_span!("raster.execute");
    for op in lowered.ops.iter() {
        match op {
            LowLevelOp::Clear { color } => {
                crate::trace_counter!("raster.ops.clear", 1);
                fill_rect_copy(
                    ctx.surface,
                    ctx.current_clip.x(),
                    ctx.current_clip.y(),
                    ctx.current_clip.width(),
                    ctx.current_clip.height(),
                    color.to_u32(),
                );
            }
            LowLevelOp::PushClip { rect } => ctx.push_clip(*rect),
            LowLevelOp::PopClip => ctx.pop_clip(),
            LowLevelOp::PushTransform { t } => ctx.push_transform(*t),
            LowLevelOp::PopTransform => ctx.pop_transform(),
            LowLevelOp::FillRect { rect, color, aa: _ } => {
                crate::trace_counter!("raster.ops.fill", 1);
                let tr = ctx.current_transform.transform_rect(*rect);
                if let Some(cl) = ctx.current_clip.intersection(&tr) {
                    let c = color.to_u32();
                    if (c >> 24) == 255 {
                        fill_rect_copy(ctx.surface, cl.x(), cl.y(), cl.width(), cl.height(), c);
                    } else {
                        fill_rect_blend(ctx.surface, cl.x(), cl.y(), cl.width(), cl.height(), c);
                    }
                }
            }
            LowLevelOp::FillLinearGradient {
                rect,
                color1,
                color2,
            } => {
                crate::trace_counter!("raster.ops.fill", 1);
                let tr = ctx.current_transform.transform_rect(*rect);
                if let Some(cl) = ctx.current_clip.intersection(&tr) {
                    fill_rect_linear_gradient(ctx.surface, &tr, &cl, *color1, *color2);
                }
            }
            LowLevelOp::BlitSnapshot {
                bs_id,
                width,
                height,
                stride,
                src,
                dst,
            } => {
                crate::trace_counter!("raster.ops.blit_snap", 1);
                let td = ctx.current_transform.transform_rect(*dst);
                if let Some(cd) = ctx.current_clip.intersection(&td) {
                    let bs = ThingId::from_u64(*bs_id);
                    if let Some(ptr) = ctx.cache.get_or_map(bs, *stride, *height) {
                        let len = (*stride * *height) as usize;
                        let src_surf =
                            unsafe { Surface::new(ptr as *mut u8, len, *width, *height, *stride) };
                        blit_surface(ctx.surface, &src_surf, src, &td, &cd);
                    }
                }
            }
            LowLevelOp::BlitOpaque {
                image,
                src,
                dst,
                filter,
            } => {
                crate::trace_counter!("raster.ops.blit", 1);
                let td = ctx.current_transform.transform_rect(*dst);
                if let Some(cd) = ctx.current_clip.intersection(&td) {
                    blit_opaque(ctx.surface, image, src, &td, &cd, *filter);
                }
            }
            LowLevelOp::BlitAlpha {
                image,
                src,
                dst,
                filter,
                blend,
                const_alpha,
            } => {
                crate::trace_counter!("raster.ops.blit_alpha", 1);
                let td = ctx.current_transform.transform_rect(*dst);
                if let Some(cd) = ctx.current_clip.intersection(&td) {
                    blit_alpha(
                        ctx.surface,
                        image,
                        src,
                        &td,
                        &cd,
                        *filter,
                        *blend,
                        *const_alpha,
                    );
                }
            }
            LowLevelOp::TextSpan {
                text,
                pos,
                size,
                color,
                font_name,
                font_debug,
            } => {
                crate::trace_counter!("raster.ops.text", 1);
                let p = ctx.current_transform.transform_point_f(pos.x, pos.y);
                rasterize_text_locally(
                    ctx.surface,
                    text,
                    p.0,
                    p.1,
                    *size,
                    color.to_u32(),
                    &ctx.current_clip,
                    font_name.as_deref(),
                    *font_debug,
                );
            }
            LowLevelOp::StrokeRect { rect, color, width } => {
                crate::trace_counter!("raster.ops.stroke", 1);
                let tr = ctx.current_transform.transform_rect(*rect);
                stroke_rect_clipped_blend(
                    ctx.surface,
                    &tr,
                    *width,
                    color.to_u32(),
                    &ctx.current_clip,
                );
            }
            LowLevelOp::Line {
                from,
                to,
                color,
                width,
            } => {
                crate::trace_counter!("raster.ops.stroke", 1);
                let p0 = ctx.current_transform.transform_point_f(from.x, from.y);
                let p1 = ctx.current_transform.transform_point_f(to.x, to.y);
                line(
                    ctx.surface,
                    p0.0,
                    p0.1,
                    p1.0,
                    p1.1,
                    *width,
                    color.to_u32(),
                    &ctx.current_clip,
                );
            }
            LowLevelOp::FillCircle {
                center,
                radius,
                color,
            } => {
                crate::trace_counter!("raster.ops.fill", 1);
                let c = ctx.current_transform.transform_point(*center);
                fill_circle_blend(
                    ctx.surface,
                    c.x,
                    c.y,
                    *radius,
                    color.to_u32(),
                    &ctx.current_clip,
                );
            }
            LowLevelOp::FillArc {
                center,
                radius,
                start_angle,
                end_angle,
                color,
                aa,
            } => {
                crate::trace_counter!("raster.ops.fill", 1);
                let c = ctx.current_transform.transform_point(*center);
                fill_arc_clipped_blend(
                    ctx.surface,
                    c.x,
                    c.y,
                    *radius,
                    *start_angle,
                    *end_angle,
                    color.to_u32(),
                    *aa,
                    &ctx.current_clip,
                );
            }
            LowLevelOp::FillPath {
                path,
                color,
                fill_rule,
                aa,
            } => {
                crate::trace_counter!("raster.ops.fill", 1);
                fill_path(
                    ctx.surface,
                    path,
                    &ctx.current_transform,
                    color.to_u32(),
                    *fill_rule,
                    *aa,
                    &ctx.current_clip,
                );
            }
            LowLevelOp::StrokePath {
                path,
                color,
                width,
                cap,
                join,
                miter_limit,
                aa,
            } => {
                crate::trace_counter!("raster.ops.stroke", 1);
                stroke_path(
                    ctx.surface,
                    path,
                    &ctx.current_transform,
                    color.to_u32(),
                    *width,
                    *cap,
                    *join,
                    *miter_limit,
                    *aa,
                    &ctx.current_clip,
                );
            }
        }
    }
    crate::trace_counter!(
        "raster.execute_ns",
        stem::monotonic_ns().saturating_sub(start)
    );
}

#[inline(always)]
fn scale_ch(c: u8, a: u8) -> u32 {
    let t = c as u32 * a as u32;
    (t + (t >> 8) + 1) >> 8
}
#[inline(always)]
fn blend_ch(s: u8, d: u8, sa: u8) -> u8 {
    if sa == 255 {
        return s;
    }
    if sa == 0 {
        return d;
    }
    (scale_ch(s, sa) + scale_ch(d, 255 - sa)) as u8
}

fn blend_pixel(surface: &mut Surface, x: i32, y: i32, sr: u8, sg: u8, sb: u8, sa: u8) {
    if sa == 0 {
        return;
    }
    let offset = (surface.stride_bytes >> 2) * (y as usize) + (x as usize);
    let ptr = surface.ptr as *mut u32;
    unsafe {
        let dp = ptr.add(offset);
        let dv = *dp;
        if sa == 255 {
            *dp = ((sa as u32) << 24) | ((sr as u32) << 16) | ((sg as u32) << 8) | sb as u32;
            return;
        }
        let da = ((dv >> 24) & 0xFF) as u8;
        let (dr, dg, db) = (
            ((dv >> 16) & 0xFF) as u8,
            ((dv >> 8) & 0xFF) as u8,
            (dv & 0xFF) as u8,
        );
        // Alpha blend: out_a = sa + da * (255 - sa)
        // This is strictly 'src over' assuming un-premultiplied color blending approx
        let out_a = sa as u32 + ((da as u32 * (255 - sa as u32)) >> 8);
        // Correct color blending requires weighing by alpha, but for now we stick to simple channel blending
        // which matches the existing logic but adds Alpha write.
        // Actually existing logic `blend_ch` interpolates channels based on SA. This is correct for SrcOver if Dst is opaque.
        // If Dst is transparent, we need to respect that.
        // But for cursor (dst=0), simple blend_ch(s, 0, sa) = scale_ch(s, sa).
        // This is premultiplied color result?
        // Let's just write the blended rgb and the computed alpha.

        *dp = (out_a << 24)
            | ((blend_ch(sr, dr, sa) as u32) << 16)
            | ((blend_ch(sg, dg, sa) as u32) << 8)
            | (blend_ch(sb, db, sa) as u32);
    }
}

pub fn fill_rect_copy(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, color: u32) {
    let x0 = x.max(0);
    let y0 = y.max(0);
    let x1 = (x + w).min(surface.width());
    let y1 = (y + h).min(surface.height());
    for yy in y0..y1 {
        for xx in x0..x1 {
            surface.put_px(xx, yy, color);
        }
    }
}

/// Blit cursor overlay directly to surface (post-damage, always on top).
///
/// This function is called after the main DrawList rendering to composite
/// the pre-rasterized cursor snapshot. It bypasses the damage tracking
/// system since cursor movement should not trigger window repaints.
///
/// The cursor snapshot is expected to have pre-composited shadow layers.
pub fn blit_cursor_overlay(surface: &mut Surface, cursor: &Image, x: i32, y: i32) {
    let sw = surface.width();
    let sh = surface.height();

    for sy in 0..cursor.height as i32 {
        let dy = y + sy;
        if dy < 0 || dy >= sh {
            continue;
        }

        for sx in 0..cursor.width as i32 {
            let dx = x + sx;
            if dx < 0 || dx >= sw {
                continue;
            }

            let px = cursor.pixels[(sy as usize) * (cursor.width as usize) + (sx as usize)];
            let sa = ((px >> 24) & 0xFF) as u8;

            if sa == 0 {
                continue;
            }

            if sa == 255 {
                surface.put_px(dx, dy, px);
            } else {
                blend_pixel(
                    surface,
                    dx,
                    dy,
                    ((px >> 16) & 0xFF) as u8,
                    ((px >> 8) & 0xFF) as u8,
                    (px & 0xFF) as u8,
                    sa,
                );
            }
        }
    }
}

/// Draw a simple crosshair cursor fallback (e.g. while asset is loading).
pub fn draw_crosshair(surface: &mut Surface, x: i32, y: i32, color: u32) {
    let size = 8;
    let gap = 2;
    // Horizontal
    for dx in -size..=-gap {
        if x + dx >= 0 && x + dx < surface.width() && y >= 0 && y < surface.height() {
            surface.put_px(x + dx, y, color);
        }
        if x - dx >= 0 && x - dx < surface.width() && y >= 0 && y < surface.height() {
            surface.put_px(x - dx, y, color);
        }
    }
    // Vertical
    for dy in -size..=-gap {
        if x >= 0 && x < surface.width() && y + dy >= 0 && y + dy < surface.height() {
            surface.put_px(x, y + dy, color);
        }
        if x >= 0 && x < surface.width() && y - dy >= 0 && y - dy < surface.height() {
            surface.put_px(x, y - dy, color);
        }
    }
}
pub fn fill_rect_blend(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, color: u32) {
    let a = ((color >> 24) & 0xFF) as u8;
    if a == 255 {
        fill_rect_copy(surface, x, y, w, h, color);
        return;
    }
    if a == 0 {
        return;
    }
    let x0 = x.max(0);
    let y0 = y.max(0);
    let x1 = (x + w).min(surface.width());
    let y1 = (y + h).min(surface.height());
    let (sr, sg, sb) = (
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    );
    for yy in y0..y1 {
        for xx in x0..x1 {
            blend_pixel(surface, xx, yy, sr, sg, sb, a);
        }
    }
}
pub fn fill_rect_linear_gradient(
    surface: &mut Surface,
    rect: &Rect,
    clip: &Rect,
    color1: Color,
    color2: Color,
) {
    let x0 = clip.x().max(0);
    let y0 = clip.y().max(0);
    let x1 = (clip.x() + clip.width()).min(surface.width());
    let y1 = (clip.y() + clip.height()).min(surface.height());

    let (r1, g1, b1, a1) = (color1.r, color1.g, color1.b, color1.a);
    let (r2, g2, b2, a2) = (color2.r, color2.g, color2.b, color2.a);

    let width = rect.width();
    if width <= 0 {
        return;
    }

    for yy in y0..y1 {
        for xx in x0..x1 {
            let t = (xx - rect.x()) as f32 / width as f32;
            let t = t.clamp(0.0, 1.0);

            let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
            let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
            let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
            let a = (a1 as f32 + (a2 as f32 - a1 as f32) * t) as u8;

            if a == 255 {
                surface.put_px(
                    xx,
                    yy,
                    (255 << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
                );
            } else if a > 0 {
                blend_pixel(surface, xx, yy, r, g, b, a);
            }
        }
    }
}
fn stroke_rect_clipped_blend(
    surface: &mut Surface,
    rect: &Rect,
    width: i32,
    color: u32,
    clip: &Rect,
) {
    let parts = [
        Rect::new(rect.x(), rect.y(), rect.width(), width),
        Rect::new(
            rect.x(),
            rect.y() + rect.height() - width,
            rect.width(),
            width,
        ),
        Rect::new(rect.x(), rect.y() + width, width, rect.height() - 2 * width),
        Rect::new(
            rect.x() + rect.width() - width,
            rect.y() + width,
            width,
            rect.height() - 2 * width,
        ),
    ];
    for p in parts {
        if let Some(c) = p.intersection(clip) {
            fill_rect_blend(surface, c.x(), c.y(), c.width(), c.height(), color);
        }
    }
}
pub fn fill_circle_blend(surface: &mut Surface, cx: i32, cy: i32, r: i32, color: u32, clip: &Rect) {
    let a = ((color >> 24) & 0xFF) as u8;
    if a == 0 {
        return;
    }
    let (x0, y0, x1, y1) = (
        (cx - r).max(clip.x()).max(0),
        (cy - r).max(clip.y()).max(0),
        (cx + r).min(clip.x() + clip.width()).min(surface.width()),
        (cy + r).min(clip.y() + clip.height()).min(surface.height()),
    );
    let (sr, sg, sb) = (
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    );
    for y in y0..y1 {
        for x in x0..x1 {
            let dx = x - cx;
            let dy = y - cy;
            if (dx * dx) + (dy * dy) <= r * r {
                blend_pixel(surface, x, y, sr, sg, sb, a);
            }
        }
    }
}
pub fn fill_arc_clipped_blend(
    surface: &mut Surface,
    cx: i32,
    cy: i32,
    r: i32,
    s_deg: f32,
    e_deg: f32,
    color: u32,
    aa: EdgeAA,
    clip: &Rect,
) {
    let sa = ((color >> 24) & 0xFF) as u8;
    if sa == 0 {
        return;
    }
    let m = if aa != EdgeAA::None { 1 } else { 0 };
    let (x0, y0, x1, y1) = (
        (cx - r - m).max(clip.x()).max(0),
        (cy - r - m).max(clip.y()).max(0),
        (cx + r + m)
            .min(clip.x() + clip.width())
            .min(surface.width()),
        (cy + r + m)
            .min(clip.y() + clip.height())
            .min(surface.height()),
    );
    let (sr, sg, sb) = (
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    );
    let (mut s, mut e) = (s_deg, e_deg);
    while s < 0.0 {
        s += 360.0;
    }
    while s >= 360.0 {
        s -= 360.0;
    }
    while e < s {
        e += 360.0;
    }
    for y in y0..y1 {
        for x in x0..x1 {
            let (dx, dy) = (x as f32 + 0.5 - cx as f32, y as f32 + 0.5 - cy as f32);
            let d2 = dx * dx + dy * dy;
            if d2 <= (r as f32 + 1.0) * (r as f32 + 1.0) {
                let mut a = libm::atan2f(dy, dx) * 180.0 / 3.14159265;
                while a < s {
                    a += 360.0;
                }
                if a <= e {
                    let cov = if aa != EdgeAA::None {
                        (r as f32 - libm::sqrtf(d2) + 0.5).clamp(0.0, 1.0)
                    } else {
                        if d2 <= (r * r) as f32 {
                            1.0
                        } else {
                            0.0
                        }
                    };
                    if cov > 0.0 {
                        blend_pixel(surface, x, y, sr, sg, sb, (sa as f32 * cov) as u8);
                    }
                }
            }
        }
    }
}
pub fn line(
    surface: &mut Surface,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    width: f32,
    xrgb: u32,
    clip: &Rect,
) {
    if width > 1.2 {
        // Use thick line expansion: convert to quad and fill.
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = libm::sqrtf(dx * dx + dy * dy);
        if len < 0.1 {
            return;
        }
        let nx = -dy / len;
        let ny = dx / len;
        let w2 = width * 0.5;

        use crate::isa::{Path2D, PathVerb, PointF};
        let verbs = vec![
            PathVerb::MoveTo(PointF {
                x: x0 + nx * w2,
                y: y0 + ny * w2,
            }),
            PathVerb::LineTo(PointF {
                x: x1 + nx * w2,
                y: y1 + ny * w2,
            }),
            PathVerb::LineTo(PointF {
                x: x1 - nx * w2,
                y: y1 - ny * w2,
            }),
            PathVerb::LineTo(PointF {
                x: x0 - nx * w2,
                y: y0 - ny * w2,
            }),
            PathVerb::Close,
        ];
        let path = Path2D { verbs };
        fill_path(
            surface,
            &path,
            &Transform2D::identity(),
            xrgb,
            crate::isa::FillRule::NonZero,
            EdgeAA::Coverage8,
            clip,
        );
        return;
    }

    // Basic Bresenham for narrow lines
    let mut xi = x0 as i32;
    let mut yi = y0 as i32;
    let x1i = x1 as i32;
    let y1i = y1 as i32;

    let (dx, dy) = ((x1i - xi).abs(), -(y1i - yi).abs());
    let (sx, sy) = (if xi < x1i { 1 } else { -1 }, if yi < y1i { 1 } else { -1 });
    let mut err = dx + dy;
    loop {
        if xi >= clip.x()
            && xi < clip.x() + clip.width()
            && yi >= clip.y()
            && yi < clip.y() + clip.height()
        {
            if xi >= 0 && xi < surface.width() && yi >= 0 && yi < surface.height() {
                surface.put_px(xi, yi, xrgb);
            }
        }
        if xi == x1i && yi == y1i {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            xi += sx;
        }
        if e2 <= dx {
            err += dx;
            yi += sy;
        }
    }
}
fn blit_opaque(
    surface: &mut Surface,
    image: &Image,
    src: &Rect,
    fd: &Rect,
    cd: &Rect,
    _fm: FilterMode,
) {
    let (sx_f, sy_f) = (
        src.width() as f32 / fd.width() as f32,
        src.height() as f32 / fd.height() as f32,
    );
    for dy in cd.y()..cd.y() + cd.height() {
        for dx in cd.x()..cd.x() + cd.width() {
            let (sx, sy) = (
                (src.x() as f32 + (dx - fd.x()) as f32 * sx_f) as i32,
                (src.y() as f32 + (dy - fd.y()) as f32 * sy_f) as i32,
            );
            if sx >= 0 && sx < image.width as i32 && sy >= 0 && sy < image.height as i32 {
                surface.put_px(
                    dx,
                    dy,
                    image.pixels[(sy as usize) * (image.width as usize) + (sx as usize)],
                );
            }
        }
    }
}
fn blit_alpha(
    surface: &mut Surface,
    image: &Image,
    src: &Rect,
    fd: &Rect,
    cd: &Rect,
    _fm: FilterMode,
    blend: BlendMode,
    ca: Option<u8>,
) {
    let cav = ca.unwrap_or(255) as u32;
    let (sx_f, sy_f) = (
        src.width() as f32 / fd.width() as f32,
        src.height() as f32 / fd.height() as f32,
    );
    for dy in cd.y()..cd.y() + cd.height() {
        for dx in cd.x()..cd.x() + cd.width() {
            let (sx, sy) = (
                (src.x() as f32 + (dx - fd.x()) as f32 * sx_f) as i32,
                (src.y() as f32 + (dy - fd.y()) as f32 * sy_f) as i32,
            );
            if sx >= 0 && sx < image.width as i32 && sy >= 0 && sy < image.height as i32 {
                let px = image.pixels[(sy as usize) * (image.width as usize) + (sx as usize)];
                let mut a = (px >> 24) & 0xFF;
                if cav != 255 {
                    a = (a * cav) / 255;
                }
                if a == 0 {
                    continue;
                }
                if blend == BlendMode::Src || a == 255 {
                    surface.put_px(dx, dy, px);
                    continue;
                }
                blend_pixel(
                    surface,
                    dx,
                    dy,
                    ((px >> 16) & 0xFF) as u8,
                    ((px >> 8) & 0xFF) as u8,
                    (px & 0xFF) as u8,
                    a as u8,
                );
            }
        }
    }
}

/// Render text using atlas-based fontd IPC (batch EnsureGlyphs).
/// Returns true if rendering was successful, false to fallback to old path.
fn rasterize_text_atlas(
    surface: &mut Surface,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: u32,
    clip: &Rect,
    rf: Option<&str>,
) -> bool {
    // Check if font client is available
    if !font_client::is_available() {
        return false;
    }

    // Get face_id from font graph
    let face_id = font_graph::try_with_graph_if_ready(|graph| {
        let stack = graph.resolve_stack(rf);
        stack
            .iter()
            .find_map(|f| graph.select_face_for_family(*f, FontStyle::default()))
    })
    .flatten();

    let face_id = match face_id {
        Some(id) => id,
        None => return false,
    };

    // Get metrics
    let metrics = match font_client::get_metrics(face_id, size as u16) {
        Some(m) => m,
        None => return false,
    };

    // Collect glyph IDs for all characters
    let glyph_ids: Vec<u32> = text
        .chars()
        .filter(|c| *c != '\n' && *c != '\r')
        .map(|c| c as u32)
        .collect();

    if glyph_ids.is_empty() {
        return true;
    }

    // Batch request all glyphs
    let entries = font_client::ensure_glyphs(face_id, size as u16, &glyph_ids);
    if entries.is_empty() {
        return false; // No glyphs available yet
    }

    // Use the glyph cache directly for lookup
    let (mut pen_x, pen_y) = (x, y + metrics.ascent as f32);
    let (sr, sg, sb, sa) = (
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
        ((color >> 24) & 0xFF) as u8,
    );

    // Render each character
    for ch in text.chars() {
        if ch == '\n' || ch == '\r' {
            continue; // Skip for now (single line)
        }

        let glyph_id = ch as u32;
        if let Some(g) = font_client::get_glyph(face_id, size as u16, glyph_id) {
            // Empty glyph (space)
            if g.w == 0 || g.h == 0 {
                pen_x += g.advance as f32;
                continue;
            }

            // Blit from atlas
            let gx = pen_x as i32 + g.bearing_x as i32;
            let gy = pen_y as i32 - g.bearing_y as i32;

            // Get atlas mapping and blit
            font_client::with_atlas(face_id, size as u16, |atlas| {
                for row in 0..g.h as i32 {
                    for col in 0..g.w as i32 {
                        let cx = gx + col;
                        let cy = gy + row;

                        // Clip check
                        if cx < clip.x()
                            || cx >= clip.x() + clip.width()
                            || cy < clip.y()
                            || cy >= clip.y() + clip.height()
                        {
                            continue;
                        }

                        // Get alpha from atlas
                        let atlas_x = g.x as u32 + col as u32;
                        let atlas_y = g.y as u32 + row as u32;
                        let a = atlas.get_pixel(atlas_x, atlas_y);

                        if a > 0 {
                            let blended_a = scale_ch(a, sa) as u8;
                            blend_pixel(surface, cx, cy, sr, sg, sb, blended_a);
                        }
                    }
                }
            });

            pen_x += g.advance as f32;
        } else {
            // Glyph not in cache - request and draw placeholder
            pen_x += size * 0.4;
        }
    }

    true
}

fn rasterize_text_locally(
    surface: &mut Surface,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: u32,
    clip: &Rect,
    rf: Option<&str>,
    fd: bool,
) {
    let t_start = stem::monotonic_ns();
    let mut stats = (0u64, 0u64, 0u64, 0u64);
    if !font_graph::has_fonts_ready() {
        rasterize_text_fallback(surface, text, x, y, size, color, clip, rf, fd);
        return;
    }
    let mut handled = false;
    if let Some(r) = rf {
        if !font_graph::try_with_graph_if_ready(|g| g.has_font(r)).unwrap_or(false) {
            rasterize_text_fallback(surface, text, x, y, size, color, clip, rf, fd);
            return;
        }
    }
    let (mut r_ns_t, mut b_ns_t) = (0u64, 0u64);
    font_graph::with_graph(|graph| {
        let stack = graph.resolve_stack(rf);
        if stack.is_empty() {
            return;
        }
        let primary_face_id = stack
            .iter()
            .find_map(|f| graph.select_face_for_family(*f, FontStyle::default()));
        let primary_font = match primary_face_id.and_then(|id| graph.font_for_face(id)) {
            Some(f) => f,
            None => return,
        };
        handled = true;
        let met = match primary_font.font.horizontal_line_metrics(size) {
            Some(m) => m,
            None => fontdue::LineMetrics {
                ascent: size * 0.8f32,
                descent: size * 0.2f32,
                line_gap: 0.0,
                new_line_size: size * 1.2f32,
            },
        };
        let (mut pen_x, mut pen_y) = (x, y + met.ascent);
        for ch in text.chars() {
            if ch == '\n' {
                pen_x = x;
                pen_y += libm::fmaxf(met.new_line_size, size * 1.1f32);
                continue;
            }
            if ch == '\r' {
                continue;
            }

            let res = graph
                .resolve_face_for_glyph(&stack, FontStyle::default(), ch as u32)
                .or_else(|| primary_face_id.and_then(|id| graph.resolved_face_by_id(id)));

            let r = match res {
                Some(r) => r,
                None => {
                    pen_x += size * 0.4f32;
                    continue;
                }
            };

            let f = match graph.font_for_face(r.face_id) {
                Some(f) => f,
                None => {
                    pen_x += size * 0.4f32;
                    continue;
                }
            };

            // 1. Try local cache first
            let config = GlyphRasterConfig {
                glyph_index: f.font.lookup_glyph_index(ch),
                px: size,
                font_hash: f.font.file_hash(),
            };
            let key = (config.px.to_bits(), config.glyph_index, config.font_hash);

            let mut cached_result = None;
            {
                let cache = f.glyph_cache.lock();
                if let Some(cached) = cache.get(&key) {
                    cached_result = Some(cached.clone());
                }
            }

            // 2. Try graph if not in local cache
            if cached_result.is_none() {
                if let Some(gid) = graph.find_glyph(r.face_id, size as u16, ch as u32) {
                    if let Some(res) = f.get_glyph_from_graph(gid) {
                        let mut cache = f.glyph_cache.lock();
                        cache.insert(key, res.clone());
                        cached_result = Some(res);
                    }
                }
            }

            if let Some((m, b)) = cached_result {
                // Draw REAL glyph
                let r_start = stem::monotonic_ns();
                stats.0 += 1;
                let (gx, gy) = (
                    (pen_x + m.xmin as f32) as i32,
                    pen_y as i32 - m.height as i32 - m.ymin,
                );
                let (sr, sg, sb, sa) = (
                    ((color >> 16) & 0xFF) as u8,
                    ((color >> 8) & 0xFF) as u8,
                    (color & 0xFF) as u8,
                    ((color >> 24) & 0xFF) as u8,
                );
                for r in 0..m.height {
                    for c in 0..m.width {
                        let (cx, cy) = (gx + c as i32, gy + r as i32);
                        if cx >= clip.x()
                            && cx < clip.x() + clip.width()
                            && cy >= clip.y()
                            && cy < clip.y() + clip.height()
                        {
                            let a = b[r * (m.width as usize) + c];
                            if a > 0 {
                                blend_pixel(surface, cx, cy, sr, sg, sb, scale_ch(a, sa) as u8);
                                stats.1 += 1;
                            }
                        }
                    }
                }
                pen_x += m.advance_width;
                r_ns_t += stem::monotonic_ns().saturating_sub(r_start);
            } else {
                // 3. Request if still MISSING and draw placeholder
                graph.request_glyph(r.face_id, size as u16, ch as u32);

                let pw = (size * 0.4) as i32;
                let ph = (size * 0.8) as i32;
                let px = pen_x as i32;
                let py = (pen_y - size * 0.8) as i32;

                // Draw a simple box placeholder
                fill_rect_blend(surface, px, py, pw, ph, (color & 0x7FFFFFFF) | 0x40000000);
                pen_x += size * 0.5f32;
            }
        }
    });
    if !handled {
        rasterize_text_fallback(surface, text, x, y, size, color, clip, rf, fd);
    }
    // Diagnostic: log text rasterization stats for debugging rendering issues
    static mut LOG_COUNT: u64 = 0;
    unsafe {
        LOG_COUNT += 1;
        if LOG_COUNT <= 10 || LOG_COUNT % 500 == 0 {
            stem::info!(
                "[raster] text glyphs={} pixels={} handled={} text_chars={}",
                stats.0,
                stats.1,
                handled,
                text.len()
            );
        }
    }
    crate::trace_counter!("text.glyphs", stats.0);
    crate::trace_counter!("text.pixels", stats.1);
    crate::trace_counter!("text.raster_ns", r_ns_t);
    crate::trace_counter!("text.blit_ns", b_ns_t);
    crate::trace_counter!("text.ns", stem::monotonic_ns().saturating_sub(t_start));
}

fn blit_surface(
    dst_surface: &mut Surface,
    src_surface: &Surface,
    src_rect: &Rect,
    fd: &Rect,
    cd: &Rect,
) {
    let (sx_f, sy_f) = (
        src_rect.width() as f32 / fd.width() as f32,
        src_rect.height() as f32 / fd.height() as f32,
    );
    for dy in cd.y()..cd.y() + cd.height() {
        for dx in cd.x()..cd.x() + cd.width() {
            let (sx, sy) = (
                (src_rect.x() as f32 + (dx - fd.x()) as f32 * sx_f) as i32,
                (src_rect.y() as f32 + (dy - fd.y()) as f32 * sy_f) as i32,
            );
            let px = src_surface.get_px(sx, sy);
            let mut a = (px >> 24) & 0xFF;
            if a == 0 {
                continue;
            }
            if a == 255 {
                dst_surface.put_px(dx, dy, px);
                continue;
            }
            blend_pixel(
                dst_surface,
                dx,
                dy,
                ((px >> 16) & 0xFF) as u8,
                ((px >> 8) & 0xFF) as u8,
                (px & 0xFF) as u8,
                a as u8,
            );
        }
    }
}

fn rasterize_text_fallback(
    surface: &mut Surface,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: u32,
    clip: &Rect,
    rf: Option<&str>,
    _fd: bool,
) {
    let fonts = crate::ASSETS.get_fonts();
    if fonts.is_empty() {
        return;
    }
    let font = if let Some(r) = rf {
        let needle = r.to_lowercase();
        let needle_no_ext = needle
            .strip_suffix(".ttf")
            .or_else(|| needle.strip_suffix(".otf"))
            .unwrap_or(&needle);
        let mut match_font = fonts.iter().find(|f| {
            let name = f.name.to_lowercase();
            name.contains(&needle) || name.contains(needle_no_ext)
        });
        if match_font.is_none() {
            let needle_norm = normalize_font_token(needle_no_ext);
            if !needle_norm.is_empty() {
                match_font = fonts.iter().find(|f| {
                    let name_norm = normalize_font_token(&f.name);
                    name_norm.contains(&needle_norm)
                });
            }
        }
        match_font
            .or_else(|| fonts.iter().find(|f| f.name.contains("NotoSans-Regular")))
            .unwrap_or(&fonts[0])
    } else {
        fonts
            .iter()
            .find(|f| f.name.contains("NotoSans-Regular"))
            .unwrap_or(&fonts[0])
    };
    let (sa, sr, sg, sb) = (
        ((color >> 24) & 0xFF) as u8,
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    );
    let met = match font.font.horizontal_line_metrics(size) {
        Some(m) => m,
        None => fontdue::LineMetrics {
            ascent: size * 0.8f32,
            descent: size * 0.2f32,
            line_gap: 0.0,
            new_line_size: size * 1.2f32,
        },
    };
    let (mut px, mut py) = (x, y + met.ascent);
    for ch in text.chars() {
        if ch == '\n' {
            px = x;
            py += libm::fmaxf(met.new_line_size, size * 1.1f32);
            continue;
        }
        if ch == '\r' {
            continue;
        }
        let gi = font.font.lookup_glyph_index(ch);
        if gi == 0 {
            px += size * 0.4f32;
            continue;
        }
        let (m, b) = font.get_glyph(GlyphRasterConfig {
            glyph_index: gi,
            px: size,
            font_hash: font.font.file_hash(),
        });
        let (gx, gy) = (
            (px + m.xmin as f32) as i32,
            py as i32 - m.height as i32 - m.ymin,
        );
        for r in 0..m.height {
            for c in 0..m.width {
                let (cx, cy) = (gx + c as i32, gy + r as i32);
                if cx >= clip.x()
                    && cx < clip.x() + clip.width()
                    && cy >= clip.y()
                    && cy < clip.y() + clip.height()
                {
                    let a = b[r * (m.width as usize) + c];
                    if a > 0 {
                        blend_pixel(surface, cx, cy, sr, sg, sb, scale_ch(a, sa) as u8);
                    }
                }
            }
        }
        px += m.advance_width;
    }
}

fn normalize_font_token(token: &str) -> alloc::string::String {
    let mut out = alloc::string::String::with_capacity(token.len());
    for ch in token.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        }
    }
    out
}

// Fixed point 16.16
type Fixed = i32;
const FIXED_SHIFT: i32 = 16;
const FIXED_ONE: i32 = 1 << FIXED_SHIFT;
const SUPERSAMPLE_SCALE: i32 = 2;
const SUPERSAMPLE_SAMPLES: u8 = (SUPERSAMPLE_SCALE * SUPERSAMPLE_SCALE) as u8;
fn float_to_fixed(f: f32) -> Fixed {
    (f * (FIXED_ONE as f32)) as i32
}
fn int_to_fixed(i: i32) -> Fixed {
    i << FIXED_SHIFT
}
fn fixed_floor(f: Fixed) -> i32 {
    f >> FIXED_SHIFT
}
// fn fixed_ceil(f: Fixed) -> i32 { (f + FIXED_ONE - 1) >> FIXED_SHIFT }

struct Edge {
    y_max: i32,   // scanline int
    x: Fixed,     // current x at y_min (or current scanline)
    dx_dy: Fixed, // slope
    y_min: i32,   // scanline int (start)
    winding: i32, // 1 or -1
}

fn flatten_quad<F>(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    flatness_sq: f32,
    add_edge_fn: &mut F,
) where
    F: FnMut((f32, f32), (f32, f32)),
{
    // Simple flatness check: distance from p1 to (p0+p2)/2
    let mid_x = (p0.0 + p2.0) * 0.5;
    let mid_y = (p0.1 + p2.1) * 0.5;
    let dx = p1.0 - mid_x;
    let dy = p1.1 - mid_y;
    if dx * dx + dy * dy < flatness_sq {
        add_edge_fn(p0, p2);
    } else {
        let p01 = ((p0.0 + p1.0) * 0.5, (p0.1 + p1.1) * 0.5);
        let p12 = ((p1.0 + p2.0) * 0.5, (p1.1 + p2.1) * 0.5);
        let p012 = ((p01.0 + p12.0) * 0.5, (p01.1 + p12.1) * 0.5);
        flatten_quad(p0, p01, p012, flatness_sq, add_edge_fn);
        flatten_quad(p012, p12, p2, flatness_sq, add_edge_fn);
    }
}

fn dist_sq_point_line_segment(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let len_sq = dx * dx + dy * dy;
    if len_sq == 0.0 {
        let ddx = p.0 - a.0;
        let ddy = p.1 - a.1;
        return ddx * ddx + ddy * ddy;
    }
    let t = ((p.0 - a.0) * dx + (p.1 - a.1) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);
    let px = a.0 + t * dx;
    let py = a.1 + t * dy;
    let ddx = p.0 - px;
    let ddy = p.1 - py;
    ddx * ddx + ddy * ddy
}

fn flatten_cubic<F>(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    flatness_sq: f32,
    add_edge_fn: &mut F,
) where
    F: FnMut((f32, f32), (f32, f32)),
{
    // Distance from control points to the chord p0-p3
    let d1_sq = dist_sq_point_line_segment(p1, p0, p3);
    let d2_sq = dist_sq_point_line_segment(p2, p0, p3);

    // We check against flatness_sq.
    // Spec says strictly: if d < Tolerance, then flat.
    // We use max distance of any control point.
    if d1_sq.max(d2_sq) < flatness_sq {
        add_edge_fn(p0, p3);
    } else {
        let p01 = ((p0.0 + p1.0) * 0.5, (p0.1 + p1.1) * 0.5);
        let p12 = ((p1.0 + p2.0) * 0.5, (p1.1 + p2.1) * 0.5);
        let p23 = ((p2.0 + p3.0) * 0.5, (p2.1 + p3.1) * 0.5);
        let p012 = ((p01.0 + p12.0) * 0.5, (p01.1 + p12.1) * 0.5);
        let p123 = ((p12.0 + p23.0) * 0.5, (p12.1 + p23.1) * 0.5);
        let p0123 = ((p012.0 + p123.0) * 0.5, (p012.1 + p123.1) * 0.5);
        flatten_cubic(p0, p01, p012, p0123, flatness_sq, add_edge_fn);
        flatten_cubic(p0123, p123, p23, p3, flatness_sq, add_edge_fn);
    }
}

fn build_edges(path: &crate::isa::Path2D, transform: &Transform2D, scale: i32) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::with_capacity(path.verbs.len());

    let add_edge = |e: &mut Vec<Edge>, p0: (f32, f32), p1: (f32, f32)| {
        let (x0, y0) = transform.transform_point_f(p0.0, p0.1);
        let (x1, y1) = transform.transform_point_f(p1.0, p1.1);

        let y0_i = libm::floorf(y0 * scale as f32) as i32;
        let y1_i = libm::floorf(y1 * scale as f32) as i32;

        if y0_i == y1_i {
            return;
        }

        let (p_start, p_end, dir) = if y0_i < y1_i {
            ((x0, y0), (x1, y1), 1)
        } else {
            ((x1, y1), (x0, y0), -1)
        };

        let dy = p_end.1 - p_start.1;
        let dx = p_end.0 - p_start.0;
        let slope = if dy != 0.0 {
            // Slope is dx/dy.
            // We want change in Scaled X per 1 unit of Scaled Y.
            // d(ScaledX)/d(ScaledY) = (dx * scale) / (dy * scale) = dx/dy.
            // Original code incorrectly multiplied by scale.
            // We also clamp to prevent fixed-point overflow for horizontal-ish lines.
            let s = dx / dy;
            let clamped = s.clamp(-30000.0, 30000.0);
            float_to_fixed(clamped)
        } else {
            0
        };

        let y_start_int = y0_i.min(y1_i);
        let y_end_int = y0_i.max(y1_i);
        let y_isect = (y_start_int as f32 + 0.5) / scale as f32;
        let x_current =
            float_to_fixed((p_start.0 + (y_isect - p_start.1) * (dx / dy)) * scale as f32);

        e.push(Edge {
            y_min: y_start_int,
            y_max: y_end_int,
            x: x_current,
            dx_dy: slope,
            winding: dir,
        });
    };

    let scale_sq = (transform.a * transform.a + transform.b * transform.b)
        .max(transform.c * transform.c + transform.d * transform.d);

    // User requested target 0.25px or better.
    // flatness_sq is error^2.
    // If we want 0.22px error, sq is ~0.05.
    // Old: 0.25 / scale_sq (0.5px error).
    let flatness_sq = 0.05 / (scale_sq * scale as f32 * scale as f32).max(0.01);

    let mut current_p: Option<(f32, f32)> = None;
    let mut start_p: Option<(f32, f32)> = None;

    for verb in &path.verbs {
        match verb {
            crate::isa::PathVerb::MoveTo(p) => {
                start_p = Some((p.x, p.y));
                current_p = Some((p.x, p.y));
            }
            crate::isa::PathVerb::LineTo(p) => {
                if let Some(c) = current_p {
                    add_edge(&mut edges, c, (p.x, p.y));
                    current_p = Some((p.x, p.y));
                }
            }
            crate::isa::PathVerb::QuadTo(p1, p) => {
                if let Some(c) = current_p {
                    flatten_quad(c, (p1.x, p1.y), (p.x, p.y), flatness_sq, &mut |p0, p1| {
                        add_edge(&mut edges, p0, p1);
                    });
                    current_p = Some((p.x, p.y));
                }
            }
            crate::isa::PathVerb::CubicTo(p1, p2, p) => {
                if let Some(c) = current_p {
                    flatten_cubic(
                        c,
                        (p1.x, p1.y),
                        (p2.x, p2.y),
                        (p.x, p.y),
                        flatness_sq,
                        &mut |p0, p1| {
                            add_edge(&mut edges, p0, p1);
                        },
                    );
                    current_p = Some((p.x, p.y));
                }
            }
            crate::isa::PathVerb::Close => {
                if let (Some(c), Some(s)) = (current_p, start_p) {
                    if c != s {
                        add_edge(&mut edges, c, s);
                        current_p = Some(s);
                    }
                }
            }
        }
    }

    // Instrumentation as requested
    static mut LOG_COUNT: u64 = 0;
    unsafe {
        LOG_COUNT += 1;
        if LOG_COUNT <= 5 || LOG_COUNT % 1000 == 0 {
            stem::info!(
                "[raster] build_edges: verbs={} flat_sq={} edges={}",
                path.verbs.len(),
                flatness_sq,
                edges.len()
            );
        }
    }

    edges
}

pub fn fill_path(
    surface: &mut Surface,
    path: &crate::isa::Path2D,
    transform: &Transform2D,
    color: u32,
    fill_rule: crate::isa::FillRule,
    aa: EdgeAA,
    clip: &Rect,
) {
    let sa = ((color >> 24) & 0xFF) as u8;
    if sa == 0 {
        return;
    }
    if aa == EdgeAA::Coverage8 {
        fill_path_aa(surface, path, transform, color, fill_rule, clip);
        return;
    }
    let sr = ((color >> 16) & 0xFF) as u8;
    let sg = ((color >> 8) & 0xFF) as u8;
    let sb = (color & 0xFF) as u8;

    let mut edges = build_edges(path, transform, 1);

    // Sort edges by y_min
    edges.sort_by(|a, b| a.y_min.cmp(&b.y_min));

    // 2. Scanline Sweep
    let y_min = clip.y();
    let y_max = clip.y() + clip.height();

    let mut active_edges: Vec<Edge> = Vec::with_capacity(16);
    let mut edge_idx = 0;

    let (sa, sr, sg, sb) = (
        ((color >> 24) & 0xFF) as u8,
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    );

    for y in y_min..y_max {
        // Add new edges
        while edge_idx < edges.len() && edges[edge_idx].y_min <= y {
            if edges[edge_idx].y_max > y {
                active_edges.push(Edge { ..edges[edge_idx] }); // Push copy
            }
            edge_idx += 1;
        }

        // Remove finished edges
        active_edges.retain(|e| e.y_max > y);

        if active_edges.is_empty() {
            continue;
        }

        // Sort by x
        active_edges.sort_by(|a, b| a.x.cmp(&b.x));

        // Fill spans
        match fill_rule {
            crate::isa::FillRule::EvenOdd => {
                // Pair: 0-1, 2-3
                let mut i = 0;
                while i + 1 < active_edges.len() {
                    let x0 = fixed_floor(active_edges[i].x);
                    let x1 = fixed_floor(active_edges[i + 1].x);
                    let start = x0.max(clip.x()).min(clip.x() + clip.width());
                    let end = x1.max(clip.x()).min(clip.x() + clip.width());
                    if end > start {
                        if sa == 255 {
                            fill_rect_copy(surface, start, y, end - start, 1, color);
                        } else {
                            for xx in start..end {
                                blend_pixel(surface, xx, y, sr, sg, sb, sa);
                            }
                        }
                    }
                    i += 2;
                }
            }
            crate::isa::FillRule::NonZero => {
                let mut winding = 0;
                let mut start_x = 0;
                for i in 0..active_edges.len() {
                    let x = fixed_floor(active_edges[i].x);
                    if winding == 0 {
                        start_x = x;
                    }
                    winding += active_edges[i].winding;
                    if winding == 0 {
                        let end_x = x;
                        let start = start_x.max(clip.x()).min(clip.x() + clip.width());
                        let end = end_x.max(clip.x()).min(clip.x() + clip.width());
                        if end > start {
                            if sa == 255 {
                                fill_rect_copy(surface, start, y, end - start, 1, color);
                            } else {
                                for xx in start..end {
                                    blend_pixel(surface, xx, y, sr, sg, sb, sa);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Update x for next scanline
        for e in &mut active_edges {
            e.x += e.dx_dy;
        }
    }
}

/// Rasterize a path with 2x2 supersampling coverage.
///
/// This keeps the rasterizer deterministic while producing partial coverage
/// on edges (4 samples per pixel). The resulting coverage scales alpha before
/// blending, matching the existing `blend_pixel` path.
///
/// ```rust,ignore
/// use crate::isa::{FillRule, Path2D, PathVerb, PointF};
/// use crate::isa::EdgeAA;
/// use crate::isa::Rect;
/// use crate::surface::Surface;
/// let mut buffer = vec![0u8; 4 * 4 * 4];
/// let mut surface = unsafe { Surface::zeroed(buffer.as_mut_ptr(), buffer.len(), 4, 4, 16) };
/// let path = Path2D {
///     verbs: vec![
///         PathVerb::MoveTo(PointF { x: 0.0, y: 0.0 }),
///         PathVerb::LineTo(PointF { x: 3.0, y: 0.0 }),
///         PathVerb::LineTo(PointF { x: 0.0, y: 3.0 }),
///         PathVerb::Close,
///     ],
/// };
/// fill_path(
///     &mut surface,
///     &path,
///     &crate::isa::Transform2D::identity(),
///     0xFFFF_FFFF,
///     FillRule::NonZero,
///     EdgeAA::Coverage8,
///     &Rect::new(0, 0, 4, 4),
/// );
/// ```
fn fill_path_aa(
    surface: &mut Surface,
    path: &crate::isa::Path2D,
    transform: &Transform2D,
    color: u32,
    fill_rule: crate::isa::FillRule,
    clip: &Rect,
) {
    let sa = ((color >> 24) & 0xFF) as u8;
    if sa == 0 {
        return;
    }
    let sr = ((color >> 16) & 0xFF) as u8;
    let sg = ((color >> 8) & 0xFF) as u8;
    let sb = (color & 0xFF) as u8;

    let mut edges = build_edges(path, transform, SUPERSAMPLE_SCALE);
    edges.sort_by(|a, b| a.y_min.cmp(&b.y_min));

    let clip_w = clip.width();
    let clip_h = clip.height();
    if clip_w <= 0 || clip_h <= 0 {
        return;
    }

    let mut coverage = vec![0u8; (clip_w as usize) * (clip_h as usize)];

    let y_min = clip.y() * SUPERSAMPLE_SCALE;
    let y_max = (clip.y() + clip.height()) * SUPERSAMPLE_SCALE;
    let clip_x_sub = clip.x() * SUPERSAMPLE_SCALE;
    let clip_x_sub_max = (clip.x() + clip.width()) * SUPERSAMPLE_SCALE;

    let mut active_edges: Vec<Edge> = Vec::with_capacity(16);
    let mut edge_idx = 0;

    // Calculate bounding box of all edges to narrow scan area
    if edges.is_empty() {
        return;
    }
    let mut b_xmin = edges[0].x;
    let mut b_xmax = edges[0].x;
    let mut b_ymin = edges[0].y_min;
    let mut b_ymax = edges[0].y_max;
    for e in &edges {
        b_xmin = b_xmin.min(e.x).min(e.x + e.dx_dy * (e.y_max - e.y_min));
        b_xmax = b_xmax.max(e.x).max(e.x + e.dx_dy * (e.y_max - e.y_min));
        b_ymin = b_ymin.min(e.y_min);
        b_ymax = b_ymax.max(e.y_max);
    }

    let path_clip_xmin = fixed_floor(b_xmin).max(clip_x_sub);
    let path_clip_xmax = (fixed_floor(b_xmax) + 1).min(clip_x_sub_max);
    let path_clip_ymin = b_ymin.max(y_min);
    let path_clip_ymax = b_ymax.min(y_max);

    if path_clip_xmax <= path_clip_xmin || path_clip_ymax <= path_clip_ymin {
        return;
    }

    crate::trace_counter!("raster.path.edges", edges.len());

    for y_sub in path_clip_ymin..path_clip_ymax {
        while edge_idx < edges.len() && edges[edge_idx].y_min <= y_sub {
            if edges[edge_idx].y_max > y_sub {
                active_edges.push(Edge { ..edges[edge_idx] });
            }
            edge_idx += 1;
        }

        active_edges.retain(|e| e.y_max > y_sub);

        if active_edges.is_empty() {
            continue;
        }

        active_edges.sort_by(|a, b| a.x.cmp(&b.x));

        match fill_rule {
            crate::isa::FillRule::EvenOdd => {
                let mut i = 0;
                while i + 1 < active_edges.len() {
                    let x0 = fixed_floor(active_edges[i].x);
                    let x1 = fixed_floor(active_edges[i + 1].x);
                    let start = x0.max(path_clip_xmin).min(path_clip_xmax);
                    let end = x1.max(path_clip_xmin).min(path_clip_xmax);
                    if end > start {
                        for x_sub in start..end {
                            let px = x_sub / SUPERSAMPLE_SCALE;
                            let py = y_sub / SUPERSAMPLE_SCALE;
                            let ix = (px - clip.x()) as usize;
                            let iy = (py - clip.y()) as usize;
                            coverage[iy * (clip_w as usize) + ix] =
                                coverage[iy * (clip_w as usize) + ix].saturating_add(1);
                        }
                    }
                    i += 2;
                }
            }
            crate::isa::FillRule::NonZero => {
                let mut winding = 0;
                let mut start_x = 0;
                for i in 0..active_edges.len() {
                    let x = fixed_floor(active_edges[i].x);
                    if winding == 0 {
                        start_x = x;
                    }
                    winding += active_edges[i].winding;
                    if winding == 0 {
                        let end_x = x;
                        let start = start_x.max(path_clip_xmin).min(path_clip_xmax);
                        let end = end_x.max(path_clip_xmin).min(path_clip_xmax);
                        if end > start {
                            for x_sub in start..end {
                                let px = x_sub / SUPERSAMPLE_SCALE;
                                let py = y_sub / SUPERSAMPLE_SCALE;
                                let ix = (px - clip.x()) as usize;
                                let iy = (py - clip.y()) as usize;
                                coverage[iy * (clip_w as usize) + ix] =
                                    coverage[iy * (clip_w as usize) + ix].saturating_add(1);
                            }
                        }
                    }
                }
            }
        }

        for e in &mut active_edges {
            e.x += e.dx_dy;
        }
    }

    let p_start = (path_clip_ymin / SUPERSAMPLE_SCALE).max(clip.y());
    let p_end = ((path_clip_ymax + SUPERSAMPLE_SCALE - 1) / SUPERSAMPLE_SCALE)
        .min(clip.y() + clip.height());
    let px_start = (path_clip_xmin / SUPERSAMPLE_SCALE).max(clip.x());
    let px_end =
        ((path_clip_xmax + SUPERSAMPLE_SCALE - 1) / SUPERSAMPLE_SCALE).min(clip.x() + clip.width());

    for py in p_start..p_end {
        let iy = (py - clip.y()) as usize;
        for px in px_start..px_end {
            let ix = (px - clip.x()) as usize;
            let cov = coverage[iy * (clip_w as usize) + ix];
            if cov == 0 {
                continue;
            }
            let alpha = ((sa as u16 * cov as u16) / SUPERSAMPLE_SAMPLES as u16) as u8;
            blend_pixel(surface, px, py, sr, sg, sb, alpha);
        }
    }
}

pub fn stroke_path(
    surface: &mut Surface,
    path: &crate::isa::Path2D,
    transform: &Transform2D,
    color: u32,
    width: f32,
    _cap: crate::isa::LineCap,
    _join: crate::isa::LineJoin,
    _miter: f32,
    aa: EdgeAA,
    clip: &Rect,
) {
    // Simple implementation: convert segments to quads and fill.
    // For now, implementing "Stroke as thick lines" - very basic.
    // Better: Stroke expansion to a new Path, then fill.
    // Given memory constraints, strict stroke expansion is complex.
    // Fallback: Just draw lines using Bresenham with thickness (approx).
    // Or scanline fill of quads.
    // "Stroke as filled expanded geometry" was requested.

    // Convert lines to quads:
    // For each segment P0->P1, compute normal, offset by width/2.
    // Build a temp Path2D with quads, then fill.

    let w = width as f32 / 2.0;
    if w <= 0.0 {
        return;
    }

    let mut stroke_verbs = Vec::new();
    let mut start_p: Option<(f32, f32)> = None;
    let mut current_p: Option<(f32, f32)> = None;

    let mut add_segment = |v: &mut Vec<crate::isa::PathVerb>, p0: (f32, f32), p1: (f32, f32)| {
        let dx = p1.0 - p0.0;
        let dy = p1.1 - p0.1;
        let len = libm::sqrtf(dx * dx + dy * dy);
        if len < 0.1 {
            return;
        }
        let nx = -dy / len;
        let ny = dx / len;

        // P0 offset
        let p0_l = (p0.0 + nx * w, p0.1 + ny * w);
        let p0_r = (p0.0 - nx * w, p0.1 - ny * w);
        // P1 offset
        let p1_l = (p1.0 + nx * w, p1.1 + ny * w);
        let p1_r = (p1.0 - nx * w, p1.1 - ny * w);

        // Quad: p0_l -> p1_l -> p1_r -> p0_r
        use crate::isa::{PathVerb, PointF};
        v.push(PathVerb::MoveTo(PointF {
            x: p0_l.0,
            y: p0_l.1,
        }));
        v.push(PathVerb::LineTo(PointF {
            x: p1_l.0,
            y: p1_l.1,
        }));
        v.push(PathVerb::LineTo(PointF {
            x: p1_r.0,
            y: p1_r.1,
        }));
        v.push(PathVerb::LineTo(PointF {
            x: p0_r.0,
            y: p0_r.1,
        }));
        v.push(PathVerb::Close);
    };

    let mut start_p: Option<(f32, f32)> = None;
    let mut current_p: Option<(f32, f32)> = None;

    let scale_sq = (transform.a * transform.a + transform.b * transform.b)
        .max(transform.c * transform.c + transform.d * transform.d);
    let flatness_sq = 0.25 / scale_sq.max(0.01);

    for verb in &path.verbs {
        match verb {
            crate::isa::PathVerb::MoveTo(p) => {
                start_p = Some((p.x, p.y));
                current_p = Some((p.x, p.y));
            }
            crate::isa::PathVerb::LineTo(p) => {
                if let Some(c) = current_p {
                    add_segment(&mut stroke_verbs, c, (p.x, p.y));
                    current_p = Some((p.x, p.y));
                }
            }
            crate::isa::PathVerb::QuadTo(p1, p) => {
                if let Some(c) = current_p {
                    flatten_quad(c, (p1.x, p1.y), (p.x, p.y), flatness_sq, &mut |p0, p1| {
                        add_segment(&mut stroke_verbs, p0, p1);
                    });
                    current_p = Some((p.x, p.y));
                }
            }
            crate::isa::PathVerb::CubicTo(p1, p2, p) => {
                if let Some(c) = current_p {
                    flatten_cubic(
                        c,
                        (p1.x, p1.y),
                        (p2.x, p2.y),
                        (p.x, p.y),
                        flatness_sq,
                        &mut |p0, p1| {
                            add_segment(&mut stroke_verbs, p0, p1);
                        },
                    );
                    current_p = Some((p.x, p.y));
                }
            }
            crate::isa::PathVerb::Close => {
                if let (Some(c), Some(s)) = (current_p, start_p) {
                    if c != s {
                        add_segment(&mut stroke_verbs, c, s);
                        current_p = Some(s);
                    }
                }
            }
        }
    }

    let stroke_path = crate::isa::Path2D {
        verbs: stroke_verbs,
    };
    fill_path(
        surface,
        &stroke_path,
        transform,
        color,
        crate::isa::FillRule::NonZero,
        aa,
        clip,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::{FillRule, Path2D, PathVerb, PointF};

    fn make_surface(width: u32, height: u32) -> (Surface, Vec<u8>) {
        let mut buffer = vec![0u8; (width * height * 4) as usize];
        let surface =
            unsafe { Surface::zeroed(buffer.as_mut_ptr(), buffer.len(), width, height, width * 4) };
        (surface, buffer)
    }

    #[test]
    fn fill_path_aa_produces_partial_coverage() {
        let (mut surface, _buffer) = make_surface(4, 4);
        let path = Path2D {
            verbs: vec![
                PathVerb::MoveTo(PointF { x: 0.0, y: 0.0 }),
                PathVerb::LineTo(PointF { x: 3.0, y: 0.0 }),
                PathVerb::LineTo(PointF { x: 0.0, y: 3.0 }),
                PathVerb::Close,
            ],
        };

        fill_path(
            &mut surface,
            &path,
            &Transform2D::identity(),
            0xFFFF_FFFF,
            FillRule::NonZero,
            EdgeAA::Coverage8,
            &Rect::new(0, 0, 4, 4),
        );

        let mut saw_partial = false;
        let mut saw_full = false;
        for y in 0..4 {
            for x in 0..4 {
                let alpha = ((surface.get_px(x, y) >> 24) & 0xFF) as u8;
                if alpha == 255 {
                    saw_full = true;
                } else if alpha > 0 {
                    saw_partial = true;
                }
            }
        }

        assert!(saw_full, "expected fully covered pixels after AA fill");
        assert!(
            saw_partial,
            "expected partially covered pixels after AA fill"
        );
    }
}
