use alloc::vec::Vec;
use crate::drawlist::{DrawList, DrawCmd, Insets};
use crate::asset::Image;
use crate::isa::{BlendMode, FilterMode, Transform2D, Color, Rect, Point}; // Use ISA types

// Low Level Operations - Portable Render ISA
// This is the strict contract that the presenter must execute.
#[derive(Clone, Debug)]
pub enum LowLevelOp {
    // Control & State
    Clear { color: Color },
    PushClip { rect: Rect },
    PopClip,
    PushTransform { t: Transform2D },
    PopTransform,
    
    // Geometry primitives
    FillRect { rect: Rect, color: Color },
    StrokeRect { rect: Rect, color: Color, width: i32 },
    Line { from: Point, to: Point, color: Color, width: i32 },
    FillCircle { center: Point, radius: i32, color: Color },
    
    // Image Operations
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
    
    // Text
    TextSpan { text: alloc::sync::Arc<str>, pos: Point, size: f32, color: Color },
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
                // Note: v0 ISA only supports translation
                // For now, assume translation-only usage or warn
                let isa_t = Transform2D { tx: t.dx, ty: t.dy };
                out.ops.push(LowLevelOp::PushTransform { t: isa_t });
            },
            DrawCmd::PopTransform => out.ops.push(LowLevelOp::PopTransform),
            
            // Primitives
            DrawCmd::Clear { color } => out.ops.push(LowLevelOp::Clear { color: *color }),
            DrawCmd::FillRect { rect, color } => out.ops.push(LowLevelOp::FillRect { rect: *rect, color: *color }),
            DrawCmd::StrokeRect { rect, color, width } => out.ops.push(LowLevelOp::StrokeRect { rect: *rect, color: *color, width: *width }),
            DrawCmd::FillCircle { center, radius, color } => out.ops.push(LowLevelOp::FillCircle { center: *center, radius: *radius, color: *color }),
            DrawCmd::Line { from, to, color, width } => out.ops.push(LowLevelOp::Line { from: *from, to: *to, color: *color, width: *width }),

            // Images
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
                // Shadow
                let shadow_x = position.x - frame.hotspot_x as i32 + 2; 
                let shadow_y = position.y - frame.hotspot_y as i32 + 2;
                let w = frame.image.width as i32;
                let h = frame.image.height as i32;
                let src = Rect::new(0, 0, w, h);
                let dst_shadow = Rect::new(shadow_x, shadow_y, w, h);
                
                out.ops.push(LowLevelOp::BlitAlpha {
                    image: frame.image.clone(),
                    src,
                    dst: dst_shadow,
                    filter: FilterMode::Nearest,
                    blend: BlendMode::SrcOver,
                    const_alpha: Some(77), // ~30% opacity shadow (heuristic)
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
            
            // Text
            DrawCmd::DrawText { text, position, size, color } => {
                out.ops.push(LowLevelOp::TextSpan {
                    text: text.clone(),
                    pos: *position,
                    size: *size,
                    color: *color,
                });
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawlist::DrawList;
    use crate::asset::{Image, CursorFrame};
    use crate::frame::AssetGeneration;
    use crate::geometry::{Point, Rect, Color};
    use alloc::vec;
    use alloc::sync::Arc;

    // Helper to create a dummy image
    fn make_dummy_image(w: u32, h: u32) -> Image {
        Image {
            width: w,
            height: h,
            pixels: Arc::from(vec![0u32; (w * h) as usize].into_boxed_slice()),
            gen: AssetGeneration(0),
        }
    }

    #[test]
    fn test_lower_simple_rect() {
        let mut list = DrawList::new();
        list.rect(10, 10, 50, 50, Color::new(255, 0, 0, 255));
        
        let lowered = lower(&list);
        
        assert_eq!(lowered.ops.len(), 1);
        if let LowLevelOp::FillRect { rect, color } = &lowered.ops[0] {
            assert_eq!(rect.x(), 10);
            assert_eq!(rect.width(), 50);
            assert_eq!(*color, Color::new(255, 0, 0, 255));
        } else {
            panic!("Expected FillRect");
        }
    }

    #[test]
    fn test_lower_cursor_generates_shadow_and_main() {
        let mut list = DrawList::new();
        let frame = CursorFrame {
            image: make_dummy_image(16, 16),
            hotspot_x: 0,
            hotspot_y: 0,
            delay_ms: 0,
        };
        list.cursor(&frame, 100, 100);

        let lowered = lower(&list);
        
        // Should have 2 ops: Shadow + Main
        assert_eq!(lowered.ops.len(), 2);
        
        // Verify Shadow (first op)
        if let LowLevelOp::BlitAlpha { dst, const_alpha, .. } = &lowered.ops[0] {
            assert!(const_alpha.is_some()); // Shadow has alpha mod
            assert_eq!(dst.x(), 102); // 100 + 2 offset
        } else {
            panic!("Expected Shadow BlitAlpha first");
        }

        // Verify Main (second op)
        if let LowLevelOp::BlitAlpha { dst, const_alpha, .. } = &lowered.ops[1] {
            assert!(const_alpha.is_none()); // Main cursor no extra alpha mod
            assert_eq!(dst.x(), 100);
        } else {
            panic!("Expected Main BlitAlpha second");
        }
    }

    #[test]
    fn test_lower_nine_slice_decomposition() {
        let mut list = DrawList::new();
        let image = make_dummy_image(30, 30); // 30x30 image
        let insets = Insets::new(10, 10, 10, 10);
        let dst = Rect::new(0, 0, 100, 100);
        
        list.nine_slice(&image, dst, insets);
        
        let lowered = lower(&list);
        
        // Should generate 9 ops if all parts have size
        assert_eq!(lowered.ops.len(), 9);
        
        // Verify center op (index 4) maps to center of dest
        // 0 1 2
        // 3 4 5
        // 6 7 8
        if let LowLevelOp::BlitAlpha { dst, src, .. } = &lowered.ops[4] {
            assert_eq!(src.width(), 10);
            assert_eq!(dst.width(), 80); // 100 - 10 - 10
        } else {
             panic!("Expected Blit for center patch");
        }
    }
}
