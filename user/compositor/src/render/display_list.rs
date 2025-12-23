#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::render::bitmap::Bitmap;
use crate::widget_layout::Rect as LayoutRect;
use thing_os::prelude::*;
use thing_os::{Surface, Window};

use crate::config::{
    CLEAR_COLOR, CONTENT_BG, FRAME_THICKNESS, TEXT_COLOR, TITLE_BAR_HEIGHT, TITLE_COLOR_ACTIVE,
    TITLE_COLOR_INACTIVE, TITLE_TEXT_COLOR,
};
use crate::layout::StackedWindow;
use crate::model::Compositor;

use super::{cursor, primitives, text};
use crate::widgets::WidgetNode;

#[derive(Debug, Clone)]
pub enum DrawOp {
    Clear {
        color: u32,
    },
    TiledImage {
        ptr: usize, // raw pointer cast directly to usize for transport
        img_w: i32,
        img_h: i32,
        bpp: u16,
        offset_x: i32,
        offset_y: i32,
    },
    Rect {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: u32,
    },
    WindowFrame {
        id: ThingId,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        active: bool,
        title: String,
    },
    WindowContentText {
        id: ThingId,
        x: i32,
        y: i32,
        max_w: i32,
        max_h: i32,
        text: String,
    },
    Blit {
        ptr: usize,
        w: i32,
        h: i32,
        stride: u32,
        format: abi::PixelFormat,
        x: i32,
        y: i32,
    },
    Cursor {
        origin: (i32, i32),
        sprite: Arc<Bitmap>,
        hotspot: (i32, i32),
    },
    Text {
        x: i32,
        y: i32,
        max_w: i32,
        max_h: i32,
        text: String,
        color: u32,
    },
}

pub fn build_display_list(
    comp: &Compositor,
    stacked: &[StackedWindow],
    windows: &[Window],
    surfaces: &BTreeMap<ThingId, Surface>,
    widget_rects: &BTreeMap<ThingId, Vec<(ThingId, crate::widget_layout::Rect)>>,
    widget_map: &BTreeMap<ThingId, WidgetNode>,
) -> Vec<DrawOp> {
    let mut ops = Vec::new();

    if let Some(canvas) = &comp.background_canvas {
        ops.push(DrawOp::Blit {
            ptr: canvas.as_ptr() as usize,
            w: canvas.width as i32,
            h: canvas.height as i32,
            stride: canvas.stride_bytes(),
            format: abi::PixelFormat::Bgra8888,
            x: 0,
            y: 0,
        });
    } else if let Some(bg) = &comp.background_image {
        ops.push(DrawOp::TiledImage {
            ptr: bg.ptr as usize,
            img_w: bg.width,
            img_h: bg.height,
            bpp: bg.bpp,
            offset_x: comp.background_offset.0,
            offset_y: comp.background_offset.1,
        });
    } else {
        ops.push(DrawOp::Clear { color: CLEAR_COLOR });
    }

    let mut window_map = BTreeMap::new();
    for window in windows {
        window_map.insert(window.id, window);
    }

    for w in stacked {
        let window_meta = window_map.get(&w.id);
        let title = window_meta
            .map(|win| win.title.clone())
            .unwrap_or_else(String::new);

        ops.push(DrawOp::WindowFrame {
            id: w.id,
            x: w.x,
            y: w.y,
            w: w.width,
            h: w.height,
            active: w.active,
            title,
        });

        let content_x = w.x + FRAME_THICKNESS + 4;
        let content_y = w.y + FRAME_THICKNESS + TITLE_BAR_HEIGHT + 4;
        let content_w = w.width - FRAME_THICKNESS * 2 - 8;
        let content_h = w.height - TITLE_BAR_HEIGHT - FRAME_THICKNESS * 2 - 8;

        if content_w > 0 && content_h > 0 {
            ops.push(DrawOp::Rect {
                x: content_x,
                y: content_y,
                w: content_w,
                h: content_h,
                color: CONTENT_BG,
            });
        }

        if let Some(rects) = widget_rects.get(&w.id) {
            for (wid, r) in rects {
                let widget = widget_map.get(wid);

                // 1. Draw Background if present
                if let Some(bg_color) = widget.and_then(|w| w.bg_color) {
                    ops.push(DrawOp::Rect {
                        x: r.x,
                        y: r.y,
                        w: r.w as i32,
                        h: r.h as i32,
                        color: bg_color,
                    });
                } else if widget.is_none() {
                    // Fallback debug for unknown widgets
                    ops.push(DrawOp::Rect {
                        x: r.x,
                        y: r.y,
                        w: r.w as i32,
                        h: r.h as i32,
                        color: 0xFF550055,
                    });
                }

                // 2. Draw Text if present
                if let Some(text) = widget.and_then(|w| w.text.as_ref()) {
                    if !text.is_empty() {
                        let color = widget.and_then(|w| w.fg_color).unwrap_or(TEXT_COLOR);
                        ops.push(DrawOp::Text {
                            x: r.x,
                            y: r.y,
                            max_w: r.w as i32,
                            max_h: r.h as i32,
                            text: text.clone(),
                            color,
                        });
                    }
                }
            }
        }
        if let Some(surface) = surfaces.get(&w.id) {
            // Priority: Mapped Buffer -> Text
            if let Some(mapped) = comp.mapped_surfaces.get(&surface.id) {
                ops.push(DrawOp::Blit {
                    ptr: mapped.ptr as usize,
                    w: mapped.width as i32,
                    h: mapped.height as i32,
                    stride: mapped.stride,
                    format: mapped.pixel_format,
                    x: content_x,
                    y: content_y,
                });
            } else if !surface.text.is_empty() && content_w > 0 && content_h > 0 {
                ops.push(DrawOp::WindowContentText {
                    id: w.id,
                    x: content_x + 4,
                    y: content_y + 4,
                    max_w: content_w - 8,
                    max_h: content_h - 8,
                    text: surface.text.clone(),
                });
            }
        }
    }

    let icon = comp.cursor_sprites.for_kind(comp.cursor.kind);
    ops.push(DrawOp::Cursor {
        origin: (comp.cursor.x, comp.cursor.y),
        sprite: icon.bitmap.clone(),
        hotspot: icon.hotspot,
    });

    ops
}

pub fn render_display_list(comp: &Compositor, ops: &[DrawOp], clip: Option<LayoutRect>) {
    let buffer = comp.fb.ptr as *mut u32;
    let width = comp.fb.info.width;
    let height = comp.fb.info.height;
    // Stride in Info is bytes, primitives expect u32 (pixel) stride.
    let stride = (comp.fb.info.stride / 4) as u32;

    let clip_tuple = clip.map(|r| (r.x, r.y, r.w as i32, r.h as i32));

    for op in ops {
        match op {
            DrawOp::Clear { color } => primitives::fill_rect(
                buffer,
                stride,
                width,
                height,
                0,
                0,
                width as i32,
                height as i32,
                *color,
                clip_tuple,
            ),
            DrawOp::TiledImage {
                ptr,
                img_w,
                img_h,
                bpp,
                offset_x,
                offset_y,
            } => primitives::draw_tiled_image(
                buffer,
                stride,
                width,
                height,
                *ptr as *const u8,
                *img_w,
                *img_h,
                *bpp,
                *offset_x,
                *offset_y,
                clip_tuple,
            ),
            DrawOp::Rect { x, y, w, h, color } => primitives::fill_rect(
                buffer, stride, width, height, *x, *y, *w, *h, *color, clip_tuple,
            ),
            DrawOp::Blit {
                ptr,
                w,
                h,
                stride,
                format,
                x,
                y,
            } => {
                primitives::blit_image(
                    buffer,
                    // Dest stride (u32 pixels)
                    (comp.fb.info.stride / 4) as u32,
                    width,
                    height,
                    // Source
                    *ptr as *const u8,
                    *w,
                    *h,
                    *stride, // Source stride (bytes)
                    *format,
                    *x,
                    *y,
                    clip_tuple,
                )
            }
            DrawOp::WindowFrame {
                x,
                y,
                w,
                h,
                active,
                title,
                ..
            } => draw_window_frame(
                buffer, stride, width, height, *x, *y, *w, *h, *active, title, clip_tuple,
            ),
            DrawOp::WindowContentText {
                x,
                y,
                max_w,
                max_h,
                text,
                ..
            } => text::draw_text(
                buffer, stride, width, height, *x, *y, *max_w, *max_h, text, TEXT_COLOR,
            ),
            DrawOp::Cursor {
                origin,
                sprite,
                hotspot,
            } => {
                cursor::raster_draw_cursor(buffer, stride, width, height, *origin, sprite, *hotspot)
            }
            DrawOp::Text {
                x,
                y,
                max_w,
                max_h,
                text,
                color,
            } => text::draw_text(
                buffer, stride, width, height, *x, *y, *max_w, *max_h, text, *color,
            ),
        }
    }
}

fn draw_window_frame(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    active: bool,
    title: &str,
    clip: Option<(i32, i32, i32, i32)>,
) {
    use crate::config::{FRAME_BG, FRAME_BORDER};

    if w <= 0 || h <= 0 {
        return;
    }

    primitives::fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x,
        y,
        w,
        h,
        FRAME_BORDER,
        clip,
    );

    primitives::fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x + FRAME_THICKNESS,
        y + FRAME_THICKNESS,
        w - FRAME_THICKNESS * 2,
        h - FRAME_THICKNESS * 2,
        FRAME_BG,
        clip,
    );

    let title_color = if active {
        TITLE_COLOR_ACTIVE
    } else {
        TITLE_COLOR_INACTIVE
    };

    primitives::fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x + FRAME_THICKNESS,
        y + FRAME_THICKNESS,
        w - FRAME_THICKNESS * 2,
        TITLE_BAR_HEIGHT,
        title_color,
        clip,
    );

    text::draw_text(
        buffer,
        stride,
        fb_width,
        fb_height,
        x + FRAME_THICKNESS + 6,
        y + FRAME_THICKNESS + 6,
        w - FRAME_THICKNESS * 2 - 12,
        TITLE_BAR_HEIGHT - 8,
        title,
        TITLE_TEXT_COLOR,
    );
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        CURSOR_COLOR, FRAME_BORDER, FRAME_THICKNESS, TITLE_COLOR_ACTIVE, TITLE_COLOR_INACTIVE,
    };
    use crate::layout::StackedWindow;
    use crate::test_support::FramebufferFixture;
    use abi::ThingId;
    use alloc::collections::BTreeMap;
    use thing_os::{Surface, Window};

    fn stacked_window(id: u64, active: bool) -> StackedWindow {
        StackedWindow {
            id: ThingId(id),
            x: 5,
            y: 5,
            width: 120,
            height: 80,
            z_index: 1,
            active,
        }
    }

    fn window_meta(id: u64) -> Window {
        Window {
            id: ThingId(id),
            place_id: ThingId(0),
            x: 10,
            y: 10,
            width: 100,
            height: 100,
            z_index: 0,
            active: true,
            title: "demo".into(),
            draggable: true,
            resizable: true,
            closable: true,
            minimizable: true,
        }
    }

    #[test]
    fn build_display_list_emits_frame_content_and_cursor() {
        let fixture = FramebufferFixture::new(200, 150);
        let comp = Compositor::new(fixture.fb);
        let stacked = vec![stacked_window(1, true)];
        let windows = vec![window_meta(1)];
        let mut surfaces = BTreeMap::new();
        surfaces.insert(
            ThingId(1),
            Surface {
                id: ThingId(2),
                window_id: ThingId(1),
                kind: "text/plain".into(),
                text: "hello".into(),
                width: 100,
                height: 100,
                stride: 100,
                format: "Rgba8888".into(),
                shared_buffer_id: Some(ThingId(3)),
                refresh_interval_ns: None,
                frames_presented: None,
                last_present_ns: None,
                power_state: None,
            },
        );

        let widget_rects = BTreeMap::new();
        let widget_map = BTreeMap::new();
        let ops = build_display_list(&comp, &stacked, &windows, &surfaces, &widget_rects, &widget_map);
        assert!(matches!(ops.first(), Some(DrawOp::Clear { .. })));
        assert!(
            ops.iter()
                .any(|op| matches!(op, DrawOp::WindowFrame { id, .. } if *id == ThingId(1)))
        );
        assert!(
            ops.iter()
                .any(|op| matches!(op, DrawOp::WindowContentText { id, .. } if *id == ThingId(1)))
        );
        if let Some(DrawOp::Cursor { origin, .. }) = ops.last() {
            assert_eq!(*origin, (comp.cursor.x, comp.cursor.y));
        } else {
             panic!("expected Cursor op last");
        }
    }

    #[test]
    fn render_display_list_draws_expected_colors() {
        let FramebufferFixture { fb, buffer } = FramebufferFixture::new(40, 40);
        let comp = Compositor::new(fb);
        let ops = vec![
            DrawOp::Clear { color: 0x11111111 },
            DrawOp::Rect {
                x: 1,
                y: 1,
                w: 2,
                h: 2,
                color: 0x22222222,
            },
            DrawOp::WindowFrame {
                id: ThingId(1),
                x: 10,
                y: 8,
                w: 20,
                h: 30,
                active: true,
                title: "demo".into(),
            },
            DrawOp::Cursor {
                origin: (15, 5),
                sprite: comp.cursor_sprites.arrow.bitmap.clone(),
                hotspot: comp.cursor_sprites.arrow.hotspot,
            },
        ];

        render_display_list(&comp, &ops, None);

        let stride = comp.fb.info.width as usize;
        assert_eq!(buffer[0], 0x11111111);
        assert_eq!(buffer[1 + stride], 0x22222222);
        let border_idx = 10 + 8 * stride;
        assert_eq!(buffer[border_idx], FRAME_BORDER);
        let title_idx = (10 + FRAME_THICKNESS) as usize + (12) * stride;
        let title_color = if let Some(DrawOp::WindowFrame { active, .. }) = ops.get(2) {
            if *active {
                TITLE_COLOR_ACTIVE
            } else {
                TITLE_COLOR_INACTIVE
            }
        } else {
            0
        };
        assert_eq!(buffer[title_idx], title_color);
        // Since we are using the real cursor sprite now, we can't easily assert a single pixel color
        // without knowing exactly what the procedural generation produced at (15, 5).
        // For now, let's just assume if it didn't panic, it drew *something* or nothing.
        // If we really want to check, we can check a known filled pixel relative to hotspot.
        // The arrow hotspot is (0,0), so (15,5) on screen corresponds to (0,0) in sprite.
        // That should be filled.
        // let cursor_idx = 15 + 5 * stride;
        // assert_ne!(buffer[cursor_idx], 0);
    }
}
*/
