use crate::draw_cmd::DrawCmd;
use crate::painter::{Painter, CpuPainter, Clip};
use crate::scene_cache::BytespaceMappingCache;
use crate::scene::Rect;
use alloc::vec::Vec;
use thing_std::log_info;

/// Execute a list of commands into a mutable buffer.
pub fn execute_cmds_into_scene(
    cmds: &[DrawCmd],
    scene_buffer: &mut [u32],
    width: u32,
    height: u32,
    mapping_cache: &mut BytespaceMappingCache,
) {
    let mut painter = CpuPainter::new(scene_buffer, width, height);

    for cmd in cmds {
        match cmd {
            DrawCmd::FillRect { rect, color } => {
                painter.fill_rect(*rect, *color);
            }
            DrawCmd::FillRoundedRect { rect, radius, color } => {
                painter.fill_rounded_rect(*rect, *radius, *color);
            }
            DrawCmd::StrokeRoundedRect { rect, radius, thickness, color } => {
                painter.stroke_rounded_rect(*rect, *radius, *thickness, *color);
            }
            DrawCmd::Clear { color } => {
                painter.clear(*color);
            }
            DrawCmd::TextRun { x, y, text, color, font_size } => {
                // Call existing text drawing helper
                // Note: draw_text_on_painter currently uses crates/bloom/src/text.rs
                // We just delegate to it.
                // It works on any Painter.
                crate::text::draw_text_on_painter(&mut painter, *x, *y, text, *color, *font_size);
            }
            DrawCmd::BlitRgbaPremulBytespace { bytespace, src_rect, dst_x, dst_y, src_stride, src_len } => {
                 if let Some(buf) = mapping_cache.get_or_map_ro(*bytespace, *src_len) {
                     // Check bounds against len? 
                     // stride * height is min. 
                     // We just assume mapped buf is sufficient if get_or_map_ro checks len.
                     
                     // Convert to u32 slice
                     let u32_buf = unsafe {
                         core::slice::from_raw_parts(buf.as_ptr() as *const u32, buf.len() / 4)
                     };
                     painter.blit_rgba_alpha(*dst_x, *dst_y, u32_buf, *src_stride, src_rect.h);
                 }
            }
            DrawCmd::Shadow { x, y, width, height, radius, color, offset_x, offset_y, blur_radius } => {
                // Construct mask and params
                 painter.draw_shadow_mask(
                    *x, *y,
                     crate::shadow::ShadowMask::RoundedRect { 
                        width: *width, 
                        height: *height, 
                        radius: *radius 
                     },
                     crate::shadow::ShadowParams {
                         offset_x: *offset_x,
                         offset_y: *offset_y,
                         blur_radius: *blur_radius as u32,
                         color: *color,
                     }
                 );
            }
            DrawCmd::FillPanel { rect, radius, bg_rgba, title_bar_height } => {
                painter.fill_panel(*rect, *radius, *bg_rgba, *title_bar_height);
            }
            DrawCmd::SetClip { rect } => {
                painter.set_clip(crate::painter::Clip::from_rect(*rect));
            }
        }
    }
}
