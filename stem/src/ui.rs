use crate::thing::ThingId;
use crate::thing::sys::{create_node, prop_set, link, bytespace_create, bytespace_write};
use abi::schema::{kinds, keys, rels};

pub struct UiBuilder;

impl UiBuilder {
    pub fn create_root() -> ThingId {
        let id = create_node(kinds::UI_ROOT).expect("create UI_ROOT");
        crate::info!("UiBuilder: created root {}", id.0);
        id
    }

    pub fn create_window(parent: ThingId, title: &str) -> ThingId {
        let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
        link(win, rels::CHILD_OF, parent).expect("link window child_of");
        link(parent, rels::HAS_CHILD, win).expect("link window has_child");
        
        Self::set_string_prop(win, keys::UI_TITLE, title);
        win
    }

    pub fn create_panel(parent: ThingId) -> ThingId {
        let panel = create_node(kinds::UI_PANEL).expect("create UI_PANEL");
        link(panel, rels::CHILD_OF, parent).expect("link panel child_of");
        link(parent, rels::HAS_CHILD, panel).expect("link panel has_child");
        panel
    }

    pub fn create_text(parent: ThingId, text: &str) -> ThingId {
        let node = create_node(kinds::UI_TEXT).expect("create UI_TEXT");
        link(node, rels::CHILD_OF, parent).expect("link text child_of");
        link(parent, rels::HAS_CHILD, node).expect("link text has_child");
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
        if value.is_empty() {
            prop_set(id, key_name, 0).ok();
            return;
        }
        let bs_id = bytespace_create(value.len(), 0, 0).expect("create bytespace");
        bytespace_write(bs_id, 0, value.as_bytes()).ok();
        prop_set(id, key_name, bs_id.0).ok();
    }
}
