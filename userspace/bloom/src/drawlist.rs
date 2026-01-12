extern crate alloc;

use alloc::vec::Vec;

/// A tiny render-agnostic list of drawing commands.
#[derive(Clone, Debug, Default)]
pub struct DrawList {
    pub cmds: Vec<DrawCmd>,
}

impl DrawList {
    pub fn new() -> Self {
        Self { cmds: Vec::new() }
    }

    pub fn push(&mut self, cmd: DrawCmd) {
        self.cmds.push(cmd);
    }
}

/// Minimal drawing commands. Colors are 0xAARRGGBB.
#[derive(Clone, Copy, Debug)]
pub enum DrawCmd {
    Clear(u32),
    Rect {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        rgba: u32,
    },
}
