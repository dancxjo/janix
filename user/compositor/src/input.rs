use thing_os::prelude::*;

use crate::config::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH, TITLE_BAR_HEIGHT};
use crate::graph;
use crate::layout::{StackedWindow, hit_test};
use crate::model::{Compositor, CursorState, DragState};

use thing_os::resident::{map_resident, Resident, ResidentMapPerms, ResidentError};
use thing_os::resident::mouse::{MouseStreamMapped, MouseStreamThing, MouseEntry, MouseStreamHeader};
use thing_os::{list_things_by_kind};

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
    pub fn process_mouse_packets<S: Sys>(&mut self, sys: &mut S, layout: &[StackedWindow]) {
        if self.mouse_stream.is_none() {
             let streams = list_things_by_kind::<S, MouseStreamThing>(sys);
             if let Some(thing) = streams.first() {
                 if let Ok(map_resp) = map_resident(sys, thing.id, ResidentMapPerms::READ) {
                      unsafe {
                          // TODO: Verify map_resp.byte_len against expected size?
                          let obj = Resident::<()>::new(thing.id, map_resp.user_addr as *mut u8, map_resp.byte_len as usize);
                          self.mouse_stream = Some(MouseStreamMapped::new(obj));
                          let msg = alloc::format!("compositor: mouse stream mapped id={:?} addr={:?} len={}", thing.id, map_resp.user_addr, map_resp.byte_len);
                          println(sys, alloc::boxed::Box::leak(msg.into_boxed_str()));
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
                  let msg = alloc::format!("compositor: head advanced old_tail={} new_tail={} n={}", self.mouse_head, new_head, events.len());
                  println(sys, alloc::boxed::Box::leak(msg.into_boxed_str()));
             }
             self.mouse_head = new_head;
             
             for entry in events {
                  self.apply_mouse_event(
                       sys, 
                       entry.dx as i64, 
                       entry.dy as i64, 
                       entry.buttons as u64, 
                       layout
                  );
             }
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
        dx: i64,
        dy: i64,
        buttons: u64,
        layout: &[StackedWindow],
    ) {
        let previous =
            self.cursor
                .apply_packet(dx, dy, buttons, self.fb.info.width as i32, self.fb.info.height as i32);

        if CursorState::left_pressed_changed(previous, buttons) {
            if CursorState::left_down(buttons) {
                self.handle_left_press(sys, layout);
            } else {
                self.handle_left_release();
            }
        } else if CursorState::left_down(buttons) {
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
                thing_os::graph_kinds::PROP_WINDOW_X,
                PropValue::I64(clamped_x as i64),
            ),
            (
                thing_os::graph_kinds::PROP_WINDOW_Y,
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
    use crate::test_support::{FramebufferFixture, MockSys, list_responses, success};
    use abi::{KernelRequest, KernelResponse, PropValue, ThingId, graph_kinds};


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
        let previous = cursor.apply_packet(-10, 20, 3, 8, 8);
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
                .map(|p| p.1.clone());
            let z_index = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_Z_INDEX)
                .map(|p| p.1.clone());
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

        comp.apply_mouse_event(&mut sys, 0, 0, 1, &[win]);
        comp.apply_mouse_event(&mut sys, -50, 30, 1, &[stacked_window(9)]);

        assert_eq!(comp.drag.as_ref().map(|d| d.window_id), Some(ThingId(9)));
        assert_eq!(comp.drag.as_ref().map(|d| d.last_sent_x), Some(0));
        assert_eq!(comp.drag.as_ref().map(|d| d.last_sent_y), Some(0));

        let requests = sys.drain_requests();
        assert_eq!(requests.len(), 2);
        if let KernelRequest::ThingUpdate { props, .. } = &requests[1] {
            let x = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_WINDOW_X)
                .map(|p| p.1.clone());
            let y = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_WINDOW_Y)
                .map(|p| p.1.clone());
            assert_eq!(x, Some(PropValue::I64(0)));
            assert_eq!(y, Some(PropValue::I64(0)));
        } else {
            panic!("expected position update");
        }
    }

    #[test]
    fn test_process_mouse_stream() {
        use abi::resident_layout::ResidentHeader;
        use thing_os::resident::mouse::{MouseStreamMapped, MouseStreamThing, MouseStreamHeader, MouseEntry};
        use thing_os::resident::{ResidentMapResp, ResidentMapPerms, Resident};
        use alloc::vec;

        let fb = FramebufferFixture::new(100, 100);
        let mut comp = Compositor::new(fb.fb);
        
        // Create backing store with CORRECT layout
        // Header + MouseStreamHeader + Entries
        let capacity = 10;
        let header_size = core::mem::size_of::<ResidentHeader>(); // 32
        let stream_header_size = core::mem::size_of::<MouseStreamHeader>(); // 16
        let entries_size = capacity as usize * core::mem::size_of::<MouseEntry>(); // 10*8=80
        let total_size = header_size + stream_header_size + entries_size;
        
        // Align up to power of 2 for allocator safety? No, vec is fine.
        let mut backing = vec![0u8; total_size];
        let ptr = backing.as_mut_ptr();
        
        unsafe {
             let header = ptr as *mut ResidentHeader;
             (*header).magic = ResidentHeader::MAGIC;
             (*header).version = 1;
             (*header).data_off = header_size as u32; // Offset to start of stream header
             (*header).total_len = total_size as u32; 
             
             let obj = Resident::<()>::new(ThingId(99), ptr, total_size);
             let mut stream = MouseStreamMapped::new(obj);
             stream.init(capacity);
             
             // Append some events
             stream.append(MouseEntry { buttons: 0, flags: 0, dx: 10, dy: 5, _pad: 0 });
             stream.append(MouseEntry { buttons: 0, flags: 0, dx: -5, dy: -2, _pad: 0 });
        }
        
        let responses = vec![
             // response for find_thing (load_thing call 0)
             KernelResponse::ThingData {
                 id: ThingId(10),
                 kind: MouseStreamThing::KIND,
                 props: &[],
             },
             // response for map_resident
             KernelResponse::ResidentMapped {
                 resp: ResidentMapResp {
                      user_addr: ptr as u64,
                      byte_len: total_size as u32,
                      _pad: 0,
                 }
             }
        ];
        
        let mut sys = MockSys::with_responses(responses);
        comp.process_mouse_packets(&mut sys, &[]);
        
        // Cursor starts center (50, 50).
        // Event 1: dx=10, dy=5 -> (60, 45) (y is subtracted)
        // Event 2: dx=-5, dy=-2 -> (55, 47) (y subtracted: 45 - (-2) = 47)
        
        assert_eq!(comp.cursor.x, 55);
        assert_eq!(comp.cursor.y, 47);
        assert_eq!(comp.mouse_head, 2);
    }
}
