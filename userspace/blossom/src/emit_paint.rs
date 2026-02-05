extern crate alloc;

use alloc::vec::Vec;

use abi::schema::keys;
use abi::ui_paint::{ImageFit, PaintBuilder};
use abi::ui_scene::NodeKind;
use alloc::string::ToString;
use stem::thing::sys::{bytespace_info, bytespace_read, prop_get};
use stem::thing::{HandleId, ThingId};

use crate::layout::{LayoutRect, WINDOW_BORDER, WINDOW_TITLE_HEIGHT};
use crate::scene::{SceneGraph, SceneNode};

pub fn emit_paint(
    scene: &SceneGraph,
    layout: &[LayoutRect],
    window_bg: u32,
    title_override: Option<&str>,
    is_focused: bool,
) -> Vec<u8> {
    let mut builder = PaintBuilder::new();
    emit_node(
        scene,
        scene.root,
        layout,
        window_bg,
        title_override,
        is_focused,
        0,
        0,
        &mut builder,
    );
    builder.finish()
}

fn emit_node(
    scene: &SceneGraph,
    index: usize,
    layout: &[LayoutRect],
    window_bg: u32,
    title_override: Option<&str>,
    is_focused: bool,
    offset_x: i32,
    offset_y: i32,
    builder: &mut PaintBuilder,
) {
    let node = &scene.nodes[index];
    let base = layout[index];
    let rect = LayoutRect {
        x: base.x + offset_x,
        y: base.y + offset_y,
        w: base.w,
        h: base.h,
    };

    // Use stem::info loosely for tracing node types
    if matches!(
        node.kind,
        NodeKind::Scroll | NodeKind::Flex | NodeKind::Separator | NodeKind::Spacer
    ) {
        // stem::info!("BLOSSOM: emit_node kind={:?} rect={:?}", node.kind, rect);
    }

    match node.kind {
        NodeKind::Window => {
            draw_window_chrome(
                scene,
                node,
                rect,
                window_bg,
                title_override,
                is_focused,
                builder,
            );
            let content = window_content_rect(node, rect);
            if content.w > 0 && content.h > 0 {
                builder.push_clip(content.x, content.y, content.w, content.h);
                for &child in &node.children {
                    emit_node(
                        scene,
                        child,
                        layout,
                        window_bg,
                        title_override,
                        is_focused,
                        offset_x,
                        offset_y,
                        builder,
                    );
                }
                builder.pop_clip();
            }
            return;
        }
        NodeKind::Rect => {
            if let Some(meta) = node.rect_meta {
                builder.fill_rect(rect.x, rect.y, rect.w, rect.h, meta.color);
            }
        }
        NodeKind::Text => {
            if let Some(meta) = node.text_meta {
                let text = scene.string(meta.text).unwrap_or("");
                let font_from_thing;
                let font = match meta.font_kind {
                    abi::ui_scene::FontKeyKind::Name => {
                        scene.string(meta.font_name).unwrap_or("NotoSans-Regular")
                    }
                    abi::ui_scene::FontKeyKind::Thing => {
                        font_from_thing = read_font_name(meta.font_thing);
                        font_from_thing.as_deref().unwrap_or("NotoSans-Regular")
                    }
                    _ => "NotoSans-Regular",
                };
                let size = if meta.size > 0 { meta.size as i32 } else { 16 };
                let baseline = rect.y + size;
                builder.draw_text_run(
                    rect.x, rect.y, rect.w, rect.h, baseline, font, size, text, meta.color,
                );
            }
        }
        NodeKind::Image => {
            if let Some(meta) = node.image_meta {
                let key = scene.string(meta.key).unwrap_or("");
                let fit = match meta.fit {
                    abi::ui_scene::ImageFit::Fill => ImageFit::Fill,
                    abi::ui_scene::ImageFit::Contain => ImageFit::Contain,
                    abi::ui_scene::ImageFit::Cover => ImageFit::Cover,
                    abi::ui_scene::ImageFit::None => ImageFit::None,
                };
                builder.blit_image(rect.x, rect.y, rect.w, rect.h, fit, key);
            }
        }
        NodeKind::Icon => {
            if let Some(meta) = node.icon_meta {
                let name = scene.string(meta.name).unwrap_or("");
                builder.draw_icon(rect.x, rect.y, rect.w, rect.h, name);
            }
        }
        NodeKind::Scroll => {
            if let Some(meta) = node.scroll_meta {
                if meta.clip {
                    builder.push_clip(rect.x, rect.y, rect.w, rect.h);
                }
                let child_offset_y = offset_y - meta.scroll_y_px;
                for &child in &node.children {
                    emit_node(
                        scene,
                        child,
                        layout,
                        window_bg,
                        title_override,
                        is_focused,
                        offset_x,
                        child_offset_y,
                        builder,
                    );
                }
                if meta.clip {
                    builder.pop_clip();
                }
            }
            return;
        }
        NodeKind::Line => {
            if let Some(meta) = node.line_meta {
                let min_x = meta.x1.min(meta.x2);
                let min_y = meta.y1.min(meta.y2);
                let dx = rect.x - min_x;
                let dy = rect.y - min_y;
                builder.stroke_line(
                    meta.x1 + dx,
                    meta.y1 + dy,
                    meta.x2 + dx,
                    meta.y2 + dy,
                    meta.width.max(1),
                    meta.color,
                );
            }
        }
        NodeKind::Separator => {
            if let Some(meta) = node.separator_meta {
                builder.fill_rect(rect.x, rect.y, rect.w, rect.h, meta.color);
            }
        }
        NodeKind::Spacer => {}
        NodeKind::Checkbox => {
            draw_checkbox(scene, node, rect, builder);
        }
        NodeKind::TextInput => {
            draw_text_input(scene, node, rect, builder);
        }
        _ => {}
    }

    for &child in &node.children {
        emit_node(
            scene,
            child,
            layout,
            window_bg,
            title_override,
            is_focused,
            offset_x,
            offset_y,
            builder,
        );
    }
}

fn read_font_name(id: u64) -> Option<alloc::string::String> {
    if id == 0 {
        return None;
    }
    let bs = prop_get(ThingId::from_u64(id), keys::FONT_NAME).ok()?;
    if bs == 0 {
        return None;
    }
    let bs_id = ThingId::from_u64(bs);
    let size = bytespace_info(bs_id).ok()?;
    if size == 0 {
        return Some(alloc::string::String::new());
    }
    let mut buf = alloc::vec![0u8; size];
    let read = bytespace_read(bs_id, 0, &mut buf).ok()?;
    let text = core::str::from_utf8(&buf[..read]).ok()?;
    Some(text.trim_end_matches('\0').to_string())
}

fn draw_checkbox(
    scene: &SceneGraph,
    node: &SceneNode,
    rect: LayoutRect,
    builder: &mut PaintBuilder,
) {
    let box_size = rect.h.min(16).max(0);
    let box_rect = LayoutRect {
        x: rect.x,
        y: rect.y + (rect.h - box_size) / 2,
        w: box_size,
        h: box_size,
    };
    let checked = node.checkbox_meta.map(|m| m.checked).unwrap_or(false);
    let color = if checked { 0xFFFFFFFF } else { 0xFF444444 };
    builder.fill_rect(box_rect.x, box_rect.y, box_rect.w, box_rect.h, color);

    if let Some(meta) = node.checkbox_meta {
        let label = scene.string(meta.label).unwrap_or("");
        let text_rect = LayoutRect {
            x: rect.x + box_size + 8,
            y: rect.y,
            w: (rect.w - box_size - 8).max(0),
            h: rect.h,
        };
        let baseline = text_rect.y + 16;
        builder.draw_text_run(
            text_rect.x,
            text_rect.y,
            text_rect.w,
            text_rect.h,
            baseline,
            "NotoSans-Regular",
            16,
            label,
            0xFFFFFFFF,
        );
    }
}

fn draw_text_input(
    scene: &SceneGraph,
    node: &SceneNode,
    rect: LayoutRect,
    builder: &mut PaintBuilder,
) {
    // Draw border (using a gray color)
    let border_color = 0xFF888888;
    builder.fill_rect(rect.x, rect.y, rect.w, rect.h, border_color);
    
    // Draw background (white)
    let padding = 2;
    let inner_x = rect.x + padding;
    let inner_y = rect.y + padding;
    let inner_w = (rect.w - padding * 2).max(0);
    let inner_h = (rect.h - padding * 2).max(0);
    builder.fill_rect(inner_x, inner_y, inner_w, inner_h, 0xFFFFFFFF);
    
    if let Some(meta) = node.text_input_meta {
        let text_padding = 8;
        let text_x = rect.x + text_padding;
        let text_y = rect.y + padding;
        let text_w = (rect.w - text_padding * 2).max(0);
        let text_h = (rect.h - padding * 2).max(0);
        
        let value = scene.string(meta.value).unwrap_or("");
        let display_text = if value.is_empty() {
            scene.string(meta.placeholder).unwrap_or("")
        } else {
            value
        };
        
        let text_color = if value.is_empty() {
            0xFF888888 // Gray for placeholder
        } else {
            0xFF000000 // Black for actual text
        };
        
        let font_size = 16;
        let baseline = text_y + font_size + 2;
        builder.draw_text_run(
            text_x,
            text_y,
            text_w,
            text_h,
            baseline,
            "NotoSans-Regular",
            font_size,
            display_text,
            text_color,
        );
        
        // Draw caret if focused
        if meta.focused {
            // Simple caret at end of text for now (cursor position handling to be added)
            let caret_x = text_x + (display_text.len() as i32 * 8); // Rough approximation
            let caret_h = (text_h - 4).max(0);
            let caret_y = text_y + 2;
            builder.fill_rect(caret_x, caret_y, 2, caret_h, 0xFF000000);
        }
    }
}

fn draw_window_chrome(
    scene: &SceneGraph,
    node: &SceneNode,
    rect: LayoutRect,
    window_bg: u32,
    title_override: Option<&str>,
    is_focused: bool,
    builder: &mut PaintBuilder,
) {
    let border = WINDOW_BORDER;
    let title_h = WINDOW_TITLE_HEIGHT;
    let chrome_color = 0xFF1E1E22;
    let title_color = 0xFF2A2A30;
    let border_color = 0xFF0A0A0F;

    // Windows 98 Gradient colors: Magenta to Boot Color
    let gradient_start = 0xFFFF00FF;
    let gradient_end = 0xFF2E7FD1; // Boot Screen Color

    builder.fill_rect(rect.x, rect.y, rect.w, rect.h, border_color);

    let inner_rect = LayoutRect {
        x: rect.x + border,
        y: rect.y + border,
        w: (rect.w - border * 2).max(0),
        h: (rect.h - border * 2).max(0),
    };
    builder.fill_rect(
        inner_rect.x,
        inner_rect.y,
        inner_rect.w,
        inner_rect.h,
        chrome_color,
    );

    let title_rect = LayoutRect {
        x: inner_rect.x,
        y: inner_rect.y,
        w: inner_rect.w,
        h: title_h.min(inner_rect.h),
    };
    if title_rect.h > 0 {
        if is_focused {
            builder.fill_linear_gradient(
                title_rect.x,
                title_rect.y,
                title_rect.w,
                title_rect.h,
                gradient_start,
                gradient_end,
            );
        } else {
            builder.fill_rect(
                title_rect.x,
                title_rect.y,
                title_rect.w,
                title_rect.h,
                title_color,
            );
        }
        let title = node
            .window_meta
            .and_then(|meta| scene.string(meta.title))
            .or(title_override);
        if let Some(text) = title {
            let baseline = title_rect.y + 14;
            builder.draw_text_run(
                title_rect.x + 8,
                title_rect.y + 4,
                (title_rect.w - 16).max(0),
                (title_rect.h - 8).max(0),
                baseline,
                "NotoSans-Regular",
                14,
                text,
                0xFFE6E6E6,
            );
        }
    }

    let content = window_content_rect(node, rect);
    if window_bg != 0 && content.w > 0 && content.h > 0 {
        builder.fill_rect(content.x, content.y, content.w, content.h, window_bg);
    }
}

fn window_content_rect(node: &SceneNode, rect: LayoutRect) -> LayoutRect {
    let x = rect.x + WINDOW_BORDER + node.padding.left;
    let y = rect.y + WINDOW_BORDER + WINDOW_TITLE_HEIGHT + node.padding.top;
    let w = rect
        .w
        .saturating_sub(WINDOW_BORDER * 2 + node.padding.left + node.padding.right);
    let h = rect.h.saturating_sub(
        WINDOW_BORDER * 2 + WINDOW_TITLE_HEIGHT + node.padding.top + node.padding.bottom,
    );
    LayoutRect {
        x,
        y,
        w: w.max(0),
        h: h.max(0),
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::scene::SceneGraph;
    use abi::ui_paint::{PaintOpTag, PaintReader};
    use abi::ui_scene::{EdgeInsets, NodeKind, SizeKind, SizeSpec, StringRef, TextMeta};

    #[test]
    fn emit_text_op() {
        let mut scene = SceneGraph {
            nodes: Vec::new(),
            strings: b"Hello".to_vec(),
            root: 0,
        };
        scene.nodes.push(SceneNode {
            id: 0,
            parent: None,
            children: Vec::new(),
            kind: NodeKind::Text,
            width: SizeSpec {
                kind: SizeKind::Auto,
                value: 0,
            },
            height: SizeSpec {
                kind: SizeKind::Auto,
                value: 0,
            },
            flex_basis: SizeSpec {
                kind: SizeKind::Auto,
                value: 0,
            },
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            margin: EdgeInsets {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            padding: EdgeInsets {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            flex_grow: 0.0,
            flex_shrink: 0.0,
            window_meta: None,
            flex_meta: None,
            text_meta: Some(TextMeta {
                text: StringRef { offset: 0, len: 5 },
                font: StringRef { offset: 0, len: 0 },
                size: 12,
                color: 0xFFFFFFFF,
            }),
            rect_meta: None,
            image_meta: None,
            checkbox_meta: None,
        });

        let layout = vec![LayoutRect {
            x: 0,
            y: 0,
            w: 100,
            h: 20,
        }];
        let bytes = emit_paint(&scene, &layout, 0, None, true);
        let mut reader = PaintReader::new(&bytes).expect("reader");
        let op = reader.next().expect("op");
        assert_eq!(op.tag, PaintOpTag::DrawTextRun);
    }
}
