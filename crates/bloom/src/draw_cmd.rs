//! Draw command enumeration for retained-mode rendering.
//!
//! All scene content is recorded as `DrawCmd` variants during the record phase.
//! The executor processes these commands in a single pass to render the scene.
//!
//! ## Command Categories
//!
//! - **Primitives**: FillRect, FillRoundedRect, StrokeRoundedRect, Clear
//! - **Blits**: BlitRgbaPremulBytespace, TileBitmap  
//! - **Text**: TextRun (stores String for deferred rendering)
//! - **Effects**: Shadow, FillPanel (gradient+stripe)
//! - **Clipping**: SetClip, PushClip, PopClip
//!
//! ## Bounds Tracking
//!
//! Each command implements `bounds()` to return its spatial extent,
//! enabling damage tracking and culling during execution.

use crate::scene::{Rect, Point};
use crate::assets::bitmap::BitmapHandle;
use abi::ids::ThingId;
use serde::{Serialize, Deserialize};

use alloc::string::String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DrawCmd {
    // === Primitives (matching ABI or basic shapes) ===
    FillRect { rect: Rect, color: u32 },
    FillRectVGrad { rect: Rect, radius: u16, top_color: u32, bottom_color: u32 },
    FillRoundedRect { rect: Rect, radius: u16, color: u32 },
    StrokeRoundedRect { rect: Rect, radius: u16, thickness: u16, color: u32 },
    StrokeRoundedRectTop { rect: Rect, radius: u16, thickness: u16, color: u32 },
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

    // Tile a bitmap (e.g. wallpaper)
    TileBitmap {
        dst: Rect,            // destination region in target buffer coords
        bitmap: BitmapHandle, // handle to ARGB32 pixels
        bmp_w: u32,
        bmp_h: u32,
        origin: Point,        // phase/alignment
        opacity: u8,          // 255 for now
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
        top_only: bool,
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
    PushClip { rect: Rect },
    PopClip,
}

impl DrawCmd {
    /// Returns the bounding box of the command, if it has a fixed spatial extent.
    /// Returns None if the command is global (Clear), state change (SetClip), or dynamic (TextRun).
    pub fn bounds(&self) -> Option<Rect> {
        match self {
            DrawCmd::FillRect { rect, .. } => Some(*rect),
            DrawCmd::FillRectVGrad { rect, .. } => Some(*rect),
            DrawCmd::FillRoundedRect { rect, .. } => Some(*rect),
            DrawCmd::StrokeRoundedRect { rect, .. } => Some(*rect),
            DrawCmd::StrokeRoundedRectTop { rect, .. } => Some(*rect),
            DrawCmd::FillPanel { rect, .. } => Some(*rect),
            DrawCmd::TileBitmap { dst, .. } => Some(*dst),
            DrawCmd::BlitRgbaPremulBytespace { dst_x, dst_y, src_rect, .. } => {
                Some(Rect { x: *dst_x, y: *dst_y, w: src_rect.w, h: src_rect.h })
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
            // Clear covers everything (conceptually), but for culling we return None so the executor knows it "touches everything" or "cannot be culled by rect".
            DrawCmd::Clear { .. } => None, 
            DrawCmd::TextRun { .. } => None, // Text is hard to bound without font metrics, assume None (always draw)
            DrawCmd::SetClip { .. } => None,
            DrawCmd::PushClip { .. } => None,
            DrawCmd::PopClip => None,
        }
    }
}
