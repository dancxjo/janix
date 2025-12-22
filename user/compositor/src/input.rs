use alloc::string::ToString;
use thing_os::prelude::*;

use crate::config::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH, TITLE_BAR_HEIGHT};
use crate::graph;
use crate::layout::{StackedWindow, hit_test};
use crate::model::{Compositor, CursorState, DragState};
use abi::ThingId;
use thing_models::PropValue;
use thing_models::graph_kinds;
use thing_os::update_props;

use thing_os::list_things_by_kind;
use thing_os::resident::mouse::{
    MouseEntry, MouseStreamHeader, MouseStreamMapped, MouseStream,
};
use thing_os::resident::{Resident, ResidentError, ResidentMapPerms, map_resident};

impl CursorState {
    pub fn apply_packet(&mut self, dx: i64, dy: i64, buttons: u64, max_x: i32, max_y: i32) -> u64 {
        let previous = self.buttons;
        self.x = (self.x + dx as i32).clamp(0, max_x.saturating_sub(1));
        self.y = (self.y - dy as i32).clamp(0, max_y.saturating_sub(1));
        self.buttons = buttons;
        previous
    }

    pub fn left_pressed_changed(previous: u64, current: u64) -> bool {
        (previous & 1) != (current & 1)
    }

    pub fn left_down(current: u64) -> bool {
        current & 1 != 0
    }
}

impl Compositor {
    pub fn process_mouse_packets(&mut self, layout: &[StackedWindow]) {
        if self.mouse_stream.is_none() {
            let streams = list_things_by_kind::<MouseStream>();
            if let Some(thing) = streams.first() {
                if let Ok(map_resp) = map_resident(thing.id, ResidentMapPerms::READ) {
                    unsafe {
                        // TODO: Verify map_resp.byte_len against expected size?
                        let obj = Resident::<()>::new(
                            thing.id,
                            map_resp.user_addr as *mut u8,
                            map_resp.byte_len as usize,
                        );
                        self.mouse_stream = Some(MouseStreamMapped::new(obj));
                        let msg = alloc::format!(
                            "compositor: mouse stream mapped id={:?} addr={:?} len={}",
                            thing.id,
                            map_resp.user_addr,
                            map_resp.byte_len
                        );
                        println!("{}", alloc::boxed::Box::leak(msg.into_boxed_str()));
                    }
                }
            }
        }

        if let Some(stream) = &self.mouse_stream {
            // Use a stack-local buffer or a persistent buffer in Compositor to avoid alloc?
            // For now, persistent buffer on stack is fine if small, or just Vec inside Compositor struct.
            // But implementing changes here: local Vec.
            let mut events = Vec::new();
            // Note: in a real loop we'd reuse this Vec across frames.

            let new_head = stream.read_entries_into(self.mouse_head, &mut events);
            if new_head != self.mouse_head {
                // Log advancement occasionally or on change? Too verbose if constant.
                // Plan says "Log mapping and head advancement".
                // Let's log if it jumps significantly or just debug.
                // "compositor: head advanced old_tail=... new_tail=... n=..."
                // let msg = alloc::format!("compositor: head advanced old={:?} new={:?} n={}", self.mouse_head, new_head, events.len());
                // println(sys, alloc::boxed::Box::leak(msg.into_boxed_str()));
                // Actually, commenting this out for noise unless I want strict verification.
                // User plan verification step: "Check for: compositor: head advanced".
                // So I MUST log it.
                let msg = alloc::format!(
                    "compositor: head advanced old_tail={} new_tail={} n={}",
                    self.mouse_head,
                    new_head,
                    events.len()
                );
                println!("{}", alloc::boxed::Box::leak(msg.into_boxed_str()));
            }
            self.mouse_head = new_head;

            if !events.is_empty() {
                let msg = alloc::format!("DEBUG: processing {} events", events.len());
                println!("{}", alloc::boxed::Box::leak(msg.into_boxed_str()));

                use crate::config::{MOUSE_SCALE_DEN, MOUSE_SCALE_NUM};

                let mut pending_dx: i64 = 0;
                let mut pending_dy: i64 = 0;
                // Initialize with the first event's button state so we don't flash-trigger
                let mut pending_buttons = events[0].buttons;

                for entry in events {
                    if entry.buttons != pending_buttons {
                        // Flush collected motion for the *previous* button state
                        let sdx = pending_dx * MOUSE_SCALE_NUM as i64 / MOUSE_SCALE_DEN as i64;
                        let sdy = pending_dy * MOUSE_SCALE_NUM as i64 / MOUSE_SCALE_DEN as i64;
                        self.apply_mouse_event(sdx, sdy, pending_buttons as u64, layout);

                        // Reset for new state
                        pending_dx = 0;
                        pending_dy = 0;
                        pending_buttons = entry.buttons;
                    }

                    pending_dx += entry.dx as i64;
                    pending_dy += entry.dy as i64;
                }

                // Flush final batch
                let sdx = pending_dx * MOUSE_SCALE_NUM as i64 / MOUSE_SCALE_DEN as i64;
                let sdy = pending_dy * MOUSE_SCALE_NUM as i64 / MOUSE_SCALE_DEN as i64;
                self.apply_mouse_event(sdx, sdy, pending_buttons as u64, layout);
            }
        }

        if let Some(drag) = &self.drag {
            if !layout.iter().any(|w| w.id == drag.window_id) {
                self.drag = None;
            }
        }
    }

    fn apply_mouse_event(&mut self, dx: i64, dy: i64, buttons: u64, layout: &[StackedWindow]) {
        let old_x = self.cursor.x;
        let old_y = self.cursor.y;

        let previous = self.cursor.apply_packet(
            dx,
            dy,
            buttons,
            self.fb.info.width as i32,
            self.fb.info.height as i32,
        );

        if self.cursor.x != old_x || self.cursor.y != old_y {
            // Add damage for old and new cursor positions
            // Ideally we know the exact cursor size.
            // For now, assume ample size (e.g. 32x32) or look up from sprites.
            // Let's use 32x32 as a safe default for standard cursors.
            // Wait, we have self.cursor_sprites available!
            // But determining *which* sprite needs checking cursor kind.
            // Let's just use a safe bounding box 64x64 or 32x32.
            let trash_size = 48; // safe upper bound
            let sprite = self.cursor_sprites.for_kind(self.cursor.kind);
            let w = sprite.bitmap.width as u32;
            let h = sprite.bitmap.height as u32; // actually use real size

            // Old position damage
            let hotspot = sprite.hotspot;
            self.add_damage(crate::widget_layout::Rect::new(
                old_x - hotspot.0,
                old_y - hotspot.1,
                w,
                h,
            ));

            // New position damage
            self.add_damage(crate::widget_layout::Rect::new(
                self.cursor.x - hotspot.0,
                self.cursor.y - hotspot.1,
                w,
                h,
            ));
        }

        if CursorState::left_pressed_changed(previous, buttons) {
            if CursorState::left_down(buttons) {
                self.handle_left_press(layout);
            } else {
                self.handle_left_release();
            }
        } else if CursorState::left_down(buttons) {
            self.continue_drag();
        }
    }

    fn handle_left_press(&mut self, layout: &[StackedWindow]) {
        if let Some(window) = hit_test(layout, self.cursor.x, self.cursor.y) {
            self.ensure_window_active_from_layout(window, layout);

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

    fn continue_drag(&mut self) {
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
                graph_kinds::PROP_WINDOW_X.to_string(),
                PropValue::I64(clamped_x as i64),
            ),
            (
                graph_kinds::PROP_WINDOW_Y.to_string(),
                PropValue::I64(clamped_y as i64),
            ),
        ];
        let _ = update_props(drag.window_id, &updates);
    }
}
