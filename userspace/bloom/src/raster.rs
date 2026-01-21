use alloc::vec::Vec;
use crate::damage::{Damage, Rect as DamageRect};
use crate::drawlist::DrawList;
use crate::lowered::{lower, LowLevelOp, LoweredDraw};
use crate::surface::Surface;
use crate::asset::Image;
use crate::isa::{BlendMode, FilterMode, Transform2D, Rect, EdgeAA};
use crate::font_graph::{self, FontStyle};
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
        Self { surface, clip_stack: Vec::with_capacity(4), transform_stack: Vec::with_capacity(4), current_clip: fr, current_transform: Transform2D::identity(), solid_text }
    }
    fn push_clip(&mut self, rect: Rect) {
        self.clip_stack.push(self.current_clip);
        let tr = self.current_transform.transform_rect(rect);
        if let Some(i) = self.current_clip.intersection(&tr) { self.current_clip = i; }
        else { self.current_clip = Rect::new(0,0,0,0); }
    }
    fn pop_clip(&mut self) { if let Some(p) = self.clip_stack.pop() { self.current_clip = p; } }
    fn push_transform(&mut self, t: Transform2D) { self.transform_stack.push(self.current_transform); self.current_transform = self.current_transform.combine(&t); }
    fn pop_transform(&mut self) { if let Some(p) = self.transform_stack.pop() { self.current_transform = p; } }
}

pub fn execute(surface: &mut Surface, list: &DrawList, solid_text: bool) {
    let lowered = lower(list);
    let mut ctx = RasterContext::new(surface, solid_text);
    execute_lowered_on_context(&mut ctx, &lowered);
}

pub fn execute_with_damage(surface: &mut Surface, list: &DrawList, damage: &Damage, solid_text: bool) {
    if damage.is_full { execute(surface, list, solid_text); return; }
    let lowered = lower(list);
    execute_lowered_with_damage(surface, &lowered, damage, solid_text);
}

pub fn execute_lowered_with_damage(surface: &mut Surface, lowered: &LoweredDraw, damage: &Damage, solid_text: bool) {
    let mut dr = [DamageRect::default(); 8]; let mut count = 0;
    for r in damage.iter() { if count < 8 { dr[count] = r; count += 1; } }
    let start = stem::monotonic_ns();
    for i in 0..count {
        let d = dr[i];
        crate::trace_span!("raster.rect.total");
        let mut ctx = RasterContext::new(surface, solid_text);
        ctx.current_clip = Rect::new(d.x, d.y, d.w, d.h);
        execute_lowered_on_context(&mut ctx, lowered);
    }
    crate::trace_counter!("raster.execute_ns", stem::monotonic_ns().saturating_sub(start));
}

fn execute_lowered_on_context(ctx: &mut RasterContext, lowered: &LoweredDraw) {
    for op in lowered.ops.iter() {
        match op {
            LowLevelOp::Clear { color } => fill_rect_copy(ctx.surface, ctx.current_clip.x(), ctx.current_clip.y(), ctx.current_clip.width(), ctx.current_clip.height(), color.to_u32()),
            LowLevelOp::PushClip { rect } => ctx.push_clip(*rect),
            LowLevelOp::PopClip => ctx.pop_clip(),
            LowLevelOp::PushTransform { t } => ctx.push_transform(*t),
            LowLevelOp::PopTransform => ctx.pop_transform(),
            LowLevelOp::FillRect { rect, color, aa: _ } => {
                let tr = ctx.current_transform.transform_rect(*rect);
                if let Some(cl) = ctx.current_clip.intersection(&tr) {
                    let c = color.to_u32();
                    if (c >> 24) == 255 { fill_rect_copy(ctx.surface, cl.x(), cl.y(), cl.width(), cl.height(), c); }
                    else { fill_rect_blend(ctx.surface, cl.x(), cl.y(), cl.width(), cl.height(), c); }
                }
            },
            LowLevelOp::BlitOpaque { image, src, dst, filter } => {
                let td = ctx.current_transform.transform_rect(*dst);
                if let Some(cd) = ctx.current_clip.intersection(&td) { blit_opaque(ctx.surface, image, src, &td, &cd, *filter); }
            },
            LowLevelOp::BlitAlpha { image, src, dst, filter, blend, const_alpha } => {
                let td = ctx.current_transform.transform_rect(*dst);
                if let Some(cd) = ctx.current_clip.intersection(&td) { blit_alpha(ctx.surface, image, src, &td, &cd, *filter, *blend, *const_alpha); }
            },
            LowLevelOp::TextSpan { text, pos, size, color, font_name, font_debug } => {
                let p = ctx.current_transform.transform_point(*pos);
                rasterize_text_locally(ctx.surface, text, p.x, p.y, *size, color.to_u32(), &ctx.current_clip, font_name.as_deref(), *font_debug);
            },
            LowLevelOp::StrokeRect { rect, color, width } => {
                let tr = ctx.current_transform.transform_rect(*rect);
                stroke_rect_clipped_blend(ctx.surface, &tr, *width, color.to_u32(), &ctx.current_clip);
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
        }
    }
}

#[inline(always)] fn scale_ch(c: u8, a: u8) -> u32 { let t = c as u32 * a as u32; (t + (t >> 8) + 1) >> 8 }
#[inline(always)] fn blend_ch(s: u8, d: u8, sa: u8) -> u8 { if sa == 255 { return s; } if sa == 0 { return d; } (scale_ch(s, sa) + scale_ch(d, 255-sa)) as u8 }

fn blend_pixel(surface: &mut Surface, x: i32, y: i32, sr: u8, sg: u8, sb: u8, sa: u8) {
    if sa == 0 { return; }
    let offset = (surface.stride_bytes >> 2) * (y as usize) + (x as usize);
    let ptr = surface.ptr as *mut u32;
    unsafe {
        let dp = ptr.add(offset); let dv = *dp;
        if sa == 255 { *dp = ((sr as u32) << 16) | ((sg as u32) << 8) | sb as u32; return; }
        let (dr, dg, db) = (((dv >> 16) & 0xFF) as u8, ((dv >> 8) & 0xFF) as u8, (dv & 0xFF) as u8);
        *dp = ((blend_ch(sr, dr, sa) as u32) << 16) | ((blend_ch(sg, dg, sa) as u32) << 8) | (blend_ch(sb, db, sa) as u32);
    }
}

pub fn fill_rect_copy(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, color: u32) {
    let x0 = x.max(0); let y0 = y.max(0); let x1 = (x + w).min(surface.width()); let y1 = (y + h).min(surface.height());
    for yy in y0..y1 { for xx in x0..x1 { surface.put_px(xx, yy, color); } }
}
pub fn fill_rect_blend(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, color: u32) {
    let a = ((color >> 24) & 0xFF) as u8; if a == 255 { fill_rect_copy(surface, x, y, w, h, color); return; }
    if a == 0 { return; }
    let x0 = x.max(0); let y0 = y.max(0); let x1 = (x + w).min(surface.width()); let y1 = (y + h).min(surface.height());
    let (sr, sg, sb) = (((color >> 16) & 0xFF) as u8, ((color >> 8) & 0xFF) as u8, (color & 0xFF) as u8);
    for yy in y0..y1 { for xx in x0..x1 { blend_pixel(surface, xx, yy, sr, sg, sb, a); } }
}
fn stroke_rect_clipped_blend(surface: &mut Surface, rect: &Rect, width: i32, color: u32, clip: &Rect) {
    let parts = [Rect::new(rect.x(), rect.y(), rect.width(), width), Rect::new(rect.x(), rect.y() + rect.height() - width, rect.width(), width), Rect::new(rect.x(), rect.y() + width, width, rect.height() - 2*width), Rect::new(rect.x() + rect.width() - width, rect.y() + width, width, rect.height() - 2*width)];
    for p in parts { if let Some(c) = p.intersection(clip) { fill_rect_blend(surface, c.x(), c.y(), c.width(), c.height(), color); } }
}
pub fn fill_circle_blend(surface: &mut Surface, cx: i32, cy: i32, r: i32, color: u32) {
    let a = ((color >> 24) & 0xFF) as u8; if a == 0 { return; }
    let (x0, y0, x1, y1) = ((cx - r).max(0), (cy - r).max(0), (cx + r).min(surface.width()), (cy + r).min(surface.height()));
    let (sr, sg, sb) = (((color >> 16) & 0xFF) as u8, ((color >> 8) & 0xFF) as u8, (color & 0xFF) as u8);
    for y in y0..y1 { for x in x0..x1 { 
        let dx = x - cx; let dy = y - cy;
        if (dx*dx) + (dy*dy) <= r*r { blend_pixel(surface, x, y, sr, sg, sb, a); } 
    } }
}
pub fn fill_arc_clipped_blend(surface: &mut Surface, cx: i32, cy: i32, r: i32, s_deg: f32, e_deg: f32, color: u32, aa: EdgeAA, clip: &Rect) {
    let sa = ((color >> 24) & 0xFF) as u8; if sa == 0 { return; }
    let m = if aa != EdgeAA::None { 1 } else { 0 };
    let (x0, y0, x1, y1) = ((cx-r-m).max(clip.x()).max(0), (cy-r-m).max(clip.y()).max(0), (cx+r+m).min(clip.x()+clip.width()).min(surface.width()), (cy+r+m).min(clip.y()+clip.height()).min(surface.height()));
    let (sr, sg, sb) = (((color >> 16) & 0xFF) as u8, ((color >> 8) & 0xFF) as u8, (color & 0xFF) as u8);
    let (mut s, mut e) = (s_deg, e_deg); while s < 0.0 { s += 360.0; } while s >= 360.0 { s -= 360.0; } while e < s { e += 360.0; }
    for y in y0..y1 { for x in x0..x1 {
        let (dx, dy) = (x as f32 + 0.5 - cx as f32, y as f32 + 0.5 - cy as f32); let d2 = dx*dx + dy*dy;
        if d2 <= (r as f32 + 1.0)*(r as f32 + 1.0) {
            let mut a = libm::atan2f(dy, dx) * 180.0 / 3.14159265; while a < s { a += 360.0; }
            if a <= e {
                let cov = if aa != EdgeAA::None { (r as f32 - libm::sqrtf(d2) + 0.5).clamp(0.0, 1.0) } else { if d2 <= (r*r) as f32 { 1.0 } else { 0.0 } };
                if cov > 0.0 { blend_pixel(surface, x, y, sr, sg, sb, (sa as f32 * cov) as u8); }
            }
        }
    } }
}
pub fn line(surface: &mut Surface, mut x0: i32, mut y0: i32, x1: i32, y1: i32, xrgb: u32) {
    let (dx, dy) = ((x1-x0).abs(), -(y1-y0).abs()); let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
    let mut err = dx + dy;
    loop {
        if x0 >= 0 && x0 < surface.width() && y0 >= 0 && y0 < surface.height() { surface.put_px(x0, y0, xrgb); }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err; if e2 >= dy { err += dy; x0 += sx; } if e2 <= dx { err += dx; y0 += sy; }
    }
}
fn blit_opaque(surface: &mut Surface, image: &Image, src: &Rect, fd: &Rect, cd: &Rect, _fm: FilterMode) {
    let (sx_f, sy_f) = (src.width() as f32 / fd.width() as f32, src.height() as f32 / fd.height() as f32);
    for dy in cd.y()..cd.y()+cd.height() { for dx in cd.x()..cd.x()+cd.width() {
        let (sx, sy) = ((src.x() as f32 + (dx-fd.x()) as f32 * sx_f) as i32, (src.y() as f32 + (dy-fd.y()) as f32 * sy_f) as i32);
        if sx >= 0 && sx < image.width as i32 && sy >= 0 && sy < image.height as i32 { 
            surface.put_px(dx, dy, image.pixels[(sy as usize)*(image.width as usize) + (sx as usize)]); 
        }
    } }
}
fn blit_alpha(surface: &mut Surface, image: &Image, src: &Rect, fd: &Rect, cd: &Rect, _fm: FilterMode, blend: BlendMode, ca: Option<u8>) {
    let cav = ca.unwrap_or(255) as u32;
    let (sx_f, sy_f) = (src.width() as f32 / fd.width() as f32, src.height() as f32 / fd.height() as f32);
    for dy in cd.y()..cd.y()+cd.height() { for dx in cd.x()..cd.x()+cd.width() {
        let (sx, sy) = ((src.x() as f32 + (dx-fd.x()) as f32 * sx_f) as i32, (src.y() as f32 + (dy-fd.y()) as f32 * sy_f) as i32);
        if sx >= 0 && sx < image.width as i32 && sy >= 0 && sy < image.height as i32 {
            let px = image.pixels[(sy as usize)*(image.width as usize) + (sx as usize)]; 
            let mut a = (px >> 24) & 0xFF; if cav != 255 { a = (a * cav) / 255; }
            if a == 0 { continue; } if blend == BlendMode::Src || a == 255 { surface.put_px(dx, dy, px); continue; }
            blend_pixel(surface, dx, dy, ((px >> 16) & 0xFF) as u8, ((px >> 8) & 0xFF) as u8, (px & 0xFF) as u8, a as u8);
        }
    } }
}

fn rasterize_text_locally(surface: &mut Surface, text: &str, x: i32, y: i32, size: f32, color: u32, clip: &Rect, rf: Option<&str>, fd: bool) {
    let t_start = stem::monotonic_ns(); let mut stats = (0u64, 0u64, 0u64, 0u64); 
    if !font_graph::has_fonts_ready() { rasterize_text_fallback(surface, text, x, y, size, color, clip, rf, fd); return; }
    let mut handled = false;
    if let Some(r) = rf { if !font_graph::try_with_graph_if_ready(|g| g.has_font(r)).unwrap_or(false) { rasterize_text_fallback(surface, text, x, y, size, color, clip, rf, fd); return; } }
    let (mut r_ns_t, mut b_ns_t) = (0u64, 0u64);
    font_graph::with_graph(|graph| {
        let stack = graph.resolve_stack(rf); if stack.is_empty() { return; }
        let primary_face_id = stack.iter().find_map(|f| graph.select_face_for_family(*f, FontStyle::default()));
        let primary_font = match primary_face_id.and_then(|id| graph.font_for_face(id)) { Some(f) => f, None => return };
        handled = true;
        let met = match primary_font.font.horizontal_line_metrics(size) { Some(m) => m, None => fontdue::LineMetrics { ascent: size*0.8f32, descent: size*0.2f32, line_gap: 0.0, new_line_size: size*1.2f32 } };
        let (mut pen_x, mut pen_y) = (x as f32, y as f32 + met.ascent);
        for ch in text.chars() {
            if ch == '\n' { pen_x = x as f32; pen_y += libm::fmaxf(met.new_line_size, size*1.1f32); continue; } if ch == '\r' { continue; }
            let res = graph.resolve_face_for_glyph(&stack, FontStyle::default(), ch as u32).or_else(|| primary_face_id.and_then(|id| graph.resolved_face_by_id(id)));
            let f = match res.and_then(|r| graph.font_for_face(r.face_id)) { Some(f) => f, None => { pen_x += size*0.4f32; continue; } };
            let gi = f.font.lookup_glyph_index(ch); if gi == 0 { pen_x += size*0.4f32; continue; }
            let r_start = stem::monotonic_ns(); let (m, b) = f.get_glyph(GlyphRasterConfig { glyph_index: gi, px: size, font_hash: f.font.file_hash() });
            r_ns_t += stem::monotonic_ns().saturating_sub(r_start); stats.0 += 1;
            let (gx, gy) = ((pen_x + m.xmin as f32) as i32, pen_y as i32 - m.height as i32 - m.ymin);
            let (sr, sg, sb, sa) = (((color >> 16) & 0xFF) as u8, ((color >> 8) & 0xFF) as u8, (color & 0xFF) as u8, ((color >> 24) & 0xFF) as u8);
            let b_start = stem::monotonic_ns();
            for r in 0..m.height { for c in 0..m.width {
                let (cx, cy) = (gx+c as i32, gy+r as i32);
                if cx >= clip.x() && cx < clip.x()+clip.width() && cy >= clip.y() && cy < clip.y()+clip.height() {
                    let a = b[r*(m.width as usize)+c]; if a > 0 { blend_pixel(surface, cx, cy, sr, sg, sb, scale_ch(a, sa) as u8); stats.1 += 1; }
                }
            } }
            b_ns_t += stem::monotonic_ns().saturating_sub(b_start); pen_x += m.advance_width;
        }
    });
    if !handled { rasterize_text_fallback(surface, text, x, y, size, color, clip, rf, fd); }
    crate::trace_counter!("text.glyphs", stats.0); crate::trace_counter!("text.pixels", stats.1);
    crate::trace_counter!("text.raster_ns", r_ns_t); crate::trace_counter!("text.blit_ns", b_ns_t);
    crate::trace_counter!("text.ns", stem::monotonic_ns().saturating_sub(t_start));
}
fn rasterize_text_fallback(surface: &mut Surface, text: &str, x: i32, y: i32, size: f32, color: u32, clip: &Rect, rf: Option<&str>, _fd: bool) {
    let fonts = crate::ASSETS.get_fonts(); if fonts.is_empty() { return; }
    let font = if let Some(r) = rf { fonts.iter().find(|f| f.name.contains(r)).or_else(|| fonts.iter().find(|f| f.name.contains("NotoSans-Regular"))).unwrap_or(&fonts[0]) }
    else { fonts.iter().find(|f| f.name.contains("NotoSans-Regular")).unwrap_or(&fonts[0]) };
    let (sa, sr, sg, sb) = (((color >> 24) & 0xFF) as u8, ((color >> 16) & 0xFF) as u8, ((color >> 8) & 0xFF) as u8, (color & 0xFF) as u8);
    let met = match font.font.horizontal_line_metrics(size) { Some(m) => m, None => fontdue::LineMetrics { ascent: size*0.8f32, descent: size*0.2f32, line_gap: 0.0, new_line_size: size*1.2f32 } };
    let (mut px, mut py) = (x as f32, y as f32 + met.ascent);
    for ch in text.chars() {
        if ch == '\n' { px = x as f32; py += libm::fmaxf(met.new_line_size, size*1.1f32); continue; } if ch == '\r' { continue; }
        let gi = font.font.lookup_glyph_index(ch); if gi == 0 { px += size*0.4f32; continue; }
        let (m, b) = font.get_glyph(GlyphRasterConfig { glyph_index: gi, px: size, font_hash: font.font.file_hash() });
        let (gx, gy) = ((px + m.xmin as f32) as i32, py as i32 - m.height as i32 - m.ymin);
        for r in 0..m.height { for c in 0..m.width {
            let (cx, cy) = (gx+c as i32, gy+r as i32);
            if cx >= clip.x() && cx < clip.x()+clip.width() && cy >= clip.y() && cy < clip.y()+clip.height() {
                let a = b[r*(m.width as usize)+c]; if a > 0 { blend_pixel(surface, cx, cy, sr, sg, sb, scale_ch(a, sa) as u8); }
            }
        } }
        px += m.advance_width;
    }
}
