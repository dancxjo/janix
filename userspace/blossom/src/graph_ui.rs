extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use abi::ids::HandleId;
use abi::schema::{keys, kinds, rels, ui_kind};
use abi::types::Edge;
use stem::thing::ThingId;

use abi::ui_paint::PaintBuilder;
use alloc::string::ToString;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiNodeKind {
    Column,
    Button,
    Checkbox,
    Text,
    TextInput,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct UiNode {
    pub id: ThingId,
    pub kind: UiNodeKind,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub label_id: Option<ThingId>,
    pub text: Option<String>,
    pub placeholder: Option<String>,
    pub checked: bool,
    pub pressed: bool,
    pub action_id: u64,
    pub value_id: u64,
    pub focused: bool,
    pub cursor: u32,
    pub visible: bool,
    pub enabled: bool,
}

#[derive(Clone, Debug)]
pub struct UiTree {
    pub nodes: Vec<UiNode>,
    pub root: usize,
}

pub struct UiSymbols {
    pub rel_has_child: u64,
    pub rel_root_ui: u64,
    pub kind_button: u64,
    pub kind_checkbox: u64,
    pub kind_text: u64,
    pub kind_column: u64,
}

impl UiSymbols {
    pub fn intern_sys() -> Self {
        Self {
            rel_has_child: stem::thing::sys::intern(rels::HAS_CHILD).unwrap_or(0) as u64,
            rel_root_ui: stem::thing::sys::intern(rels::ROOT_UI).unwrap_or(0) as u64,
            kind_button: stem::thing::sys::intern(kinds::UI_BUTTON).unwrap_or(0) as u64,
            kind_checkbox: stem::thing::sys::intern(kinds::UI_CHECKBOX).unwrap_or(0) as u64,
            kind_text: stem::thing::sys::intern(kinds::UI_TEXT).unwrap_or(0) as u64,
            kind_column: stem::thing::sys::intern(kinds::UI_COLUMN).unwrap_or(0) as u64,
        }
    }
}

pub trait UiGraph {
    fn get_kind(&self, id: ThingId) -> Option<u64>;
    fn get_prop(&self, id: ThingId, key: &str) -> Option<u64>;
    fn set_prop(&mut self, id: ThingId, key: &str, val: u64);
    fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> usize;
    fn read_bytespace(&self, id: ThingId) -> Option<Vec<u8>>;
}

pub struct SysGraph;

impl UiGraph for SysGraph {
    fn get_kind(&self, id: ThingId) -> Option<u64> {
        stem::thing::sys::get_kind(id).ok().map(|k| k.0)
    }

    fn get_prop(&self, id: ThingId, key: &str) -> Option<u64> {
        stem::thing::sys::prop_get(id, key).ok()
    }

    fn set_prop(&mut self, id: ThingId, key: &str, val: u64) {
        let _ = stem::thing::sys::prop_set(id, key, val);
    }

    fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> usize {
        stem::thing::sys::get_edges(id, out).unwrap_or(0)
    }

    fn read_bytespace(&self, id: ThingId) -> Option<Vec<u8>> {
        crate::read_bytespace(id).ok()
    }
}

pub fn find_root_ui(
    graph: &impl UiGraph,
    symbols: &UiSymbols,
    window_id: ThingId,
) -> Option<ThingId> {
    let mut edges = [Edge::default(); 64];
    let count = graph.get_edges(window_id, &mut edges);
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() == symbols.rel_root_ui {
            return Some(edge.to);
        }
    }
    None
}

pub fn build_tree(graph: &impl UiGraph, symbols: &UiSymbols, root_id: ThingId) -> Option<UiTree> {
    let mut nodes = Vec::new();
    let mut map: BTreeMap<ThingId, usize> = BTreeMap::new();
    let mut stack = Vec::new();
    stack.push((root_id, None));

    while let Some((id, parent)) = stack.pop() {
        if map.contains_key(&id) {
            continue;
        }
        let kind = graph.get_kind(id)?;
        let ui_kind_value = graph.get_prop(id, keys::UI_KIND).unwrap_or(0);
        let node_kind = if kind == symbols.kind_button {
            UiNodeKind::Button
        } else if kind == symbols.kind_checkbox {
            UiNodeKind::Checkbox
        } else if kind == symbols.kind_text {
            UiNodeKind::Text
        } else if kind == symbols.kind_column {
            UiNodeKind::Column
        } else if ui_kind_value == ui_kind::TEXT_INPUT {
            UiNodeKind::TextInput
        } else {
            UiNodeKind::Unknown
        };

        let visible = graph.get_prop(id, keys::UI_VISIBLE).unwrap_or(1) != 0;
        let enabled = graph.get_prop(id, keys::UI_ENABLED).unwrap_or(1) != 0;
        let checked = graph.get_prop(id, keys::UI_CHECKBOX_CHECKED).unwrap_or(0) != 0;
        let action_id = graph.get_prop(id, keys::UI_BUTTON_ACTION_ID).unwrap_or(0);
        let pressed = graph.get_prop(id, keys::UI_BUTTON_PRESSED).unwrap_or(0) != 0;
        let value_id = graph.get_prop(id, keys::UI_CHECKBOX_VALUE_ID).unwrap_or(0);
        let label_id = graph
            .get_prop(id, keys::UI_BUTTON_LABEL)
            .or_else(|| graph.get_prop(id, keys::UI_CHECKBOX_LABEL))
            .and_then(|val| {
                if val == 0 {
                    None
                } else {
                    Some(ThingId::from_u64(val))
                }
            });

        let text = if node_kind == UiNodeKind::Text || node_kind == UiNodeKind::TextInput {
            let bs = graph
                .get_prop(id, keys::UI_TEXT)
                .or_else(|| graph.get_prop(id, keys::UI_INPUT_VALUE))
                .unwrap_or(0);
            if bs != 0 {
                let bytes = graph.read_bytespace(ThingId::from_u64(bs))?;
                core::str::from_utf8(&bytes).ok().map(|s| s.to_string())
            } else {
                None
            }
        } else {
            None
        };
        let placeholder = if node_kind == UiNodeKind::TextInput {
            let bs = graph
                .get_prop(id, keys::UI_PLACEHOLDER_TEXT)
                .or_else(|| graph.get_prop(id, keys::UI_PLACEHOLDER))
                .unwrap_or(0);
            if bs != 0 {
                let bytes = graph.read_bytespace(ThingId::from_u64(bs))?;
                core::str::from_utf8(&bytes).ok().map(|s| s.to_string())
            } else {
                None
            }
        } else {
            None
        };
        let focused = graph.get_prop(id, keys::UI_FOCUSED).unwrap_or(0) != 0;
        let cursor = graph
            .get_prop(id, keys::UI_CURSOR)
            .or_else(|| graph.get_prop(id, keys::UI_CURSOR_POS))
            .unwrap_or(0) as u32;

        let index = nodes.len();
        nodes.push(UiNode {
            id,
            kind: node_kind,
            parent,
            children: Vec::new(),
            label_id,
            text,
            placeholder,
            checked,
            pressed,
            action_id,
            value_id,
            focused,
            cursor,
            visible,
            enabled,
        });
        map.insert(id, index);

        let mut edges = [Edge::default(); 64];
        let count = graph.get_edges(id, &mut edges);
        for edge in edges.iter().take(count) {
            if edge.predicate.to_u64_lossy() == symbols.rel_has_child {
                stack.push((edge.to, Some(index)));
            }
        }
    }

    for idx in 0..nodes.len() {
        if let Some(parent) = nodes[idx].parent {
            nodes[parent].children.push(idx);
        }
    }

    let root = map.get(&root_id).copied()?;
    Some(UiTree { nodes, root })
}

pub fn layout_tree(tree: &UiTree, root_rect: LayoutRect) -> Vec<LayoutRect> {
    let mut rects = vec![LayoutRect::default(); tree.nodes.len()];
    layout_node(tree, tree.root, root_rect, &mut rects);
    rects
}

pub fn write_bounds(graph: &mut impl UiGraph, tree: &UiTree, rects: &[LayoutRect]) {
    for (idx, rect) in rects.iter().enumerate() {
        let id = tree.nodes[idx].id;
        graph.set_prop(id, keys::UI_X, rect.x as u64);
        graph.set_prop(id, keys::UI_Y, rect.y as u64);
        graph.set_prop(id, keys::UI_WIDTH, rect.w as u64);
        graph.set_prop(id, keys::UI_HEIGHT, rect.h as u64);
    }
}

pub fn emit_paint(
    tree: &UiTree,
    rects: &[LayoutRect],
    window_bg: u32,
    _is_focused: bool,
) -> Vec<u8> {
    let mut builder = PaintBuilder::new();
    let root_rect = rects[tree.root];
    builder.fill_rect(
        root_rect.x,
        root_rect.y,
        root_rect.w,
        root_rect.h,
        window_bg,
    );
    emit_node(tree, rects, tree.root, &mut builder);
    builder.finish()
}

const COLUMN_PADDING: i32 = 8;
const COLUMN_GAP: i32 = 8;
const BUTTON_HEIGHT: i32 = 32;
const CHECKBOX_HEIGHT: i32 = 28;
const TEXT_HEIGHT: i32 = 20;
const TEXT_INPUT_HEIGHT: i32 = 32;
const CHECKBOX_BOX: i32 = 16;
const BUTTON_BG: u32 = 0xFFC0C0C0;
const BUTTON_BG_PRESSED: u32 = 0xFFB0B0B0;
const BUTTON_BG_DISABLED: u32 = 0xFF808080;
const BUTTON_BORDER: u32 = 0xFF202020;
const TEXT_COLOR: u32 = 0xFF101010;
const CHECKBOX_BORDER: u32 = 0xFF202020;
const CHECKBOX_FILL: u32 = 0xFFFFFFFF;
const INPUT_BG: u32 = 0xFFFFFFFF;
const INPUT_BORDER: u32 = 0xFF303030;
const INPUT_BORDER_FOCUS: u32 = 0xFF1A73E8;
const INPUT_PLACEHOLDER: u32 = 0xFF6E6E6E;

fn layout_node(tree: &UiTree, index: usize, rect: LayoutRect, rects: &mut [LayoutRect]) {
    rects[index] = rect;
    let node = &tree.nodes[index];
    if node.children.is_empty() {
        return;
    }
    if node.kind == UiNodeKind::Column {
        layout_column(tree, index, rect, rects);
    } else if node.kind == UiNodeKind::Button {
        let label_rect = LayoutRect {
            x: rect.x + 8,
            y: rect.y,
            w: rect.w.saturating_sub(16),
            h: rect.h,
        };
        for &child in &node.children {
            rects[child] = label_rect;
        }
    } else if node.kind == UiNodeKind::Checkbox {
        let box_rect = checkbox_box_rect(rect);
        let label_rect = LayoutRect {
            x: box_rect.x + box_rect.w + 8,
            y: rect.y,
            w: rect.w.saturating_sub(box_rect.w + 8),
            h: rect.h,
        };
        for &child in &node.children {
            rects[child] = label_rect;
        }
    }
}

fn layout_column(tree: &UiTree, index: usize, rect: LayoutRect, rects: &mut [LayoutRect]) {
    let mut y = rect.y + COLUMN_PADDING;
    let x = rect.x + COLUMN_PADDING;
    let w = (rect.w - COLUMN_PADDING * 2).max(0);
    for &child in &tree.nodes[index].children {
        let child_kind = tree.nodes[child].kind;
        let h = match child_kind {
            UiNodeKind::Button => BUTTON_HEIGHT,
            UiNodeKind::Checkbox => CHECKBOX_HEIGHT,
            UiNodeKind::Text => TEXT_HEIGHT,
            UiNodeKind::TextInput => TEXT_INPUT_HEIGHT,
            UiNodeKind::Column => BUTTON_HEIGHT,
            UiNodeKind::Unknown => TEXT_HEIGHT,
        };
        let child_rect = LayoutRect { x, y, w, h };
        rects[child] = child_rect;
        layout_node(tree, child, child_rect, rects);
        y = y.saturating_add(h).saturating_add(COLUMN_GAP);
    }
}

fn emit_node(tree: &UiTree, rects: &[LayoutRect], index: usize, builder: &mut PaintBuilder) {
    let node = &tree.nodes[index];
    if !node.visible {
        return;
    }
    let rect = rects[index];
    match node.kind {
        UiNodeKind::Button => draw_button(tree, rects, index, rect, builder),
        UiNodeKind::Checkbox => draw_checkbox(tree, rects, index, rect, builder),
        UiNodeKind::TextInput => draw_text_input(node, rect, builder),
        UiNodeKind::Text => {
            if !is_label_child(tree, index) {
                if let Some(text) = &node.text {
                    draw_text(rect, text, builder);
                }
            }
        }
        _ => {}
    }
    for &child in &node.children {
        emit_node(tree, rects, child, builder);
    }
}

fn draw_button(
    tree: &UiTree,
    rects: &[LayoutRect],
    index: usize,
    rect: LayoutRect,
    builder: &mut PaintBuilder,
) {
    let node = &tree.nodes[index];
    let bg = if !node.enabled {
        BUTTON_BG_DISABLED
    } else if node.pressed {
        BUTTON_BG_PRESSED
    } else {
        BUTTON_BG
    };
    builder.fill_rect(rect.x, rect.y, rect.w, rect.h, bg);
    // Border
    builder.fill_rect(rect.x, rect.y, rect.w, 1, BUTTON_BORDER);
    builder.fill_rect(rect.x, rect.y + rect.h - 1, rect.w, 1, BUTTON_BORDER);
    builder.fill_rect(rect.x, rect.y, 1, rect.h, BUTTON_BORDER);
    builder.fill_rect(rect.x + rect.w - 1, rect.y, 1, rect.h, BUTTON_BORDER);

    if let Some(label_id) = node.label_id {
        if let Some((_label_idx, text)) = label_text(tree, label_id) {
            let text_rect = LayoutRect {
                x: rect.x + 8,
                y: rect.y,
                w: rect.w.saturating_sub(16),
                h: rect.h,
            };
            draw_text(text_rect, text, builder);
        }
    }
}

fn draw_checkbox(
    tree: &UiTree,
    rects: &[LayoutRect],
    index: usize,
    rect: LayoutRect,
    builder: &mut PaintBuilder,
) {
    let node = &tree.nodes[index];
    let box_rect = checkbox_box_rect(rect);
    builder.fill_rect(
        box_rect.x,
        box_rect.y,
        box_rect.w,
        box_rect.h,
        CHECKBOX_FILL,
    );
    builder.fill_rect(box_rect.x, box_rect.y, box_rect.w, 1, CHECKBOX_BORDER);
    builder.fill_rect(
        box_rect.x,
        box_rect.y + box_rect.h - 1,
        box_rect.w,
        1,
        CHECKBOX_BORDER,
    );
    builder.fill_rect(box_rect.x, box_rect.y, 1, box_rect.h, CHECKBOX_BORDER);
    builder.fill_rect(
        box_rect.x + box_rect.w - 1,
        box_rect.y,
        1,
        box_rect.h,
        CHECKBOX_BORDER,
    );

    if node.checked {
        let x1 = box_rect.x + 3;
        let y1 = box_rect.y + box_rect.h / 2;
        let x2 = box_rect.x + box_rect.w / 2;
        let y2 = box_rect.y + box_rect.h - 4;
        let x3 = box_rect.x + box_rect.w - 3;
        let y3 = box_rect.y + 3;
        builder.stroke_line(x1, y1, x2, y2, 2, TEXT_COLOR);
        builder.stroke_line(x2, y2, x3, y3, 2, TEXT_COLOR);
    }

    if let Some(label_id) = node.label_id {
        if let Some((_label_idx, text)) = label_text(tree, label_id) {
            let text_rect = LayoutRect {
                x: box_rect.x + box_rect.w + 8,
                y: rect.y,
                w: rect.w.saturating_sub(box_rect.w + 8),
                h: rect.h,
            };
            draw_text(text_rect, text, builder);
        }
    }
}

fn draw_text_input(node: &UiNode, rect: LayoutRect, builder: &mut PaintBuilder) {
    let border = if node.focused {
        INPUT_BORDER_FOCUS
    } else {
        INPUT_BORDER
    };
    builder.fill_rect(rect.x, rect.y, rect.w, rect.h, INPUT_BG);
    builder.fill_rect(rect.x, rect.y, rect.w, 1, border);
    builder.fill_rect(rect.x, rect.y + rect.h - 1, rect.w, 1, border);
    builder.fill_rect(rect.x, rect.y, 1, rect.h, border);
    builder.fill_rect(rect.x + rect.w - 1, rect.y, 1, rect.h, border);

    let text_rect = LayoutRect {
        x: rect.x + 8,
        y: rect.y,
        w: rect.w.saturating_sub(16),
        h: rect.h,
    };
    let content = node.text.as_deref().unwrap_or("");
    if !content.is_empty() {
        draw_text(text_rect, content, builder);
    } else if let Some(placeholder) = node.placeholder.as_deref() {
        draw_text_colored(text_rect, placeholder, INPUT_PLACEHOLDER, builder);
    }
    if node.focused {
        let cursor = core::cmp::min(node.cursor as usize, content.len()) as i32;
        let cursor_x = text_rect.x + cursor.saturating_mul(8);
        let cursor_h = (rect.h - 10).max(1);
        builder.fill_rect(cursor_x, rect.y + 5, 1, cursor_h, TEXT_COLOR);
    }
}

fn draw_text(rect: LayoutRect, text: &str, builder: &mut PaintBuilder) {
    draw_text_colored(rect, text, TEXT_COLOR, builder);
}

fn draw_text_colored(rect: LayoutRect, text: &str, color: u32, builder: &mut PaintBuilder) {
    let size = 16;
    let baseline = rect.y + (rect.h + size) / 2 - 2;
    builder.draw_text_run(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        baseline,
        "NotoSans-Regular",
        size,
        text,
        color,
    );
}

fn label_text<'a>(tree: &'a UiTree, label_id: ThingId) -> Option<(usize, &'a str)> {
    for (idx, node) in tree.nodes.iter().enumerate() {
        if node.id == label_id {
            return node.text.as_deref().map(|t| (idx, t));
        }
    }
    None
}

fn is_label_child(tree: &UiTree, index: usize) -> bool {
    if let Some(parent_idx) = tree.nodes[index].parent {
        let parent = &tree.nodes[parent_idx];
        parent.kind == UiNodeKind::Button || parent.kind == UiNodeKind::Checkbox
    } else {
        false
    }
}

pub fn checkbox_box_rect(rect: LayoutRect) -> LayoutRect {
    let size = CHECKBOX_BOX.min(rect.h).max(0);
    LayoutRect {
        x: rect.x,
        y: rect.y + (rect.h - size) / 2,
        w: size,
        h: size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::errors::Errno;
    use abi::ui_event::{self, UiEvent, UiEventKind};
    use abi::ui_paint::PaintReader;
    use alloc::collections::BTreeMap;
    use alloc::string::String;
    use stem::errors::{Error, Result};
    use stem::petals::graph::{GraphBackend, UiTreeBuilder};

    #[derive(Default)]
    struct TestGraph {
        next_id: u64,
        props: BTreeMap<(u64, String), u64>,
        edges: Vec<Edge>,
        bytespaces: BTreeMap<u64, Vec<u8>>,
        kinds: BTreeMap<u64, u64>,
    }

    impl TestGraph {
        fn new() -> Self {
            Self {
                next_id: 1,
                ..Default::default()
            }
        }
    }

    impl GraphBackend for TestGraph {
        fn create_node(&mut self, kind: &str) -> Result<ThingId> {
            let id = ThingId::from_u64(self.next_id);
            self.next_id += 1;
            let kind_id = match kind {
                kinds::UI_BUTTON => 101,
                kinds::UI_CHECKBOX => 102,
                kinds::UI_TEXT => 103,
                kinds::UI_COLUMN => 104,
                _ => 1,
            };
            self.kinds.insert(id.to_u64_lossy(), kind_id);
            Ok(id)
        }

        fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<()> {
            let rel_id = match rel {
                rels::HAS_CHILD => 201,
                rels::CHILD_OF => 202,
                rels::ROOT_UI => 203,
                _ => 0,
            };
            self.edges.push(Edge {
                from: src,
                predicate: ThingId::from_u64(rel_id),
                to: dst,
                flags: 0,
            });
            Ok(())
        }

        fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> Result<()> {
            self.props
                .insert((id.to_u64_lossy(), key.to_string()), value);
            Ok(())
        }

        fn prop_get(&mut self, id: ThingId, key: &str) -> Result<u64> {
            Ok(self
                .props
                .get(&(id.to_u64_lossy(), key.to_string()))
                .copied()
                .unwrap_or(0))
        }

        fn get_edges(&mut self, id: ThingId, out: &mut [Edge]) -> Result<usize> {
            let mut count = 0usize;
            for edge in &self.edges {
                if edge.from == id && count < out.len() {
                    out[count] = *edge;
                    count += 1;
                }
            }
            Ok(count)
        }

        fn bytespace_create(&mut self, len: usize) -> Result<ThingId> {
            let id = ThingId::from_u64(self.next_id);
            self.next_id += 1;
            self.bytespaces.insert(id.to_u64_lossy(), vec![0u8; len]);
            Ok(id)
        }

        fn bytespace_info(&mut self, id: ThingId) -> Result<usize> {
            Ok(self
                .bytespaces
                .get(&id.to_u64_lossy())
                .map(|b| b.len())
                .unwrap_or(0))
        }

        fn bytespace_read(&mut self, id: ThingId, offset: usize, out: &mut [u8]) -> Result<usize> {
            if let Some(buf) = self.bytespaces.get(&id.to_u64_lossy()) {
                if offset >= buf.len() {
                    return Ok(0);
                }
                let n = core::cmp::min(out.len(), buf.len() - offset);
                out[..n].copy_from_slice(&buf[offset..offset + n]);
                Ok(n)
            } else {
                Err(Error::Errno(Errno::ENOENT))
            }
        }

        fn bytespace_write(&mut self, id: ThingId, offset: usize, bytes: &[u8]) -> Result<()> {
            if let Some(buf) = self.bytespaces.get_mut(&id.to_u64_lossy()) {
                let end = offset + bytes.len();
                buf[offset..end].copy_from_slice(bytes);
                Ok(())
            } else {
                Err(Error::Errno(Errno::ENOENT))
            }
        }
    }

    impl UiGraph for TestGraph {
        fn get_kind(&self, id: ThingId) -> Option<u64> {
            self.kinds.get(&id.to_u64_lossy()).copied()
        }

        fn get_prop(&self, id: ThingId, key: &str) -> Option<u64> {
            self.props
                .get(&(id.to_u64_lossy(), key.to_string()))
                .copied()
        }

        fn set_prop(&mut self, id: ThingId, key: &str, val: u64) {
            self.props.insert((id.to_u64_lossy(), key.to_string()), val);
        }

        fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> usize {
            let mut count = 0;
            for edge in &self.edges {
                if edge.from == id && count < out.len() {
                    out[count] = *edge;
                    count += 1;
                }
            }
            count
        }

        fn read_bytespace(&self, id: ThingId) -> Option<Vec<u8>> {
            self.bytespaces.get(&id.to_u64_lossy()).cloned()
        }
    }

    #[test]
    fn column_layout_orders_children() {
        let graph = TestGraph::new();
        let mut builder = UiTreeBuilder::new(graph, ThingId::from_u64(99));
        builder
            .column(|b| {
                b.checkbox("One", 1, false)?;
                b.checkbox("Two", 2, true)?;
                b.button("Go", 9)?;
                Ok(())
            })
            .unwrap();
        let (root, mut graph) = builder.finish_with_graph().unwrap();

        let symbols = UiSymbols {
            rel_has_child: 201,
            rel_root_ui: 203,
            kind_button: 101,
            kind_checkbox: 102,
            kind_text: 103,
            kind_column: 104,
        };
        let tree = build_tree(&graph, &symbols, root).unwrap();
        let rects = layout_tree(
            &tree,
            LayoutRect {
                x: 0,
                y: 0,
                w: 200,
                h: 200,
            },
        );
        let child_rects: Vec<LayoutRect> = tree.nodes[tree.root]
            .children
            .iter()
            .map(|i| rects[*i])
            .collect();
        assert!(child_rects[0].y < child_rects[1].y);
        assert!(child_rects[1].y < child_rects[2].y);

        let total_height = child_rects.iter().map(|r| r.h).sum::<i32>()
            + COLUMN_GAP * (child_rects.len() as i32 - 1)
            + COLUMN_PADDING * 2;
        assert!(total_height <= 200);

        let box_rect = checkbox_box_rect(child_rects[0]);
        assert!(box_rect.y >= child_rects[0].y);
        assert!(box_rect.y + box_rect.h <= child_rects[0].y + child_rects[0].h);

        write_bounds(&mut graph, &tree, &rects);
        let x = graph
            .get_prop(tree.nodes[tree.root].id, keys::UI_X)
            .unwrap_or(1);
        assert_eq!(x, 0);
    }

    #[test]
    fn graph_roundtrip_emits_event() {
        let mut graph = TestGraph::new();
        let window_id = ThingId::from_u64(99);
        let mut builder = UiTreeBuilder::new(graph, window_id);
        let root = builder
            .column(|b| {
                b.checkbox("One", 11, false)?;
                b.checkbox("Two", 22, true)?;
                b.button("Go", 9)?;
                Ok(())
            })
            .unwrap();
        let (_root, mut graph) = builder.finish_with_graph().unwrap();
        graph
            .props
            .insert((window_id.to_u64_lossy(), keys::UI_WIDTH.to_string()), 220);
        graph
            .props
            .insert((window_id.to_u64_lossy(), keys::UI_HEIGHT.to_string()), 140);
        graph.props.insert(
            (window_id.to_u64_lossy(), keys::UI_BG_COLOR.to_string()),
            0xFFCCCCCC,
        );

        let symbols = UiSymbols {
            rel_has_child: 201,
            rel_root_ui: 203,
            kind_button: 101,
            kind_checkbox: 102,
            kind_text: 103,
            kind_column: 104,
        };
        let tree = build_tree(&graph, &symbols, root).unwrap();
        let rects = layout_tree(
            &tree,
            LayoutRect {
                x: 0,
                y: 0,
                w: 220,
                h: 140,
            },
        );
        write_bounds(&mut graph, &tree, &rects);
        let paint = emit_paint(&tree, &rects, 0xFFCCCCCC, false);
        let mut reader = PaintReader::new(&paint).expect("paint reader");
        assert!(reader.next().is_some());

        // Simulate click on first checkbox and emit event into queue bytespace.
        let checkbox_id = tree
            .nodes
            .iter()
            .find(|n| n.kind == UiNodeKind::Checkbox)
            .map(|n| n.id)
            .unwrap();
        let checked = graph
            .props
            .get(&(
                checkbox_id.to_u64_lossy(),
                keys::UI_CHECKBOX_CHECKED.to_string(),
            ))
            .copied()
            .unwrap_or(0);
        let new_checked = if checked == 0 { 1 } else { 0 };
        graph.props.insert(
            (
                checkbox_id.to_u64_lossy(),
                keys::UI_CHECKBOX_CHECKED.to_string(),
            ),
            new_checked,
        );
        let event = UiEvent::toggled(
            window_id.to_u64_lossy(),
            checkbox_id.to_u64_lossy(),
            new_checked != 0,
            11,
        );
        let mut buf = [0u8; 128];
        let n = ui_event::encode(&event, &mut buf).unwrap();
        let queue = graph
            .props
            .get(&(window_id.to_u64_lossy(), keys::UI_EVENT_QUEUE.to_string()))
            .copied()
            .unwrap();
        graph.bytespaces.insert(queue, buf[..n].to_vec());

        let (decoded, _) = ui_event::decode_one(&buf[..n]).unwrap();
        assert_eq!(decoded.kind(), Some(UiEventKind::Toggled));
        match decoded {
            UiEvent::Toggled { target, checked, .. } => {
                assert_eq!(target, checkbox_id.to_u64_lossy());
                assert_eq!(checked != 0, new_checked != 0);
            }
            _ => panic!("expected Toggled"),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LayoutRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
