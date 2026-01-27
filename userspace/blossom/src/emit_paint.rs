extern crate alloc;

use alloc::vec::Vec;

use abi::ui_paint::{ImageFit, PaintBuilder};
use abi::ui_scene::NodeKind;

use crate::layout::LayoutRect;
use crate::scene::{SceneGraph, SceneNode};

pub fn emit_paint(scene: &SceneGraph, layout: &[LayoutRect]) -> Vec<u8> {
    let mut builder = PaintBuilder::new();
    emit_node(scene, scene.root, layout, &mut builder);
    builder.finish()
}

fn emit_node(scene: &SceneGraph, index: usize, layout: &[LayoutRect], builder: &mut PaintBuilder) {
    let node = &scene.nodes[index];
    let rect = layout[index];

    match node.kind {
        NodeKind::Window => {
            builder.push_clip(rect.x, rect.y, rect.w, rect.h);
            for &child in &node.children {
                emit_node(scene, child, layout, builder);
            }
            builder.pop_clip();
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
        emit_node(scene, child, layout, builder);
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
        let bytes = emit_paint(&scene, &layout);
        let mut reader = PaintReader::new(&bytes).expect("reader");
        let op = reader.next().expect("op");
        assert_eq!(op.tag, PaintOpTag::DrawTextRun);
    }
}
