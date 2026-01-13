extern crate alloc;

use alloc::vec::Vec;

#[derive(Clone, Copy, Debug)]
pub enum DrawCmd {
    Clear { xrgb: u32 },
    Rect { x: i32, y: i32, w: i32, h: i32, xrgb: u32 },
    Line { x0: i32, y0: i32, x1: i32, y1: i32, xrgb: u32 },
    BlitImage { image: crate::asset::Image, x: i32, y: i32 },
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
        // Since Image now holds &'static [u32], we can just pass the pointer to the Image struct
        // But wait, DrawList lives longer than the stack frame of `get_clouds()`.
        // `get_clouds` returns `Image` struct (by value, it's small: w, h, &slice).
        // The slice is 'static.
        // We need `DrawCmd` to hold... the Image data?
        // `DrawCmd::BlitImage` stored `*const Image`.
        // If we pass `&Image` from `get_clouds`, that `Image` temporary on stack dies.
        // We need `DrawCmd` to hold (width, height, *const data).
        // OR `DrawCmd` holds a pointer to a Heap `Image`.
        //
        // Let's change `DrawCmd` to hold the fields directly or a safe reference.
        // `Image` is Copy/Clone-able? No, it has `&[u32]`. It is Copy-able effectively.
        // Let's make `Image` Copy/Clone.
        
        // Wait, I can't modify `DrawCmd` easily here without seeing it all again?
        // Let's modify `DrawCmd` in `drawlist.rs` to store `Image` (the struct) if it's small.
        // w(4)+h(4)+slice(16) = 24 bytes. That's fine for an enum variant.
        
        self.cmds.push(DrawCmd::BlitImage { 
            image: image.clone(), // We'll derive Clone for Image
            x, y 
        });
    }

    pub fn iter(&self) -> core::slice::Iter<'_, DrawCmd> {
        self.cmds.iter()
    }
}
