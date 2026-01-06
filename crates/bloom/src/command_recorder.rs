use alloc::vec::Vec;
use crate::draw_cmd::DrawCmd;
use crate::painter::{Painter, Clip};
use crate::scene::Rect;
use crate::shadow::{ShadowMask, ShadowParams};
use alloc::string::ToString;
use abi::ids::ThingId;

pub struct CommandRecorder {
    pub cmds: Vec<DrawCmd>,
    clip_stack: Vec<Clip>,
    current_clip: Clip,
    screen_width: u32,
    screen_height: u32,
}

impl CommandRecorder {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            cmds: Vec::new(),
            clip_stack: Vec::new(),
            current_clip: Clip::full(width, height),
            screen_width: width,
            screen_height: height,
        }
    }

    pub fn finish(self) -> Vec<DrawCmd> {
        self.cmds
    }
}

impl Painter for CommandRecorder {
    fn clear(&mut self, color: u32) {
        self.cmds.push(DrawCmd::Clear { color });
    }

    fn set_clip(&mut self, clip: Clip) {
        self.current_clip = clip;
        self.cmds.push(DrawCmd::SetClip { rect: clip.rect });
    }

    fn clip(&self) -> Clip {
        self.current_clip
    }

    fn fill_rect(&mut self, rect: Rect, color: u32) {
        self.cmds.push(DrawCmd::FillRect { rect, color });
    }

    fn fill_rect_vgrad(&mut self, rect: Rect, radius: u16, top_color: u32, bottom_color: u32) {
        self.cmds.push(DrawCmd::FillRectVGrad { rect, radius, top_color, bottom_color });
    }

    fn fill_rounded_rect(&mut self, rect: Rect, radius: u16, color: u32) {
        self.cmds.push(DrawCmd::FillRoundedRect { rect, radius, color });
    }

    fn stroke_rounded_rect(&mut self, rect: Rect, radius: u16, thickness: u16, color: u32) {
        self.cmds.push(DrawCmd::StrokeRoundedRect { rect, radius, thickness, color });
    }

    fn draw_cursor_frame(&mut self, _frame: &crate::assets::cursor::CursorFrame, _x: i32, _y: i32) {
        // Cursor is separate pass, ignore or warn?
        // ui.rs doesn't draw cursor. app.rs does.
        // app.rs draws cursor *after* the scene build.
        // Since we are replacing `rebuild_scene` painter, and cursor is outside it, 
        // we generally won't see this called on the recorder.
    }

    fn draw_fallback_cursor(&mut self, _x: i32, _y: i32) {
        // Same as above
    }

    fn draw_shadow_mask(&mut self, x: i32, y: i32, mask: ShadowMask<'_>, params: ShadowParams) {
        match mask {
            ShadowMask::RoundedRect { width, height, radius } => {
                self.cmds.push(DrawCmd::Shadow {
                    x, y, 
                    width, height, 
                    radius, 
                    color: params.color,
                    offset_x: params.offset_x,
                    offset_y: params.offset_y,
                    blur_radius: params.blur_radius as u16,
                });
            }
            ShadowMask::SpriteAlpha { .. } => {
                // Not supported in command list yet (used for cursor)
            }
        }
    }

    fn blit_rgba(&mut self, _dst_x: i32, _dst_y: i32, _src: &[u32], _src_w: u32, _src_h: u32) {
        // "BlitRgbaPremulInline" equivalent?
        // `src` is a slice. We can't keep it easily.
        // Usage in Bloom: `paint_canvas` (but that maps bytespace, so we should use bytespace op),
        // `paint_drawlist` (also bytespace).
        // `paint_progress_background`? Fills rect/clears.
        // `ui.rs:329` calls `blit_rgba_alpha`.
        // If `ui.rs` is updated to call `blit_from_bytespace` explicitly, we are good.
        // But `Painter` trait doesn't have `blit_from_bytespace`.
        // We need to extend `Painter` trait?
        // Or we implement `blit_rgba` by checking if we have a way to know the source.
        // BUT `ui.rs` passes `pixels` pointer. The recorder can't convert that back to ID.
        //
        // CRITICAL: ui.rs needs to be Refactored to call a "new" method on Painter or we cast?
        // We can add `blit_bytespace` to the Painter trait?
        // The User said: "Implement your existing Painter trait... blit_rgba_premul(...) -> push DrawCmd::Blit..."
        // But `blit_rgba` takes `&[u32]`.
        // The user also said: "5) Make Canvas and DrawList first-class command sources ... In record mode, Canvas emits: DrawCmd::BlitRgbaPremulBytespace ... It must not map bytespaces."
        // This means `ui.rs` MUST be changed to emit command directly or call a new trait method.
        //
        // Plan: Add `blit_bytespace` to `Painter` trait (with default impl?).
        // Or just specialize `paint_canvas` to check if `painter` is a `CommandRecorder`? No, that's messy dynamic dispatch.
        // Better: Add `blit_bytespace` to `Painter` trait.
    }

    fn blit_rgba_alpha(&mut self, _dst_x: i32, _dst_y: i32, _src: &[u32], _src_w: u32, _src_h: u32) {
        // Same issue. We need `blit_bytespace`.
    }

    fn blit_asset(&mut self, dst_x: i32, dst_y: i32, id: ThingId, src_w: u32, src_h: u32, src_stride: u32, src_len: usize, _cache: &mut crate::scene_cache::BytespaceMappingCache) {
        self.cmds.push(DrawCmd::BlitRgbaPremulBytespace {
            bytespace: id,
            src_rect: Rect { x: 0, y: 0, w: src_w, h: src_h }, 
            dst_x,
            dst_y,
            src_stride,
            src_len,
        });
    }

    fn blit_rgba_alpha_rect(&mut self, _dst_x: i32, _dst_y: i32, _src: &[u32], _src_stride: u32, _w: u32, _h: u32) {
        // Not supported in recording yet
    }

    fn screen_size(&self) -> (u32, u32) {
        (self.screen_width, self.screen_height)
    }

    fn fill_panel(&mut self, rect: Rect, radius: u16, bg_rgba: u32, title_bar_height: Option<i32>) {
        self.cmds.push(DrawCmd::FillPanel { rect, radius, bg_rgba, title_bar_height });
    }

    fn damage(&mut self, _rect: Rect) {
        // No-op for recording
    }

    fn take_damage(&mut self) -> Option<Rect> {
        None
    }

    fn copy_region(&mut self, _src: &[u32], _src_w: u32, _region: Rect) {
        // Not supported in recording yet (requires asset ID)
    }

    fn draw_text(&mut self, x: i32, y: i32, text: &str, color: u32, font_size: f32) {
        self.cmds.push(DrawCmd::TextRun {
            x,
            y,
            text: alloc::string::String::from(text),
            color,
            font_size,
        });
    }
}
