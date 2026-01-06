use crate::scene::Rect;
use abi::ids::ThingId;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(Clone, Debug)]
pub enum DrawCmd {
    // === Primitives (matching ABI or basic shapes) ===
    FillRect { rect: Rect, color: u32 },
    FillRectVGrad { rect: Rect, radius: u16, top_color: u32, bottom_color: u32 },
    FillRoundedRect { rect: Rect, radius: u16, color: u32 },
    StrokeRoundedRect { rect: Rect, radius: u16, thickness: u16, color: u32 },
    Clear { color: u32 },
    
    // === Advanced / Internal Ops ===
    
    // Blit from a bytespace (e.g. Canvas, DrawList asset)
    // NOTE: src_stride in PIXELS (u32), not bytes, for simplicity with current CpuPainter
    BlitRgbaPremulBytespace { 
        bytespace: ThingId, 
        src_rect: Rect, 
        dst_x: i32, 
        dst_y: i32, 
        src_stride: u32,
        src_len: usize,
    },


    // Blit from an in-memory buffer (e.g. Wallpaper cache, DrawList temporary text)
    // We hold a reference or Arc? For v1, let's just hold a Vec or slice if lifetime allows.
    // To avoid complex lifetimes in `DrawCmd` (which needs to be stored in Vec<DrawCmd>), 
    // we can't easily hold `&[u32]`.
    // We can hold `Vec<u32>` (cloned) OR `Arc<Vec<u32>>`.
    // Or we can rely on `BlitBytespace` for heavy things and `FillRect` for simple things.
    // What about `DrawList` text? It's in the mapped buffer. We need to parse it.
    // If we parse it into `DrawCmd::TextRun`, we are fine.
    // But `DrawList` might have other inline assets? Currently only `DrawCmd` from ABI.
    // ABI `Blit` has `asset: u64` which is an ID?
    // Let's stick to `BlitRgbaPremulBytespace` for now.
    
    // Text drawing
    // We need to store the string because we are recording now and executing later.
    TextRun {
        x: i32,
        y: i32,
        text: String,
        color: u32,
        font_size: f32, // Simplified for v1
    },

    // Shadows (Internal)
    Shadow {
        x: i32, 
        y: i32, 
        width: u32,
        height: u32,
        radius: u16,
        color: u32, // encoded params
        // For standard shadow mask, we might need more data.
        // Existing `draw_shadow_mask` takes `ShadowMask` enum (SpriteAlpha or RoundedRect).
        // If RoundedRect, we can store params.
        // If SpriteAlpha (cursor), we assume cursor is drawn separate top-pass for now.
        // So this is for Window Shadows (RoundedRect).
        offset_x: i32,
        offset_y: i32,
        blur_radius: u16,
    },
    
    // Panel Gradient (Specific to Bloom Window style)
    // "paint_panel" in ui.rs does `fill_panel`.
    FillPanel {
        rect: Rect,
        radius: u16,
        bg_rgba: u32,
        title_bar_height: Option<i32>,
    },
    
    SetClip { rect: Rect },
}

impl DrawCmd {
    pub fn bounds(&self) -> Rect {
        match self {
            DrawCmd::FillRect { rect, .. } => *rect,
            DrawCmd::FillRectVGrad { rect, .. } => *rect,
            DrawCmd::FillRoundedRect { rect, .. } => *rect,
            DrawCmd::StrokeRoundedRect { rect, .. } => *rect, // stroke extends half thickness? ignore for now
            DrawCmd::Clear { .. } => Rect { x: 0, y: 0, w: 10000, h: 10000 }, // Full screen?
            DrawCmd::BlitRgbaPremulBytespace { dst_x, dst_y, src_rect, .. } => {
                Rect { x: *dst_x, y: *dst_y, w: src_rect.w, h: src_rect.h }
            }
            DrawCmd::TextRun { x, y, text, font_size, .. } => {
                let w = (text.len() as f32 * font_size * 0.6) as u32;
                Rect { x: *x, y: *y, w, h: *font_size as u32 }
            }
            DrawCmd::Shadow { x, y, width, height, offset_x, offset_y, blur_radius, .. } => {
                // Shadow expands bounds
                let blur = *blur_radius as i32;
                Rect { 
                    x: *x + offset_x - blur, 
                    y: *y + offset_y - blur, 
                    w: *width + (blur as u32 * 2), 
                    h: *height + (blur as u32 * 2) 
                }
            }
            DrawCmd::FillPanel { rect, .. } => *rect,
            DrawCmd::SetClip { .. } => Rect { x: 0, y: 0, w: 0, h: 0 },
        }
    }
}
