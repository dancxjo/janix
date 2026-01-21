use crate::asset::Image;
use crate::damage::{Damage, Rect as DamageRect};
use crate::drawlist::DrawList;
use crate::font_graph::{self, FontStyle};
use crate::isa::{BlendMode, EdgeAA, FilterMode, Rect, Transform2D};
use crate::lowered::{lower, LowLevelOp, LoweredDraw};
use crate::surface::Surface;
use alloc::vec::Vec;
use fontdue::layout::GlyphRasterConfig;

struct RasterContext<'a> {
    surface: &'a mut Surface,
    clip_stack: Vec<Rect>,
    transform_stack: Vec<Transform2D>,
    current_clip: Rect,
    current_transform: Transform2D,
    solid_text: bool,
}

impl<'a> RasterContext<'a> {
    fn new(surface: &'a mut Surface, solid_text: bool) -> Self {
        let fr = Rect::new(0, 0, surface.width(), surface.height());
        Self {
            surface,
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
    let mut ctx = RasterContext::new(surface, solid_text);
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
    for i in 0..count {
        let d = dr[i];
        crate::trace_span!("raster.rect.total");
        let mut ctx = RasterContext::new(surface, solid_text);
        ctx.current_clip = Rect::new(d.x, d.y, d.w, d.h);
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
                width: _,
            } => {
                crate::trace_counter!("raster.ops.stroke", 1);
                let p0 = ctx.current_transform.transform_point(*from);
                let p1 = ctx.current_transform.transform_point(*to);
                line(
                    ctx.surface,
                    p0.x,
                    p0.y,
                    p1.x,
                    p1.y,
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
                aa: _,
            } => {
                crate::trace_counter!("raster.ops.fill", 1);
                // TODO: AA support
                fill_path(
                    ctx.surface,
                    path,
                    &ctx.current_transform,
                    color.to_u32(),
                    *fill_rule,
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
                aa: _,
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
            *dp = ((sr as u32) << 16) | ((sg as u32) << 8) | sb as u32;
            return;
        }
        let (dr, dg, db) = (
            ((dv >> 16) & 0xFF) as u8,
            ((dv >> 8) & 0xFF) as u8,
            (dv & 0xFF) as u8,
        );
        *dp = ((blend_ch(sr, dr, sa) as u32) << 16)
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
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    xrgb: u32,
    clip: &Rect,
) {
    let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
    let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
    let mut err = dx + dy;
    loop {
        if x0 >= clip.x()
            && x0 < clip.x() + clip.width()
            && y0 >= clip.y()
            && y0 < clip.y() + clip.height()
        {
            if x0 >= 0 && x0 < surface.width() && y0 >= 0 && y0 < surface.height() {
                surface.put_px(x0, y0, xrgb);
            }
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
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

fn rasterize_text_locally(
    surface: &mut Surface,
    text: &str,
    x: i32,
    y: i32,
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
        let (mut pen_x, mut pen_y) = (x as f32, y as f32 + met.ascent);
        for ch in text.chars() {
            if ch == '\n' {
                pen_x = x as f32;
                pen_y += libm::fmaxf(met.new_line_size, size * 1.1f32);
                continue;
            }
            if ch == '\r' {
                continue;
            }
            let res = graph
                .resolve_face_for_glyph(&stack, FontStyle::default(), ch as u32)
                .or_else(|| primary_face_id.and_then(|id| graph.resolved_face_by_id(id)));
            let f = match res.and_then(|r| graph.font_for_face(r.face_id)) {
                Some(f) => f,
                None => {
                    pen_x += size * 0.4f32;
                    continue;
                }
            };
            let gi = f.font.lookup_glyph_index(ch);
            if gi == 0 {
                pen_x += size * 0.4f32;
                continue;
            }
            let r_start = stem::monotonic_ns();
            let (m, b) = f.get_glyph(GlyphRasterConfig {
                glyph_index: gi,
                px: size,
                font_hash: f.font.file_hash(),
            });
            r_ns_t += stem::monotonic_ns().saturating_sub(r_start);
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
            let b_start = stem::monotonic_ns();
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
            b_ns_t += stem::monotonic_ns().saturating_sub(b_start);
            pen_x += m.advance_width;
        }
    });
    if !handled {
        rasterize_text_fallback(surface, text, x, y, size, color, clip, rf, fd);
    }
    crate::trace_counter!("text.glyphs", stats.0);
    crate::trace_counter!("text.pixels", stats.1);
    crate::trace_counter!("text.raster_ns", r_ns_t);
    crate::trace_counter!("text.blit_ns", b_ns_t);
    crate::trace_counter!("text.ns", stem::monotonic_ns().saturating_sub(t_start));
}

fn rasterize_text_fallback(
    surface: &mut Surface,
    text: &str,
    x: i32,
    y: i32,
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
        fonts
            .iter()
            .find(|f| f.name.contains(r))
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
    let (mut px, mut py) = (x as f32, y as f32 + met.ascent);
    for ch in text.chars() {
        if ch == '\n' {
            px = x as f32;
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

// Fixed point 16.16
type Fixed = i32;
const FIXED_SHIFT: i32 = 16;
const FIXED_ONE: i32 = 1 << FIXED_SHIFT;
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

fn flatten_quad<F>(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), transform: &Transform2D, add_edge_fn: &mut F) 
where F: FnMut((f32, f32), (f32, f32)) {
    // Simple flatness check: distance from p1 to (p0+p2)/2
    let mid_x = (p0.0 + p2.0) * 0.5;
    let mid_y = (p0.1 + p2.1) * 0.5;
    let dx = p1.0 - mid_x;
    let dy = p1.1 - mid_y;
    if dx*dx + dy*dy < 0.25 {
        add_edge_fn(p0, p2);
    } else {
        let p01 = ((p0.0 + p1.0) * 0.5, (p0.1 + p1.1) * 0.5);
        let p12 = ((p1.0 + p2.0) * 0.5, (p1.1 + p2.1) * 0.5);
        let p012 = ((p01.0 + p12.0) * 0.5, (p01.1 + p12.1) * 0.5);
        flatten_quad(p0, p01, p012, transform, add_edge_fn);
        flatten_quad(p012, p12, p2, transform, add_edge_fn);
    }
}

fn flatten_cubic<F>(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), transform: &Transform2D, add_edge_fn: &mut F) 
where F: FnMut((f32, f32), (f32, f32)) {
    let mid_x = (p0.0 + p3.0) * 0.5;
    let mid_y = (p0.1 + p3.1) * 0.5;
    let dx1 = p1.0 - mid_x; let dy1 = p1.1 - mid_y;
    let dx2 = p2.0 - mid_x; let dy2 = p2.1 - mid_y;
    if dx1*dx1 + dy1*dy1 + dx2*dx2 + dy2*dy2 < 0.5 {
        add_edge_fn(p0, p3);
    } else {
        let p01 = ((p0.0 + p1.0) * 0.5, (p0.1 + p1.1) * 0.5);
        let p12 = ((p1.0 + p2.0) * 0.5, (p1.1 + p2.1) * 0.5);
        let p23 = ((p2.0 + p3.0) * 0.5, (p2.1 + p3.1) * 0.5);
        let p012 = ((p01.0 + p12.0) * 0.5, (p01.1 + p12.1) * 0.5);
        let p123 = ((p12.0 + p23.0) * 0.5, (p12.1 + p23.1) * 0.5);
        let p0123 = ((p012.0 + p123.0) * 0.5, (p012.1 + p123.1) * 0.5);
        flatten_cubic(p0, p01, p012, p0123, transform, add_edge_fn);
        flatten_cubic(p0123, p123, p23, p3, transform, add_edge_fn);
    }
}

pub fn fill_path(
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

    let mut edges: Vec<Edge> = Vec::with_capacity(path.verbs.len());

    let add_edge = |e: &mut Vec<Edge>, p0: (f32, f32), p1: (f32, f32)| {
        let (x0, y0) = transform.transform_point_f(p0.0, p0.1);
        let (x1, y1) = transform.transform_point_f(p1.0, p1.1);

        let y0_i = libm::floorf(y0) as i32;
        let y1_i = libm::floorf(y1) as i32;

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
            float_to_fixed(dx / dy)
        } else {
            0
        };

        let y_start_int = y0_i.min(y1_i);
        let y_end_int = y0_i.max(y1_i);
        let y_isect = (y_start_int as f32) + 0.5;
        let x_current = float_to_fixed(p_start.0 + (y_isect - p_start.1) * (dx / dy));

        e.push(Edge {
            y_min: y_start_int,
            y_max: y_end_int,
            x: x_current,
            dx_dy: slope,
            winding: dir,
        });
    };

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
                    flatten_quad(c, (p1.x, p1.y), (p.x, p.y), transform, &mut |p0, p1| {
                        add_edge(&mut edges, p0, p1);
                    });
                    current_p = Some((p.x, p.y));
                }
            }
            crate::isa::PathVerb::CubicTo(p1, p2, p) => {
                if let Some(c) = current_p {
                    flatten_cubic(c, (p1.x, p1.y), (p2.x, p2.y), (p.x, p.y), transform, &mut |p0, p1| {
                        add_edge(&mut edges, p0, p1);
                    });
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

pub fn stroke_path(
    surface: &mut Surface,
    path: &crate::isa::Path2D,
    transform: &Transform2D,
    color: u32,
    width: i32,
    _cap: crate::isa::LineCap,
    _join: crate::isa::LineJoin,
    _miter: f32,
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
                    flatten_quad(c, (p1.x, p1.y), (p.x, p.y), transform, &mut |p0, p1| {
                         add_segment(&mut stroke_verbs, p0, p1);
                    });
                    current_p = Some((p.x, p.y));
                }
            }
            crate::isa::PathVerb::CubicTo(p1, p2, p) => {
                if let Some(c) = current_p {
                    flatten_cubic(c, (p1.x, p1.y), (p2.x, p2.y), (p.x, p.y), transform, &mut |p0, p1| {
                         add_segment(&mut stroke_verbs, p0, p1);
                    });
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
        clip,
    );
}
