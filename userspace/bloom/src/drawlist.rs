extern crate alloc;

use alloc::vec::Vec;

#[derive(Clone, Copy, Debug)]
pub enum DrawCmd {
    Clear { xrgb: u32 },
    Rect { x: i32, y: i32, w: i32, h: i32, xrgb: u32 },
    Line { x0: i32, y0: i32, x1: i32, y1: i32, xrgb: u32 },
}

pub struct DrawList {
    cmds: Vec<DrawCmd>,
}

impl DrawList {
    pub fn new() -> Self {
        Self { cmds: Vec::new() }
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

    pub fn iter(&self) -> core::slice::Iter<'_, DrawCmd> {
        self.cmds.iter()
    }
}
