#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use userland::prelude::*;
use userland_std::{Surface, Window};

use crate::config::{
    CLEAR_COLOR, CONTENT_BG, FRAME_THICKNESS, TEXT_COLOR, TITLE_BAR_HEIGHT, TITLE_COLOR_ACTIVE,
    TITLE_COLOR_INACTIVE, TITLE_TEXT_COLOR,
};
use crate::layout::StackedWindow;
use crate::model::Compositor;

use super::{cursor, primitives, text};

#[derive(Debug, Clone)]
pub enum DrawOp {
    Clear {
        color: u32,
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
    Cursor {
        x: i32,
        y: i32,
    },
}

pub fn build_display_list(
    comp: &Compositor,
    stacked: &[StackedWindow],
    windows: &[Window],
    surfaces: &BTreeMap<ThingId, Surface>,
) -> Vec<DrawOp> {
    let mut ops = Vec::new();

    ops.push(DrawOp::Clear { color: CLEAR_COLOR });

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

        if let Some(surface) = surfaces.get(&w.id) {
            if !surface.text.is_empty() && content_w > 0 && content_h > 0 {
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

    ops.push(DrawOp::Cursor {
        x: comp.cursor.x,
        y: comp.cursor.y,
    });

    ops
}

pub fn render_display_list(comp: &Compositor, ops: &[DrawOp]) {
    let buffer = comp.fb.ptr as *mut u32;
    let width = comp.fb.info.width;
    let height = comp.fb.info.height;
    let stride = comp.fb.info.stride;

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
            ),
            DrawOp::Rect { x, y, w, h, color } => {
                primitives::fill_rect(buffer, stride, width, height, *x, *y, *w, *h, *color)
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
                buffer, stride, width, height, *x, *y, *w, *h, *active, title,
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
            DrawOp::Cursor { x, y } => cursor::draw_cursor(buffer, stride, width, height, *x, *y),
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;
    use crate::config::{
        CURSOR_COLOR, FRAME_BORDER, FRAME_THICKNESS, TITLE_COLOR_ACTIVE, TITLE_COLOR_INACTIVE,
    };
    use crate::layout::StackedWindow;
    use crate::test_support::FramebufferFixture;
    use abi::ThingId;
    use userland_std::{Surface, Window};

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
            place_id: ThingId(1),
            x: 5,
            y: 5,
            width: 120,
            height: 80,
            z_index: 1,
            active: true,
            title: format!("window-{id}"),
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
                id: ThingId(10),
                window_id: ThingId(1),
                kind: "text/plain".into(),
                text: "hello".into(),
            },
        );

        let ops = build_display_list(&comp, &stacked, &windows, &surfaces);
        assert!(matches!(ops.first(), Some(DrawOp::Clear { .. })));
        assert!(ops.iter().any(|op| matches!(op, DrawOp::WindowFrame { id, .. } if *id == ThingId(1))));
        assert!(ops
            .iter()
            .any(|op| matches!(op, DrawOp::WindowContentText { id, .. } if *id == ThingId(1))));
        assert!(matches!(
            ops.last(),
            Some(DrawOp::Cursor { x, y }) if *x == comp.cursor.x && *y == comp.cursor.y
        ));
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
            DrawOp::Cursor { x: 15, y: 5 },
        ];

        render_display_list(&comp, &ops);

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
        let cursor_idx = 15 + 5 * stride;
        assert_eq!(buffer[cursor_idx], CURSOR_COLOR);
    }
}
