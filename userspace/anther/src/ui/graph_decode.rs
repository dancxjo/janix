extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::ids::HandleId;
use abi::schema::{keys, kinds, rels, ui_kind};
use abi::types::Edge;
use stem::thing::sys::{bytespace_info, bytespace_read, get_edges, get_kind, intern, prop_get};
use stem::thing::ThingId;

const UI_ORDER_KEY: &str = "ui.order";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiNodeKind {
    Window,
    Column,
    Row,
    Box,
    Text,
    TextInput,
    Button,
    Checkbox,
    Unknown,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UiLayout {
    pub gap: u64,
    pub padding: u64,
    pub margin: u64,
    pub flex_grow: u64,
    pub width: Option<u64>,
    pub height: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct UiNode {
    pub id: ThingId,
    pub kind: UiNodeKind,
    pub text: Option<String>,
    pub placeholder: Option<String>,
    pub checked: bool,
    pub action_id: u64,
    pub visible: bool,
    pub enabled: bool,
    pub layout: UiLayout,
    pub children: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct UiTree {
    pub window_id: ThingId,
    pub root: usize,
    pub nodes: Vec<UiNode>,
}

#[derive(Debug)]
pub enum DecodeError {
    NotFound,
    Graph,
}

pub trait GraphRead {
    fn get_kind(&self, id: ThingId) -> Option<u64>;
    fn get_prop(&self, id: ThingId, key: &str) -> Option<u64>;
    fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> usize;
    fn read_bytespace_utf8(&self, id: ThingId) -> Option<String>;
}

pub struct SysGraph;

impl GraphRead for SysGraph {
    fn get_kind(&self, id: ThingId) -> Option<u64> {
        get_kind(id).ok().map(|k| k.0)
    }

    fn get_prop(&self, id: ThingId, key: &str) -> Option<u64> {
        prop_get(id, key).ok()
    }

    fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> usize {
        get_edges(id, out).unwrap_or(0)
    }

    fn read_bytespace_utf8(&self, id: ThingId) -> Option<String> {
        let size = bytespace_info(id).ok()?;
        if size == 0 {
            return Some(String::new());
        }
        let mut out = Vec::with_capacity(size);
        out.resize(size, 0);
        let mut offset = 0;
        while offset < size {
            let n = bytespace_read(id, offset, &mut out[offset..]).ok()?;
            if n == 0 {
                break;
            }
            offset += n;
        }
        out.truncate(offset);
        core::str::from_utf8(&out).ok().map(|s| s.to_string())
    }
}

#[derive(Clone, Copy)]
pub struct Symbols {
    rel_root_ui: u64,
    rel_has_child: u64,
    kind_button: u64,
    kind_checkbox: u64,
    kind_text: u64,
    kind_column: u64,
    kind_window: u64,
}

impl Symbols {
    pub fn intern_sys() -> Self {
        Self {
            rel_root_ui: intern(rels::ROOT_UI).unwrap_or(0) as u64,
            rel_has_child: intern(rels::HAS_CHILD).unwrap_or(0) as u64,
            kind_button: intern(kinds::UI_BUTTON).unwrap_or(0) as u64,
            kind_checkbox: intern(kinds::UI_CHECKBOX).unwrap_or(0) as u64,
            kind_text: intern(kinds::UI_TEXT).unwrap_or(0) as u64,
            kind_column: intern(kinds::UI_COLUMN).unwrap_or(0) as u64,
            kind_window: intern(kinds::UI_WINDOW).unwrap_or(0) as u64,
        }
    }
}

pub fn decode_window_tree(window_id: ThingId) -> Result<(UiTree, u64), DecodeError> {
    let graph = SysGraph;
    let symbols = Symbols::intern_sys();
    let scene_gen = prop_get(window_id, keys::UI_SCENE_GEN).unwrap_or(0);
    let tree = decode_window_tree_with(&graph, &symbols, window_id)?;
    Ok((tree, scene_gen))
}

pub fn decode_window_tree_with(
    graph: &impl GraphRead,
    symbols: &Symbols,
    window_id: ThingId,
) -> Result<UiTree, DecodeError> {
    let root_id = find_root_ui(graph, symbols, window_id).ok_or(DecodeError::NotFound)?;

    let mut nodes = Vec::new();
    let mut idx_by_id = BTreeMap::<ThingId, usize>::new();
    let root_idx = decode_node(graph, symbols, root_id, &mut nodes, &mut idx_by_id)?;

    Ok(UiTree {
        window_id,
        root: root_idx,
        nodes,
    })
}

fn find_root_ui(graph: &impl GraphRead, symbols: &Symbols, window_id: ThingId) -> Option<ThingId> {
    let mut edges = [Edge::default(); 64];
    let count = graph.get_edges(window_id, &mut edges);
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() == symbols.rel_root_ui {
            return Some(edge.to);
        }
    }
    None
}

fn decode_node(
    graph: &impl GraphRead,
    symbols: &Symbols,
    id: ThingId,
    nodes: &mut Vec<UiNode>,
    idx_by_id: &mut BTreeMap<ThingId, usize>,
) -> Result<usize, DecodeError> {
    if let Some(existing) = idx_by_id.get(&id).copied() {
        return Ok(existing);
    }

    let kind_id = graph.get_kind(id).ok_or(DecodeError::Graph)?;
    let node_kind = classify_node_kind(graph, symbols, id, kind_id);

    let text = node_text(graph, id, node_kind);
    let placeholder = node_placeholder(graph, id, node_kind);

    let mut node = UiNode {
        id,
        kind: node_kind,
        text,
        placeholder,
        checked: graph.get_prop(id, keys::UI_CHECKBOX_CHECKED).unwrap_or(0) != 0,
        action_id: graph.get_prop(id, keys::UI_BUTTON_ACTION_ID).unwrap_or(0),
        visible: graph.get_prop(id, keys::UI_VISIBLE).unwrap_or(1) != 0,
        enabled: graph.get_prop(id, keys::UI_ENABLED).unwrap_or(1) != 0,
        layout: UiLayout {
            gap: graph.get_prop(id, keys::UI_GAP).unwrap_or(0),
            padding: graph.get_prop(id, keys::UI_PADDING).unwrap_or(0),
            margin: graph.get_prop(id, "ui.margin").unwrap_or(0),
            flex_grow: graph.get_prop(id, "ui.flex_grow").unwrap_or(0),
            width: graph.get_prop(id, keys::UI_WIDTH),
            height: graph.get_prop(id, keys::UI_HEIGHT),
        },
        children: Vec::new(),
    };

    let my_idx = nodes.len();
    nodes.push(UiNode {
        id,
        kind: node.kind,
        text: node.text.take(),
        placeholder: node.placeholder.take(),
        checked: node.checked,
        action_id: node.action_id,
        visible: node.visible,
        enabled: node.enabled,
        layout: node.layout,
        children: Vec::new(),
    });
    idx_by_id.insert(id, my_idx);

    let ordered_children = child_ids_ordered(graph, symbols, id);
    for child_id in ordered_children {
        let child_idx = decode_node(graph, symbols, child_id, nodes, idx_by_id)?;
        if let Some(cur) = nodes.get_mut(my_idx) {
            cur.children.push(child_idx);
        }
    }

    Ok(my_idx)
}

fn child_ids_ordered(graph: &impl GraphRead, symbols: &Symbols, parent: ThingId) -> Vec<ThingId> {
    let mut edges = [Edge::default(); 128];
    let count = graph.get_edges(parent, &mut edges);

    let mut rows: Vec<(u64, u64, ThingId)> = Vec::new();
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() != symbols.rel_has_child {
            continue;
        }
        let child = edge.to;
        let order = graph
            .get_prop(child, UI_ORDER_KEY)
            .or_else(|| graph.get_prop(child, keys::UI_RANK))
            .unwrap_or(u64::MAX);
        rows.push((order, child.to_u64_lossy(), child));
    }

    rows.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| a.1.cmp(&b.1))
    });

    rows.into_iter().map(|(_, _, id)| id).collect()
}

fn classify_node_kind(
    graph: &impl GraphRead,
    symbols: &Symbols,
    id: ThingId,
    kind_id: u64,
) -> UiNodeKind {
    let tag = graph.get_prop(id, keys::UI_KIND).unwrap_or(0);
    match tag {
        ui_kind::WINDOW => return UiNodeKind::Window,
        ui_kind::COLUMN => return UiNodeKind::Column,
        ui_kind::ROW => return UiNodeKind::Row,
        ui_kind::TEXT => return UiNodeKind::Text,
        ui_kind::TEXT_INPUT => return UiNodeKind::TextInput,
        ui_kind::BUTTON => return UiNodeKind::Button,
        ui_kind::CHECKBOX => return UiNodeKind::Checkbox,
        _ => {}
    }

    if kind_id == symbols.kind_window {
        UiNodeKind::Window
    } else if kind_id == symbols.kind_column {
        UiNodeKind::Column
    } else if kind_id == symbols.kind_text {
        UiNodeKind::Text
    } else if kind_id == symbols.kind_button {
        UiNodeKind::Button
    } else if kind_id == symbols.kind_checkbox {
        UiNodeKind::Checkbox
    } else {
        UiNodeKind::Box
    }
}

fn node_text(graph: &impl GraphRead, id: ThingId, kind: UiNodeKind) -> Option<String> {
    let bs = match kind {
        UiNodeKind::Text => graph.get_prop(id, keys::UI_TEXT).unwrap_or(0),
        UiNodeKind::TextInput => graph.get_prop(id, keys::UI_INPUT_VALUE).unwrap_or(0),
        UiNodeKind::Button => graph
            .get_prop(id, keys::UI_BUTTON_LABEL)
            .or_else(|| graph.get_prop(id, keys::UI_TEXT))
            .unwrap_or(0),
        UiNodeKind::Checkbox => graph
            .get_prop(id, keys::UI_CHECKBOX_LABEL)
            .or_else(|| graph.get_prop(id, keys::UI_TEXT))
            .unwrap_or(0),
        _ => graph.get_prop(id, keys::UI_TEXT).unwrap_or(0),
    };
    if bs == 0 {
        return None;
    }
    graph.read_bytespace_utf8(ThingId::from_u64(bs))
}

fn node_placeholder(graph: &impl GraphRead, id: ThingId, kind: UiNodeKind) -> Option<String> {
    if kind != UiNodeKind::TextInput {
        return None;
    }
    let bs = graph
        .get_prop(id, keys::UI_PLACEHOLDER_TEXT)
        .or_else(|| graph.get_prop(id, keys::UI_PLACEHOLDER))
        .unwrap_or(0);
    if bs == 0 {
        return None;
    }
    graph.read_bytespace_utf8(ThingId::from_u64(bs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;

    #[derive(Default)]
    struct MockGraph {
        kinds: BTreeMap<u64, u64>,
        props: BTreeMap<(u64, &'static str), u64>,
        edges: BTreeMap<u64, Vec<Edge>>,
        bytes: BTreeMap<u64, String>,
    }

    impl GraphRead for MockGraph {
        fn get_kind(&self, id: ThingId) -> Option<u64> {
            self.kinds.get(&id.to_u64_lossy()).copied()
        }

        fn get_prop(&self, id: ThingId, key: &str) -> Option<u64> {
            self.props.get(&(id.to_u64_lossy(), leak(key))).copied()
        }

        fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> usize {
            let Some(edges) = self.edges.get(&id.to_u64_lossy()) else {
                return 0;
            };
            let n = core::cmp::min(out.len(), edges.len());
            out[..n].copy_from_slice(&edges[..n]);
            n
        }

        fn read_bytespace_utf8(&self, id: ThingId) -> Option<String> {
            self.bytes.get(&id.to_u64_lossy()).cloned()
        }
    }

    fn leak(s: &str) -> &'static str {
        match s {
            keys::UI_KIND => keys::UI_KIND,
            keys::UI_TEXT => keys::UI_TEXT,
            keys::UI_INPUT_VALUE => keys::UI_INPUT_VALUE,
            keys::UI_PLACEHOLDER_TEXT => keys::UI_PLACEHOLDER_TEXT,
            keys::UI_VISIBLE => keys::UI_VISIBLE,
            keys::UI_ENABLED => keys::UI_ENABLED,
            keys::UI_GAP => keys::UI_GAP,
            keys::UI_PADDING => keys::UI_PADDING,
            keys::UI_WIDTH => keys::UI_WIDTH,
            keys::UI_HEIGHT => keys::UI_HEIGHT,
            _ => keys::UI_RANK,
        }
    }

    #[test]
    fn decode_tree_from_window_root() {
        let mut g = MockGraph::default();
        let window = ThingId::from_u64(1);
        let root = ThingId::from_u64(2);
        let label = ThingId::from_u64(3);
        let input = ThingId::from_u64(4);

        let rel_root_ui = 10;
        let rel_has_child = 11;

        g.kinds.insert(window.to_u64_lossy(), 100);
        g.kinds.insert(root.to_u64_lossy(), 101);
        g.kinds.insert(label.to_u64_lossy(), 102);
        g.kinds.insert(input.to_u64_lossy(), 103);

        g.props.insert((root.to_u64_lossy(), keys::UI_KIND), ui_kind::COLUMN);
        g.props.insert((label.to_u64_lossy(), keys::UI_KIND), ui_kind::TEXT);
        g.props.insert((input.to_u64_lossy(), keys::UI_KIND), ui_kind::TEXT_INPUT);
        g.props.insert((label.to_u64_lossy(), keys::UI_TEXT), 50);
        g.props.insert((input.to_u64_lossy(), keys::UI_INPUT_VALUE), 51);
        g.props.insert((input.to_u64_lossy(), keys::UI_PLACEHOLDER_TEXT), 52);

        g.bytes.insert(50, "Query".to_string());
        g.bytes.insert(51, "abc".to_string());
        g.bytes.insert(52, "Type here".to_string());

        g.edges.insert(
            window.to_u64_lossy(),
            alloc::vec![Edge {
                from: window,
                predicate: ThingId::from_u64(rel_root_ui),
                to: root,
            }],
        );
        g.edges.insert(
            root.to_u64_lossy(),
            alloc::vec![
                Edge {
                    from: root,
                    predicate: ThingId::from_u64(rel_has_child),
                    to: label,
                },
                Edge {
                    from: root,
                    predicate: ThingId::from_u64(rel_has_child),
                    to: input,
                },
            ],
        );

        let symbols = Symbols {
            rel_root_ui,
            rel_has_child,
            kind_button: 0,
            kind_checkbox: 0,
            kind_text: 0,
            kind_column: 0,
            kind_window: 0,
        };

        let tree = decode_window_tree_with(&g, &symbols, window).expect("decode");
        assert_eq!(tree.nodes.len(), 3);

        let root_node = &tree.nodes[tree.root];
        assert_eq!(root_node.kind, UiNodeKind::Column);
        assert_eq!(root_node.children.len(), 2);

        let first = &tree.nodes[root_node.children[0]];
        assert_eq!(first.kind, UiNodeKind::Text);
        assert_eq!(first.text.as_deref(), Some("Query"));

        let second = &tree.nodes[root_node.children[1]];
        assert_eq!(second.kind, UiNodeKind::TextInput);
        assert_eq!(second.text.as_deref(), Some("abc"));
        assert_eq!(second.placeholder.as_deref(), Some("Type here"));
    }
}
