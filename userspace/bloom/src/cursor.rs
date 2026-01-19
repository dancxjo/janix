use crate::damage::Rect;
use crate::drawlist::DrawList;
use crate::asset::{CursorAsset, CursorFrame};
use crate::geometry::Color;
use crate::svg::SvgParser;

// Embed the cursor SVG
const CURSOR_SVG: &str = include_str!("default.svg");

static mut TARGET_COLOR: Color = Color::from_u32(0xFF00AAFF);

pub fn set_target_color(color: Color) {
    unsafe {
        TARGET_COLOR = color;
    }
}

pub struct CursorState {
    pub x: i32,
    pub y: i32,
    buttons: u32,
    asset: Option<CursorAsset>,
}

impl CursorState {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, buttons: 0, asset: None }
    }
    
    pub fn set_asset(&mut self, asset: CursorAsset) {
        self.asset = Some(asset);
    }

    pub fn apply_move(&mut self, dx: i16, dy: i16, w: i32, h: i32) {
        let mut nx = self.x + dx as i32;
        let mut ny = self.y + dy as i32;
        if nx < 0 { nx = 0; }
        if ny < 0 { ny = 0; }
        if nx >= w { nx = w.saturating_sub(1); }
        if ny >= h { ny = h.saturating_sub(1); }
        self.x = nx;
        self.y = ny;
    }

    pub fn button_down(&mut self, button: u8) {
        if button < 32 {
            self.buttons |= 1u32 << button;
        }
    }

    pub fn button_up(&mut self, button: u8) {
        if button < 32 {
            self.buttons &= !(1u32 << button);
        }
    }

    fn color(&self) -> Color {
        if self.buttons & 0x1 != 0 {
            Color::from_u32(0x00FF0000)
        } else if self.buttons & 0x2 != 0 {
            Color::from_u32(0x0000FFFF)
        } else if self.buttons & 0x4 != 0 {
            Color::from_u32(0x00FFFF00)
        } else {
            Color::from_u32(0x00FFFFFF)
        }
    }
    
    fn current_frame(&self) -> Option<&CursorFrame> {
        match &self.asset {
            Some(CursorAsset::Static(frame)) => Some(frame),
            Some(CursorAsset::Animated { frames, .. }) => {
                // TODO: Animation logic using time
                // For now, return first frame
                frames.first()
            }
            None => None,
        }
    }

    /// Compute the bounding box of the cursor at its current position.
    pub fn bbox(&self) -> Rect {
        // SVG Cursor box
        // Base size 32x32, scale 3.0 -> 96x96
        // Hotspot assumed at top-left for now, or maybe (0,0) of the SVG.
        // Let's assume (0,0) is the tip.
        let scale = 3.0;
        let size = (32.0 * scale) as i32;
        Rect::new(self.x, self.y, size, size)
    }

    pub fn emit_drawlist(&self, list: &mut DrawList) {
        // Render SVG Cursor
        // We ignore the bitmap asset if we want to force the SVG one,
        // OR we can prefer SVG if available.
        // The prompt says "Put the normal cursor in the modules as an svg".
        // And "replace the current cursor set".
        // So we should always use the SVG.

        let scale = 3.0;
        let color = if self.buttons != 0 {
             self.color()
        } else {
             unsafe { TARGET_COLOR }
        };

        SvgParser::render(CURSOR_SVG, list, self.x, self.y, scale, color);
    }
}
