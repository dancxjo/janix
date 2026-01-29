extern crate alloc;

use alloc::vec::Vec;

use crate::errors::{Error, Result};
use crate::thing::sys::{bytespace_create, bytespace_write, create_node, link, prop_get, prop_set};
use crate::thing::ThingId;
use abi::errors::Errno;
use abi::ids::HandleId;
use abi::schema::{keys, kinds, rels, ui_kind};
use abi::ui_event::UI_EVENT_BYTES;

pub trait GraphBackend {
    fn create_node(&mut self, kind: &str) -> Result<ThingId>;
    fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<()>;
    fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> Result<()>;
    fn prop_get(&mut self, id: ThingId, key: &str) -> Result<u64>;
    fn bytespace_create(&mut self, len: usize) -> Result<ThingId>;
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

    fn bytespace_create(&mut self, len: usize) -> Result<ThingId> {
        bytespace_create(len, 0, 0).map_err(Error::Errno)
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
}

pub struct UiTreeBuilder<G: GraphBackend> {
    graph: G,
    window_id: ThingId,
    root: Option<ThingId>,
    parent_stack: Vec<ThingId>,
    created: Vec<ThingId>,
}

impl<G: GraphBackend> UiTreeBuilder<G> {
    pub fn new(graph: G, window_id: ThingId) -> Self {
        Self {
            graph,
            window_id,
            root: None,
            parent_stack: Vec::new(),
            created: Vec::new(),
        }
    }

    pub fn column<F>(&mut self, f: F) -> Result<ThingId>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        let id = self.create_node(kinds::UI_COLUMN, ui_kind::COLUMN)?;
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
        let id = self.create_node(kinds::UI_BUTTON, ui_kind::BUTTON)?;
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
        let id = self.create_node(kinds::UI_CHECKBOX, ui_kind::CHECKBOX)?;
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

    pub fn text(&mut self, text: &str) -> Result<ThingId> {
        if text.is_empty() {
            return Err(Error::Errno(Errno::EINVAL));
        }
        let id = self.text_node(text)?;
        self.attach_child(self.parent_stack.last().copied(), id)?;
        Ok(id)
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

    fn create_node(&mut self, kind: &str, ui_kind: u64) -> Result<ThingId> {
        let id = self.graph.create_node(kind)?;
        self.graph.prop_set(id, keys::UI_KIND, ui_kind)?;
        self.graph.prop_set(id, keys::UI_VISIBLE, 1)?;
        self.graph.prop_set(id, keys::UI_ENABLED, 1)?;
        self.created.push(id);
        if self.root.is_none() {
            self.root = Some(id);
        } else if self.parent_stack.is_empty() {
            return Err(Error::Errno(Errno::EINVAL));
        }
        Ok(id)
    }

    fn text_node(&mut self, text: &str) -> Result<ThingId> {
        let id = self.create_node(kinds::UI_TEXT, ui_kind::TEXT)?;
        self.set_string_prop(id, keys::UI_TEXT, text)?;
        Ok(id)
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

#[cfg(test)]
mod tests {
    use super::*;
    use abi::types::Edge;
    use alloc::string::String;
    use alloc::collections::BTreeMap;

    #[derive(Default)]
    struct FakeGraph {
        next_id: u64,
        props: BTreeMap<(u64, String), u64>,
        edges: Vec<Edge>,
    }

    impl FakeGraph {
        fn new() -> Self {
            Self {
                next_id: 1,
                props: BTreeMap::new(),
                edges: Vec::new(),
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

        fn bytespace_create(&mut self, _len: usize) -> Result<ThingId> {
            let id = ThingId::from_u64(self.next_id);
            self.next_id += 1;
            Ok(id)
        }

        fn bytespace_write(&mut self, _id: ThingId, _offset: usize, _bytes: &[u8]) -> Result<()> {
            Ok(())
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
}
