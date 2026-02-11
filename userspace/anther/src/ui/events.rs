extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::ids::HandleId;
use abi::schema::keys;
use abi::ui_event::{self, UiEvent};
use stem::thing::sys::{
    bytespace_create, bytespace_info, bytespace_read, bytespace_write, prop_get, prop_set,
};
use stem::thing::ThingId;

const EVENT_BUF_SIZE: usize = 256;

#[derive(Debug)]
pub enum EventError {
    BadRequest(&'static str),
    Internal,
}

pub trait EventGraph {
    fn prop_get(&self, id: ThingId, key: &str) -> Option<u64>;
    fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> bool;
    fn bytespace_create(&mut self, len: usize) -> Option<ThingId>;
    fn bytespace_len(&self, id: ThingId) -> Option<usize>;
    fn bytespace_read_range(&self, id: ThingId, offset: usize, len: usize) -> Option<Vec<u8>>;
    fn bytespace_write(&mut self, id: ThingId, offset: usize, data: &[u8]) -> bool;
}

pub struct SysEventGraph;

impl EventGraph for SysEventGraph {
    fn prop_get(&self, id: ThingId, key: &str) -> Option<u64> {
        prop_get(id, key).ok()
    }

    fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> bool {
        prop_set(id, key, value).is_ok()
    }

    fn bytespace_create(&mut self, len: usize) -> Option<ThingId> {
        bytespace_create(len, 0, 0).ok()
    }

    fn bytespace_len(&self, id: ThingId) -> Option<usize> {
        bytespace_info(id).ok()
    }

    fn bytespace_read_range(&self, id: ThingId, offset: usize, len: usize) -> Option<Vec<u8>> {
        if len == 0 {
            return Some(Vec::new());
        }
        let mut out = Vec::with_capacity(len);
        out.resize(len, 0);
        let mut read = 0;
        while read < len {
            let n = bytespace_read(id, offset.saturating_add(read), &mut out[read..]).ok()?;
            if n == 0 {
                break;
            }
            read += n;
        }
        out.truncate(read);
        Some(out)
    }

    fn bytespace_write(&mut self, id: ThingId, offset: usize, data: &[u8]) -> bool {
        bytespace_write(id, offset, data).is_ok()
    }
}

pub fn ingest_event_request(body: &[u8]) -> Result<Vec<u8>, EventError> {
    let text = core::str::from_utf8(body)
        .map_err(|_| EventError::BadRequest("body must be utf-8 json"))?;
    let mut graph = SysEventGraph;
    let gen = ingest_event_json_with(&mut graph, text)?;
    Ok(format_ok_json(gen).into_bytes())
}

pub fn ingest_event_json_with(graph: &mut impl EventGraph, body: &str) -> Result<u64, EventError> {
    let evt = parse_event_json(body)?;
    append_event(graph, evt.window_id, &evt.event)
}

struct ParsedEvent {
    window_id: ThingId,
    event: UiEvent,
}

fn parse_event_json(body: &str) -> Result<ParsedEvent, EventError> {
    let kind = json_string(body, "kind").ok_or(EventError::BadRequest("missing kind"))?;
    let window = json_u64(body, "window").ok_or(EventError::BadRequest("missing window"))?;
    let target = json_u64(body, "target").unwrap_or(window);

    let event = match kind.as_str() {
        "activate" | "click" => UiEvent::activate(window, target),
        "focus" => UiEvent::focus(window, target),
        "submit" => UiEvent::submit(window, target),
        "text_input" | "input" => {
            let text = json_string(body, "text").unwrap_or_default();
            UiEvent::text_input(window, target, text.as_bytes())
        }
        "scroll" => {
            let dx = json_i32(body, "dx").unwrap_or(0);
            let dy = json_i32(body, "dy").unwrap_or(0);
            let mods = json_u64(body, "mods").unwrap_or(0) as u16;
            UiEvent::Scroll {
                window,
                target,
                dx,
                dy,
                mods,
            }
        }
        "pointer_down" => {
            let button = json_u64(body, "button").unwrap_or(0) as u8;
            let x = json_i32(body, "x").unwrap_or(0);
            let y = json_i32(body, "y").unwrap_or(0);
            let mods = json_u64(body, "mods").unwrap_or(0) as u16;
            UiEvent::PointerDown {
                window,
                target,
                button,
                x,
                y,
                mods,
            }
        }
        "pointer_up" => {
            let button = json_u64(body, "button").unwrap_or(0) as u8;
            let x = json_i32(body, "x").unwrap_or(0);
            let y = json_i32(body, "y").unwrap_or(0);
            let mods = json_u64(body, "mods").unwrap_or(0) as u16;
            UiEvent::PointerUp {
                window,
                target,
                button,
                x,
                y,
                mods,
            }
        }
        _ => return Err(EventError::BadRequest("unsupported kind")),
    };

    Ok(ParsedEvent {
        window_id: ThingId::from_u64(window),
        event,
    })
}

fn append_event(
    graph: &mut impl EventGraph,
    window_id: ThingId,
    event: &UiEvent,
) -> Result<u64, EventError> {
    let mut encoded = [0u8; EVENT_BUF_SIZE];
    let written = ui_event::encode(event, &mut encoded).ok_or(EventError::Internal)?;

    let existing_queue = graph
        .prop_get(window_id, keys::UI_EVENT_LOG)
        .map(ThingId::from_u64)
        .unwrap_or_default();
    let existing_bytes = if existing_queue == ThingId::default() {
        Vec::new()
    } else {
        let full_len = graph
            .bytespace_len(existing_queue)
            .ok_or(EventError::Internal)?;
        let raw_cursor = graph
            .prop_get(window_id, keys::UI_EVENT_CURSOR)
            .unwrap_or(0) as usize;
        let cursor = raw_cursor.min(full_len);
        let unread_len = full_len.saturating_sub(cursor);
        graph
            .bytespace_read_range(existing_queue, cursor, unread_len)
            .ok_or(EventError::Internal)?
    };

    let new_len = existing_bytes.len().saturating_add(written);
    let new_queue = graph
        .bytespace_create(new_len)
        .ok_or(EventError::Internal)?;

    if !existing_bytes.is_empty() && !graph.bytespace_write(new_queue, 0, &existing_bytes) {
        return Err(EventError::Internal);
    }
    if !graph.bytespace_write(new_queue, existing_bytes.len(), &encoded[..written]) {
        return Err(EventError::Internal);
    }

    if !graph.prop_set(window_id, keys::UI_EVENT_LOG, new_queue.to_u64_lossy()) {
        return Err(EventError::Internal);
    }
    // Rebased queue starts at zero even when we compact consumed prefix.
    let _ = graph.prop_set(window_id, keys::UI_EVENT_CURSOR, 0);

    let current = graph.prop_get(window_id, keys::UI_EVENT_GEN).unwrap_or(0);
    let next = current.saturating_add(1);
    if !graph.prop_set(window_id, keys::UI_EVENT_GEN, next) {
        return Err(EventError::Internal);
    }

    Ok(next)
}

fn format_ok_json(event_gen: u64) -> String {
    let mut s = String::new();
    s.push_str("{\"ok\":true,\"ui_event_gen\":");
    s.push_str(&event_gen.to_string());
    s.push('}');
    s
}

fn json_string(body: &str, key: &str) -> Option<String> {
    let idx = find_value_start(body, key)?;
    let bytes = body.as_bytes();
    if *bytes.get(idx)? != b'"' {
        return None;
    }
    let mut out = String::new();
    let mut i = idx + 1;
    while i < bytes.len() {
        let ch = bytes[i];
        if ch == b'"' {
            return Some(out);
        }
        if ch == b'\\' {
            i += 1;
            let esc = *bytes.get(i)?;
            match esc {
                b'"' => out.push('"'),
                b'\\' => out.push('\\'),
                b'n' => out.push('\n'),
                b'r' => out.push('\r'),
                b't' => out.push('\t'),
                _ => out.push(esc as char),
            }
            i += 1;
            continue;
        }
        out.push(ch as char);
        i += 1;
    }
    None
}

fn json_u64(body: &str, key: &str) -> Option<u64> {
    let idx = find_value_start(body, key)?;
    let rest = &body[idx..];
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    if end == 0 {
        return None;
    }
    rest[..end].parse::<u64>().ok()
}

fn json_i32(body: &str, key: &str) -> Option<i32> {
    let idx = find_value_start(body, key)?;
    let rest = &body[idx..];
    let end = rest
        .find(|c: char| !(c.is_ascii_digit() || c == '-'))
        .unwrap_or(rest.len());
    if end == 0 {
        return None;
    }
    rest[..end].parse::<i32>().ok()
}

fn find_value_start(body: &str, key: &str) -> Option<usize> {
    let bytes = body.as_bytes();
    let mut i = 0;
    let mut in_string = false;
    let mut escaped = false;

    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if b != b'"' {
            i += 1;
            continue;
        }

        let key_start = i + 1;
        let mut j = key_start;
        let mut key_escaped = false;
        while j < bytes.len() {
            let c = bytes[j];
            if key_escaped {
                key_escaped = false;
            } else if c == b'\\' {
                key_escaped = true;
            } else if c == b'"' {
                break;
            }
            j += 1;
        }
        if j >= bytes.len() {
            return None;
        }

        if &body[key_start..j] == key {
            let mut k = j + 1;
            while k < bytes.len()
                && (bytes[k] == b' ' || bytes[k] == b'\n' || bytes[k] == b'\r' || bytes[k] == b'\t')
            {
                k += 1;
            }
            if k >= bytes.len() || bytes[k] != b':' {
                i = j + 1;
                continue;
            }
            k += 1;
            while k < bytes.len()
                && (bytes[k] == b' ' || bytes[k] == b'\n' || bytes[k] == b'\r' || bytes[k] == b'\t')
            {
                k += 1;
            }
            return Some(k);
        }

        in_string = false;
        i = j + 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;

    #[derive(Default)]
    struct MockGraph {
        props: BTreeMap<(u64, &'static str), u64>,
        bytespaces: BTreeMap<u64, Vec<u8>>,
        next_id: u64,
    }

    impl EventGraph for MockGraph {
        fn prop_get(&self, id: ThingId, key: &str) -> Option<u64> {
            self.props.get(&(id.to_u64_lossy(), leak(key))).copied()
        }

        fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> bool {
            self.props.insert((id.to_u64_lossy(), leak(key)), value);
            true
        }

        fn bytespace_create(&mut self, len: usize) -> Option<ThingId> {
            self.next_id += 1;
            self.bytespaces.insert(self.next_id, alloc::vec![0u8; len]);
            Some(ThingId::from_u64(self.next_id))
        }

        fn bytespace_len(&self, id: ThingId) -> Option<usize> {
            self.bytespaces.get(&id.to_u64_lossy()).map(|b| b.len())
        }

        fn bytespace_read_range(&self, id: ThingId, offset: usize, len: usize) -> Option<Vec<u8>> {
            let buf = self.bytespaces.get(&id.to_u64_lossy())?;
            if offset > buf.len() {
                return None;
            }
            let end = offset.saturating_add(len).min(buf.len());
            Some(buf[offset..end].to_vec())
        }

        fn bytespace_write(&mut self, id: ThingId, offset: usize, data: &[u8]) -> bool {
            let Some(buf) = self.bytespaces.get_mut(&id.to_u64_lossy()) else {
                return false;
            };
            if offset + data.len() > buf.len() {
                return false;
            }
            buf[offset..offset + data.len()].copy_from_slice(data);
            true
        }
    }

    fn leak(key: &str) -> &'static str {
        match key {
            keys::UI_EVENT_LOG => keys::UI_EVENT_LOG,
            keys::UI_EVENT_CURSOR => keys::UI_EVENT_CURSOR,
            keys::UI_EVENT_GEN => keys::UI_EVENT_GEN,
            _ => keys::UI_EVENT_LOG,
        }
    }

    #[test]
    fn ingest_text_input_and_append() {
        let mut graph = MockGraph::default();
        let window = ThingId::from_u64(7);

        let payload1 = "{\"kind\":\"focus\",\"window\":7,\"target\":9}";
        let payload2 = "{\"kind\":\"text_input\",\"window\":7,\"target\":9,\"text\":\"abc\"}";

        let gen1 = ingest_event_json_with(&mut graph, payload1).expect("ingest1");
        let gen2 = ingest_event_json_with(&mut graph, payload2).expect("ingest2");
        assert_eq!(gen1, 1);
        assert_eq!(gen2, 2);

        let queue_id = graph
            .prop_get(window, keys::UI_EVENT_LOG)
            .expect("queue id");
        let bytes = graph
            .bytespace_read_range(ThingId::from_u64(queue_id), 0, usize::MAX)
            .expect("queue bytes");

        let (evt1, c1) = ui_event::decode_one(&bytes).expect("decode1");
        let (evt2, _c2) = ui_event::decode_one(&bytes[c1..]).expect("decode2");

        assert!(matches!(
            evt1,
            UiEvent::Focus {
                window: 7,
                target: 9
            }
        ));
        match evt2 {
            UiEvent::TextInput {
                window,
                target,
                text_len,
                text,
            } => {
                assert_eq!(window, 7);
                assert_eq!(target, 9);
                assert_eq!(&text[..text_len as usize], b"abc");
            }
            _ => panic!("unexpected event"),
        }
    }

    #[test]
    fn parse_ignores_key_names_inside_string_values() {
        let body = "{\"kind\":\"text_input\",\"text\":\"window:999 target:888\",\"window\":7,\"target\":9}";
        let parsed = parse_event_json(body).expect("parse");
        match parsed.event {
            UiEvent::TextInput { window, target, .. } => {
                assert_eq!(window, 7);
                assert_eq!(target, 9);
            }
            _ => panic!("unexpected event"),
        }
    }

    #[test]
    fn append_compacts_consumed_prefix() {
        let mut graph = MockGraph::default();
        let window = ThingId::from_u64(7);

        let payload1 = "{\"kind\":\"focus\",\"window\":7,\"target\":9}";
        let payload2 = "{\"kind\":\"submit\",\"window\":7,\"target\":9}";

        let _ = ingest_event_json_with(&mut graph, payload1).expect("ingest1");
        let first_log_id = graph
            .prop_get(window, keys::UI_EVENT_LOG)
            .expect("first queue id");
        let first_bytes = graph
            .bytespace_read_range(ThingId::from_u64(first_log_id), 0, usize::MAX)
            .expect("first queue bytes");
        let (_, consumed_first) = ui_event::decode_one(&first_bytes).expect("decode first");

        assert!(graph.prop_set(window, keys::UI_EVENT_CURSOR, consumed_first as u64));

        let _ = ingest_event_json_with(&mut graph, payload2).expect("ingest2");
        let second_log_id = graph
            .prop_get(window, keys::UI_EVENT_LOG)
            .expect("second queue id");
        let second_bytes = graph
            .bytespace_read_range(ThingId::from_u64(second_log_id), 0, usize::MAX)
            .expect("second queue bytes");

        assert_eq!(graph.prop_get(window, keys::UI_EVENT_CURSOR), Some(0));
        let (evt, consumed) = ui_event::decode_one(&second_bytes).expect("decode second");
        assert_eq!(consumed, second_bytes.len());
        assert!(matches!(
            evt,
            UiEvent::Submit {
                window: 7,
                target: 9
            }
        ));
    }
}
