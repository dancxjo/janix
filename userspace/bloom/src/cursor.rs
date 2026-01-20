use crate::damage::Rect;
use crate::drawlist::DrawList;
use crate::asset::{CursorAsset, CursorFrame};
use crate::geometry::Color;

#[cfg(feature = "svg-cursors")]
use crate::svg::SvgParser;

#[cfg(feature = "svg-cursors")]
const CURSOR_SVG: &str = include_str!("default.svg");

#[cfg(feature = "svg-cursors")]
static mut TARGET_COLOR: Color = Color::from_u32(0xFF00AAFF);

#[cfg(feature = "svg-cursors")]
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
        #[cfg(feature = "svg-cursors")]
        {
            // SVG cursor: 32x32 base, scale 3.0 = 96x96
            let scale = 3.0;
            let size = (32.0 * scale) as i32;
            Rect::new(self.x, self.y, size, size)
        }
        
        #[cfg(not(feature = "svg-cursors"))]
        {
            if let Some(frame) = self.current_frame() {
                let dx = self.x - frame.hotspot_x as i32;
                let dy = self.y - frame.hotspot_y as i32;
                Rect::new(dx, dy, frame.image.width as i32 + 3, frame.image.height as i32 + 3)
            } else {
                // Fallback cursor size (crosshair)
                Rect::new(self.x - 5, self.y - 5, 11, 11)
            }
        }
    }

    pub fn emit_drawlist(&self, list: &mut DrawList) {
        #[cfg(feature = "svg-cursors")]
        {
            let scale = 3.0;
            let color = if self.buttons != 0 {
                self.color()
            } else {
                unsafe { TARGET_COLOR }
            };
            SvgParser::render(CURSOR_SVG, list, self.x, self.y, scale, color);
        }
        
        #[cfg(not(feature = "svg-cursors"))]
        {
            if let Some(frame) = self.current_frame() {
                 list.cursor(frame, self.x, self.y);
            } else {
                // Procedural Fallback: Crosshair
                let color = self.color();
                let x = self.x;
                let y = self.y;
                
                // Horizontal line
                list.line(x - 5, y, x + 5, y, color);
                // Vertical line
                list.line(x, y - 5, x, y + 5, color);
                // Center dot
                list.rect(x, y, 1, 1, color);
            }
        }
    }
}
