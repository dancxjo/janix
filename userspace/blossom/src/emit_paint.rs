extern crate alloc;

use alloc::vec::Vec;

use abi::ui_paint::{ImageFit, PaintBuilder};
use abi::ui_scene::NodeKind;

use crate::layout::{LayoutRect, WINDOW_BORDER, WINDOW_TITLE_HEIGHT};
use crate::scene::{SceneGraph, SceneNode};

pub fn emit_paint(
    scene: &SceneGraph,
    layout: &[LayoutRect],
    window_bg: u32,
    title_override: Option<&str>,
) -> Vec<u8> {
    let mut builder = PaintBuilder::new();
    emit_node(scene, scene.root, layout, window_bg, title_override, &mut builder);
    builder.finish()
}

fn emit_node(
    scene: &SceneGraph,
    index: usize,
    layout: &[LayoutRect],
    window_bg: u32,
    title_override: Option<&str>,
    builder: &mut PaintBuilder,
) {
    let node = &scene.nodes[index];
    let rect = layout[index];

    match node.kind {
        NodeKind::Window => {
            draw_window_chrome(scene, node, rect, window_bg, title_override, builder);
            let content = window_content_rect(node, rect);
            if content.w > 0 && content.h > 0 {
                builder.push_clip(content.x, content.y, content.w, content.h);
                for &child in &node.children {
                    emit_node(scene, child, layout, window_bg, title_override, builder);
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
                let font = scene.string(meta.font).unwrap_or("NotoSans-Regular");
                let size = if meta.size > 0 { meta.size } else { 16 };
                let baseline = rect.y + size;
                builder.draw_text_run(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    baseline,
                    font,
                    size,
                    text,
                    meta.color,
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
        NodeKind::Checkbox => {
            draw_checkbox(scene, node, rect, builder);
        }
        _ => {}
    }

    for &child in &node.children {
        emit_node(scene, child, layout, window_bg, title_override, builder);
    }
}

fn draw_checkbox(scene: &SceneGraph, node: &SceneNode, rect: LayoutRect, builder: &mut PaintBuilder) {
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

fn draw_window_chrome(
    scene: &SceneGraph,
    node: &SceneNode,
    rect: LayoutRect,
    window_bg: u32,
    title_override: Option<&str>,
    builder: &mut PaintBuilder,
) {
    let border = WINDOW_BORDER;
    let title_h = WINDOW_TITLE_HEIGHT;
    let chrome_color = 0xFF1E1E22;
    let title_color = 0xFF2A2A30;
    let border_color = 0xFF0A0A0F;

    builder.fill_rect(rect.x, rect.y, rect.w, rect.h, border_color);

    let inner_rect = LayoutRect {
        x: rect.x + border,
        y: rect.y + border,
        w: (rect.w - border * 2).max(0),
        h: (rect.h - border * 2).max(0),
    };
    builder.fill_rect(inner_rect.x, inner_rect.y, inner_rect.w, inner_rect.h, chrome_color);

    let title_rect = LayoutRect {
        x: inner_rect.x,
        y: inner_rect.y,
        w: inner_rect.w,
        h: title_h.min(inner_rect.h),
    };
    if title_rect.h > 0 {
        builder.fill_rect(title_rect.x, title_rect.y, title_rect.w, title_rect.h, title_color);
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
    use abi::ui_scene::{EdgeInsets, NodeKind, SizeKind, SizeSpec, TextMeta, StringRef};

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
            width: SizeSpec { kind: SizeKind::Auto, value: 0 },
            height: SizeSpec { kind: SizeKind::Auto, value: 0 },
            flex_basis: SizeSpec { kind: SizeKind::Auto, value: 0 },
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            margin: EdgeInsets { left: 0, top: 0, right: 0, bottom: 0 },
            padding: EdgeInsets { left: 0, top: 0, right: 0, bottom: 0 },
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

        let layout = vec![LayoutRect { x: 0, y: 0, w: 100, h: 20 }];
        let bytes = emit_paint(&scene, &layout, 0, None);
        let mut reader = PaintReader::new(&bytes).expect("reader");
        let op = reader.next().expect("op");
        assert_eq!(op.tag, PaintOpTag::DrawTextRun);
    }
}
