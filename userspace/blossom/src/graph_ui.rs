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
    Row,
    Button,
    Checkbox,
    Text,
    TextInput,
    ListItem,
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
    pub icon_color: u32,
    pub selected: bool,
    pub key: Option<String>,
    pub classes: Vec<String>,
    pub inline_color: Option<u32>,
    pub inline_background: Option<u32>,
    pub inline_font_name: Option<String>,
    pub inline_font_size: Option<i32>,
    pub inline_padding: Option<i32>,
    pub inline_gap: Option<i32>,
}

#[derive(Clone, Debug)]
pub struct UiTree {
    pub nodes: Vec<UiNode>,
    pub root: usize,
}

#[derive(Clone, Debug)]
pub struct ComputedStyle {
    pub color: u32,
    pub background: Option<u32>,
    pub font_name: String,
    pub font_size: i32,
    pub padding: i32,
    pub gap: i32,
    pub border_width: i32,
    pub border_color: u32,
    pub min_width: i32,
    pub min_height: i32,
    pub cursor_color: u32,
}

#[derive(Clone, Debug, Default)]
struct StyleRule {
    match_kind: Option<u64>,
    match_class: Option<String>,
    match_key: Option<String>,
    match_focused: bool,
    color: Option<u32>,
    background: Option<u32>,
    font_name: Option<String>,
    font_size: Option<i32>,
    padding: Option<i32>,
    gap: Option<i32>,
    border_width: Option<i32>,
    border_color: Option<u32>,
    min_width: Option<i32>,
    min_height: Option<i32>,
    cursor_color: Option<u32>,
}

pub struct UiSymbols {
    pub rel_has_child: u64,
    pub rel_child_of: u64,
    pub rel_root_ui: u64,
    pub kind_window: u64,
    pub kind_button: u64,
    pub kind_checkbox: u64,
    pub kind_text: u64,
    pub kind_column: u64,
}

impl UiSymbols {
    pub fn intern_sys() -> Self {
        Self {
            rel_has_child: stem::thing::sys::intern(rels::HAS_CHILD).unwrap_or(0) as u64,
            rel_child_of: stem::thing::sys::intern(rels::CHILD_OF).unwrap_or(0) as u64,
            rel_root_ui: stem::thing::sys::intern(rels::ROOT_UI).unwrap_or(0) as u64,
            kind_window: stem::thing::sys::intern(kinds::UI_WINDOW).unwrap_or(0) as u64,
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
    // Primary path: explicit window ->ROOT_UI-> root edge.
    let mut edges = [Edge::default(); 64];
    let count = graph.get_edges(window_id, &mut edges);
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() == symbols.rel_root_ui {
            return Some(edge.to);
        }
    }

    // Fallback: accept window ->HAS_CHILD-> node where node ->CHILD_OF-> window.
    // This keeps paint alive if ROOT_UI edges are absent during graph transitions.
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() != symbols.rel_has_child {
            continue;
        }
        if graph.get_kind(edge.to) == Some(symbols.kind_window) {
            continue;
        }

        let mut child_edges = [Edge::default(); 64];
        let child_count = graph.get_edges(edge.to, &mut child_edges);
        for child_edge in child_edges.iter().take(child_count) {
            if child_edge.predicate.to_u64_lossy() == symbols.rel_child_of
                && child_edge.to == window_id
            {
                return Some(edge.to);
            }
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
        } else if ui_kind_value == ui_kind::LIST_ITEM {
            UiNodeKind::ListItem
        } else if ui_kind_value == ui_kind::ROW {
            UiNodeKind::Row
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

        let text = if node_kind == UiNodeKind::Text || node_kind == UiNodeKind::TextInput || node_kind == UiNodeKind::ListItem {
            let bs = graph
                .get_prop(id, keys::UI_TEXT)
                .or_else(|| graph.get_prop(id, keys::UI_INPUT_VALUE))
                .unwrap_or(0);
            if bs != 0 {
                read_string_bs(graph, bs)
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
                read_string_bs(graph, bs)
            } else {
                None
            }
        } else {
            None
        };
        let key = graph
            .get_prop(id, keys::UI_KEY)
            .and_then(|bs| read_string_bs(graph, bs));
        let classes = graph
            .get_prop(id, keys::UI_CLASS)
            .and_then(|bs| read_string_bs(graph, bs))
            .map(parse_classes)
            .unwrap_or_else(Vec::new);
        let inline_font_name = graph
            .get_prop(id, keys::UI_FONT_NAME)
            .and_then(|bs| read_string_bs(graph, bs));
        let inline_color = graph.get_prop(id, keys::UI_COLOR).map(|v| v as u32);
        let inline_background = graph.get_prop(id, keys::UI_BG_COLOR).map(|v| v as u32);
        let inline_font_size = graph.get_prop(id, keys::UI_FONT_SIZE).map(|v| v as i32);
        let inline_padding = graph.get_prop(id, keys::UI_PADDING).map(|v| v as i32);
        let inline_gap = graph.get_prop(id, keys::UI_GAP).map(|v| v as i32);
        let focused = graph.get_prop(id, keys::UI_FOCUSED).unwrap_or(0) != 0;
        let cursor = graph
            .get_prop(id, keys::UI_CURSOR)
            .or_else(|| graph.get_prop(id, keys::UI_CURSOR_POS))
            .unwrap_or(0) as u32;
        let icon_color = graph.get_prop(id, keys::UI_ICON_COLOR).unwrap_or(0xFF808080) as u32;
        let selected = graph.get_prop(id, keys::UI_SELECTED).unwrap_or(0) != 0;

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
            icon_color,
            selected,
            key,
            classes,
            inline_color,
            inline_background,
            inline_font_name,
            inline_font_size,
            inline_padding,
            inline_gap,
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

fn read_string_bs(graph: &impl UiGraph, bs: u64) -> Option<String> {
    if bs == 0 {
        return None;
    }
    let bytes = graph.read_bytespace(ThingId::from_u64(bs))?;
    core::str::from_utf8(&bytes).ok().map(|s| s.to_string())
}

fn parse_classes(raw: String) -> Vec<String> {
    raw.split_whitespace().map(|c| c.to_string()).collect()
}

fn default_style_for(node: &UiNode) -> ComputedStyle {
    let (padding, gap) = match node.kind {
        UiNodeKind::Column => (COLUMN_PADDING, COLUMN_GAP),
        UiNodeKind::Row => (0, COLUMN_GAP),
        _ => (0, COLUMN_GAP),
    };
    let (background, border_width) = match node.kind {
        UiNodeKind::Button => (Some(BUTTON_BG), 1),
        UiNodeKind::TextInput => (Some(INPUT_BG), 1),
        UiNodeKind::Checkbox => (Some(CHECKBOX_FILL), 1),
        _ => (None, 0),
    };
    ComputedStyle {
        color: TEXT_COLOR,
        background,
        font_name: String::from("NotoSans-Regular"),
        font_size: 16,
        padding,
        gap,
        border_width,
        border_color: BUTTON_BORDER,
        min_width: 0,
        min_height: 0,
        cursor_color: TEXT_COLOR,
    }
}

fn style_specificity(rule: &StyleRule) -> u16 {
    let mut score = 0u16;
    if rule.match_kind.is_some() {
        score = score.saturating_add(1);
    }
    if rule.match_class.is_some() {
        score = score.saturating_add(10);
    }
    if rule.match_key.is_some() {
        score = score.saturating_add(100);
    }
    if rule.match_focused {
        score = score.saturating_add(1);
    }
    score
}

fn rule_matches(rule: &StyleRule, node: &UiNode) -> bool {
    if let Some(kind) = rule.match_kind {
        if kind != node_kind_tag(node.kind) {
            return false;
        }
    }
    if let Some(class) = rule.match_class.as_deref() {
        if !node.classes.iter().any(|c| c == class) {
            return false;
        }
    }
    if let Some(key) = rule.match_key.as_deref() {
        if node.key.as_deref() != Some(key) {
            return false;
        }
    }
    if rule.match_focused && !node.focused {
        return false;
    }
    true
}

fn node_kind_tag(kind: UiNodeKind) -> u64 {
    match kind {
        UiNodeKind::Text => ui_kind::TEXT,
        UiNodeKind::Column => ui_kind::COLUMN,
        UiNodeKind::Row => ui_kind::ROW,
        UiNodeKind::TextInput => ui_kind::TEXT_INPUT,
        UiNodeKind::Button => ui_kind::BUTTON,
        UiNodeKind::Checkbox => ui_kind::CHECKBOX,
        UiNodeKind::ListItem => ui_kind::LIST_ITEM,
        UiNodeKind::Unknown => 0,
    }
}

fn collect_stylesheet_rules(
    graph: &impl UiGraph,
    symbols: &UiSymbols,
    stylesheet_id: ThingId,
    out: &mut Vec<StyleRule>,
) {
    let mut edges = [Edge::default(); 128];
    let count = graph.get_edges(stylesheet_id, &mut edges);
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() != symbols.rel_has_child {
            continue;
        }
        let rule_id = edge.to;
        let rule = StyleRule {
            match_kind: graph.get_prop(rule_id, keys::UI_STYLE_MATCH_KIND),
            match_class: graph
                .get_prop(rule_id, keys::UI_STYLE_MATCH_CLASS)
                .and_then(|bs| read_string_bs(graph, bs)),
            match_key: graph
                .get_prop(rule_id, keys::UI_STYLE_MATCH_KEY)
                .and_then(|bs| read_string_bs(graph, bs)),
            match_focused: graph.get_prop(rule_id, keys::UI_STYLE_MATCH_FOCUSED).unwrap_or(0) != 0,
            color: graph.get_prop(rule_id, keys::UI_STYLE_COLOR).map(|v| v as u32),
            background: graph.get_prop(rule_id, keys::UI_STYLE_BACKGROUND).map(|v| v as u32),
            font_name: graph
                .get_prop(rule_id, keys::UI_STYLE_FONT_NAME)
                .and_then(|bs| read_string_bs(graph, bs)),
            font_size: graph.get_prop(rule_id, keys::UI_STYLE_FONT_SIZE).map(|v| v as i32),
            padding: graph.get_prop(rule_id, keys::UI_STYLE_PADDING).map(|v| v as i32),
            gap: graph.get_prop(rule_id, keys::UI_STYLE_GAP).map(|v| v as i32),
            border_width: graph
                .get_prop(rule_id, keys::UI_STYLE_BORDER_WIDTH)
                .map(|v| v as i32),
            border_color: graph
                .get_prop(rule_id, keys::UI_STYLE_BORDER_COLOR)
                .map(|v| v as u32),
            min_width: graph.get_prop(rule_id, keys::UI_STYLE_MIN_WIDTH).map(|v| v as i32),
            min_height: graph
                .get_prop(rule_id, keys::UI_STYLE_MIN_HEIGHT)
                .map(|v| v as i32),
            cursor_color: graph
                .get_prop(rule_id, keys::UI_STYLE_CURSOR_COLOR)
                .map(|v| v as u32),
        };
        out.push(rule);
    }
}

fn load_window_style_rules(
    graph: &impl UiGraph,
    symbols: &UiSymbols,
    window_id: ThingId,
) -> Vec<StyleRule> {
    let mut rules = Vec::new();

    // Global default can be set directly on the window as an override path.
    if let Some(global) = graph.get_prop(window_id, keys::UI_STYLESHEET_DEFAULT) {
        if global != 0 {
            collect_stylesheet_rules(graph, symbols, ThingId::from_u64(global), &mut rules);
        }
    }

    // Preferred global path: stylesheet default on the parent ui.Crown.
    let mut edges = [Edge::default(); 16];
    let count = graph.get_edges(window_id, &mut edges);
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() != symbols.rel_child_of {
            continue;
        }
        if let Some(global) = graph.get_prop(edge.to, keys::UI_STYLESHEET_DEFAULT) {
            if global != 0 {
                collect_stylesheet_rules(graph, symbols, ThingId::from_u64(global), &mut rules);
            }
        }
    }

    if let Some(local) = graph.get_prop(window_id, keys::UI_STYLESHEET) {
        if local != 0 {
            collect_stylesheet_rules(graph, symbols, ThingId::from_u64(local), &mut rules);
        }
    }

    rules
}

pub fn compute_styles(
    graph: &impl UiGraph,
    symbols: &UiSymbols,
    window_id: ThingId,
    tree: &UiTree,
) -> Vec<ComputedStyle> {
    let rules = load_window_style_rules(graph, symbols, window_id);
    let mut out = vec![ComputedStyle {
        color: TEXT_COLOR,
        background: None,
        font_name: String::new(),
        font_size: 16,
        padding: COLUMN_PADDING,
        gap: COLUMN_GAP,
        border_width: 0,
        border_color: BUTTON_BORDER,
        min_width: 0,
        min_height: 0,
        cursor_color: TEXT_COLOR,
    }; tree.nodes.len()];

    for (idx, node) in tree.nodes.iter().enumerate() {
        let mut style = default_style_for(node);
        if let Some(parent) = node.parent {
            // v1 inheritance list: color + font.*
            style.color = out[parent].color;
            style.font_name = out[parent].font_name.clone();
            style.font_size = out[parent].font_size;
        }

        let mut color_rank: Option<(u16, usize)> = None;
        let mut bg_rank: Option<(u16, usize)> = None;
        let mut font_name_rank: Option<(u16, usize)> = None;
        let mut font_size_rank: Option<(u16, usize)> = None;
        let mut padding_rank: Option<(u16, usize)> = None;
        let mut gap_rank: Option<(u16, usize)> = None;
        let mut border_width_rank: Option<(u16, usize)> = None;
        let mut border_color_rank: Option<(u16, usize)> = None;
        let mut min_width_rank: Option<(u16, usize)> = None;
        let mut min_height_rank: Option<(u16, usize)> = None;
        let mut cursor_color_rank: Option<(u16, usize)> = None;

        for (rule_idx, rule) in rules.iter().enumerate() {
            if !rule_matches(rule, node) {
                continue;
            }
            let rank = (style_specificity(rule), rule_idx);
            if let Some(value) = rule.color {
                if color_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.color = value;
                    color_rank = Some(rank);
                }
            }
            if let Some(value) = rule.background {
                if bg_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.background = Some(value);
                    bg_rank = Some(rank);
                }
            }
            if let Some(value) = rule.font_name.as_ref() {
                if font_name_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.font_name = value.clone();
                    font_name_rank = Some(rank);
                }
            }
            if let Some(value) = rule.font_size {
                if font_size_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.font_size = value;
                    font_size_rank = Some(rank);
                }
            }
            if let Some(value) = rule.padding {
                if padding_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.padding = value;
                    padding_rank = Some(rank);
                }
            }
            if let Some(value) = rule.gap {
                if gap_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.gap = value;
                    gap_rank = Some(rank);
                }
            }
            if let Some(value) = rule.border_width {
                if border_width_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.border_width = value.max(0);
                    border_width_rank = Some(rank);
                }
            }
            if let Some(value) = rule.border_color {
                if border_color_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.border_color = value;
                    border_color_rank = Some(rank);
                }
            }
            if let Some(value) = rule.min_width {
                if min_width_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.min_width = value.max(0);
                    min_width_rank = Some(rank);
                }
            }
            if let Some(value) = rule.min_height {
                if min_height_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.min_height = value.max(0);
                    min_height_rank = Some(rank);
                }
            }
            if let Some(value) = rule.cursor_color {
                if cursor_color_rank.map(|r| r <= rank).unwrap_or(true) {
                    style.cursor_color = value;
                    cursor_color_rank = Some(rank);
                }
            }
        }

        // Inline node props override stylesheet rules for migration safety.
        if let Some(value) = node.inline_color {
            style.color = value;
        }
        if let Some(value) = node.inline_background {
            style.background = Some(value);
        }
        if let Some(value) = node.inline_font_name.as_ref() {
            style.font_name = value.clone();
        }
        if let Some(value) = node.inline_font_size {
            style.font_size = value;
        }
        if let Some(value) = node.inline_padding {
            style.padding = value;
        }
        if let Some(value) = node.inline_gap {
            style.gap = value;
        }

        if style.font_size < 8 {
            style.font_size = 8;
        }
        if style.cursor_color == 0 {
            style.cursor_color = style.color;
        }
        out[idx] = style;
    }
    out
}

pub fn layout_tree(tree: &UiTree, styles: &[ComputedStyle], root_rect: LayoutRect) -> Vec<LayoutRect> {
    let mut rects = vec![LayoutRect::default(); tree.nodes.len()];
    layout_node(tree, styles, tree.root, root_rect, &mut rects);
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

// ── Title bar chrome constants ──
// These MUST match Bloom's BLOSSOM_BORDER and BLOSSOM_TITLE_BAR_HEIGHT.
const CHROME_BORDER: i32 = 2;
const CHROME_TITLE_BAR_HEIGHT: i32 = 24;

// Focused title bar gradient (blue-ish)
const TITLE_FOCUSED_TOP: u32 = 0xFF5B9BD5;    // steel blue
const TITLE_FOCUSED_BOTTOM: u32 = 0xFF3A6EA5;  // darker blue
const TITLE_FOCUSED_TEXT: u32 = 0xFFFFFFFF;     // white text

// Unfocused title bar gradient (gray)
const TITLE_UNFOCUSED_TOP: u32 = 0xFFC0C0C0;    // light gray
const TITLE_UNFOCUSED_BOTTOM: u32 = 0xFFA0A0A0;  // medium gray
const TITLE_UNFOCUSED_TEXT: u32 = 0xFF404040;     // dark gray text

// Border color
const CHROME_BORDER_COLOR: u32 = 0xFF606060;

pub fn emit_paint(
    tree: &UiTree,
    styles: &[ComputedStyle],
    rects: &[LayoutRect],
    window_w: i32,
    window_h: i32,
    window_bg: u32,
    is_focused: bool,
    title: Option<&str>,
) -> Vec<u8> {
    let mut builder = PaintBuilder::new();

    // 1. Draw window border (2px all around)
    // Top border
    builder.fill_rect(0, 0, window_w, CHROME_BORDER, CHROME_BORDER_COLOR);
    // Bottom border
    builder.fill_rect(0, window_h - CHROME_BORDER, window_w, CHROME_BORDER, CHROME_BORDER_COLOR);
    // Left border
    builder.fill_rect(0, CHROME_BORDER, CHROME_BORDER, window_h - CHROME_BORDER * 2, CHROME_BORDER_COLOR);
    // Right border
    builder.fill_rect(window_w - CHROME_BORDER, CHROME_BORDER, CHROME_BORDER, window_h - CHROME_BORDER * 2, CHROME_BORDER_COLOR);

    // 2. Draw title bar gradient
    let tb_x = CHROME_BORDER;
    let tb_y = CHROME_BORDER;
    let tb_w = window_w - CHROME_BORDER * 2;
    let tb_h = CHROME_TITLE_BAR_HEIGHT;
    let (grad_top, grad_bot, title_color) = if is_focused {
        (TITLE_FOCUSED_TOP, TITLE_FOCUSED_BOTTOM, TITLE_FOCUSED_TEXT)
    } else {
        (TITLE_UNFOCUSED_TOP, TITLE_UNFOCUSED_BOTTOM, TITLE_UNFOCUSED_TEXT)
    };
    builder.fill_linear_gradient(tb_x, tb_y, tb_w, tb_h, grad_top, grad_bot);

    // 3. Draw title text (left-aligned with padding)
    if let Some(text) = title {
        if !text.is_empty() {
            let text_x = tb_x + 8;
            let text_y = tb_y;
            let text_w = tb_w - 16;
            let text_h = tb_h;
            let size = 14;
            let baseline = text_y + (text_h + size) / 2 - 2;
            builder.draw_text_run(
                text_x, text_y, text_w, text_h,
                baseline,
                "NotoSans-Regular",
                size,
                text,
                title_color,
            );
        }
    }

    // 4. Fill client area background
    let root_rect = rects[tree.root];
    builder.fill_rect(
        root_rect.x,
        root_rect.y,
        root_rect.w,
        root_rect.h,
        window_bg,
    );

    // 5. Emit app content nodes
    emit_node(tree, styles, rects, tree.root, &mut builder);
    builder.finish()
}

const COLUMN_PADDING: i32 = 8;
const COLUMN_GAP: i32 = 8;
const BUTTON_HEIGHT: i32 = 32;
const CHECKBOX_HEIGHT: i32 = 28;
const TEXT_HEIGHT: i32 = 20;
const TEXT_INPUT_HEIGHT: i32 = 32;
const LIST_ITEM_HEIGHT: i32 = 24;
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

fn layout_node(
    tree: &UiTree,
    styles: &[ComputedStyle],
    index: usize,
    rect: LayoutRect,
    rects: &mut [LayoutRect],
) {
    rects[index] = rect;
    let node = &tree.nodes[index];
    if node.children.is_empty() {
        return;
    }
    if node.kind == UiNodeKind::Column {
        layout_column(tree, styles, index, rect, rects);
    } else if node.kind == UiNodeKind::Row {
        layout_row(tree, styles, index, rect, rects);
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

fn text_height_for_style(style: &ComputedStyle) -> i32 {
    (style.font_size + 6).max(TEXT_HEIGHT).max(style.min_height)
}

fn layout_column(
    tree: &UiTree,
    styles: &[ComputedStyle],
    index: usize,
    rect: LayoutRect,
    rects: &mut [LayoutRect],
) {
    let padding = styles[index].padding.max(0);
    let gap = styles[index].gap.max(0);
    let mut y = rect.y + padding;
    let x = rect.x + padding;
    let w = (rect.w - padding * 2).max(0);
    for &child in &tree.nodes[index].children {
        let child_kind = tree.nodes[child].kind;
        let h = match child_kind {
            UiNodeKind::Button => BUTTON_HEIGHT.max(styles[child].min_height),
            UiNodeKind::Checkbox => CHECKBOX_HEIGHT.max(styles[child].min_height),
            UiNodeKind::Text => text_height_for_style(&styles[child]),
            UiNodeKind::TextInput => TEXT_INPUT_HEIGHT.max(styles[child].min_height),
            UiNodeKind::ListItem => LIST_ITEM_HEIGHT.max(styles[child].min_height),
            UiNodeKind::Column | UiNodeKind::Row => {
                // Container children fill remaining vertical space
                (rect.h - (y - rect.y) - padding).max(BUTTON_HEIGHT)
            }
            UiNodeKind::Unknown => text_height_for_style(&styles[child]),
        };
        let child_w = if styles[child].min_width > 0 {
            w.max(styles[child].min_width)
        } else {
            w
        };
        let child_rect = LayoutRect {
            x,
            y,
            w: child_w,
            h,
        };
        rects[child] = child_rect;
        layout_node(tree, styles, child, child_rect, rects);
        y = y.saturating_add(h).saturating_add(gap);
    }
}

fn layout_row(
    tree: &UiTree,
    styles: &[ComputedStyle],
    index: usize,
    rect: LayoutRect,
    rects: &mut [LayoutRect],
) {
    let children = &tree.nodes[index].children;
    let n = children.len() as i32;
    if n == 0 {
        return;
    }
    let gap = styles[index].gap.max(0);
    let padding = styles[index].padding.max(0);
    let total_gap = gap * (n - 1);
    let avail_w = (rect.w - padding * 2 - total_gap).max(0);
    let child_w = avail_w / n;
    let mut x = rect.x + padding;
    for (i, &child) in children.iter().enumerate() {
        // Give leftover pixels to the last child
        let w = if i as i32 == n - 1 {
            (rect.x + rect.w - padding) - x
        } else {
            child_w
        };
        let child_rect = LayoutRect {
            x,
            y: rect.y + padding,
            w,
            h: (rect.h - padding * 2).max(0),
        };
        rects[child] = child_rect;
        layout_node(tree, styles, child, child_rect, rects);
        x = x.saturating_add(w).saturating_add(gap);
    }
}

fn emit_node(
    tree: &UiTree,
    styles: &[ComputedStyle],
    rects: &[LayoutRect],
    index: usize,
    builder: &mut PaintBuilder,
) {
    let node = &tree.nodes[index];
    if !node.visible {
        return;
    }
    let rect = rects[index];
    let style = &styles[index];
    if let Some(bg) = style.background {
        builder.fill_rect(rect.x, rect.y, rect.w, rect.h, bg);
    }
    match node.kind {
        UiNodeKind::Button => draw_button(tree, styles, rects, index, rect, builder),
        UiNodeKind::Checkbox => draw_checkbox(tree, styles, rects, index, rect, builder),
        UiNodeKind::TextInput => draw_text_input(node, style, rect, builder),
        UiNodeKind::Text => {
            if !is_label_child(tree, index) {
                if let Some(text) = &node.text {
                    draw_text(style, rect, text, builder);
                }
            }
        }
        UiNodeKind::ListItem => draw_list_item(node, style, rect, builder),
        _ => {}
    }
    for &child in &node.children {
        emit_node(tree, styles, rects, child, builder);
    }
}

fn draw_button(
    tree: &UiTree,
    styles: &[ComputedStyle],
    _rects: &[LayoutRect],
    index: usize,
    rect: LayoutRect,
    builder: &mut PaintBuilder,
) {
    let node = &tree.nodes[index];
    let style = &styles[index];
    let bg = if !node.enabled {
        BUTTON_BG_DISABLED
    } else if node.pressed {
        BUTTON_BG_PRESSED
    } else {
        style.background.unwrap_or(BUTTON_BG)
    };
    builder.fill_rect(rect.x, rect.y, rect.w, rect.h, bg);
    let bw = style.border_width.max(1);
    let bc = style.border_color;
    // Border
    builder.fill_rect(rect.x, rect.y, rect.w, bw, bc);
    builder.fill_rect(rect.x, rect.y + rect.h - bw, rect.w, bw, bc);
    builder.fill_rect(rect.x, rect.y, bw, rect.h, bc);
    builder.fill_rect(rect.x + rect.w - bw, rect.y, bw, rect.h, bc);

    if let Some(label_id) = node.label_id {
        if let Some((_label_idx, text)) = label_text(tree, label_id) {
            let text_rect = LayoutRect {
                x: rect.x + 8,
                y: rect.y,
                w: rect.w.saturating_sub(16),
                h: rect.h,
            };
            draw_text(style, text_rect, text, builder);
        }
    }
}

fn draw_checkbox(
    tree: &UiTree,
    styles: &[ComputedStyle],
    _rects: &[LayoutRect],
    index: usize,
    rect: LayoutRect,
    builder: &mut PaintBuilder,
) {
    let node = &tree.nodes[index];
    let style = &styles[index];
    let box_rect = checkbox_box_rect(rect);
    builder.fill_rect(
        box_rect.x,
        box_rect.y,
        box_rect.w,
        box_rect.h,
        CHECKBOX_FILL,
    );
    builder.fill_rect(box_rect.x, box_rect.y, box_rect.w, 1, style.border_color);
    builder.fill_rect(
        box_rect.x,
        box_rect.y + box_rect.h - 1,
        box_rect.w,
        1,
        style.border_color,
    );
    builder.fill_rect(box_rect.x, box_rect.y, 1, box_rect.h, style.border_color);
    builder.fill_rect(
        box_rect.x + box_rect.w - 1,
        box_rect.y,
        1,
        box_rect.h,
        style.border_color,
    );

    if node.checked {
        let x1 = box_rect.x + 3;
        let y1 = box_rect.y + box_rect.h / 2;
        let x2 = box_rect.x + box_rect.w / 2;
        let y2 = box_rect.y + box_rect.h - 4;
        let x3 = box_rect.x + box_rect.w - 3;
        let y3 = box_rect.y + 3;
        builder.stroke_line(x1, y1, x2, y2, 2, style.color);
        builder.stroke_line(x2, y2, x3, y3, 2, style.color);
    }

    if let Some(label_id) = node.label_id {
        if let Some((_label_idx, text)) = label_text(tree, label_id) {
            let text_rect = LayoutRect {
                x: box_rect.x + box_rect.w + 8,
                y: rect.y,
                w: rect.w.saturating_sub(box_rect.w + 8),
                h: rect.h,
            };
            draw_text(style, text_rect, text, builder);
        }
    }
}

fn draw_text_input(node: &UiNode, style: &ComputedStyle, rect: LayoutRect, builder: &mut PaintBuilder) {
    let border = if node.focused {
        INPUT_BORDER_FOCUS
    } else {
        style.border_color
    };
    builder.fill_rect(rect.x, rect.y, rect.w, rect.h, style.background.unwrap_or(INPUT_BG));
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
        draw_text(style, text_rect, content, builder);
    } else if let Some(placeholder) = node.placeholder.as_deref() {
        draw_text_colored(style, text_rect, placeholder, INPUT_PLACEHOLDER, builder);
    }
    if node.focused {
        let cursor = core::cmp::min(node.cursor as usize, content.len()) as i32;
        let cursor_x = text_rect.x + cursor.saturating_mul(8);
        let cursor_h = (rect.h - 10).max(1);
        builder.fill_rect(cursor_x, rect.y + 5, 1, cursor_h, style.cursor_color);
    }
}

const LIST_ITEM_SELECTED_BG: u32 = 0xFF3078C0;
const LIST_ITEM_SELECTED_TEXT: u32 = 0xFFFFFFFF;

fn draw_list_item(node: &UiNode, style: &ComputedStyle, rect: LayoutRect, builder: &mut PaintBuilder) {
    // Draw selection highlight background
    if node.selected {
        builder.fill_rect(rect.x, rect.y, rect.w, rect.h, LIST_ITEM_SELECTED_BG);
    }

    // Draw 16×16 icon square on the left, vertically centered
    let icon_size = 16i32.min(rect.h);
    let icon_y = rect.y + (rect.h - icon_size) / 2;
    let icon_color = node.icon_color;
    builder.fill_rect(rect.x, icon_y, icon_size, icon_size, icon_color);

    // Draw text label to the right of the icon
    let text_color = if node.selected {
        LIST_ITEM_SELECTED_TEXT
    } else {
        style.color
    };
    if let Some(text) = &node.text {
        let text_rect = LayoutRect {
            x: rect.x + icon_size + 6,
            y: rect.y,
            w: rect.w.saturating_sub(icon_size + 6),
            h: rect.h,
        };
        draw_text_colored(style, text_rect, text, text_color, builder);
    }
}

fn draw_text(style: &ComputedStyle, rect: LayoutRect, text: &str, builder: &mut PaintBuilder) {
    draw_text_colored(style, rect, text, style.color, builder);
}

fn draw_text_colored(
    style: &ComputedStyle,
    rect: LayoutRect,
    text: &str,
    color: u32,
    builder: &mut PaintBuilder,
) {
    let size = style.font_size.max(8);
    let baseline = rect.y + (rect.h + size) / 2 - 2;
    builder.draw_text_run(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        baseline,
        &style.font_name,
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
                kinds::UI_WINDOW => 100,
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
            rel_child_of: 202,
            rel_root_ui: 203,
            kind_window: 100,
            kind_button: 101,
            kind_checkbox: 102,
            kind_text: 103,
            kind_column: 104,
        };
        let tree = build_tree(&graph, &symbols, root).unwrap();
        let styles = compute_styles(&graph, &symbols, ThingId::from_u64(99), &tree);
        let rects = layout_tree(
            &tree,
            &styles,
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
    fn row_layout_distributes_width_correctly() {
        let graph = TestGraph::new();
        let mut builder = UiTreeBuilder::new(graph, ThingId::from_u64(99));
        builder
            .row(|b| {
                b.button("A", 1)?;
                b.button("B", 2)?;
                b.button("C", 3)?;
                Ok(())
            })
            .unwrap();
        let (root, graph) = builder.finish_with_graph().unwrap();

        let symbols = UiSymbols {
            rel_has_child: 201,
            rel_child_of: 202,
            rel_root_ui: 203,
            kind_window: 100,
            kind_button: 101,
            kind_checkbox: 102,
            kind_text: 103,
            kind_column: 104,
        };
        let tree = build_tree(&graph, &symbols, root).unwrap();
        let styles = compute_styles(&graph, &symbols, ThingId::from_u64(99), &tree);

        let rects = layout_tree(
            &tree,
            &styles,
            LayoutRect {
                x: 0,
                y: 0,
                w: 101,
                h: 50,
            },
        );

        let child_rects: Vec<LayoutRect> = tree.nodes[tree.root]
            .children
            .iter()
            .map(|i| rects[*i])
            .collect();

        assert_eq!(child_rects.len(), 3);

        // Child 0
        assert_eq!(child_rects[0].x, 0);
        assert_eq!(child_rects[0].w, 28);

        // Child 1
        // x = 0 + 28 + 8 = 36
        assert_eq!(child_rects[1].x, 36);
        assert_eq!(child_rects[1].w, 28);

        // Child 2
        // x = 36 + 28 + 8 = 72
        assert_eq!(child_rects[2].x, 72);
        // w = 101 - 72 = 29
        assert_eq!(child_rects[2].w, 29);
    }

    #[test]
    fn graph_roundtrip_emits_event() {
        let graph = TestGraph::new();
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
            rel_child_of: 202,
            rel_root_ui: 203,
            kind_window: 100,
            kind_button: 101,
            kind_checkbox: 102,
            kind_text: 103,
            kind_column: 104,
        };
        let tree = build_tree(&graph, &symbols, root).unwrap();
        let styles = compute_styles(&graph, &symbols, window_id, &tree);
        let rects = layout_tree(
            &tree,
            &styles,
            LayoutRect {
                x: 0,
                y: 0,
                w: 220,
                h: 140,
            },
        );
        write_bounds(&mut graph, &tree, &rects);
        let paint = emit_paint(&tree, &styles, &rects, 220, 140, 0xFFCCCCCC, false, None);
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
            .get(&(window_id.to_u64_lossy(), keys::UI_EVENT_LOG.to_string()))
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

    fn set_string_prop(graph: &mut TestGraph, id: ThingId, key: &str, value: &str) {
        let bs = graph.bytespace_create(value.len()).unwrap();
        graph.bytespace_write(bs, 0, value.as_bytes()).unwrap();
        graph.prop_set(id, key, bs.to_u64_lossy()).unwrap();
    }

    #[test]
    fn stylesheet_rules_apply_specificity_and_inheritance() {
        let graph = TestGraph::new();
        let window_id = ThingId::from_u64(501);
        let mut builder = UiTreeBuilder::new(graph, window_id);
        let root = builder
            .column(|ui| {
                let t1 = ui.text("A")?;
                ui.key_node(t1, stem::petals::graph::UiKey("time"))?;
                let t2 = ui.text("B")?;
                ui.key_node(t2, stem::petals::graph::UiKey("plain"))?;
                Ok(())
            })
            .unwrap();
        let (_root, mut graph) = builder.finish_with_graph().unwrap();

        let tree = {
            let symbols = UiSymbols {
                rel_has_child: 201,
                rel_child_of: 202,
                rel_root_ui: 203,
                kind_window: 100,
                kind_button: 101,
                kind_checkbox: 102,
                kind_text: 103,
                kind_column: 104,
            };

            // Set class tags used by selectors.
            let text_ids: Vec<ThingId> = graph
                .edges
                .iter()
                .filter(|e| e.from == root && e.predicate.to_u64_lossy() == 201)
                .map(|e| e.to)
                .collect();
            set_string_prop(&mut graph, text_ids[0], keys::UI_CLASS, "clock");

            // Build stylesheet: kind(Text) < class(clock) < key(time)
            let stylesheet = graph.create_node(kinds::CSS_STYLESHEET).unwrap();
            graph
                .prop_set(window_id, keys::UI_STYLESHEET, stylesheet.to_u64_lossy())
                .unwrap();

            let rule_kind = graph.create_node(kinds::CSS_RULE).unwrap();
            graph.link(stylesheet, rels::HAS_CHILD, rule_kind).unwrap();
            graph
                .prop_set(rule_kind, keys::UI_STYLE_MATCH_KIND, ui_kind::TEXT)
                .unwrap();
            graph
                .prop_set(rule_kind, keys::UI_STYLE_COLOR, 0xFF112233)
                .unwrap();

            let rule_class = graph.create_node(kinds::CSS_RULE).unwrap();
            graph.link(stylesheet, rels::HAS_CHILD, rule_class).unwrap();
            set_string_prop(&mut graph, rule_class, keys::UI_STYLE_MATCH_CLASS, "clock");
            set_string_prop(&mut graph, rule_class, keys::UI_STYLE_FONT_NAME, "DSEG7Classic-Regular");
            graph
                .prop_set(rule_class, keys::UI_STYLE_FONT_SIZE, 64)
                .unwrap();

            let rule_key = graph.create_node(kinds::CSS_RULE).unwrap();
            graph.link(stylesheet, rels::HAS_CHILD, rule_key).unwrap();
            set_string_prop(&mut graph, rule_key, keys::UI_STYLE_MATCH_KEY, "time");
            graph
                .prop_set(rule_key, keys::UI_STYLE_COLOR, 0xFFF04040)
                .unwrap();

            // Parent color should inherit into plain text.
            let parent_rule = graph.create_node(kinds::CSS_RULE).unwrap();
            graph.link(stylesheet, rels::HAS_CHILD, parent_rule).unwrap();
            graph
                .prop_set(parent_rule, keys::UI_STYLE_MATCH_KIND, ui_kind::COLUMN)
                .unwrap();
            graph
                .prop_set(parent_rule, keys::UI_STYLE_COLOR, 0xFF00AA00)
                .unwrap();

            let tree = build_tree(&graph, &symbols, root).unwrap();
            let styles = compute_styles(&graph, &symbols, window_id, &tree);

            let text_a = tree
                .nodes
                .iter()
                .position(|n| n.key.as_deref() == Some("time"))
                .unwrap();
            let text_b = tree
                .nodes
                .iter()
                .position(|n| n.key.as_deref() == Some("plain"))
                .unwrap();

            assert_eq!(styles[text_a].color, 0xFFF04040);
            assert_eq!(styles[text_a].font_size, 64);
            assert_eq!(styles[text_a].font_name, "DSEG7Classic-Regular");
            assert_eq!(styles[text_b].color, 0xFF112233);
            tree
        };

        assert_eq!(tree.nodes[tree.root].kind, UiNodeKind::Column);
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LayoutRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
