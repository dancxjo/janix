extern crate alloc;

use alloc::fmt::Write;
use alloc::string::String;
use alloc::vec::Vec;

use abi::schema::{keys, kinds};
use abi::ids::HandleId;
use stem::thing::sys::{find, prop_get};
use stem::thing::ThingId;

pub mod events;
pub mod graph_decode;
pub mod render_html;

#[derive(Debug)]
pub enum UiError {
    BadRequest(&'static str),
    NotFound,
    Internal,
}

pub fn render_window_html(window_id_raw: u64, if_scene_gen: Option<u64>) -> Result<Option<Vec<u8>>, UiError> {
    let window_id = ThingId::from_u64(window_id_raw);
    let (tree, scene_gen) = graph_decode::decode_window_tree(window_id).map_err(map_decode_error)?;

    if let Some(prev) = if_scene_gen {
        if prev == scene_gen {
            return Ok(None);
        }
    }

    let html = render_html::render_window_html(&tree, scene_gen);
    Ok(Some(html.into_bytes()))
}

pub fn render_window_json(window_id_raw: u64) -> Result<Vec<u8>, UiError> {
    let window_id = ThingId::from_u64(window_id_raw);
    let (tree, scene_gen) = graph_decode::decode_window_tree(window_id).map_err(map_decode_error)?;
    let json = render_html::render_tree_json(&tree, scene_gen);
    Ok(json.into_bytes())
}

pub fn ingest_event(body: &[u8]) -> Result<Vec<u8>, UiError> {
    events::ingest_event_request(body).map_err(|err| match err {
        events::EventError::BadRequest(msg) => UiError::BadRequest(msg),
        events::EventError::Internal => UiError::Internal,
    })
}

pub fn list_windows_json() -> Vec<u8> {
    let mut windows = [ThingId::default(); 256];
    let count = find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);

    let mut out = String::new();
    let _ = write!(out, "{{\"count\":{},\"windows\":[", count);
    for (i, id) in windows.iter().take(count).enumerate() {
        if i > 0 {
            out.push(',');
        }
        let scene_gen = prop_get(*id, keys::UI_SCENE_GEN).unwrap_or(0);
        let event_gen = prop_get(*id, keys::UI_EVENT_GEN).unwrap_or(0);
        let _ = write!(
            out,
            "{{\"id\":{},\"scene_gen\":{},\"event_gen\":{}}}",
            id.to_u64_lossy(),
            scene_gen,
            event_gen
        );
    }
    out.push_str("]}");
    out.into_bytes()
}

fn map_decode_error(err: graph_decode::DecodeError) -> UiError {
    match err {
        graph_decode::DecodeError::NotFound => UiError::NotFound,
        graph_decode::DecodeError::Graph => UiError::Internal,
    }
}
