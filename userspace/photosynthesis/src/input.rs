//! Input handling for Photosynthesis viewport control.
//!
//! Reads pointer and keyboard state from system graph and translates to viewport controller actions.

use abi::hid::{Key, Mods};
use abi::schema::{hid, keyboard as kb, pointer};
use stem::petals::PanZoomController;
use stem::thing::ThingId;
use stem::thing::sys::{find, prop_get};

/// Input state for tracking pointer position, drag, and keyboard.
pub struct InputState {
    /// Left button currently held
    pub left_down: bool,
    /// Last known pointer screen position
    pub pointer_x: i32,
    pub pointer_y: i32,
    /// Previous pointer position (for delta tracking)
    prev_x: i32,
    prev_y: i32,
    /// Last seen keyboard generation (for edge detection)
    last_keyboard_gen: u64,
    /// Cached bristle node ID
    bristle_node: Option<ThingId>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            left_down: false,
            pointer_x: 400,
            pointer_y: 300,
            prev_x: 400,
            prev_y: 300,
            last_keyboard_gen: 0,
            bristle_node: None,
        }
    }

    /// Find and cache the bristle service node.
    fn get_bristle(&mut self) -> Option<ThingId> {
        if self.bristle_node.is_some() {
            return self.bristle_node;
        }
        // Find svc.Input node (bristle)
        let mut nodes = [ThingId::default(); 4];
        if let Ok(count) = find(hid::SVC_INPUT, &mut nodes) {
            if count > 0 {
                self.bristle_node = Some(nodes[0]);
                return self.bristle_node;
            }
        }
        None
    }
}

/// Poll pointer and keyboard state from system graph and apply to viewport controller.
///
/// Returns true if the viewport was updated (needs redraw).
pub fn poll_and_apply(
    ctrl: &mut PanZoomController,
    state: &mut InputState,
) -> bool {
    let mut updated = false;

    let bristle = match state.get_bristle() {
        Some(b) => b,
        None => return false,
    };

    // --- Pointer handling ---
    let new_x = prop_get(bristle, pointer::POINTER_X).unwrap_or(state.pointer_x as u64) as i32;
    let new_y = prop_get(bristle, pointer::POINTER_Y).unwrap_or(state.pointer_y as u64) as i32;
    let buttons = prop_get(bristle, pointer::POINTER_BUTTONS).unwrap_or(0);

    let left_now = (buttons & 1) != 0;
    let left_was = state.left_down;

    let moved = new_x != state.pointer_x || new_y != state.pointer_y;
    state.prev_x = state.pointer_x;
    state.prev_y = state.pointer_y;
    state.pointer_x = new_x;
    state.pointer_y = new_y;

    // Handle left button press: start drag
    if left_now && !left_was {
        state.left_down = true;
        ctrl.begin_drag(state.pointer_x as f32, state.pointer_y as f32);
    }

    // Handle left button release: end drag
    if !left_now && left_was {
        state.left_down = false;
        ctrl.end_drag();
        updated = true;
    }

    // Handle drag movement
    if state.left_down && moved && ctrl.is_dragging() {
        ctrl.update_drag(state.pointer_x as f32, state.pointer_y as f32);
        updated = true;
    }

    state.left_down = left_now;

    // --- Keyboard handling ---
    let keyboard_gen = prop_get(bristle, kb::KEYBOARD_GEN).unwrap_or(0);
    if keyboard_gen != state.last_keyboard_gen && keyboard_gen > 0 {
        state.last_keyboard_gen = keyboard_gen;

        // New key event - read state
        let key_code = prop_get(bristle, kb::KEYBOARD_LAST_KEY).unwrap_or(0);
        let key_edge = prop_get(bristle, kb::KEYBOARD_KEY_EDGE).unwrap_or(0);
        let mods_val = prop_get(bristle, kb::KEYBOARD_MODS).unwrap_or(0);

        if key_edge == 1 {
            // Key down event
            let key = Key::from_raw(key_code as u16);
            let mods = Mods(mods_val as u8);
            updated |= handle_key_down(key, mods, ctrl);
        }
    }

    updated
}

/// Handle keyboard shortcuts for viewport control.
fn handle_key_down(key: Key, mods: Mods, ctrl: &mut PanZoomController) -> bool {
    let alt = mods.has_alt();

    // Alt+Equal (Plus): Zoom in
    if alt && key == Key::Equal {
        ctrl.handle_keyboard_zoom(true);
        return true;
    }

    // Alt+Minus: Zoom out
    if alt && key == Key::Minus {
        ctrl.handle_keyboard_zoom(false);
        return true;
    }

    // Arrow keys: Pan
    match key {
        Key::Up => {
            ctrl.handle_arrow_key(true, false, false, false);
            return true;
        }
        Key::Down => {
            ctrl.handle_arrow_key(false, true, false, false);
            return true;
        }
        Key::Left => {
            ctrl.handle_arrow_key(false, false, true, false);
            return true;
        }
        Key::Right => {
            ctrl.handle_arrow_key(false, false, false, true);
            return true;
        }
        _ => {}
    }

    // Home: Reset view
    if key == Key::Home {
        ctrl.handle_home();
        return true;
    }

    // Page Up/Down: Large scroll
    if key == Key::PageUp {
        ctrl.handle_page(true);
        return true;
    }
    if key == Key::PageDown {
        ctrl.handle_page(false);
        return true;
    }

    false
}
