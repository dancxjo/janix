//! Simple UI helper API for windowed applications.

extern crate alloc;

use alloc::string::ToString;
use alloc::vec::Vec;
use runtime::Sys;
use crate::graph_kinds;
use crate::{Mode, Place, Surface, Window};
use crate::{PropValue, ThingId};
use crate::{create_thing, list_things_by_kind, register_schema_for, update_props};

#[derive(Clone, Copy, Debug)]
pub struct WindowHandle {
    pub id: ThingId,
}

pub fn ensure_ui_schemas<S: Sys>(sys: &mut S) {
    let _ = register_schema_for::<Place>(sys);
    let _ = register_schema_for::<Mode>(sys);
    let _ = register_schema_for::<Window>(sys);
    let _ = register_schema_for::<Surface>(sys);
}

pub fn create_window<S: Sys>(sys: &mut S, title: &str, mode_index: u8) -> Option<WindowHandle> {
    ensure_ui_schemas(sys);

    let modes: Vec<Mode> = list_things_by_kind(sys);
    let mode = modes.into_iter().find(|m| m.index == mode_index)?;
    let place_id = mode.place_id.unwrap_or(ThingId(0));

    let window = Window {
        id: ThingId(0),
        place_id,
        x: 40,
        y: 40,
        width: 480,
        height: 320,
        z_index: 0,
        active: false,
        title: title.to_string(),
        draggable: true,
        resizable: true,
        closable: true,
        minimizable: true,
    };

    let id = create_thing(sys, &window)?;
    let _ = crate::add_link(sys, place_id, graph_kinds::LINK_PLACE_WINDOW, id);
    Some(WindowHandle { id })
}

pub fn set_window_text<S: Sys>(sys: &mut S, window: WindowHandle, text: &str) {
    ensure_ui_schemas(sys);
    let surfaces: Vec<Surface> = list_things_by_kind(sys);
    if let Some(surface) = surfaces.iter().find(|s| s.window_id == window.id) {
        let _ = update_props(
            sys,
            surface.id,
            &[(
                graph_kinds::PROP_SURFACE_TEXT,
                PropValue::Str(text.to_string()),
            )],
        );
        return;
    }

    let surface = Surface {
        id: ThingId(0),
        window_id: window.id,
        kind: "text".to_string(),
        text: text.to_string(),
        width: 0,
        height: 0,
        stride: 0,
        format: "Text".to_string(),
        shared_buffer_id: None,
        refresh_interval_ns: None,
        frames_presented: None,
        last_present_ns: None,
        power_state: None,
    };
    if let Some(id) = create_thing(sys, &surface) {
        let _ = crate::add_link(sys, window.id, graph_kinds::LINK_WINDOW_SURFACE, id);
    }
}

pub fn append_window_text<S: Sys>(sys: &mut S, window: WindowHandle, text: &str) {
    ensure_ui_schemas(sys);
    let mut surfaces: Vec<Surface> = list_things_by_kind(sys);
    if let Some(surface) = surfaces.iter_mut().find(|s| s.window_id == window.id) {
        surface.text.push_str(text);
        let _ = update_props(
            sys,
            surface.id,
            &[(
                graph_kinds::PROP_SURFACE_TEXT,
                PropValue::Str(surface.text.clone()),
            )],
        );
    } else {
        set_window_text(sys, window, text);
    }
}
