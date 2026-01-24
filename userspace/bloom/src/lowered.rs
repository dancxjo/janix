use alloc::vec::Vec;
use alloc::string::String;
use crate::drawlist::{DrawList, DrawCmd, Insets};
use crate::asset::Image;
use crate::isa::{BlendMode, FilterMode, Transform2D, Color, Rect, Point, EdgeAA};

// Low Level Operations - Portable Render ISA
// This is the strict contract that the presenter must execute.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum LowLevelOp {
    // Control & State
    Clear { color: Color },
    PushClip { rect: Rect },
    PopClip,
    PushTransform { t: Transform2D },
    PopTransform,
    
    // Geometry primitives
    FillRect { rect: Rect, color: Color, aa: EdgeAA },
    StrokeRect { rect: Rect, color: Color, width: i32 },
    #[allow(dead_code)]
    Line { from: Point, to: Point, color: Color, width: i32 },
    FillCircle { center: Point, radius: i32, color: Color },
    FillArc { center: Point, radius: i32, start_angle: f32, end_angle: f32, color: Color, aa: EdgeAA },
    
    // Image Operations
    BlitSnapshot {
        bs_id: u64,
        width: u32,
        height: u32,
        stride: u32,
        src: Rect,
        dst: Rect,
    },

    /// Blit opaque image with explicit scaling. 
    /// If src.size != dst.size, must scale according to filter.
    BlitOpaque { 
        image: Image, 
        src: Rect, 
        dst: Rect, 
        filter: FilterMode,
    },
    
    /// Blit with alpha blending and optional constant alpha modulation.
    /// src over blending is mandatory.
    /// const_alpha optionally modulates the source alpha (and color).
    BlitAlpha { 
        image: Image, 
        src: Rect, 
        dst: Rect, 
        filter: FilterMode,
        blend: BlendMode, 
        const_alpha: Option<u8>,
    },

    // Modern Text & Vector
    TextSpan {
        text: String,
        pos: Point,
        size: f32,
        color: Color,
        font_name: Option<String>,
        font_debug: bool,
    },
    FillPath {
        path: alloc::sync::Arc<crate::isa::Path2D>,
        color: Color,
        fill_rule: crate::isa::FillRule,
        aa: EdgeAA,
    },
    StrokePath {
        path: alloc::sync::Arc<crate::isa::Path2D>,
        color: Color,
        width: f32,
        cap: crate::isa::LineCap,
        join: crate::isa::LineJoin,
        miter_limit: f32,
        aa: EdgeAA,
    },
}

pub struct LoweredDraw {
    pub ops: Vec<LowLevelOp>,
}

impl LoweredDraw {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }
}

/// Transform high-level DrawList into strict Render ISA ops
/// This function is responsible for:
/// 1. Decomposing high-level concepts (NineSlice, Cursor) into ISA primitives.
/// 2. Managing state stacks *if* DrawCmds imply them (not yet, but good practice).
/// 3. Asserting variants for rasterizer safety.
pub fn lower(list: &DrawList) -> LoweredDraw {
    let mut out = LoweredDraw::new();
    
    for cmd in list.iter() {
        match cmd {
            // Frame control (pass-through for now, or map to ISA state)
            DrawCmd::BeginFrame { .. } => {},
            DrawCmd::EndFrame => {},
            DrawCmd::PushClip { rect } => out.ops.push(LowLevelOp::PushClip { rect: *rect }),
            DrawCmd::PopClip => out.ops.push(LowLevelOp::PopClip),
            DrawCmd::PushTransform { transform: t } => {
                // Map DrawCmd geometry::Transform to ISA Transform2D
                // geometry: m11, m12, m21, m22, dx, dy
                // isa: a, b, c, d, tx, ty (a=m11, c=m12, b=m21, d=m22)
                let isa_t = Transform2D { 
                    a: t.m11, b: t.m21, 
                    c: t.m12, d: t.m22, 
                    tx: t.dx, ty: t.dy 
                };
                out.ops.push(LowLevelOp::PushTransform { t: isa_t });
            },
            DrawCmd::PopTransform => out.ops.push(LowLevelOp::PopTransform),
            
            // Primitives
            DrawCmd::Clear { color } => out.ops.push(LowLevelOp::Clear { color: *color }),
            DrawCmd::FillRect { rect, color, aa } => out.ops.push(LowLevelOp::FillRect { rect: *rect, color: *color, aa: *aa }),
            DrawCmd::FillRoundRect { rect, radius, color, aa } => {
                let r = *radius;
                if r <= 0 {
                    out.ops.push(LowLevelOp::FillRect { rect: *rect, color: *color, aa: *aa });
                } else {
                    // Decompose into 3 rectangles and 4 arcs to avoid overlaps
                    let x = rect.x();
                    let y = rect.y();
                    let w = rect.width();
                    let h = rect.height();
                    let r = r.min(w / 2).min(h / 2);
                    
                    // 1. Central full-height strip
                    out.ops.push(LowLevelOp::FillRect { 
                        rect: Rect::new(x + r, y, w - 2*r, h), 
                        color: *color, 
                        aa: *aa 
                    });
                    
                    // 2. Left strip
                    out.ops.push(LowLevelOp::FillRect { 
                        rect: Rect::new(x, y + r, r, h - 2*r), 
                        color: *color, 
                        aa: *aa 
                    });
                    
                    // 3. Right strip
                    out.ops.push(LowLevelOp::FillRect { 
                        rect: Rect::new(x + w - r, y + r, r, h - 2*r), 
                        color: *color, 
                        aa: *aa 
                    });
                    
                    // 4. Corners (Arcs)
                    // TL
                    out.ops.push(LowLevelOp::FillArc { 
                        center: Point::new(x + r, y + r), 
                        radius: r, 
                        start_angle: 180.0, 
                        end_angle: 270.0, 
                        color: *color,
                        aa: *aa
                    });
                    // TR
                    out.ops.push(LowLevelOp::FillArc { 
                        center: Point::new(x + w - r, y + r), 
                        radius: r, 
                        start_angle: 270.0, 
                        end_angle: 360.0, 
                        color: *color,
                        aa: *aa
                    });
                    // BL
                    out.ops.push(LowLevelOp::FillArc { 
                        center: Point::new(x + r, y + h - r), 
                        radius: r, 
                        start_angle: 90.0, 
                        end_angle: 180.0, 
                        color: *color,
                        aa: *aa
                    });
                    // BR
                    out.ops.push(LowLevelOp::FillArc { 
                        center: Point::new(x + w - r, y + h - r), 
                        radius: r, 
                        start_angle: 0.0, 
                        end_angle: 90.0, 
                        color: *color,
                        aa: *aa
                    });
                }
            },
            DrawCmd::StrokeRect { rect, color, width } => out.ops.push(LowLevelOp::StrokeRect { rect: *rect, color: *color, width: *width }),
            DrawCmd::FillCircle { center, radius, color } => out.ops.push(LowLevelOp::FillCircle { center: *center, radius: *radius, color: *color }),
            DrawCmd::FillArc { center, radius, start_angle, end_angle, color, aa } => {
                out.ops.push(LowLevelOp::FillArc { 
                    center: *center, 
                    radius: *radius, 
                    start_angle: *start_angle, 
                    end_angle: *end_angle, 
                    color: *color,
                    aa: *aa
                });
            },
            DrawCmd::Line { from, to, color, width } => out.ops.push(LowLevelOp::Line { from: *from, to: *to, color: *color, width: *width }),

            // Images
            DrawCmd::DrawSnapshot { bs_id, width, height, stride, dest } => {
                let src = Rect::new(0, 0, *width as i32, *height as i32);
                out.ops.push(LowLevelOp::BlitSnapshot {
                    bs_id: *bs_id,
                    width: *width,
                    height: *height,
                    stride: *stride,
                    src,
                    dst: *dest,
                });
            },
            DrawCmd::DrawImage { image, dest } => {
                // 1:1 blit, Opaque (unless image has alpha? DrawCmd doesn't specify Opaque vs Alpha variant strictly yet)
                // For safety, use BlitAlpha for generic images if they might have alpha, or BlitOpaque if we know.
                // Bloom assets often have alpha. Let's assume BlitAlpha for generic DrawImage for now.
                // Actually, existing impl used Blit for Opaque/Simple. Let's use BlitAlpha with SrcOver to be safe/general.
                let src = Rect::new(0, 0, image.width as i32, image.height as i32);
                
                // DrawImage in DrawCmd now takes a DEST rect (which implies scaling if different size?)
                // Or is it just position and size? Usually DrawImage(dest) implies fill dest.
                // But previous behavior was DrawImage(pos).
                // If it is dest, we use it directly.
                out.ops.push(LowLevelOp::BlitAlpha {
                    image: image.clone(),
                    src,
                    dst: *dest,
                    filter: FilterMode::Nearest,
                    blend: BlendMode::SrcOver,
                    const_alpha: None,
                });
            },
            DrawCmd::DrawImageRegion { image, src, dest } => {
                out.ops.push(LowLevelOp::BlitAlpha {
                    image: image.clone(),
                    src: *src,
                    dst: *dest,
                    filter: FilterMode::Nearest,
                    blend: BlendMode::SrcOver,
                    const_alpha: None, // No extra modulation by default
                });
            },
            
            // Complex Decompositions
            DrawCmd::DrawNineSlice { image, dest, margins } => {
                lower_nine_slice(&mut out, image, dest, margins);
            }
            DrawCmd::Cursor { frame, position } => {
                // Windows 2000 style 3-layer shadow
                let cx = position.x - frame.hotspot_x as i32;
                let cy = position.y - frame.hotspot_y as i32;
                let w = frame.image.width as i32;
                let h = frame.image.height as i32;
                let src = Rect::new(0, 0, w, h);

                // Layer 1: Tight
                out.ops.push(LowLevelOp::BlitAlpha {
                    image: frame.image.clone(),
                    src,
                    dst: Rect::new(cx + 1, cy + 1, w, h),
                    filter: FilterMode::Nearest,
                    blend: BlendMode::SrcOver,
                    const_alpha: Some(48), // ~19%
                });

                // Layer 2: Medium
                out.ops.push(LowLevelOp::BlitAlpha {
                    image: frame.image.clone(),
                    src,
                    dst: Rect::new(cx + 2, cy + 2, w, h),
                    filter: FilterMode::Nearest,
                    blend: BlendMode::SrcOver,
                    const_alpha: Some(48), // ~19%
                });

                // Layer 3: Fuzzy falloff
                out.ops.push(LowLevelOp::BlitAlpha {
                    image: frame.image.clone(),
                    src,
                    dst: Rect::new(cx + 3, cy + 3, w, h),
                    filter: FilterMode::Nearest,
                    blend: BlendMode::SrcOver,
                    const_alpha: Some(24), // ~9%
                });

                // Main cursor
                let cursor_x = position.x - frame.hotspot_x as i32;
                let cursor_y = position.y - frame.hotspot_y as i32;
                let dst_main = Rect::new(cursor_x, cursor_y, w, h);
                
                out.ops.push(LowLevelOp::BlitAlpha {
                    image: frame.image.clone(),
                    src,
                    dst: dst_main,
                    filter: FilterMode::Nearest,
                    blend: BlendMode::SrcOver,
                    const_alpha: None,
                });
            }
            
            DrawCmd::Text { text, font, rect, size, color, font_debug } => {
                out.ops.push(LowLevelOp::TextSpan {
                    text: text.clone(),
                    pos: Point::new(rect.x(), rect.y()),
                    size: *size,
                    color: *color,
                    font_name: font.clone(),
                    font_debug: *font_debug,
                });
            }

            DrawCmd::Path { path, color, fill_rule, stroke } => {
                if let Some(s) = stroke {
                    out.ops.push(LowLevelOp::StrokePath {
                        path: path.clone(),
                        color: *color,
                        width: s.width,
                        cap: s.cap,
                        join: s.join,
                        miter_limit: s.miter_limit,
                        aa: EdgeAA::Coverage8,
                    });
                } else {
                    out.ops.push(LowLevelOp::FillPath {
                        path: path.clone(),
                        color: *color,
                        fill_rule: *fill_rule,
                        aa: EdgeAA::Coverage8,
                    });
                }
            }
            
            // Ignored/Unimplemented for v0
            _ => { /* Warn or ignore */ }
        }
    }
    out
}

fn lower_nine_slice(out: &mut LoweredDraw, image: &Image, dst: &Rect, insets: &Insets) {
    let iw = image.width as i32;
    let ih = image.height as i32;
    
    // Calculate source coords
    let sx0 = 0;
    let sx1 = insets.left;
    let sx2 = iw - insets.right;
    let sx3 = iw;
    
    let sy0 = 0;
    let sy1 = insets.top;
    let sy2 = ih - insets.bottom;
    let sy3 = ih;

    // Calculate dest coords
    let dx0 = dst.x();
    let dx1 = dst.x() + insets.left;
    let dx2 = dst.x() + dst.width() - insets.right;
    let dx3 = dst.x() + dst.width();
    
    let dy0 = dst.y();
    let dy1 = dst.y() + insets.top;
    let dy2 = dst.y() + dst.height() - insets.bottom;
    let dy3 = dst.y() + dst.height();

    // Source coordinates arrays
    let src_xs = [sx0, sx1, sx2, sx3];
    let src_ys = [sy0, sy1, sy2, sy3];
    
    // Dest coordinates arrays
    let dst_xs = [dx0, dx1, dx2, dx3];
    let dst_ys = [dy0, dy1, dy2, dy3];

    for r in 0..3 {
        for c in 0..3 {
            let sw = src_xs[c+1] - src_xs[c];
            let sh = src_ys[r+1] - src_ys[r];
            let dw = dst_xs[c+1] - dst_xs[c];
            let dh = dst_ys[r+1] - dst_ys[r];

            // Only draw if the slice has area
            if sw > 0 && sh > 0 && dw > 0 && dh > 0 {
                let src = Rect::new(src_xs[c], src_ys[r], sw, sh);
                let dst = Rect::new(dst_xs[c], dst_ys[r], dw, dh);
                
                // Use Opaque if we are sure, or Alpha if the UI texture has transparency.
                // Assuming UI assets might be transparent corners, stick to BlitAlpha for correctness mostly.
                // If performance issues arise, we can check image properties or DrawCmd hints.
                // For typical UI panels, corners often have rounded transparent pixels.
                out.ops.push(LowLevelOp::BlitAlpha {
                    image: image.clone(),
                    src,
                    dst,
                    filter: FilterMode::Nearest, // Pixel art UI usually prefers Nearest
                    blend: BlendMode::SrcOver,
                    const_alpha: None,
                });
            }
        }
    }
}
