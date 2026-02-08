extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::errors::{Error, Result};
use crate::thing::sys::{
    bytespace_create, bytespace_info, bytespace_read, bytespace_write, create_node, get_edges,
    link, prop_get, prop_set,
};
use crate::thing::ThingId;
use abi::errors::Errno;
use abi::ids::HandleId;
use abi::schema::{keys, kinds, rels, ui_kind};
use abi::ui_event::UI_EVENT_BYTES;
use abi::types::Edge;

pub trait GraphBackend {
    fn create_node(&mut self, kind: &str) -> Result<ThingId>;
    fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<()>;
    fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> Result<()>;
    fn prop_get(&mut self, id: ThingId, key: &str) -> Result<u64>;
    fn get_edges(&mut self, id: ThingId, out: &mut [Edge]) -> Result<usize>;
    fn bytespace_create(&mut self, len: usize) -> Result<ThingId>;
    fn bytespace_info(&mut self, id: ThingId) -> Result<usize>;
    fn bytespace_read(&mut self, id: ThingId, offset: usize, out: &mut [u8]) -> Result<usize>;
    fn bytespace_write(&mut self, id: ThingId, offset: usize, bytes: &[u8]) -> Result<()>;
}

pub struct SysGraph;

impl GraphBackend for SysGraph {
    fn create_node(&mut self, kind: &str) -> Result<ThingId> {
        create_node(kind).map_err(Error::Errno)
    }

    fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<()> {
        link(src, rel, dst).map_err(Error::Errno)
    }

    fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> Result<()> {
        prop_set(id, key, value).map_err(Error::Errno)
    }

    fn prop_get(&mut self, id: ThingId, key: &str) -> Result<u64> {
        prop_get(id, key).map_err(Error::Errno)
    }

    fn get_edges(&mut self, id: ThingId, out: &mut [Edge]) -> Result<usize> {
        get_edges(id, out).map_err(Error::Errno)
    }

    fn bytespace_create(&mut self, len: usize) -> Result<ThingId> {
        bytespace_create(len, 0, 0).map_err(Error::Errno)
    }

    fn bytespace_info(&mut self, id: ThingId) -> Result<usize> {
        bytespace_info(id).map_err(Error::Errno)
    }

    fn bytespace_read(&mut self, id: ThingId, offset: usize, out: &mut [u8]) -> Result<usize> {
        bytespace_read(id, offset, out).map_err(Error::Errno)
    }

    fn bytespace_write(&mut self, id: ThingId, offset: usize, bytes: &[u8]) -> Result<()> {
        bytespace_write(id, offset, bytes)
            .map(|_| ())
            .map_err(Error::Errno)
    }
}

pub struct Petals;

impl Petals {
    pub fn begin_window(window_id: ThingId) -> UiTreeBuilder<SysGraph> {
        UiTreeBuilder::new(SysGraph, window_id)
    }

    /// Apply the latest UI event for a window to graph-native widget state.
    pub fn reduce_window_events(window_id: ThingId) -> Result<bool> {
        reduce_window_events_with_graph(&mut SysGraph, window_id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiKey<'a>(pub &'a str);

impl<'a> UiKey<'a> {
    pub fn as_str(self) -> &'a str {
        self.0
    }
}

impl<'a> From<&'a str> for UiKey<'a> {
    fn from(value: &'a str) -> Self {
        UiKey(value)
    }
}

pub struct UiTreeBuilder<G: GraphBackend> {
    graph: G,
    window_id: ThingId,
    root: Option<ThingId>,
    parent_stack: Vec<ThingId>,
}

impl<G: GraphBackend> UiTreeBuilder<G> {
    pub fn new(graph: G, window_id: ThingId) -> Self {
        Self {
            graph,
            window_id,
            root: None,
            parent_stack: Vec::new(),
        }
    }

    pub fn column<F>(&mut self, f: F) -> Result<ThingId>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        let id = self.create_node(kinds::UI_COLUMN, ui_kind::COLUMN, None)?;
        let prev_parent = self.parent_stack.last().copied();
        self.attach_child(prev_parent, id)?;
        self.parent_stack.push(id);
        f(self)?;
        self.parent_stack.pop();
        Ok(id)
    }

    pub fn button(&mut self, label: &str, action_id: u64) -> Result<ThingId> {
        if label.is_empty() {
            return Err(Error::Errno(Errno::EINVAL));
        }
        let id = self.create_node(kinds::UI_BUTTON, ui_kind::BUTTON, None)?;
        self.parent_stack.push(id);
        let label_node = self.text_node(label)?;
        self.parent_stack.pop();
        self.attach_child(Some(id), label_node)?;
        self.graph
            .prop_set(id, keys::UI_BUTTON_LABEL, label_node.to_u64_lossy())?;
        self.graph
            .prop_set(id, keys::UI_BUTTON_ACTION_ID, action_id)?;
        self.attach_child(self.parent_stack.last().copied(), id)?;
        Ok(id)
    }

    pub fn checkbox(&mut self, label: &str, value_id: u64, checked: bool) -> Result<ThingId> {
        if label.is_empty() {
            return Err(Error::Errno(Errno::EINVAL));
        }
        let id = self.create_node(kinds::UI_CHECKBOX, ui_kind::CHECKBOX, None)?;
        self.parent_stack.push(id);
        let label_node = self.text_node(label)?;
        self.parent_stack.pop();
        self.attach_child(Some(id), label_node)?;
        self.graph
            .prop_set(id, keys::UI_CHECKBOX_LABEL, label_node.to_u64_lossy())?;
        self.graph
            .prop_set(id, keys::UI_CHECKBOX_CHECKED, if checked { 1 } else { 0 })?;
        self.graph
            .prop_set(id, keys::UI_CHECKBOX_VALUE_ID, value_id)?;
        self.graph
            .prop_set(id, keys::UI_CHECKBOX_INDETERMINATE, 0)?;
        self.attach_child(self.parent_stack.last().copied(), id)?;
        Ok(id)
    }

    pub fn row<F>(&mut self, f: F) -> Result<ThingId>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        let id = self.create_node(kinds::UI_NODE, ui_kind::ROW, None)?;
        let prev_parent = self.parent_stack.last().copied();
        self.attach_child(prev_parent, id)?;
        self.parent_stack.push(id);
        f(self)?;
        self.parent_stack.pop();
        Ok(id)
    }

    pub fn text(&mut self, text: &str) -> Result<ThingId> {
        if text.is_empty() {
            return Err(Error::Errno(Errno::EINVAL));
        }
        let id = self.text_node(text)?;
        self.attach_child(self.parent_stack.last().copied(), id)?;
        Ok(id)
    }

    pub fn text_input(&mut self, value: &str, placeholder: &str) -> Result<ThingId> {
        let id = self.create_node(kinds::UI_NODE, ui_kind::TEXT_INPUT, None)?;
        self.set_text_input_state(id, value, placeholder)?;
        self.attach_child(self.parent_stack.last().copied(), id)?;
        Ok(id)
    }

    pub fn text_input_keyed(
        &mut self,
        key: UiKey<'_>,
        value: &str,
        placeholder: &str,
    ) -> Result<ThingId> {
        let id = self.create_node(kinds::UI_NODE, ui_kind::TEXT_INPUT, Some(key.as_str()))?;
        self.set_text_input_state(id, value, placeholder)?;
        self.attach_child(self.parent_stack.last().copied(), id)?;
        Ok(id)
    }

    pub fn text_input_with_key(
        &mut self,
        key: &str,
        value: &str,
        placeholder: &str,
    ) -> Result<ThingId> {
        self.text_input_keyed(UiKey(key), value, placeholder)
    }

    pub fn key_node(&mut self, node: ThingId, key: UiKey<'_>) -> Result<()> {
        self.set_string_prop(node, keys::UI_KEY, key.as_str())
    }

    pub fn find_by_key(&mut self, key: UiKey<'_>) -> Result<Option<ThingId>> {
        self.find_keyed_node_under_window(key.as_str())
    }

    pub fn spacer(&mut self) -> Result<ThingId> {
        let id = self.create_node(kinds::UI_NODE, ui_kind::SPACER, None)?;
        self.attach_child(self.parent_stack.last().copied(), id)?;
        Ok(id)
    }

    pub fn separator(&mut self) -> Result<ThingId> {
        let id = self.create_node(kinds::UI_NODE, ui_kind::SEPARATOR, None)?;
        self.attach_child(self.parent_stack.last().copied(), id)?;
        Ok(id)
    }

    // ── Style helpers (call after creating a node) ──

    /// Set flex gap in pixels on the last-created container node.
    pub fn set_gap(&mut self, node: ThingId, gap: u64) -> Result<()> {
        self.graph.prop_set(node, keys::UI_GAP, gap)
    }

    /// Set uniform padding in pixels.
    pub fn set_padding(&mut self, node: ThingId, padding: u64) -> Result<()> {
        self.graph.prop_set(node, keys::UI_PADDING, padding)
    }

    /// Set flex align-items (0=Start, 1=Center, 2=End, 3=Stretch).
    pub fn set_align(&mut self, node: ThingId, align: u64) -> Result<()> {
        self.graph.prop_set(node, keys::UI_ALIGN, align)
    }

    /// Set flex justify-content (0=Start, 1=Center, 2=End, 3=SpaceBetween).
    pub fn set_justify(&mut self, node: ThingId, justify: u64) -> Result<()> {
        self.graph.prop_set(node, keys::UI_JUSTIFY, justify)
    }

    /// Set font name (stored as a bytespace string).
    pub fn set_font_name(&mut self, node: ThingId, name: &str) -> Result<()> {
        self.set_string_prop(node, keys::UI_FONT_NAME, name)
    }

    /// Set font size in pixels.
    pub fn set_font_size(&mut self, node: ThingId, size: u64) -> Result<()> {
        self.graph.prop_set(node, keys::UI_FONT_SIZE, size)
    }

    /// Set foreground color as 0xAARRGGBB.
    pub fn set_color(&mut self, node: ThingId, argb: u64) -> Result<()> {
        self.graph.prop_set(node, keys::UI_COLOR, argb)
    }

    pub fn finish(mut self) -> Result<ThingId> {
        self.finalize()
    }

    pub fn finish_with_graph(mut self) -> Result<(ThingId, G)> {
        let root = self.finalize()?;
        Ok((root, self.graph))
    }

    pub fn into_graph(self) -> G {
        self.graph
    }

    fn create_node(&mut self, kind: &str, ui_kind: u64, key: Option<&str>) -> Result<ThingId> {
        let id = match key {
            Some(k) if !k.is_empty() => self.find_or_create_keyed_node(k, kind)?,
            _ => self.graph.create_node(kind)?,
        };
        self.graph.prop_set(id, keys::UI_KIND, ui_kind)?;
        self.graph.prop_set(id, keys::UI_VISIBLE, 1)?;
        self.graph.prop_set(id, keys::UI_ENABLED, 1)?;
        if let Some(k) = key {
            if !k.is_empty() {
                self.set_string_prop(id, keys::UI_KEY, k)?;
            }
        }
        if self.root.is_none() {
            self.root = Some(id);
        } else if self.parent_stack.is_empty() {
            return Err(Error::Errno(Errno::EINVAL));
        }
        Ok(id)
    }

    fn text_node(&mut self, text: &str) -> Result<ThingId> {
        let id = self.create_node(kinds::UI_TEXT, ui_kind::TEXT, None)?;
        self.set_string_prop(id, keys::UI_TEXT, text)?;
        Ok(id)
    }

    fn set_text_input_state(&mut self, id: ThingId, value: &str, placeholder: &str) -> Result<()> {
        self.set_string_prop(id, keys::UI_TEXT, value)?;
        self.set_string_prop(id, keys::UI_INPUT_VALUE, value)?;
        self.set_string_prop(id, keys::UI_PLACEHOLDER, placeholder)?;
        self.set_string_prop(id, keys::UI_PLACEHOLDER_TEXT, placeholder)?;
        self.graph.prop_set(id, keys::UI_FOCUSABLE, 1)?;
        let cursor = value.len() as u64;
        self.graph.prop_set(id, keys::UI_CURSOR, cursor)?;
        self.graph.prop_set(id, keys::UI_CURSOR_POS, cursor)?;
        Ok(())
    }

    fn find_or_create_keyed_node(&mut self, key: &str, kind: &str) -> Result<ThingId> {
        if let Some(existing) = self.find_keyed_node_under_window(key)? {
            return Ok(existing);
        }
        self.graph.create_node(kind)
    }

    fn find_keyed_node_under_window(&mut self, key: &str) -> Result<Option<ThingId>> {
        let mut visited = BTreeSet::new();
        let mut stack = self.window_root_nodes()?;
        while let Some(id) = stack.pop() {
            if !visited.insert(id) {
                continue;
            }
            if self.node_key_matches(id, key)? {
                return Ok(Some(id));
            }
            let mut edges = [Edge::default(); 128];
            let count = self.graph.get_edges(id, &mut edges)?;
            for edge in edges.iter().take(count) {
                stack.push(edge.to);
            }
        }
        Ok(None)
    }

    fn window_root_nodes(&mut self) -> Result<Vec<ThingId>> {
        let mut roots = Vec::new();
        let mut edges = [Edge::default(); 64];
        let count = self.graph.get_edges(self.window_id, &mut edges)?;
        for edge in edges.iter().take(count) {
            roots.push(edge.to);
        }
        Ok(roots)
    }

    fn node_key_matches(&mut self, id: ThingId, key: &str) -> Result<bool> {
        Ok(read_string_prop(&mut self.graph, id, keys::UI_KEY)?
            .as_deref()
            .map(|value| value == key)
            .unwrap_or(false))
    }

    fn attach_child(&mut self, parent: Option<ThingId>, child: ThingId) -> Result<()> {
        if let Some(parent_id) = parent {
            self.graph.link(child, rels::CHILD_OF, parent_id)?;
            self.graph.link(parent_id, rels::HAS_CHILD, child)?;
        }
        Ok(())
    }

    fn set_string_prop(&mut self, id: ThingId, key: &str, value: &str) -> Result<()> {
        if value.is_empty() {
            self.graph.prop_set(id, key, 0)?;
            return Ok(());
        }
        let bs_id = self.graph.bytespace_create(value.len())?;
        self.graph.bytespace_write(bs_id, 0, value.as_bytes())?;
        self.graph.prop_set(id, key, bs_id.to_u64_lossy())?;
        Ok(())
    }

    fn ensure_event_queue(&mut self) -> Result<()> {
        let existing = self
            .graph
            .prop_get(self.window_id, keys::UI_EVENT_QUEUE)
            .unwrap_or(0);
        if existing == 0 {
            let bs_id = self.graph.bytespace_create(UI_EVENT_BYTES)?;
            self.graph
                .prop_set(self.window_id, keys::UI_EVENT_QUEUE, bs_id.to_u64_lossy())?;
            self.graph.prop_set(self.window_id, keys::UI_EVENT_GEN, 0)?;
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<ThingId> {
        if let Some(root) = self.root {
            self.ensure_event_queue()?;
            self.graph.link(self.window_id, rels::ROOT_UI, root)?;
            self.graph.link(root, rels::CHILD_OF, self.window_id)?;
            self.graph.link(self.window_id, rels::HAS_CHILD, root)?;
            let current = self
                .graph
                .prop_get(self.window_id, keys::UI_SCENE_GEN)
                .unwrap_or(0);
            self.graph.prop_set(
                self.window_id,
                keys::UI_SCENE_GEN,
                current.saturating_add(1),
            )?;
            Ok(root)
        } else {
            Err(Error::Errno(Errno::EINVAL))
        }
    }
}

fn read_string_prop<G: GraphBackend>(graph: &mut G, id: ThingId, key: &str) -> Result<Option<String>> {
    let bs = graph.prop_get(id, key).unwrap_or(0);
    if bs == 0 {
        return Ok(None);
    }
    let bs_id = ThingId::from_u64(bs);
    let len = graph.bytespace_info(bs_id)?;
    if len == 0 {
        return Ok(Some(String::new()));
    }
    let mut buf = vec![0u8; len];
    let read = graph.bytespace_read(bs_id, 0, &mut buf)?;
    buf.truncate(read);
    let text = core::str::from_utf8(&buf)
        .ok()
        .map(String::from)
        .unwrap_or_else(String::new);
    Ok(Some(text))
}

fn write_string_prop<G: GraphBackend>(graph: &mut G, id: ThingId, key: &str, value: &str) -> Result<()> {
    if value.is_empty() {
        graph.prop_set(id, key, 0)?;
        return Ok(());
    }
    let bs_id = graph.bytespace_create(value.len())?;
    graph.bytespace_write(bs_id, 0, value.as_bytes())?;
    graph.prop_set(id, key, bs_id.to_u64_lossy())
}

fn window_root_nodes<G: GraphBackend>(graph: &mut G, window_id: ThingId) -> Result<Vec<ThingId>> {
    let mut roots = Vec::new();
    let mut edges = [Edge::default(); 64];
    let count = graph.get_edges(window_id, &mut edges)?;
    for edge in edges.iter().take(count) {
        roots.push(edge.to);
    }
    Ok(roots)
}

fn collect_window_nodes<G: GraphBackend>(graph: &mut G, window_id: ThingId) -> Result<Vec<ThingId>> {
    let mut visited = BTreeSet::new();
    let mut ordered = Vec::new();
    let mut stack = window_root_nodes(graph, window_id)?;
    while let Some(id) = stack.pop() {
        if !visited.insert(id) {
            continue;
        }
        ordered.push(id);
        let mut edges = [Edge::default(); 128];
        let count = graph.get_edges(id, &mut edges)?;
        for edge in edges.iter().take(count) {
            stack.push(edge.to);
        }
    }
    Ok(ordered)
}

fn clear_focus_in_window<G: GraphBackend>(graph: &mut G, window_id: ThingId) -> Result<()> {
    for id in collect_window_nodes(graph, window_id)? {
        if graph.prop_get(id, keys::UI_FOCUSABLE).unwrap_or(0) != 0
            || graph.prop_get(id, keys::UI_FOCUSED).unwrap_or(0) != 0
        {
            graph.prop_set(id, keys::UI_FOCUSED, 0)?;
        }
    }
    Ok(())
}

fn clamp_cursor(text: &str, cursor: usize) -> usize {
    core::cmp::min(cursor, text.len())
}

fn previous_char_start(text: &str, cursor: usize) -> usize {
    if cursor == 0 {
        return 0;
    }
    let mut idx = 0usize;
    for (byte_idx, _) in text.char_indices() {
        if byte_idx >= cursor {
            break;
        }
        idx = byte_idx;
    }
    idx
}

fn next_char_end(text: &str, cursor: usize) -> usize {
    if cursor >= text.len() {
        return text.len();
    }
    for (byte_idx, ch) in text[cursor..].char_indices() {
        if byte_idx == 0 {
            return cursor + ch.len_utf8();
        }
    }
    text.len()
}

pub fn reduce_window_events_with_graph<G: GraphBackend>(graph: &mut G, window_id: ThingId) -> Result<bool> {
    use abi::ui_event::{UiEventKind, UiEventWire};

    let queue_bs = graph.prop_get(window_id, keys::UI_EVENT_QUEUE).unwrap_or(0);
    if queue_bs == 0 {
        return Ok(false);
    }
    let mut buf = [0u8; UI_EVENT_BYTES];
    let read = graph.bytespace_read(ThingId::from_u64(queue_bs), 0, &mut buf)?;
    if read < UI_EVENT_BYTES {
        return Ok(false);
    }
    let Some(event) = UiEventWire::decode(&buf) else {
        return Ok(false);
    };
    if event.window_id != window_id.to_u64_lossy() {
        return Ok(false);
    }
    let target = ThingId::from_u64(event.target_id);
    let Some(kind) = UiEventKind::from_raw(event.kind) else {
        return Ok(false);
    };
    match kind {
        UiEventKind::Focus => {
            clear_focus_in_window(graph, window_id)?;
            graph.prop_set(target, keys::UI_FOCUSED, 1)?;
            if graph.prop_get(target, keys::UI_CURSOR).unwrap_or(0) == 0 {
                let current_text = read_string_prop(graph, target, keys::UI_TEXT)?.unwrap_or_default();
                let cursor = current_text.len() as u64;
                graph.prop_set(target, keys::UI_CURSOR, cursor)?;
                graph.prop_set(target, keys::UI_CURSOR_POS, cursor)?;
            }
        }
        UiEventKind::Blur => {
            graph.prop_set(target, keys::UI_FOCUSED, 0)?;
        }
        UiEventKind::TextInsert => {
            let mut text = read_string_prop(graph, target, keys::UI_TEXT)?.unwrap_or_default();
            let cursor = clamp_cursor(&text, graph.prop_get(target, keys::UI_CURSOR).unwrap_or(0) as usize);
            let insert = core::str::from_utf8(event.text_bytes()).unwrap_or("");
            if !insert.is_empty() {
                text.insert_str(cursor, insert);
                let next_cursor = cursor.saturating_add(insert.len()) as u64;
                write_string_prop(graph, target, keys::UI_TEXT, &text)?;
                write_string_prop(graph, target, keys::UI_INPUT_VALUE, &text)?;
                graph.prop_set(target, keys::UI_CURSOR, next_cursor)?;
                graph.prop_set(target, keys::UI_CURSOR_POS, next_cursor)?;
            }
        }
        UiEventKind::TextBackspace => {
            let mut text = read_string_prop(graph, target, keys::UI_TEXT)?.unwrap_or_default();
            let cursor = clamp_cursor(&text, graph.prop_get(target, keys::UI_CURSOR).unwrap_or(0) as usize);
            if cursor > 0 {
                let prev = previous_char_start(&text, cursor);
                text.replace_range(prev..cursor, "");
                write_string_prop(graph, target, keys::UI_TEXT, &text)?;
                write_string_prop(graph, target, keys::UI_INPUT_VALUE, &text)?;
                graph.prop_set(target, keys::UI_CURSOR, prev as u64)?;
                graph.prop_set(target, keys::UI_CURSOR_POS, prev as u64)?;
            }
        }
        UiEventKind::TextDelete => {
            let mut text = read_string_prop(graph, target, keys::UI_TEXT)?.unwrap_or_default();
            let cursor = clamp_cursor(&text, graph.prop_get(target, keys::UI_CURSOR).unwrap_or(0) as usize);
            let next = next_char_end(&text, cursor);
            if next > cursor {
                text.replace_range(cursor..next, "");
                write_string_prop(graph, target, keys::UI_TEXT, &text)?;
                write_string_prop(graph, target, keys::UI_INPUT_VALUE, &text)?;
                graph.prop_set(target, keys::UI_CURSOR, cursor as u64)?;
                graph.prop_set(target, keys::UI_CURSOR_POS, cursor as u64)?;
            }
        }
        UiEventKind::CursorMove => {
            let text = read_string_prop(graph, target, keys::UI_TEXT)?.unwrap_or_default();
            let cursor = clamp_cursor(&text, graph.prop_get(target, keys::UI_CURSOR).unwrap_or(0) as usize);
            let mut next = cursor as i64 + event.delta as i64;
            next = next.clamp(0, text.len() as i64);
            graph.prop_set(target, keys::UI_CURSOR, next as u64)?;
            graph.prop_set(target, keys::UI_CURSOR_POS, next as u64)?;
        }
        UiEventKind::Submit => {}
        UiEventKind::Clicked | UiEventKind::Toggled => {}
    }
    let current = graph.prop_get(window_id, keys::UI_SCENE_GEN).unwrap_or(0);
    graph.prop_set(window_id, keys::UI_SCENE_GEN, current.saturating_add(1))?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::types::Edge;
    use alloc::collections::BTreeMap;
    use alloc::string::String;

    #[derive(Default)]
    struct FakeGraph {
        next_id: u64,
        props: BTreeMap<(u64, String), u64>,
        edges: Vec<Edge>,
        bytespaces: BTreeMap<u64, Vec<u8>>,
    }

    impl FakeGraph {
        fn new() -> Self {
            Self {
                next_id: 1,
                props: BTreeMap::new(),
                edges: Vec::new(),
                bytespaces: BTreeMap::new(),
            }
        }
    }

    impl GraphBackend for FakeGraph {
        fn create_node(&mut self, _kind: &str) -> Result<ThingId> {
            let id = ThingId::from_u64(self.next_id);
            self.next_id += 1;
            Ok(id)
        }

        fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<()> {
            let pred = ThingId::from_u64(match rel {
                rels::HAS_CHILD => 1,
                rels::CHILD_OF => 2,
                rels::ROOT_UI => 3,
                _ => 0,
            });
            self.edges.push(Edge {
                from: src,
                predicate: pred,
                to: dst,
                flags: 0,
            });
            Ok(())
        }

        fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> Result<()> {
            self.props.insert((id.to_u64_lossy(), String::from(key)), value);
            Ok(())
        }

        fn prop_get(&mut self, id: ThingId, key: &str) -> Result<u64> {
            Ok(self
                .props
                .get(&(id.to_u64_lossy(), String::from(key)))
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
                let end = offset.saturating_add(bytes.len());
                if end > buf.len() {
                    buf.resize(end, 0);
                }
                buf[offset..end].copy_from_slice(bytes);
                Ok(())
            } else {
                Err(Error::Errno(Errno::ENOENT))
            }
        }
    }

    #[test]
    fn button_requires_label() {
        let mut builder = UiTreeBuilder::new(FakeGraph::new(), ThingId::from_u64(10));
        let err = builder.button("", 42).unwrap_err();
        assert_eq!(err, Error::Errno(Errno::EINVAL));
    }

    #[test]
    fn tree_is_connected_with_parent_links() {
        let graph = FakeGraph::new();
        let mut builder = UiTreeBuilder::new(graph, ThingId::from_u64(10));
        builder
            .column(|b| {
                b.checkbox("One", 1, false)?;
                b.checkbox("Two", 2, true)?;
                b.button("Go", 9)?;
                Ok(())
            })
            .unwrap();
        let (_root, graph) = builder.finish_with_graph().unwrap();
        let child_edges = graph
            .edges
            .iter()
            .filter(|e| e.predicate.to_u64_lossy() == 2)
            .count();
        assert!(child_edges >= 3);
    }

    #[test]
    fn stable_ids_for_same_build_order() {
        fn build_ids() -> Vec<u64> {
            let graph = FakeGraph::new();
            let mut builder = UiTreeBuilder::new(graph, ThingId::from_u64(10));
            builder
                .column(|b| {
                    b.checkbox("One", 1, false)?;
                    b.checkbox("Two", 2, true)?;
                    b.button("Go", 9)?;
                    Ok(())
                })
                .unwrap();
            let (_root, graph) = builder.finish_with_graph().unwrap();
            graph.props.keys().map(|(id, _)| *id).collect()
        }
        assert_eq!(build_ids(), build_ids());
    }

    #[test]
    fn keyed_text_input_reuses_thing_id_across_rebuilds() {
        let mut graph = FakeGraph::new();
        let window = ThingId::from_u64(44);

        let mut first = UiTreeBuilder::new(graph, window);
        let mut first_input = ThingId::default();
        first
            .column(|ui| {
                first_input = ui.text_input_keyed(UiKey("query_input"), "", "Search")?;
                Ok(())
            })
            .unwrap();
        let (_root, first_graph) = first.finish_with_graph().unwrap();
        graph = first_graph;

        let mut second = UiTreeBuilder::new(graph, window);
        let mut second_input = ThingId::default();
        second
            .column(|ui| {
                second_input = ui.text_input_keyed(UiKey("query_input"), "abc", "Search")?;
                Ok(())
            })
            .unwrap();
        assert_eq!(first_input, second_input);
    }

    #[test]
    fn keyed_text_input_reuse_survives_sibling_reorder() {
        let mut graph = FakeGraph::new();
        let window = ThingId::from_u64(55);

        let mut first = UiTreeBuilder::new(graph, window);
        let mut input_a = ThingId::default();
        first
            .column(|ui| {
                ui.text("A")?;
                input_a = ui.text_input_keyed(UiKey("query_input"), "", "Search")?;
                ui.text("B")?;
                Ok(())
            })
            .unwrap();
        let (_root, first_graph) = first.finish_with_graph().unwrap();
        graph = first_graph;

        let mut second = UiTreeBuilder::new(graph, window);
        let mut input_b = ThingId::default();
        second
            .column(|ui| {
                ui.text("B")?;
                input_b = ui.text_input_keyed(UiKey("query_input"), "", "Search")?;
                ui.text("A")?;
                Ok(())
            })
            .unwrap();
        assert_eq!(input_a, input_b);
    }

    #[test]
    fn reducer_applies_focus_insert_and_backspace() {
        let graph = FakeGraph::new();
        let window = ThingId::from_u64(99);
        let mut builder = UiTreeBuilder::new(graph, window);
        let mut input = ThingId::default();
        builder
            .column(|ui| {
                input = ui.text_input_keyed(UiKey("query_input"), "", "Search")?;
                Ok(())
            })
            .unwrap();
        let (_root, mut graph) = builder.finish_with_graph().unwrap();

        let focus = abi::ui_event::UiEventWire::new_focus(window.to_u64_lossy(), input.to_u64_lossy());
        let mut buf = [0u8; UI_EVENT_BYTES];
        focus.encode(&mut buf).unwrap();
        let queue = graph.prop_get(window, keys::UI_EVENT_QUEUE).unwrap();
        graph.bytespace_write(ThingId::from_u64(queue), 0, &buf).unwrap();
        assert!(reduce_window_events_with_graph(&mut graph, window).unwrap());

        let insert = abi::ui_event::UiEventWire::new_text_insert(
            window.to_u64_lossy(),
            input.to_u64_lossy(),
            b"hi",
        );
        insert.encode(&mut buf).unwrap();
        graph.bytespace_write(ThingId::from_u64(queue), 0, &buf).unwrap();
        assert!(reduce_window_events_with_graph(&mut graph, window).unwrap());

        let backspace = abi::ui_event::UiEventWire::new_text_backspace(
            window.to_u64_lossy(),
            input.to_u64_lossy(),
        );
        backspace.encode(&mut buf).unwrap();
        graph.bytespace_write(ThingId::from_u64(queue), 0, &buf).unwrap();
        assert!(reduce_window_events_with_graph(&mut graph, window).unwrap());

        let text = read_string_prop(&mut graph, input, keys::UI_TEXT)
            .unwrap()
            .unwrap_or_default();
        assert_eq!(text, "h");
        assert_eq!(graph.prop_get(input, keys::UI_CURSOR).unwrap(), 1);
    }
}
