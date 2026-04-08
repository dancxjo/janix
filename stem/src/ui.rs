use crate::thing::sys::{create_node, link, prop_set};
use crate::thing::ThingId;
use abi::ids::HandleId;
use abi::schema::{keys, kinds, rels};

pub struct UiBuilder;

impl UiBuilder {
    pub fn create_root() -> ThingId {
        match create_node(kinds::UI_CROWN) {
            Ok(id) => {
                crate::info!("UiBuilder: created root {}", id.to_u64_lossy());
                id
            }
            Err(err) => {
                crate::warn!("UiBuilder: create_root unavailable: {:?}", err);
                ThingId::default()
            }
        }
    }

    pub fn create_window(parent: ThingId, title: &str) -> ThingId {
        let win = match create_node(kinds::UI_WINDOW) {
            Ok(id) => id,
            Err(err) => {
                crate::warn!("UiBuilder: create_window unavailable: {:?}", err);
                return ThingId::default();
            }
        };

        if let Err(err) = link(win, rels::CHILD_OF, parent) {
            crate::warn!("UiBuilder: create_window child_of link failed: {:?}", err);
        }
        if let Err(err) = link(parent, rels::HAS_CHILD, win) {
            crate::warn!("UiBuilder: create_window has_child link failed: {:?}", err);
        }

        Self::set_string_prop(win, keys::UI_TITLE, title);
        win
    }

    pub fn create_panel(parent: ThingId) -> ThingId {
        let panel = match create_node(kinds::UI_PANEL) {
            Ok(id) => id,
            Err(err) => {
                crate::warn!("UiBuilder: create_panel unavailable: {:?}", err);
                return ThingId::default();
            }
        };

        if let Err(err) = link(panel, rels::CHILD_OF, parent) {
            crate::warn!("UiBuilder: create_panel child_of link failed: {:?}", err);
        }
        if let Err(err) = link(parent, rels::HAS_CHILD, panel) {
            crate::warn!("UiBuilder: create_panel has_child link failed: {:?}", err);
        }
        panel
    }

    pub fn create_text(parent: ThingId, text: &str) -> ThingId {
        let node = match create_node(kinds::UI_TEXT) {
            Ok(id) => id,
            Err(err) => {
                crate::warn!("UiBuilder: create_text unavailable: {:?}", err);
                return ThingId::default();
            }
        };

        if let Err(err) = link(node, rels::CHILD_OF, parent) {
            crate::warn!("UiBuilder: create_text child_of link failed: {:?}", err);
        }
        if let Err(err) = link(parent, rels::HAS_CHILD, node) {
            crate::warn!("UiBuilder: create_text has_child link failed: {:?}", err);
        }
        Self::set_string_prop(node, keys::UI_TEXT, text);
        node
    }

    pub fn set_pos(id: ThingId, x: i32, y: i32) {
        prop_set(id, keys::UI_X, x as u64).ok();
        prop_set(id, keys::UI_Y, y as u64).ok();
    }

    pub fn set_size(id: ThingId, w: i32, h: i32) {
        prop_set(id, keys::UI_WIDTH, w as u64).ok();
        prop_set(id, keys::UI_HEIGHT, h as u64).ok();
    }

    pub fn set_color(id: ThingId, argb: u32) {
        prop_set(id, keys::UI_COLOR, argb as u64).ok();
    }

    pub fn set_text(id: ThingId, text: &str) {
        Self::set_string_prop(id, keys::UI_TEXT, text);
    }

    fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
        // ENOSYS
    }
}
