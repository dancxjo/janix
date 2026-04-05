//! Input handling for Photosynthesis viewport control.
//!
//! Reads pointer and keyboard state from system graph and translates to viewport controller actions.

use abi::hid::{Key, Mods};
use abi::schema::{hid, keyboard as kb, pointer};
use stem::petals::PanZoomController;
use stem::thing::sys::{find, prop_get};
use stem::thing::ThingId;

/// Click result when a click is detected
#[derive(Debug, Clone, Copy)]
pub struct ClickEvent {
    pub x: i32,
    pub y: i32,
}

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
    /// Track click for pinning (position must not change much)
    click_start_x: i32,
    click_start_y: i32,
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
            click_start_x: 0,
            click_start_y: 0,
        }
    }

    /// Find and cache the bristle service node.
    fn get_bristle(&mut self) -> Option<ThingId> {
        if let Some(node) = self.bristle_node {
            if prop_get(node, pointer::POINTER_X).is_ok() || prop_get(node, kb::KEYBOARD_GEN).is_ok() {
                return Some(node);
            }
        }

        // Find the newest svc.Input node. Old orphaned nodes can remain in the graph
        // if Bristle registered before task ownership was established.
        let mut nodes = [ThingId::default(); 16];
        if let Ok(count) = find(hid::SVC_INPUT, &mut nodes) {
            let count = count.min(nodes.len());
            if count > 0 {
                let mut best = nodes[0];
                for node in nodes.iter().take(count).skip(1) {
                    if node.to_u64_lossy() > best.to_u64_lossy() {
                        best = *node;
                    }
                }
                self.bristle_node = Some(best);
                return self.bristle_node;
            }
        }
        None
    }
}

/// Poll pointer and keyboard state from system graph and apply to viewport controller.
///
/// Returns (viewport_updated, click_event, toggle_debug, key_event) where click_event is Some if a click occurred,
/// toggle_debug is true if the debug key was pressed, and key_event contains the key and modifiers if a key was pressed.
pub fn poll_and_apply(ctrl: &mut PanZoomController, state: &mut InputState) -> (bool, Option<ClickEvent>, bool, Option<(Key, Mods)>) {
    let mut updated = false;
    let mut click_event = None;
    let mut toggle_debug = false;
    let mut key_event = None;

    let bristle = match state.get_bristle() {
        Some(b) => b,
        None => return (false, None, false, None),
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
        state.click_start_x = state.pointer_x;
        state.click_start_y = state.pointer_y;
        ctrl.begin_drag(state.pointer_x as f32, state.pointer_y as f32, stem::monotonic_ns());
    }

    // Handle left button release: end drag or register click
    if !left_now && left_was {
        state.left_down = false;
        
        // Check if this was a click (minimal movement) vs a drag
        let dx = (state.pointer_x - state.click_start_x).abs();
        let dy = (state.pointer_y - state.click_start_y).abs();
        let was_click = dx < 5 && dy < 5;
        
        if was_click {
            click_event = Some(ClickEvent {
                x: state.pointer_x,
                y: state.pointer_y,
            });
        } else {
            ctrl.end_drag();
        }
        
        updated = true;
    }

    // Handle drag movement
    if state.left_down && moved && ctrl.is_dragging() {
        ctrl.update_drag(state.pointer_x as f32, state.pointer_y as f32, stem::monotonic_ns());
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
            
            // Store key event for caller to handle
            key_event = Some((key, mods));
            
            let (kb_updated, kb_toggle_debug) = handle_key_down(key, mods, ctrl);
            updated |= kb_updated;
            toggle_debug = kb_toggle_debug;
        }
    }

    (updated, click_event, toggle_debug, key_event)
}

/// Handle keyboard shortcuts for viewport control.
/// Returns (viewport_updated, toggle_debug)
fn handle_key_down(key: Key, mods: Mods, ctrl: &mut PanZoomController) -> (bool, bool) {
    let alt = mods.has_alt();
    let mut updated = false;
    let mut toggle_debug = false;

    // F8: Cycle locale
    if key == Key::F8 {
        stem::i18n::cycle_locale();
        stem::info!("Locale switched to: {:?}", stem::i18n::current_locale());
        return (true, false);
    }

    // D: Toggle debug mode
    if key == Key::D {
        toggle_debug = true;
        return (false, true);
    }

    // Alt+Equal (Plus): Zoom in
    if alt && key == Key::Equal {
        ctrl.handle_keyboard_zoom(true);
        return (true, false);
    }

    // Alt+Minus: Zoom out
    if alt && key == Key::Minus {
        ctrl.handle_keyboard_zoom(false);
        return (true, false);
    }

    // Arrow keys: Pan
    match key {
        Key::Up => {
            ctrl.handle_arrow_key(true, false, false, false);
            updated = true;
        }
        Key::Down => {
            ctrl.handle_arrow_key(false, true, false, false);
            updated = true;
        }
        Key::Left => {
            ctrl.handle_arrow_key(false, false, true, false);
            updated = true;
        }
        Key::Right => {
            ctrl.handle_arrow_key(false, false, false, true);
            updated = true;
        }
        _ => {}
    }

    // Home: Reset view
    if key == Key::Home {
        ctrl.handle_home();
        updated = true;
    }

    // Page Up/Down: Large scroll
    if key == Key::PageUp {
        ctrl.handle_page(true);
        updated = true;
    }
    if key == Key::PageDown {
        ctrl.handle_page(false);
        updated = true;
    }

    (updated, toggle_debug)
}
