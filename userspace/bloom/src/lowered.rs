//! Two-Phase Lowering: DrawList → LowLevelOp
//!
//! This module implements the hardware-agnostic lowering step.
//! Every high-level DrawCmd is decomposed into primitive operations
//! that any target (CPU, GPU, VirtIO) can execute.

extern crate alloc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::asset::{CursorFrame, Image};
use crate::damage::Rect;
use crate::drawlist::{DrawCmd, DrawList, Insets};

/// Low-level primitive operations that any compositor target can execute.
#[derive(Clone, Debug)]
pub enum LowLevelOp {
    /// Fill entire surface with solid color
    Clear { xrgb: u32 },
    /// Fill rectangle with solid color
    FillRect { x: i32, y: i32, w: i32, h: i32, xrgb: u32 },
    /// Copy pixels from image to surface (no alpha)
    Blit { image: Image, src: Rect, dst: Rect },
    /// Copy pixels with alpha blending (for cursors, shadows)
    BlitAlpha { image: Image, src: Rect, dst: Rect, shadow_factor: Option<u8> },
    /// Draw line (Bresenham's)
    Line { x0: i32, y0: i32, x1: i32, y1: i32, xrgb: u32 },
    /// Draw text span (rasterized by the target using fontdue)
    TextSpan { text: Arc<str>, x: i32, y: i32, size: f32, color: u32 },
}

/// Container for lowered operations
pub struct LoweredDraw {
    pub ops: Vec<LowLevelOp>,
}

impl LoweredDraw {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }
}

/// Lower an entire DrawList into LowLevelOps
pub fn lower(list: &DrawList) -> LoweredDraw {
    let mut out = LoweredDraw::new();

    for cmd in list.iter() {
        match cmd {
            DrawCmd::Clear { xrgb } => {
                out.ops.push(LowLevelOp::Clear { xrgb: *xrgb });
            }
            DrawCmd::Rect { x, y, w, h, xrgb } => {
                out.ops.push(LowLevelOp::FillRect {
                    x: *x, y: *y, w: *w, h: *h, xrgb: *xrgb,
                });
            }
            DrawCmd::Line { x0, y0, x1, y1, xrgb } => {
                out.ops.push(LowLevelOp::Line {
                    x0: *x0, y0: *y0, x1: *x1, y1: *y1, xrgb: *xrgb,
                });
            }
            DrawCmd::BlitImage { image, x, y } => {
                lower_blit(image, *x, *y, &mut out.ops);
            }
            DrawCmd::Cursor { frame, x, y } => {
                lower_cursor(frame, *x, *y, &mut out.ops);
            }
            DrawCmd::NineSlice { image, dst, insets } => {
                lower_nine_slice(image, dst, insets, &mut out.ops);
            }
            DrawCmd::Text { text, x, y, size, color } => {
                lower_text(text, *x, *y, *size, *color, &mut out.ops);
            }
        }
    }

    out
}

fn lower_blit(image: &Image, x: i32, y: i32, ops: &mut Vec<LowLevelOp>) {
    let src = Rect::new(0, 0, image.width as i32, image.height as i32);
    let dst = Rect::new(x, y, image.width as i32, image.height as i32);
    ops.push(LowLevelOp::Blit { image: image.clone(), src, dst });
}

fn lower_cursor(frame: &CursorFrame, x: i32, y: i32, ops: &mut Vec<LowLevelOp>) {
    // Apply hotspot offset
    let dx = x - frame.hotspot_x as i32;
    let dy = y - frame.hotspot_y as i32;

    // Shadow: emit a darkened, offset copy FIRST
    // Shadow is 2px offset, 30% opacity (shadow_factor = 77 out of 255)
    let shadow_offset = 2;
    ops.push(LowLevelOp::BlitAlpha {
        image: frame.image.clone(),
        src: Rect::new(0, 0, frame.image.width as i32, frame.image.height as i32),
        dst: Rect::new(dx + shadow_offset, dy + shadow_offset, frame.image.width as i32, frame.image.height as i32),
        shadow_factor: Some(77), // ~30% opacity for shadow
    });

    // Main cursor (full alpha)
    ops.push(LowLevelOp::BlitAlpha {
        image: frame.image.clone(),
        src: Rect::new(0, 0, frame.image.width as i32, frame.image.height as i32),
        dst: Rect::new(dx, dy, frame.image.width as i32, frame.image.height as i32),
        shadow_factor: None,
    });
}

fn lower_text(text: &Arc<str>, x: i32, y: i32, size: f32, color: u32, ops: &mut Vec<LowLevelOp>) {
    // Text is not decomposed further - the rasterizer handles it directly
    ops.push(LowLevelOp::TextSpan {
        text: text.clone(),
        x,
        y,
        size,
        color,
    });
}

/// Nine-slice lowering: splits one image into 9 blits
fn lower_nine_slice(image: &Image, dst: &Rect, insets: &Insets, ops: &mut Vec<LowLevelOp>) {
    let iw = image.width as i32;
    let ih = image.height as i32;

    // Source slice boundaries
    let sl = insets.left;
    let st = insets.top;
    let sr = iw - insets.right;
    let sb = ih - insets.bottom;

    // Destination slice boundaries
    let dl = dst.x + insets.left;
    let dt = dst.y + insets.top;
    let dr = dst.x + dst.w - insets.right;
    let db = dst.y + dst.h - insets.bottom;

    // Helper to push a blit for a slice
    let mut emit = |sx: i32, sy: i32, sw: i32, sh: i32, dstx: i32, dsty: i32, dw: i32, dh: i32| {
        if sw > 0 && sh > 0 && dw > 0 && dh > 0 {
            ops.push(LowLevelOp::Blit {
                image: image.clone(),
                src: Rect::new(sx, sy, sw, sh),
                dst: Rect::new(dstx, dsty, dw, dh),
            });
        }
    };

    // Corners (no stretch)
    emit(0, 0, sl, st, dst.x, dst.y, sl, st);                 // top-left
    emit(sr, 0, insets.right, st, dr, dst.y, insets.right, st);   // top-right
    emit(0, sb, sl, insets.bottom, dst.x, db, sl, insets.bottom); // bottom-left
    emit(sr, sb, insets.right, insets.bottom, dr, db, insets.right, insets.bottom); // bottom-right

    // Edges (stretch in one direction)
    let mid_w = dr - dl;
    let mid_h = db - dt;
    let src_mid_w = sr - sl;
    let src_mid_h = sb - st;

    emit(sl, 0, src_mid_w, st, dl, dst.y, mid_w, st);         // top
    emit(sl, sb, src_mid_w, insets.bottom, dl, db, mid_w, insets.bottom); // bottom
    emit(0, st, sl, src_mid_h, dst.x, dt, sl, mid_h);         // left
    emit(sr, st, insets.right, src_mid_h, dr, dt, insets.right, mid_h); // right

    // Center (stretch both directions)
    emit(sl, st, src_mid_w, src_mid_h, dl, dt, mid_w, mid_h);
}
