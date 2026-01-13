extern crate alloc;

use alloc::vec::Vec;

// Re-export damage::Rect for use by other modules (lowered, raster)
pub use crate::damage::Rect;

/// Insets for nine-slice rendering
#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Debug)]
pub enum DrawCmd {
    Clear { xrgb: u32 },
    Rect { x: i32, y: i32, w: i32, h: i32, xrgb: u32 },
    Line { x0: i32, y0: i32, x1: i32, y1: i32, xrgb: u32 },
    BlitImage { image: crate::asset::Image, x: i32, y: i32 },
    Cursor { frame: crate::asset::CursorFrame, x: i32, y: i32 },
    NineSlice { image: crate::asset::Image, dst: Rect, insets: Insets },
}

impl DrawCmd {
    /// Compute the bounding box of this draw command.
    pub fn bbox(&self, screen_w: i32, screen_h: i32) -> Rect {
        match self {
            DrawCmd::Clear { .. } => Rect::new(0, 0, screen_w, screen_h),
            DrawCmd::Rect { x, y, w, h, .. } => Rect::new(*x, *y, *w, *h),
            DrawCmd::Line { x0, y0, x1, y1, .. } => {
                let min_x = (*x0).min(*x1);
                let min_y = (*y0).min(*y1);
                let max_x = (*x0).max(*x1);
                let max_y = (*y0).max(*y1);
                // Add 1 pixel for line thickness
                Rect::new(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
            }
            DrawCmd::BlitImage { image, x, y } => {
                Rect::new(*x, *y, image.width as i32, image.height as i32)
            }
            DrawCmd::Cursor { frame, x, y } => {
                // Account for hotspot offset
                let dx = *x - frame.hotspot_x as i32;
                let dy = *y - frame.hotspot_y as i32;
                Rect::new(dx, dy, frame.image.width as i32, frame.image.height as i32)
            }
            DrawCmd::NineSlice { dst, .. } => *dst,
        }
    }
}

pub struct DrawList {
    cmds: Vec<DrawCmd>,
}

impl DrawList {
    pub fn new() -> Self {
        Self { cmds: Vec::new() }
    }

    pub fn commands(&mut self) -> &mut Vec<DrawCmd> {
        &mut self.cmds
    }

    pub fn clear(&mut self, xrgb: u32) {
        self.cmds.push(DrawCmd::Clear { xrgb });
    }

    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, xrgb: u32) {
        self.cmds.push(DrawCmd::Rect { x, y, w, h, xrgb });
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, xrgb: u32) {
        self.cmds.push(DrawCmd::Line { x0, y0, x1, y1, xrgb });
    }

    pub fn blit_image(&mut self, image: &crate::asset::Image, x: i32, y: i32) {
        self.cmds.push(DrawCmd::BlitImage { 
            image: image.clone(),
            x, y 
        });
    }

    pub fn cursor(&mut self, frame: &crate::asset::CursorFrame, x: i32, y: i32) {
        self.cmds.push(DrawCmd::Cursor {
            frame: frame.clone(),
            x, y
        });
    }

    pub fn nine_slice(&mut self, image: &crate::asset::Image, dst: Rect, insets: Insets) {
        self.cmds.push(DrawCmd::NineSlice {
            image: image.clone(),
            dst,
            insets,
        });
    }

    pub fn iter(&self) -> core::slice::Iter<'_, DrawCmd> {
        self.cmds.iter()
    }
}
