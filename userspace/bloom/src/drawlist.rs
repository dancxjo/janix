extern crate alloc;

use alloc::sync::Arc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::geometry::{Point, Rect, Color, Transform, EdgeAA};

// Re-export damage::Rect for legacy compatibility where needed, 
// but we prefer geometry::Rect for new commands.
// Usage: crate::drawlist::DamageRect
#[allow(unused_imports)]
pub use crate::damage::Rect as DamageRect;

/// Insets for nine-slice rendering
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Insets {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Insets {
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self { left, top, right, bottom }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DrawCmd {
    // --- Frame & Control ---
    BeginFrame { id: u64 },
    EndFrame,
    PushClip { rect: Rect },
    PopClip,
    PushTransform { transform: Transform },
    PopTransform,
    Clear { color: Color },

    // --- Primitive Geometry ---
    FillRect { rect: Rect, color: Color, aa: EdgeAA },
    FillRoundRect { rect: Rect, radius: i32, color: Color, aa: EdgeAA },
    StrokeRect { rect: Rect, color: Color, width: i32 },
    FillCircle { center: Point, radius: i32, color: Color },
    StrokeCircle { center: Point, radius: i32, color: Color, width: i32 },
    Line { from: Point, to: Point, color: Color, width: i32 },
    FillArc { center: Point, radius: i32, start_angle: f32, end_angle: f32, color: Color, aa: EdgeAA },

    // --- Image & Bitmap Operations ---
    DrawSnapshot {
        bs_id: u64,
        width: u32,
        height: u32,
        stride: u32,
        dest: Rect,
    },
    DrawImage { image: crate::asset::Image, dest: Rect },
    DrawImageRegion { image: crate::asset::Image, src: Rect, dest: Rect },
    DrawImageTiled { image: crate::asset::Image, dest: Rect },
    DrawImageScaled { image: crate::asset::Image, dest: Rect, filter: i32 }, // filter: simple enum placeholder

    // --- 9-Slice & UI-Specific ---
    DrawNineSlice { image: crate::asset::Image, dest: Rect, margins: Insets },
    
    // Legacy Cursor (Specific to Bloom's optimization need, kept as first-class for now)
    Cursor { frame: crate::asset::CursorFrame, position: Point },
    
    // --- Modern Vector & Text ---
    Text { 
        text: String, 
        font: Option<String>, 
        rect: Rect, // dest rect for layout
        size: f32, 
        color: Color,
        font_debug: bool,
    },
    Path {
        path: Arc<crate::isa::Path2D>,
        color: Color,
        fill_rule: crate::isa::FillRule,
        stroke: Option<Stroke>,
    },
    FillPath {
        path: Arc<crate::isa::Path2D>,
        color: Color,
        fill_rule: crate::isa::FillRule,
        aa: EdgeAA,
    },
    StrokePath {
        path: Arc<crate::isa::Path2D>,
        color: Color,
        width: i32,
        cap: crate::isa::LineCap,
        join: crate::isa::LineJoin,
        miter_limit: f32,
        aa: EdgeAA,
    },

    // --- Compositing & Effects ---
    SetOpacity { alpha: u8 },
    SetBlendMode { mode: u8 }, // 0: Over, 1: Add, etc.
    Shadow { offset: Point, blur: i32, color: Color },

    // --- Debug ---
    DebugRect { rect: Rect, color: Color },
    DebugText { text: String, position: Point, color: Color },
    DebugMarker { id: u64, position: Point },
}

impl DrawCmd {
    /// Compute the bounding box of this draw command.
    pub fn bbox(&self) -> Rect {
        match self {
            DrawCmd::Clear { .. } => Rect::new(0, 0, 10000, 10000), // Ideally shouldn't ask bbox of clear w/o context
            DrawCmd::FillRect { rect, .. } => *rect,
            DrawCmd::FillRoundRect { rect, .. } => *rect,
            DrawCmd::StrokeRect { rect, width, .. } => {
                let w = *width;
                Rect::new(rect.x() - w, rect.y() - w, rect.width() + w*2, rect.height() + w*2)
            }
            DrawCmd::Line { from, to, width, .. } => {
                let min_x = from.x.min(to.x);
                let min_y = from.y.min(to.y);
                let max_x = from.x.max(to.x);
                let max_y = from.y.max(to.y);
                Rect::new(min_x - width, min_y - width, (max_x - min_x) + width*2, (max_y - min_y) + width*2)
            }
            DrawCmd::FillArc { center, radius, .. } => {
                let r = *radius;
                Rect::new(center.x - r, center.y - r, r*2, r*2)
            }
            DrawCmd::DrawSnapshot { dest, .. } => *dest,
            DrawCmd::DrawImage { dest, .. } => *dest,
            DrawCmd::DrawImageRegion { dest, .. } => *dest,
            DrawCmd::DrawNineSlice { dest, .. } => *dest,
            DrawCmd::Cursor { frame, position } => {
                 let dx = position.x - frame.hotspot_x as i32;
                 let dy = position.y - frame.hotspot_y as i32;
                 Rect::new(dx, dy, frame.image.width as i32 + 3, frame.image.height as i32 + 3)
            }
            DrawCmd::Text { rect, .. } => *rect,
            DrawCmd::Path { .. } => Rect::new(0, 0, 10000, 10000), // Paths need better bbox
            DrawCmd::FillPath { .. } => Rect::new(0, 0, 10000, 10000),
            DrawCmd::StrokePath { .. } => Rect::new(0, 0, 10000, 10000),
            // Fallback for others (return empty or minimal rect)
            _ => Rect::default(),
        }
    }
}

pub struct DrawList {
    cmds: Vec<DrawCmd>,
}

impl DrawList {
    pub fn new() -> Self {
        Self { 
            cmds: Vec::new(),
        }
    }

    pub fn commands(&mut self) -> &mut Vec<DrawCmd> {
        &mut self.cmds
    }

    pub fn commands_ref(&self) -> &Vec<DrawCmd> {
        &self.cmds
    }

    pub fn append_to(&self, target: &mut DrawList) {
        target.commands().extend(self.cmds.iter().cloned());
    }

    pub fn clear(&mut self, color: Color) {
        self.cmds.push(DrawCmd::Clear { color });
    }

    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        self.cmds.push(DrawCmd::FillRect { 
            rect: Rect::new(x, y, w, h), 
            color,
            aa: EdgeAA::None,
        });
    }

    pub fn rect_aa(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        self.cmds.push(DrawCmd::FillRect {
            rect: Rect::new(x, y, w, h),
            color,
            aa: EdgeAA::Coverage8,
        });
    }

    pub fn rounded_rect(&mut self, x: i32, y: i32, w: i32, h: i32, radius: i32, color: Color, aa: EdgeAA) {
        self.cmds.push(DrawCmd::FillRoundRect {
            rect: Rect::new(x, y, w, h),
            radius,
            color,
            aa,
        });
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        self.cmds.push(DrawCmd::Line { 
            from: Point::new(x0, y0), 
            to: Point::new(x1, y1), 
            color,
            width: 1 
        });
    }

    pub fn arc(&mut self, cx: i32, cy: i32, r: i32, start: f32, end: f32, color: Color, aa: EdgeAA) {
        self.cmds.push(DrawCmd::FillArc {
            center: Point::new(cx, cy),
            radius: r,
            start_angle: start,
            end_angle: end,
            color,
            aa,
        });
    }

    pub fn blit_image(&mut self, image: &crate::asset::Image, x: i32, y: i32) {
        let dest = Rect::new(x, y, image.width as i32, image.height as i32);
        self.cmds.push(DrawCmd::DrawImage { 
            image: image.clone(),
            dest 
        });
    }

    pub fn blit_image_tiled(&mut self, image: &crate::asset::Image, dest: Rect) {
        self.cmds.push(DrawCmd::DrawImageTiled {
            image: image.clone(),
            dest,
        });
    }

    pub fn cursor(&mut self, frame: &crate::asset::CursorFrame, x: i32, y: i32) {
        self.cmds.push(DrawCmd::Cursor {
            frame: frame.clone(),
            position: Point::new(x, y),
        });
    }

    pub fn nine_slice(&mut self, image: &crate::asset::Image, dst: Rect, insets: Insets) {
        self.cmds.push(DrawCmd::DrawNineSlice {
            image: image.clone(),
            dest: dst,
            margins: insets,
        });
    }

    pub fn text(&mut self, text: &str, font: Option<&str>, x: i32, y: i32, size: f32, color: Color) {
        self.cmds.push(DrawCmd::Text {
            text: text.to_string(),
            font: font.map(|s| s.to_string()),
            rect: Rect::new(x, y, 0, 0), // Point-based text
            size,
            color,
            font_debug: false,
        });
    }

    pub fn text_font_debug(&mut self, text: &str, font: Option<&str>, x: i32, y: i32, size: f32, color: Color, font_debug: bool) {
        self.cmds.push(DrawCmd::Text {
            text: text.to_string(),
            font: font.map(|s| s.to_string()),
            rect: Rect::new(x, y, 0, 0),
            size,
            color,
            font_debug,
        });
    }

    pub fn path(&mut self, path: Arc<crate::isa::Path2D>, color: Color, fill_rule: crate::isa::FillRule) {
        self.cmds.push(DrawCmd::Path {
            path,
            color,
            fill_rule,
            stroke: None,
        });
    }

    pub fn iter(&self) -> core::slice::Iter<'_, DrawCmd> {
        self.cmds.iter()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stroke {
    pub width: f32,
    pub cap: crate::isa::LineCap,
    pub join: crate::isa::LineJoin,
    pub miter_limit: f32,
}
