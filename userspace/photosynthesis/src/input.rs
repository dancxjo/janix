//! Input handling for Photosynthesis viewport control.
//!
//! Reads pointer and keyboard state from system graph and translates to viewport controller actions.

use abi::hid::{Key, Mods};
use abi::schema::{hid, keyboard as kb, pointer};
use stem::petals::PanZoomController;
use stem::thing::sys::{find, prop_get};
use stem::thing::ThingId;
use alloc::string::String;
use blossom::widgets::TextInputState;

/// Click result when a click is detected
#[derive(Debug, Clone, Copy)]
pub struct ClickEvent {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub enum InputResult {
    None,
    ViewportUpdated,
    FormUpdated,
    Submit(String),
    ToggleDebug,
    Click(ClickEvent),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FocusTarget {
    Graph,
    FormInput,
    FormButton,
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

    // Focus state
    pub focus: FocusTarget,
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
            focus: FocusTarget::Graph, // Start with graph focused
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
pub fn poll_and_apply(
    ctrl: &mut PanZoomController,
    state: &mut InputState,
    text_input: &mut TextInputState,
    button_pressed: &mut bool
) -> InputResult {
    let mut result = InputResult::None;

    let bristle = match state.get_bristle() {
        Some(b) => b,
        None => return InputResult::None,
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

        // Simple hit testing for focus (assuming hardcoded layout for now)
        // Form is at top. Height approx 50px?
        // Layout: <row><label/><input/><submit/></row>
        // Input: approx x=60, y=0, w=200, h=30?
        // Button: approx x=270, y=0, w=60, h=30?
        // This is fragile but sufficient for "handles mouse input" requirement without full hit testing engine.

        if state.pointer_y < 50 {
            // Click in form area
            if state.pointer_x > 50 && state.pointer_x < 250 {
                state.focus = FocusTarget::FormInput;
                text_input.focused = true;
                *button_pressed = false;
                result = InputResult::FormUpdated;
            } else if state.pointer_x > 260 && state.pointer_x < 360 {
                state.focus = FocusTarget::FormButton;
                text_input.focused = false;
                *button_pressed = true;
                result = InputResult::FormUpdated;
            } else {
                state.focus = FocusTarget::Graph;
                text_input.focused = false;
                *button_pressed = false;
                result = InputResult::FormUpdated;
            }
        } else {
            // Click in graph area
            if state.focus != FocusTarget::Graph {
                 state.focus = FocusTarget::Graph;
                 text_input.focused = false;
                 *button_pressed = false;
                 result = InputResult::FormUpdated;
            }
            ctrl.begin_drag(state.pointer_x as f32, state.pointer_y as f32, stem::monotonic_ns());
        }
    }

    // Handle left button release: end drag or register click
    if !left_now && left_was {
        state.left_down = false;
        
        if *button_pressed {
            *button_pressed = false;
            if state.focus == FocusTarget::FormButton {
                // Button click action
                return InputResult::Submit(text_input.text.clone());
            }
            result = InputResult::FormUpdated;
        }

        if state.focus == FocusTarget::Graph {
            // Check if this was a click (minimal movement) vs a drag
            let dx = (state.pointer_x - state.click_start_x).abs();
            let dy = (state.pointer_y - state.click_start_y).abs();
            let was_click = dx < 5 && dy < 5;

            if was_click {
                result = InputResult::Click(ClickEvent {
                    x: state.pointer_x,
                    y: state.pointer_y,
                });
            } else {
                ctrl.end_drag();
                result = InputResult::ViewportUpdated;
            }
        }
    }

    // Handle drag movement
    if state.left_down && moved && ctrl.is_dragging() && state.focus == FocusTarget::Graph {
        ctrl.update_drag(state.pointer_x as f32, state.pointer_y as f32, stem::monotonic_ns());
        result = InputResult::ViewportUpdated;
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

            // Tab handling for focus cycling
            if key == Key::Tab {
                let shift = mods.has_shift();
                if shift {
                    // Previous
                     match state.focus {
                        FocusTarget::Graph => state.focus = FocusTarget::FormButton,
                        FocusTarget::FormButton => state.focus = FocusTarget::FormInput,
                        FocusTarget::FormInput => state.focus = FocusTarget::Graph,
                    }
                } else {
                    // Next
                    match state.focus {
                        FocusTarget::Graph => state.focus = FocusTarget::FormInput,
                        FocusTarget::FormInput => state.focus = FocusTarget::FormButton,
                        FocusTarget::FormButton => state.focus = FocusTarget::Graph,
                    }
                }

                // Sync detailed states
                match state.focus {
                    FocusTarget::Graph => {
                        text_input.focused = false;
                        *button_pressed = false;
                    }
                    FocusTarget::FormInput => {
                        text_input.focused = true;
                        *button_pressed = false;
                    }
                    FocusTarget::FormButton => {
                        text_input.focused = false;
                        // Button focused, but not pressed yet (wait for enter/space)
                        *button_pressed = false;
                    }
                }

                return InputResult::FormUpdated;
            }

            match state.focus {
                FocusTarget::Graph => {
                    let (kb_updated, kb_toggle_debug) = handle_key_down_graph(key, mods, ctrl);
                    if kb_toggle_debug { return InputResult::ToggleDebug; }
                    if kb_updated { return InputResult::ViewportUpdated; }
                }
                FocusTarget::FormInput => {
                     // Pass to text input widget
                     if key == Key::Enter {
                         return InputResult::Submit(text_input.text.clone());
                     }
                     text_input.handle_key(key, mods.has_shift());
                     return InputResult::FormUpdated;
                }
                FocusTarget::FormButton => {
                    if key == Key::Enter || key == Key::Space {
                        return InputResult::Submit(text_input.text.clone());
                    }
                    // Allow navigation keys to bubble up? Or consume?
                    // Consuming avoids moving the graph while focused on button.
                }
            }
        }
    }

    result
}

/// Handle keyboard shortcuts for viewport control.
fn handle_key_down_graph(key: Key, mods: Mods, ctrl: &mut PanZoomController) -> (bool, bool) {
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
