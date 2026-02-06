extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use abi::ui_scene::{AlignItems, FlexDirection, JustifyContent, NodeKind, SizeKind};

use crate::scene::{SceneGraph, SceneNode};

pub mod graph;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LayoutRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub const WINDOW_BORDER: i32 = 2;
pub const WINDOW_TITLE_HEIGHT: i32 = 24;

pub fn layout_scene(scene: &SceneGraph, root_rect: LayoutRect) -> Vec<LayoutRect> {
    let mut out = vec![LayoutRect::default(); scene.nodes.len()];
    layout_node(scene, scene.root, root_rect, &mut out);
    out
}

fn layout_node(scene: &SceneGraph, index: usize, rect: LayoutRect, out: &mut [LayoutRect]) {
    out[index] = clamp_rect(rect);
    let node = &scene.nodes[index];
    let content = content_rect(rect, node);
    if node.kind == NodeKind::Canvas {
        layout_canvas(scene, node, content, out);
        return;
    }
    if node.kind == NodeKind::Scroll {
        layout_scroll(scene, node, content, out);
        return;
    }
    if node.children.is_empty() {
        return;
    }

    if let Some(flex) = node.flex_meta {
        layout_flex(
            scene,
            node,
            flex.direction,
            flex.align,
            flex.justify,
            flex.gap,
            content,
            out,
        );
    } else {
        for child in &node.children {
            let child_rect = child_rect_in_content(scene, &scene.nodes[*child], content);
            layout_node(scene, *child, child_rect, out);
        }
    }
}

fn layout_scroll(
    scene: &SceneGraph,
    node: &SceneNode,
    content: LayoutRect,
    out: &mut [LayoutRect],
) {
    if node.children.is_empty() {
        return;
    }
    let mut child_content = content;
    if let Some(meta) = node.scroll_meta {
        if matches!(meta.axis, abi::ui_scene::ScrollAxis::Vertical) {
            if meta.content_min_height_px > content.h {
                child_content.h = meta.content_min_height_px;
            }
        } else if matches!(meta.axis, abi::ui_scene::ScrollAxis::Horizontal) {
            // Horizontal scroll not fully implemented but could use content_min_width if added
        }
    }

    for &child in &node.children {
        let child_rect = child_rect_in_content(scene, &scene.nodes[child], child_content);
        layout_node(scene, child, child_rect, out);
    }
}

fn layout_canvas(
    scene: &SceneGraph,
    node: &SceneNode,
    content: LayoutRect,
    out: &mut [LayoutRect],
) {
    for &child_idx in &node.children {
        let child = &scene.nodes[child_idx];
        if let Some(line) = child.line_meta {
            let min_x = line.x1.min(line.x2);
            let min_y = line.y1.min(line.y2);
            let max_x = line.x1.max(line.x2);
            let max_y = line.y1.max(line.y2);
            let half = (line.width.max(1) as i32 + 1) / 2;
            let rect = LayoutRect {
                x: content.x + min_x - half,
                y: content.y + min_y - half,
                w: (max_x - min_x).max(0) + half * 2,
                h: (max_y - min_y).max(0) + half * 2,
            };
            layout_node(scene, child_idx, rect, out);
            continue;
        }

        let width = resolve_size_spec(child.width, content.w)
            .unwrap_or_else(|| fallback_absolute_size(child, content.w, true));
        let height = resolve_size_spec(child.height, content.h)
            .unwrap_or_else(|| fallback_absolute_size(child, content.h, false));

        let rect = LayoutRect {
            x: content.x + child.margin.left,
            y: content.y + child.margin.top,
            w: width.max(0),
            h: height.max(0),
        };
        layout_node(scene, child_idx, rect, out);
    }
}

fn fallback_absolute_size(node: &SceneNode, container: i32, is_width: bool) -> i32 {
    match node.kind {
        NodeKind::Text => node.text_meta.as_ref().map(|t| t.size as i32).unwrap_or(16),
        NodeKind::Rect | NodeKind::Image => container,
        NodeKind::Icon => node.icon_meta.map(|m| m.size.max(0)).unwrap_or(24),
        NodeKind::Spacer => node.spacer_meta.map(|m| m.height_px as i32).unwrap_or(0),
        NodeKind::Separator => node
            .separator_meta
            .map(|m| m.thickness_px as i32)
            .unwrap_or(1),
        NodeKind::Checkbox => 20,
        NodeKind::TextInput => {
            if is_width {
                200 // Default width for text input
            } else {
                32 // Default height for text input
            }
        }
        _ => {
            if is_width {
                container
            } else {
                20
            }
        }
    }
}

fn layout_flex(
    scene: &SceneGraph,
    node: &SceneNode,
    direction: FlexDirection,
    align: AlignItems,
    justify: JustifyContent,
    gap: i32,
    content: LayoutRect,
    out: &mut [LayoutRect],
) {
    let is_row = matches!(direction, FlexDirection::Row);
    let main = if is_row { content.w } else { content.h };
    let cross = if is_row { content.h } else { content.w };

    let mut bases: Vec<i32> = Vec::with_capacity(node.children.len());
    let mut total_fixed = 0i32;
    let mut total_grow = 0.0f32;
    let mut total_shrink = 0.0f32;

    for &child_idx in &node.children {
        let child = &scene.nodes[child_idx];
        let base = resolve_main_size(child, main, cross, is_row);
        bases.push(base);
        total_fixed = total_fixed
            .saturating_add(base)
            .saturating_add(main_margin(child, is_row));
        total_grow += child.flex_grow;
        total_shrink += child.flex_shrink;
    }

    if !node.children.is_empty() {
        total_fixed =
            total_fixed.saturating_add(gap.saturating_mul((node.children.len() - 1) as i32));
    }

    let mut remaining = main.saturating_sub(total_fixed);
    let mut main_sizes = bases;

    if remaining > 0 && total_grow > 0.0 {
        for (i, &child_idx) in node.children.iter().enumerate() {
            let grow = scene.nodes[child_idx].flex_grow;
            if grow > 0.0 {
                let extra = ((remaining as f32 * (grow / total_grow)) + 0.5) as i32;
                main_sizes[i] = main_sizes[i].saturating_add(extra);
            }
        }
        remaining = 0;
    } else if remaining < 0 && total_shrink > 0.0 {
        let deficit = -remaining;
        for (i, &child_idx) in node.children.iter().enumerate() {
            let shrink = scene.nodes[child_idx].flex_shrink;
            if shrink > 0.0 {
                let cut = ((deficit as f32 * (shrink / total_shrink)) + 0.5) as i32;
                main_sizes[i] = (main_sizes[i] - cut).max(0);
            }
        }
        remaining = 0;
    }

    let total_main: i32 = main_sizes
        .iter()
        .enumerate()
        .map(|(i, size)| size.saturating_add(main_margin(&scene.nodes[node.children[i]], is_row)))
        .sum::<i32>()
        + gap.saturating_mul((node.children.len().saturating_sub(1)) as i32);

    let mut cursor = match justify {
        JustifyContent::Center => content_main_start(content, is_row) + (main - total_main) / 2,
        JustifyContent::End => content_main_start(content, is_row) + (main - total_main),
        JustifyContent::Start => content_main_start(content, is_row),
    };

    for (i, &child_idx) in node.children.iter().enumerate() {
        let child = &scene.nodes[child_idx];
        let main_lead = main_margin_leading(child, is_row);
        let main_trail = main_margin_trailing(child, is_row);
        let cross_lead = cross_margin_leading(child, is_row);
        let cross_trail = cross_margin_trailing(child, is_row);
        let child_main = main_sizes[i];

        let mut child_cross = resolve_cross_size(child, cross, align, is_row);
        if matches!(align, AlignItems::Stretch) {
            child_cross = (cross - cross_lead - cross_trail).max(0);
        }

        let main_pos = cursor + main_lead;
        let cross_pos = match align {
            AlignItems::Center => {
                content_cross_start(content, is_row) + (cross - child_cross) / 2 + cross_lead
                    - cross_trail
            }
            AlignItems::End => {
                content_cross_start(content, is_row) + (cross - child_cross) - cross_trail
            }
            _ => content_cross_start(content, is_row) + cross_lead,
        };

        let rect = if is_row {
            LayoutRect {
                x: main_pos,
                y: cross_pos,
                w: child_main.max(0),
                h: child_cross.max(0),
            }
        } else {
            LayoutRect {
                x: cross_pos,
                y: main_pos,
                w: child_cross.max(0),
                h: child_main.max(0),
            }
        };
        layout_node(scene, child_idx, rect, out);
        cursor = cursor
            .saturating_add(main_lead)
            .saturating_add(child_main)
            .saturating_add(main_trail)
            .saturating_add(gap);
    }
}

fn resolve_main_size(node: &SceneNode, main: i32, cross: i32, is_row: bool) -> i32 {
    let size_spec = if is_row { node.width } else { node.height };
    let basis_spec = node.flex_basis;
    let mut size = resolve_size_spec(size_spec, main)
        .or_else(|| resolve_size_spec(basis_spec, main))
        .unwrap_or_else(|| fallback_main_size(node, main, cross, is_row));

    if let Some(min) = if is_row {
        node.min_width
    } else {
        node.min_height
    } {
        size = size.max(min);
    }
    if let Some(max) = if is_row {
        node.max_width
    } else {
        node.max_height
    } {
        size = size.min(max);
    }
    size.max(0)
}

fn resolve_cross_size(node: &SceneNode, cross: i32, align: AlignItems, is_row: bool) -> i32 {
    let size_spec = if is_row { node.height } else { node.width };
    let mut size = resolve_size_spec(size_spec, cross)
        .unwrap_or_else(|| fallback_cross_size(node, cross, align));
    if let Some(min) = if is_row {
        node.min_height
    } else {
        node.min_width
    } {
        size = size.max(min);
    }
    if let Some(max) = if is_row {
        node.max_height
    } else {
        node.max_width
    } {
        size = size.min(max);
    }
    size.max(0)
}

fn resolve_size_spec(spec: abi::ui_scene::SizeSpec, container: i32) -> Option<i32> {
    match spec.kind {
        SizeKind::Px => Some(spec.value),
        SizeKind::Pct => Some(container.saturating_mul(spec.value) / 100),
        SizeKind::Auto => None,
    }
}

fn fallback_main_size(node: &SceneNode, main: i32, cross: i32, is_row: bool) -> i32 {
    match node.kind {
        NodeKind::Text => node.text_meta.as_ref().map(|t| t.size as i32).unwrap_or(16),
        NodeKind::Rect | NodeKind::Image => main,
        NodeKind::Icon => node.icon_meta.map(|m| m.size.max(0)).unwrap_or(24),
        NodeKind::Spacer => node.spacer_meta.map(|m| m.height_px as i32).unwrap_or(0),
        NodeKind::Separator => node
            .separator_meta
            .map(|m| m.thickness_px as i32)
            .unwrap_or(1),
        NodeKind::Flex
        | NodeKind::Scroll
        | NodeKind::Canvas
        | NodeKind::Window
        | NodeKind::Checkbox
        | NodeKind::TextInput => {
            if is_row {
                cross
            } else {
                main
            }
        }
        _ => 0,
    }
}

fn fallback_cross_size(node: &SceneNode, cross: i32, align: AlignItems) -> i32 {
    match node.kind {
        NodeKind::Text => cross,
        NodeKind::Rect | NodeKind::Image => cross,
        NodeKind::Icon => node.icon_meta.map(|m| m.size.max(0)).unwrap_or(24),
        NodeKind::Spacer => 0,
        NodeKind::Separator => cross,
        NodeKind::Flex
        | NodeKind::Scroll
        | NodeKind::Canvas
        | NodeKind::Window
        | NodeKind::Checkbox
        | NodeKind::TextInput => cross,
        _ => 0,
    }
}

fn main_margin(node: &SceneNode, is_row: bool) -> i32 {
    if is_row {
        node.margin.left + node.margin.right
    } else {
        node.margin.top + node.margin.bottom
    }
}

fn main_margin_leading(node: &SceneNode, is_row: bool) -> i32 {
    if is_row {
        node.margin.left
    } else {
        node.margin.top
    }
}

fn main_margin_trailing(node: &SceneNode, is_row: bool) -> i32 {
    if is_row {
        node.margin.right
    } else {
        node.margin.bottom
    }
}

fn cross_margin_leading(node: &SceneNode, is_row: bool) -> i32 {
    if is_row {
        node.margin.top
    } else {
        node.margin.left
    }
}

fn cross_margin_trailing(node: &SceneNode, is_row: bool) -> i32 {
    if is_row {
        node.margin.bottom
    } else {
        node.margin.right
    }
}

fn content_rect(rect: LayoutRect, node: &SceneNode) -> LayoutRect {
    if matches!(node.kind, NodeKind::Window) {
        let x = rect.x + WINDOW_BORDER + node.padding.left;
        let y = rect.y + WINDOW_BORDER + WINDOW_TITLE_HEIGHT + node.padding.top;
        let w = rect
            .w
            .saturating_sub(WINDOW_BORDER * 2 + node.padding.left + node.padding.right);
        let h = rect.h.saturating_sub(
            WINDOW_BORDER * 2 + WINDOW_TITLE_HEIGHT + node.padding.top + node.padding.bottom,
        );
        return LayoutRect {
            x,
            y,
            w: w.max(0),
            h: h.max(0),
        };
    }
    LayoutRect {
        x: rect.x + node.padding.left,
        y: rect.y + node.padding.top,
        w: (rect.w - node.padding.left - node.padding.right).max(0),
        h: (rect.h - node.padding.top - node.padding.bottom).max(0),
    }
}

fn child_rect_in_content(
    _scene: &SceneGraph,
    child: &SceneNode,
    content: LayoutRect,
) -> LayoutRect {
    let mut rect = LayoutRect {
        x: content.x + child.margin.left,
        y: content.y + child.margin.top,
        w: (content.w - child.margin.left - child.margin.right).max(0),
        h: (content.h - child.margin.top - child.margin.bottom).max(0),
    };

    let width = resolve_size_spec(child.width, content.w);
    let height = resolve_size_spec(child.height, content.h);
    if let Some(w) = width {
        rect.w = w.max(0);
    }
    if let Some(h) = height {
        rect.h = h.max(0);
    }
    rect
}

fn clamp_rect(rect: LayoutRect) -> LayoutRect {
    LayoutRect {
        x: rect.x,
        y: rect.y,
        w: rect.w.max(0),
        h: rect.h.max(0),
    }
}

fn content_main_start(content: LayoutRect, is_row: bool) -> i32 {
    if is_row {
        content.x
    } else {
        content.y
    }
}

fn content_cross_start(content: LayoutRect, is_row: bool) -> i32 {
    if is_row {
        content.y
    } else {
        content.x
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::scene::{SceneGraph, SceneNode};
    use abi::ui_scene::{EdgeInsets, FlexMeta, SizeKind, SizeSpec};

    #[test]
    fn flex_column_centers_child() {
        let mut scene = SceneGraph {
            nodes: Vec::new(),
            strings: Vec::new(),
            root: 0,
        };
        scene.nodes.push(SceneNode {
            id: 0,
            parent: None,
            children: vec![1],
            kind: NodeKind::Flex,
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
            flex_meta: Some(FlexMeta {
                direction: FlexDirection::Column,
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                gap: 0,
            }),
            text_meta: None,
            rect_meta: None,
            image_meta: None,
            checkbox_meta: None,
            text_input_meta: None,
            button_meta: None,
            label_meta: None,
            message_box_meta: None,
            line_meta: None,
            icon_meta: None,
            scroll_meta: None,
            spacer_meta: None,
            separator_meta: None,
        });
        scene.nodes.push(SceneNode {
            id: 1,
            parent: Some(0),
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
            text_meta: None,
            rect_meta: None,
            image_meta: None,
            checkbox_meta: None,
            text_input_meta: None,
            button_meta: None,
            label_meta: None,
            message_box_meta: None,
            line_meta: None,
            icon_meta: None,
            scroll_meta: None,
            spacer_meta: None,
            separator_meta: None,
        });

        let rects = layout_scene(
            &scene,
            LayoutRect {
                x: 0,
                y: 0,
                w: 200,
                h: 100,
            },
        );
        assert_eq!(rects[0].w, 200);
        assert!(rects[1].y >= 0);
    }

    fn default_node(id: u32, kind: NodeKind) -> SceneNode {
        SceneNode {
            id,
            parent: None,
            children: Vec::new(),
            kind,
            width: SizeSpec { kind: SizeKind::Auto, value: 0 },
            height: SizeSpec { kind: SizeKind::Auto, value: 0 },
            flex_basis: SizeSpec { kind: SizeKind::Auto, value: 0 },
            min_width: None, min_height: None, max_width: None, max_height: None,
            margin: EdgeInsets { left: 0, top: 0, right: 0, bottom: 0 },
            padding: EdgeInsets { left: 0, top: 0, right: 0, bottom: 0 },
            flex_grow: 0.0,
            flex_shrink: 0.0,
            window_meta: None, flex_meta: None, text_meta: None, rect_meta: None,
            image_meta: None, line_meta: None, icon_meta: None, scroll_meta: None,
            spacer_meta: None, separator_meta: None, checkbox_meta: None,
            text_input_meta: None, button_meta: None, label_meta: None, message_box_meta: None,
        }
    }

    #[test]
    fn flex_row_center_center_exact() {
        let mut scene = SceneGraph {
            nodes: Vec::new(),
            strings: Vec::new(),
            root: 0,
        };

        // Root: Flex Row, 200x200, Center/Center
        let mut root = default_node(0, NodeKind::Flex);
        root.children = vec![1];
        root.width = SizeSpec { kind: SizeKind::Px, value: 200 };
        root.height = SizeSpec { kind: SizeKind::Px, value: 200 };
        root.flex_meta = Some(FlexMeta {
            direction: FlexDirection::Row,
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            gap: 0,
        });
        scene.nodes.push(root);

        // Child: Fixed 50x50
        let mut child = default_node(1, NodeKind::Rect);
        child.parent = Some(0);
        child.width = SizeSpec { kind: SizeKind::Px, value: 50 };
        child.height = SizeSpec { kind: SizeKind::Px, value: 50 };
        scene.nodes.push(child);

        let rects = layout_scene(
            &scene,
            LayoutRect {
                x: 0,
                y: 0,
                w: 200,
                h: 200,
            },
        );

        // Expected: (200-50)/2 = 75
        let child_rect = rects[1];
        assert_eq!(child_rect.x, 75, "Child X should be centered (75)");
        assert_eq!(child_rect.y, 75, "Child Y should be centered (75)");
        assert_eq!(child_rect.w, 50, "Child width should be 50");
        assert_eq!(child_rect.h, 50, "Child height should be 50");
    }
}
