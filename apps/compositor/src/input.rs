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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TITLE_BAR_HEIGHT;
    use crate::layout::StackedWindow;
    use crate::test_support::{list_responses, FramebufferFixture, MockSys, success};
    use abi::{KernelRequest, PropValue, ThingId, graph_kinds};

    fn packet(seq: u64, buttons: u8, dx: i16, dy: i16) -> MousePacketEvent {
        MousePacketEvent {
            id: ThingId(seq),
            controller_id: ThingId(1),
            port_index: 0,
            sequence_index: seq,
            timestamp_ticks: seq * 10,
            buttons,
            delta_x: dx,
            delta_y: dy,
            overflow_x: false,
            overflow_y: false,
        }
    }

    fn stacked_window(id: u64) -> StackedWindow {
        StackedWindow {
            id: ThingId(id),
            x: 10,
            y: 10,
            width: 120,
            height: 90,
            z_index: 1,
            active: false,
        }
    }

    #[test]
    fn cursor_apply_packet_clamps_and_tracks_buttons() {
        let mut cursor = CursorState::new(5, 5);
        let previous = cursor.apply_packet(&packet(1, 3, -10, 20), 8, 8);
        assert_eq!(previous, 0);
        assert_eq!(cursor.x, 0);
        assert_eq!(cursor.y, 0);
        assert_eq!(cursor.buttons, 3);
    }

    #[test]
    fn left_press_in_title_starts_drag_and_activates_window() {
        let fb = FramebufferFixture::new(300, 200);
        let mut comp = Compositor::new(fb.fb);
        comp.cursor.x = 30;
        comp.cursor.y = 12;
        let win = stacked_window(7);
        let mut sys = MockSys::with_responses(vec![success()]);

        comp.apply_mouse_event(&mut sys, &packet(1, 1, 0, 0), &[win]);
        assert_eq!(comp.active_window, Some(ThingId(7)));
        let drag = comp.drag.expect("dragging should start inside title bar");
        assert_eq!(drag.grab_offset_x, 20);
        assert_eq!(drag.grab_offset_y, 2);

        let requests = sys.drain_requests();
        assert_eq!(requests.len(), 1);
        if let KernelRequest::ThingUpdate { props, .. } = &requests[0] {
            let active = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_WINDOW_ACTIVE)
                .map(|p| p.1);
            let z_index = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_Z_INDEX)
                .map(|p| p.1);
            assert_eq!(active, Some(PropValue::Bool(true)));
            assert_eq!(z_index, Some(PropValue::I64(2)));
        } else {
            panic!("expected ThingUpdate request, got {:?}", requests[0]);
        }
    }

    #[test]
    fn dragging_updates_window_position_and_clamps() {
        let fb = FramebufferFixture::new(120, 100);
        let mut comp = Compositor::new(fb.fb);
        comp.cursor.x = 40;
        comp.cursor.y = 12 + TITLE_BAR_HEIGHT / 2;
        let win = stacked_window(9);
        let mut sys = MockSys::with_responses(vec![success(), success()]);

        comp.apply_mouse_event(&mut sys, &packet(1, 1, 0, 0), &[win]);
        comp.apply_mouse_event(&mut sys, &packet(2, 1, -50, 30), &[stacked_window(9)]);

        assert_eq!(comp.drag.as_ref().map(|d| d.window_id), Some(ThingId(9)));
        assert_eq!(comp.drag.as_ref().map(|d| d.last_sent_x), Some(0));
        assert_eq!(comp.drag.as_ref().map(|d| d.last_sent_y), Some(0));

        let requests = sys.drain_requests();
        assert_eq!(requests.len(), 2);
        if let KernelRequest::ThingUpdate { props, .. } = &requests[1] {
            let x = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_WINDOW_X)
                .map(|p| p.1);
            let y = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_WINDOW_Y)
                .map(|p| p.1);
            assert_eq!(x, Some(PropValue::I64(0)));
            assert_eq!(y, Some(PropValue::I64(0)));
        } else {
            panic!("expected position update");
        }
    }

    #[test]
    fn process_mouse_packets_skips_stale_sequences() {
        let fb = FramebufferFixture::new(80, 80);
        let mut comp = Compositor::new(fb.fb);
        let events = vec![
            MousePacketEvent {
                id: ThingId(1),
                controller_id: ThingId(1),
                port_index: 0,
                sequence_index: 1,
                timestamp_ticks: 0,
                buttons: 0,
                delta_x: 0,
                delta_y: 0,
                overflow_x: false,
                overflow_y: false,
            },
            MousePacketEvent {
                id: ThingId(2),
                controller_id: ThingId(1),
                port_index: 0,
                sequence_index: 1,
                timestamp_ticks: 1,
                buttons: 0,
                delta_x: 0,
                delta_y: 0,
                overflow_x: false,
                overflow_y: false,
            },
        ];

        let responses = list_responses(events);
        let mut sys = MockSys::with_responses(responses);
        comp.process_mouse_packets(&mut sys, &[]);
        assert_eq!(comp.last_mouse_seq, 1);
    }
}
