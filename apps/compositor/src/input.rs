use userland::prelude::*;
use userland_std::thing_models::MousePacketEvent;

use crate::config::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH, TITLE_BAR_HEIGHT};
use crate::graph;
use crate::layout::{StackedWindow, hit_test};
use crate::model::{Compositor, CursorState, DragState};

impl CursorState {
    pub fn apply_packet(&mut self, packet: &MousePacketEvent, max_x: i32, max_y: i32) -> u8 {
        let previous = self.buttons;
        self.x = (self.x + packet.delta_x as i32).clamp(0, max_x.saturating_sub(1));
        self.y = (self.y - packet.delta_y as i32).clamp(0, max_y.saturating_sub(1));
        self.buttons = packet.buttons;
        previous
    }

    pub fn left_pressed_changed(previous: u8, current: u8) -> bool {
        (previous & 1) != (current & 1)
    }

    pub fn left_down(current: u8) -> bool {
        current & 1 != 0
    }
}

impl Compositor {
    pub fn process_mouse_packets<S: Sys>(&mut self, sys: &mut S, layout: &[StackedWindow]) {
        let events = graph::mouse_packets(sys);
        for event in events {
            if event.sequence_index <= self.last_mouse_seq {
                continue;
            }
            self.last_mouse_seq = event.sequence_index;
            self.apply_mouse_event(sys, &event, layout);
        }

        if let Some(drag) = &self.drag {
            if !layout.iter().any(|w| w.id == drag.window_id) {
                self.drag = None;
            }
        }
    }

    fn apply_mouse_event<S: Sys>(
        &mut self,
        sys: &mut S,
        event: &MousePacketEvent,
        layout: &[StackedWindow],
    ) {
        let previous =
            self.cursor
                .apply_packet(event, self.fb.info.width as i32, self.fb.info.height as i32);

        if CursorState::left_pressed_changed(previous, event.buttons) {
            if CursorState::left_down(event.buttons) {
                self.handle_left_press(sys, layout);
            } else {
                self.handle_left_release();
            }
        } else if CursorState::left_down(event.buttons) {
            self.continue_drag(sys);
        }
    }

    fn handle_left_press<S: Sys>(&mut self, sys: &mut S, layout: &[StackedWindow]) {
        if let Some(window) = hit_test(layout, self.cursor.x, self.cursor.y) {
            self.ensure_window_active_from_layout(sys, window, layout);

            let in_title = self.cursor.y < window.y + TITLE_BAR_HEIGHT;
            if in_title {
                let state = DragState {
                    window_id: window.id,
                    grab_offset_x: self.cursor.x - window.x,
                    grab_offset_y: self.cursor.y - window.y,
                    width: window.width.max(MIN_WINDOW_WIDTH),
                    height: window.height.max(MIN_WINDOW_HEIGHT),
                    last_sent_x: window.x,
                    last_sent_y: window.y,
                };
                self.drag = Some(state);
            } else {
                self.drag = None;
            }
        } else {
            self.drag = None;
        }
    }

    fn handle_left_release(&mut self) {
        self.drag = None;
    }

    fn continue_drag<S: Sys>(&mut self, sys: &mut S) {
        let Some(drag) = &mut self.drag else {
            return;
        };

        let max_x = (self.fb.info.width as i32 - drag.width).max(0);
        let max_y = (self.fb.info.height as i32 - drag.height).max(0);
        let new_x = self.cursor.x - drag.grab_offset_x;
        let new_y = self.cursor.y - drag.grab_offset_y;
        let clamped_x = new_x.clamp(0, max_x);
        let clamped_y = new_y.clamp(0, max_y);

        if clamped_x == drag.last_sent_x && clamped_y == drag.last_sent_y {
            return;
        }

        drag.last_sent_x = clamped_x;
        drag.last_sent_y = clamped_y;
        let updates = [
            (
                userland_std::graph_kinds::PROP_WINDOW_X,
                PropValue::I64(clamped_x as i64),
            ),
            (
                userland_std::graph_kinds::PROP_WINDOW_Y,
                PropValue::I64(clamped_y as i64),
            ),
        ];
        let _ = update_props(sys, drag.window_id, &updates);
    }
}
