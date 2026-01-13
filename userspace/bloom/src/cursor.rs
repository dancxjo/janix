use crate::drawlist::DrawList;
use crate::asset::{CursorAsset, CursorFrame};

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

    fn color(&self) -> u32 {
        if self.buttons & 0x1 != 0 {
            0x00FF0000
        } else if self.buttons & 0x2 != 0 {
            0x0000FFFF
        } else if self.buttons & 0x4 != 0 {
            0x00FFFF00
        } else {
            0x00FFFFFF
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

    pub fn emit_drawlist(&self, list: &mut DrawList) {
        if let Some(frame) = self.current_frame() {
             list.cursor(frame, self.x, self.y);
        } else {
            // Fallback
            let color = self.color();
            let size = 10;
            list.rect(self.x, self.y, size, size, color);
        }
    }
}
