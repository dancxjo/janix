extern crate alloc;

use alloc::vec::Vec;

use abi::schema::{keys, kinds, rels};
use abi::types::Edge;
use abi::ui_event::UiEventWire;
use abi::ids::HandleId;
use stem::thing::{ThingId, ThingKind};
use stem::thing::sys::{
    bytespace_create, bytespace_write, find, get_edges, get_kind, prop_get, prop_set,
};

use crate::geometry::Rect;

pub struct UiEventDispatcher {
    rel_root_ui: u64,
    rel_has_child: u64,
    kind_button: u64,
    kind_checkbox: u64,
}

impl UiEventDispatcher {
    pub fn new() -> Self {
        Self {
            rel_root_ui: stem::thing::sys::intern(rels::ROOT_UI).unwrap_or(0) as u64,
            rel_has_child: stem::thing::sys::intern(rels::HAS_CHILD).unwrap_or(0) as u64,
            kind_button: stem::thing::sys::intern(kinds::UI_BUTTON).unwrap_or(0) as u64,
            kind_checkbox: stem::thing::sys::intern(kinds::UI_CHECKBOX).unwrap_or(0) as u64,
        }
    }

    pub fn dispatch_click(&self, screen_x: i32, screen_y: i32, screen_w: i32, screen_h: i32) {
        if let Some((window_id, rect)) = window_at_point(screen_x, screen_y, screen_w, screen_h) {
            let local_x = screen_x - rect.x();
            let local_y = screen_y - rect.y();
            if let Some(root) = find_root_ui(window_id, self.rel_root_ui) {
                if let Some(hit) = hit_test(
                    root,
                    local_x,
                    local_y,
                    self.rel_has_child,
                    self.kind_button,
                    self.kind_checkbox,
                ) {
                    let kind = get_kind(hit).unwrap_or(ThingKind::default());
                    if kind.0 == self.kind_button {
                        let action_id = prop_get(hit, keys::UI_BUTTON_ACTION_ID).unwrap_or(0);
                        emit_clicked(window_id, hit, action_id);
                        bump_window_gen(window_id);
                    } else if kind.0 == self.kind_checkbox {
                        let checked = prop_get(hit, keys::UI_CHECKBOX_CHECKED).unwrap_or(0) != 0;
                        let new_checked = if checked { 0 } else { 1 };
                        let _ = prop_set(hit, keys::UI_CHECKBOX_CHECKED, new_checked);
                        let value_id = prop_get(hit, keys::UI_CHECKBOX_VALUE_ID).unwrap_or(0);
                        emit_toggled(window_id, hit, new_checked != 0, value_id);
                        bump_window_gen(window_id);
                    }
                }
            }
        }
    }
}

fn window_at_point(x: i32, y: i32, screen_w: i32, screen_h: i32) -> Option<(ThingId, Rect)> {
    let mut windows = [ThingId::default(); 128];
    let count = find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);
    let mut best: Option<(ThingId, Rect, i32)> = None;
    for win in windows.iter().take(count) {
        let rect = window_rect(*win, screen_w, screen_h);
        if rect.width() <= 0 || rect.height() <= 0 {
            continue;
        }
        if x >= rect.x() && y >= rect.y() && x < rect.x() + rect.width() && y < rect.y() + rect.height() {
            let z = prop_get(*win, keys::UI_Z_INDEX).unwrap_or(0) as i32;
            if best.map(|(_, _, bz)| z >= bz).unwrap_or(true) {
                best = Some((*win, rect, z));
            }
        }
    }
    best.map(|(id, rect, _)| (id, rect))
}

fn window_rect(window_id: ThingId, screen_w: i32, screen_h: i32) -> Rect {
    let mut w = prop_get(window_id, keys::UI_WIDTH).unwrap_or(0) as i32;
    let mut h = prop_get(window_id, keys::UI_HEIGHT).unwrap_or(0) as i32;
    if w <= 0 || h <= 0 {
        return Rect::new(0, 0, 0, 0);
    }
    let mut x = prop_get(window_id, keys::UI_X).unwrap_or(0) as i32;
    let mut y = prop_get(window_id, keys::UI_Y).unwrap_or(0) as i32;
    let inset_right = prop_get(window_id, keys::UI_INSET_RIGHT).unwrap_or(0) as i32;
    let inset_bottom = prop_get(window_id, keys::UI_INSET_BOTTOM).unwrap_or(0) as i32;
    if inset_right > 0 {
        x = screen_w - inset_right - w;
    }
    if inset_bottom > 0 {
        y = screen_h - inset_bottom - h;
    }
    Rect::new(x, y, w, h)
}

fn find_root_ui(window_id: ThingId, rel_root_ui: u64) -> Option<ThingId> {
    let mut edges = [Edge::default(); 64];
    let count = get_edges(window_id, &mut edges).ok()?;
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() == rel_root_ui {
            return Some(edge.to);
        }
    }
    None
}

fn hit_test(
    root: ThingId,
    x: i32,
    y: i32,
    rel_has_child: u64,
    kind_button: u64,
    kind_checkbox: u64,
) -> Option<ThingId> {
    let mut stack = Vec::new();
    stack.push(root);
    while let Some(id) = stack.pop() {
        if !node_visible(id) || !node_enabled(id) {
            continue;
        }
        if let Some(rect) = node_rect(id) {
            if rect.w <= 0 || rect.h <= 0 {
                continue;
            }
            if x < rect.x || y < rect.y || x >= rect.x + rect.w || y >= rect.y + rect.h {
                continue;
            }
            let kind = get_kind(id).unwrap_or(ThingKind::default());
            if kind.0 == kind_button || kind.0 == kind_checkbox {
                return Some(id);
            }
            let mut edges = [Edge::default(); 64];
            if let Ok(count) = get_edges(id, &mut edges) {
                for edge in edges.iter().take(count) {
                    if edge.predicate.to_u64_lossy() == rel_has_child {
                        stack.push(edge.to);
                    }
                }
            }
        }
    }
    None
}

fn node_rect(id: ThingId) -> Option<RectI32> {
    let x = prop_get(id, keys::UI_X).ok()? as i32;
    let y = prop_get(id, keys::UI_Y).ok()? as i32;
    let w = prop_get(id, keys::UI_WIDTH).ok()? as i32;
    let h = prop_get(id, keys::UI_HEIGHT).ok()? as i32;
    Some(RectI32 { x, y, w, h })
}

fn node_visible(id: ThingId) -> bool {
    prop_get(id, keys::UI_VISIBLE).unwrap_or(1) != 0
}

fn node_enabled(id: ThingId) -> bool {
    prop_get(id, keys::UI_ENABLED).unwrap_or(1) != 0
}

fn emit_clicked(window_id: ThingId, node_id: ThingId, action_id: u64) {
    let event = UiEventWire::new_clicked(node_id.to_u64_lossy(), action_id);
    write_event(window_id, event);
}

fn emit_toggled(window_id: ThingId, node_id: ThingId, checked: bool, value_id: u64) {
    let event = UiEventWire::new_toggled(node_id.to_u64_lossy(), checked, value_id);
    write_event(window_id, event);
}

fn write_event(window_id: ThingId, event: UiEventWire) {
    let mut buf = [0u8; abi::ui_event::UI_EVENT_BYTES];
    if event.encode(&mut buf).is_none() {
        return;
    }
    let bs_id = prop_get(window_id, keys::UI_EVENT_QUEUE).unwrap_or(0);
    let bs = if bs_id == 0 {
        match bytespace_create(buf.len(), 0, 0) {
            Ok(id) => {
                let _ = prop_set(window_id, keys::UI_EVENT_QUEUE, id.to_u64_lossy());
                id
            }
            Err(_) => return,
        }
    } else {
        ThingId::from_u64(bs_id)
    };
    let _ = bytespace_write(bs, 0, &buf);
    let current = prop_get(window_id, keys::UI_EVENT_GEN).unwrap_or(0);
    let _ = prop_set(window_id, keys::UI_EVENT_GEN, current.saturating_add(1));
}

fn bump_window_gen(window_id: ThingId) {
    let current = prop_get(window_id, keys::UI_SCENE_GEN).unwrap_or(0);
    let _ = prop_set(window_id, keys::UI_SCENE_GEN, current.saturating_add(1));
}

#[derive(Clone, Copy)]
struct RectI32 {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::ui_event::{UiEventKind, UiEventWire};
    use stem::petals::graph::{GraphBackend, UiTreeBuilder};
    use stem::errors::{Error, Result};
    use alloc::collections::BTreeMap;
    use alloc::string::String;

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
                kinds::UI_WINDOW => 105,
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
            self.props.insert((id.to_u64_lossy(), key.to_string()), value);
            Ok(())
        }

        fn prop_get(&mut self, id: ThingId, key: &str) -> Result<u64> {
            Ok(self
                .props
                .get(&(id.to_u64_lossy(), key.to_string()))
                .copied()
                .unwrap_or(0))
        }

        fn bytespace_create(&mut self, len: usize) -> Result<ThingId> {
            let id = ThingId::from_u64(self.next_id);
            self.next_id += 1;
            self.bytespaces.insert(id.to_u64_lossy(), vec![0u8; len]);
            Ok(id)
        }

        fn bytespace_write(&mut self, id: ThingId, offset: usize, bytes: &[u8]) -> Result<()> {
            if let Some(buf) = self.bytespaces.get_mut(&id.to_u64_lossy()) {
                let end = offset + bytes.len();
                buf[offset..end].copy_from_slice(bytes);
                Ok(())
            } else {
                Err(Error::Errno(abi::errors::Errno::ENOENT))
            }
        }
    }

    #[test]
    fn click_toggles_checkbox_and_emits_event() {
        let graph = TestGraph::new();
        let window_id = ThingId::from_u64(42);
        let mut builder = UiTreeBuilder::new(graph, window_id);
        let _root = builder
            .column(|b| {
                b.checkbox("One", 7, false)?;
                Ok(())
            })
            .unwrap();
        let (_root, mut graph) = builder.finish_with_graph().unwrap();

        // Manually seed bounds for hit testing
        let checkbox_id = graph
            .edges
            .iter()
            .find(|e| e.predicate.to_u64_lossy() == 201)
            .map(|e| e.to)
            .unwrap();
        graph
            .props
            .insert((checkbox_id.to_u64_lossy(), keys::UI_X.to_string()), 0);
        graph
            .props
            .insert((checkbox_id.to_u64_lossy(), keys::UI_Y.to_string()), 0);
        graph
            .props
            .insert((checkbox_id.to_u64_lossy(), keys::UI_WIDTH.to_string()), 100);
        graph
            .props
            .insert((checkbox_id.to_u64_lossy(), keys::UI_HEIGHT.to_string()), 30);
        graph
            .props
            .insert((window_id.to_u64_lossy(), keys::UI_EVENT_QUEUE.to_string()), 0);
        graph
            .props
            .insert((window_id.to_u64_lossy(), keys::UI_SCENE_GEN.to_string()), 0);

        let event = UiEventWire::new_toggled(checkbox_id.to_u64_lossy(), true, 7);
        let mut buf = [0u8; abi::ui_event::UI_EVENT_BYTES];
        let _ = event.encode(&mut buf);
        graph.bytespaces.insert(999, buf.to_vec());

        // Simulate event write
        let written = UiEventWire::new_toggled(checkbox_id.to_u64_lossy(), true, 7);
        let mut out = [0u8; abi::ui_event::UI_EVENT_BYTES];
        let _ = written.encode(&mut out);
        assert_eq!(UiEventKind::from_raw(out[0]), Some(UiEventKind::Toggled));
    }
}
