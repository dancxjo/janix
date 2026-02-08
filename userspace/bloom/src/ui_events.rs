extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::string::ToString;
use alloc::vec::Vec;

use abi::hid::Key;
use abi::ids::HandleId;
use abi::schema::{keys, kinds, rels};
use abi::types::Edge;
use abi::ui_event::{self, UiEvent};
use stem::thing::sys::{
    bytespace_create, bytespace_write, find, get_edges, get_kind, prop_get, prop_set,
};
use stem::thing::{ThingId, ThingKind};

use crate::geometry::Rect;

pub struct UiEventDispatcher {
    rel_root_ui: u64,
    rel_has_child: u64,
    kind_button: u64,
    kind_checkbox: u64,
    focused_text_target: Option<(ThingId, ThingId)>,
}

impl UiEventDispatcher {
    pub fn new() -> Self {
        Self {
            rel_root_ui: stem::thing::sys::intern(rels::ROOT_UI).unwrap_or(0) as u64,
            rel_has_child: stem::thing::sys::intern(rels::HAS_CHILD).unwrap_or(0) as u64,
            kind_button: stem::thing::sys::intern(kinds::UI_BUTTON).unwrap_or(0) as u64,
            kind_checkbox: stem::thing::sys::intern(kinds::UI_CHECKBOX).unwrap_or(0) as u64,
            focused_text_target: None,
        }
    }

    pub fn dispatch_click(&mut self, screen_x: i32, screen_y: i32, screen_w: i32, screen_h: i32) {
        if let Some((window_id, rect)) = window_at_point(screen_x, screen_y, screen_w, screen_h) {
            let local_x = screen_x - rect.x();
            let local_y = screen_y - rect.y();
            if let Some(root) = find_root_ui(window_id, self.rel_root_ui) {
                if let Some((hit, hit_kind)) = hit_test(
                    root,
                    local_x,
                    local_y,
                    self.rel_has_child,
                    self.kind_button,
                    self.kind_checkbox,
                ) {
                    if hit_kind == HitKind::TextInput {
                        self.focused_text_target = Some((window_id, hit));
                        emit_focus(window_id, hit);
                        bump_window_gen(window_id);
                    } else if hit_kind == HitKind::Button {
                        let action_id = prop_get(hit, keys::UI_BUTTON_ACTION_ID).unwrap_or(0);
                        emit_clicked(window_id, hit, action_id);
                        bump_window_gen(window_id);
                    } else if hit_kind == HitKind::Checkbox {
                        let checked = prop_get(hit, keys::UI_CHECKBOX_CHECKED).unwrap_or(0) != 0;
                        let new_checked = if checked { 0 } else { 1 };
                        let _ = prop_set(hit, keys::UI_CHECKBOX_CHECKED, new_checked);
                        let value_id = prop_get(hit, keys::UI_CHECKBOX_VALUE_ID).unwrap_or(0);
                        emit_toggled(window_id, hit, new_checked != 0, value_id);
                        bump_window_gen(window_id);
                    }
                } else {
                    self.clear_focus();
                }
            }
        }
    }

    pub fn dispatch_keyboard(&self, pressed: &BTreeSet<Key>, prev: &BTreeSet<Key>) {
        let Some((window_id, target_id)) = self.focused_text_target else {
            return;
        };
        let shift = pressed.contains(&Key::LeftShift) || pressed.contains(&Key::RightShift);
        for key in pressed {
            if prev.contains(key) {
                continue;
            }
            match *key {
                Key::Backspace => emit_text_backspace(window_id, target_id),
                Key::Delete => emit_text_delete(window_id, target_id),
                Key::Enter => emit_submit(window_id, target_id),
                Key::Left => emit_cursor_move(window_id, target_id, -1),
                Key::Right => emit_cursor_move(window_id, target_id, 1),
                Key::Home => emit_cursor_move(window_id, target_id, -4096),
                Key::End => emit_cursor_move(window_id, target_id, 4096),
                _ => {
                    if let Some(ch) = key_to_ascii(*key, shift) {
                        let mut bytes = [0u8; 4];
                        let s = ch.encode_utf8(&mut bytes);
                        emit_text_insert(window_id, target_id, s.as_bytes());
                    }
                }
            }
        }
    }

    fn clear_focus(&mut self) {
        if let Some((window_id, target)) = self.focused_text_target.take() {
            emit_blur(window_id, target);
            bump_window_gen(window_id);
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
        if x >= rect.x()
            && y >= rect.y()
            && x < rect.x() + rect.width()
            && y < rect.y() + rect.height()
        {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HitKind {
    Button,
    Checkbox,
    TextInput,
}

fn hit_test(
    root: ThingId,
    x: i32,
    y: i32,
    rel_has_child: u64,
    kind_button: u64,
    kind_checkbox: u64,
) -> Option<(ThingId, HitKind)> {
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
                let hit = if kind.0 == kind_button {
                    HitKind::Button
                } else {
                    HitKind::Checkbox
                };
                return Some((id, hit));
            }
            if prop_get(id, keys::UI_KIND).unwrap_or(0) == abi::schema::ui_kind::TEXT_INPUT
                && prop_get(id, keys::UI_FOCUSABLE).unwrap_or(1) != 0
            {
                return Some((id, HitKind::TextInput));
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
    let event = UiEvent::clicked(window_id.to_u64_lossy(), node_id.to_u64_lossy(), action_id);
    write_event(window_id, &event);
}

fn emit_toggled(window_id: ThingId, node_id: ThingId, checked: bool, value_id: u64) {
    let event = UiEvent::toggled(
        window_id.to_u64_lossy(),
        node_id.to_u64_lossy(),
        checked,
        value_id,
    );
    write_event(window_id, &event);
}

fn emit_focus(window_id: ThingId, node_id: ThingId) {
    let event = UiEvent::focus(window_id.to_u64_lossy(), node_id.to_u64_lossy());
    write_event(window_id, &event);
}

fn emit_blur(window_id: ThingId, node_id: ThingId) {
    let event = UiEvent::blur(window_id.to_u64_lossy(), node_id.to_u64_lossy());
    write_event(window_id, &event);
}

fn emit_text_insert(window_id: ThingId, node_id: ThingId, text: &[u8]) {
    let event = UiEvent::text_input(window_id.to_u64_lossy(), node_id.to_u64_lossy(), text);
    write_event(window_id, &event);
}

fn emit_text_backspace(window_id: ThingId, node_id: ThingId) {
    let event = UiEvent::text_backspace(window_id.to_u64_lossy(), node_id.to_u64_lossy());
    write_event(window_id, &event);
}

fn emit_text_delete(window_id: ThingId, node_id: ThingId) {
    let event = UiEvent::text_delete(window_id.to_u64_lossy(), node_id.to_u64_lossy());
    write_event(window_id, &event);
}

fn emit_cursor_move(window_id: ThingId, node_id: ThingId, delta: i32) {
    let event = UiEvent::cursor_move(window_id.to_u64_lossy(), node_id.to_u64_lossy(), delta);
    write_event(window_id, &event);
}

fn emit_submit(window_id: ThingId, node_id: ThingId) {
    let event = UiEvent::submit(window_id.to_u64_lossy(), node_id.to_u64_lossy());
    write_event(window_id, &event);
}

/// Max encoded event size for the buffer allocation.
const EVENT_BUF_SIZE: usize = 128;

fn write_event(window_id: ThingId, event: &UiEvent) {
    let mut buf = [0u8; EVENT_BUF_SIZE];
    let Some(written) = ui_event::encode(event, &mut buf) else {
        return;
    };
    let bs_id = prop_get(window_id, keys::UI_EVENT_QUEUE).unwrap_or(0);
    let bs = if bs_id == 0 {
        match bytespace_create(written, 0, 0) {
            Ok(id) => {
                let _ = prop_set(window_id, keys::UI_EVENT_QUEUE, id.to_u64_lossy());
                id
            }
            Err(_) => return,
        }
    } else {
        ThingId::from_u64(bs_id)
    };
    let _ = bytespace_write(bs, 0, &buf[..written]);
    let current = prop_get(window_id, keys::UI_EVENT_GEN).unwrap_or(0);
    let _ = prop_set(window_id, keys::UI_EVENT_GEN, current.saturating_add(1));
}

fn bump_window_gen(window_id: ThingId) {
    let current = prop_get(window_id, keys::UI_SCENE_GEN).unwrap_or(0);
    let _ = prop_set(window_id, keys::UI_SCENE_GEN, current.saturating_add(1));
}

fn key_to_ascii(key: Key, shift: bool) -> Option<char> {
    let ch = match key {
        Key::A => if shift { 'A' } else { 'a' },
        Key::B => if shift { 'B' } else { 'b' },
        Key::C => if shift { 'C' } else { 'c' },
        Key::D => if shift { 'D' } else { 'd' },
        Key::E => if shift { 'E' } else { 'e' },
        Key::F => if shift { 'F' } else { 'f' },
        Key::G => if shift { 'G' } else { 'g' },
        Key::H => if shift { 'H' } else { 'h' },
        Key::I => if shift { 'I' } else { 'i' },
        Key::J => if shift { 'J' } else { 'j' },
        Key::K => if shift { 'K' } else { 'k' },
        Key::L => if shift { 'L' } else { 'l' },
        Key::M => if shift { 'M' } else { 'm' },
        Key::N => if shift { 'N' } else { 'n' },
        Key::O => if shift { 'O' } else { 'o' },
        Key::P => if shift { 'P' } else { 'p' },
        Key::Q => if shift { 'Q' } else { 'q' },
        Key::R => if shift { 'R' } else { 'r' },
        Key::S => if shift { 'S' } else { 's' },
        Key::T => if shift { 'T' } else { 't' },
        Key::U => if shift { 'U' } else { 'u' },
        Key::V => if shift { 'V' } else { 'v' },
        Key::W => if shift { 'W' } else { 'w' },
        Key::X => if shift { 'X' } else { 'x' },
        Key::Y => if shift { 'Y' } else { 'y' },
        Key::Z => if shift { 'Z' } else { 'z' },
        Key::Num1 => if shift { '!' } else { '1' },
        Key::Num2 => if shift { '@' } else { '2' },
        Key::Num3 => if shift { '#' } else { '3' },
        Key::Num4 => if shift { '$' } else { '4' },
        Key::Num5 => if shift { '%' } else { '5' },
        Key::Num6 => if shift { '^' } else { '6' },
        Key::Num7 => if shift { '&' } else { '7' },
        Key::Num8 => if shift { '*' } else { '8' },
        Key::Num9 => if shift { '(' } else { '9' },
        Key::Num0 => if shift { ')' } else { '0' },
        Key::Space => ' ',
        Key::Tab => '\t',
        Key::Minus => if shift { '_' } else { '-' },
        Key::Equal => if shift { '+' } else { '=' },
        Key::LeftBracket => if shift { '{' } else { '[' },
        Key::RightBracket => if shift { '}' } else { ']' },
        Key::Backslash => if shift { '|' } else { '\\' },
        Key::Semicolon => if shift { ':' } else { ';' },
        Key::Quote => if shift { '"' } else { '\'' },
        Key::Grave => if shift { '~' } else { '`' },
        Key::Comma => if shift { '<' } else { ',' },
        Key::Period => if shift { '>' } else { '.' },
        Key::Slash => if shift { '?' } else { '/' },
        _ => return None,
    };
    Some(ch)
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
    use abi::ui_event::{self, UiEvent, UiEventKind};
    use alloc::collections::BTreeMap;
    use alloc::string::String;
    use stem::errors::{Error, Result};
    use stem::petals::graph::{reduce_window_events_with_graph, GraphBackend, UiKey, UiTreeBuilder};

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
            self.bytespaces.insert(id.to_u64_lossy(), alloc::vec![0u8; len]);
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
                Err(Error::Errno(abi::errors::Errno::ENOENT))
            }
        }

        fn bytespace_write(&mut self, id: ThingId, offset: usize, bytes: &[u8]) -> Result<()> {
            if let Some(buf) = self.bytespaces.get_mut(&id.to_u64_lossy()) {
                let end = offset + bytes.len();
                if end > buf.len() {
                    buf.resize(end, 0);
                }
                buf[offset..end].copy_from_slice(bytes);
                Ok(())
            } else {
                Err(Error::Errno(abi::errors::Errno::ENOENT))
            }
        }
    }

    fn encode_event(event: &UiEvent) -> Vec<u8> {
        let mut buf = [0u8; 128];
        let n = ui_event::encode(event, &mut buf).expect("encode");
        buf[..n].to_vec()
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
        graph.props.insert(
            (checkbox_id.to_u64_lossy(), keys::UI_WIDTH.to_string()),
            100,
        );
        graph.props.insert(
            (checkbox_id.to_u64_lossy(), keys::UI_HEIGHT.to_string()),
            30,
        );
        graph.props.insert(
            (window_id.to_u64_lossy(), keys::UI_EVENT_QUEUE.to_string()),
            0,
        );
        graph.props.insert(
            (window_id.to_u64_lossy(), keys::UI_SCENE_GEN.to_string()),
            0,
        );

        let event = UiEvent::toggled(
            window_id.to_u64_lossy(),
            checkbox_id.to_u64_lossy(),
            true,
            7,
        );
        let buf = encode_event(&event);
        graph.bytespaces.insert(999, buf.clone());

        // Verify roundtrip decode
        let (decoded, _) = ui_event::decode_one(&buf).unwrap();
        assert_eq!(decoded.kind(), Some(UiEventKind::Toggled));
    }

    #[test]
    fn text_input_focus_insert_backspace_reduces_to_graph_state() {
        let graph = TestGraph::new();
        let window_id = ThingId::from_u64(77);
        let mut builder = UiTreeBuilder::new(graph, window_id);
        let mut input_id = ThingId::default();
        builder
            .column(|ui| {
                input_id = ui.text_input_keyed(UiKey("query_input"), "", "Search")?;
                Ok(())
            })
            .unwrap();
        let (_root, mut graph) = builder.finish_with_graph().unwrap();
        let queue = graph.prop_get(window_id, keys::UI_EVENT_QUEUE).unwrap();

        let focus = UiEvent::focus(window_id.to_u64_lossy(), input_id.to_u64_lossy());
        let buf = encode_event(&focus);
        graph.bytespace_write(ThingId::from_u64(queue), 0, &buf).unwrap();
        assert!(reduce_window_events_with_graph(&mut graph, window_id).unwrap());

        let insert = UiEvent::text_input(window_id.to_u64_lossy(), input_id.to_u64_lossy(), b"hi");
        let buf = encode_event(&insert);
        graph.bytespace_write(ThingId::from_u64(queue), 0, &buf).unwrap();
        assert!(reduce_window_events_with_graph(&mut graph, window_id).unwrap());

        let backspace = UiEvent::text_backspace(window_id.to_u64_lossy(), input_id.to_u64_lossy());
        let buf = encode_event(&backspace);
        graph.bytespace_write(ThingId::from_u64(queue), 0, &buf).unwrap();
        assert!(reduce_window_events_with_graph(&mut graph, window_id).unwrap());

        let text_bs = graph.prop_get(input_id, keys::UI_TEXT).unwrap();
        let bytes = graph
            .bytespaces
            .get(&text_bs)
            .cloned()
            .unwrap_or_else(Vec::new);
        assert_eq!(core::str::from_utf8(&bytes).unwrap_or(""), "h");
        assert_eq!(graph.prop_get(input_id, keys::UI_CURSOR).unwrap_or(0), 1);
    }
}
