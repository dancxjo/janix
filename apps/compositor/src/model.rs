use userland::prelude::*;
use userland_std::edge_targets;
use userland_std::thing_models::DisplayPresentRequest;
use userland_std::{PrimaryDisplayBuffer, graph_kinds};

use crate::layout::StackedWindow;

#[derive(Debug)]
pub struct Compositor {
    pub fb: PrimaryDisplayBuffer,
    pub cursor: CursorState,
    pub last_mouse_seq: u64,
    pub active_window: Option<ThingId>,
    pub drag: Option<DragState>,
    framebuffer_thing_id: Option<ThingId>,
    present_request_id: Option<ThingId>,
    frame_counter: u64,
}

impl Compositor {
    pub fn new(fb: PrimaryDisplayBuffer) -> Self {
        let cx = fb.info.width as i32 / 2;
        let cy = fb.info.height as i32 / 2;
        Self {
            fb,
            cursor: CursorState::new(cx, cy),
            last_mouse_seq: 0,
            active_window: None,
            drag: None,
            framebuffer_thing_id: None,
            present_request_id: None,
            frame_counter: 0,
        }
    }
    pub fn sync_active_from_layout(&mut self, stacked: &[StackedWindow]) {
        if let Some(win) = stacked.iter().find(|w| w.active) {
            self.active_window = Some(win.id);
        } else if self
            .active_window
            .map(|id| stacked.iter().any(|w| w.id == id))
            .unwrap_or(false)
        {
        } else {
            self.active_window = None;
        }
    }

    pub fn ensure_window_active_from_layout<S: Sys>(
        &mut self,
        sys: &mut S,
        window: &StackedWindow,
        stacked: &[StackedWindow],
    ) {
        if self.active_window == Some(window.id) {
            return;
        }

        if let Some(prev) = self.active_window {
            let _ = update_props(
                sys,
                prev,
                &[(graph_kinds::PROP_WINDOW_ACTIVE, PropValue::Bool(false))],
            );
        }

        let max_z = stacked.iter().map(|w| w.z_index).max().unwrap_or(0);
        let new_z = max_z.saturating_add(1);
        let updates = [
            (graph_kinds::PROP_WINDOW_ACTIVE, PropValue::Bool(true)),
            (graph_kinds::PROP_Z_INDEX, PropValue::I64(new_z as i64)),
        ];
        let _ = update_props(sys, window.id, &updates);
        self.active_window = Some(window.id);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CursorState {
    pub x: i32,
    pub y: i32,
    pub buttons: u8,
}

impl CursorState {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y, buttons: 0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DragState {
    pub window_id: ThingId,
    pub grab_offset_x: i32,
    pub grab_offset_y: i32,
    pub width: i32,
    pub height: i32,
    pub last_sent_x: i32,
    pub last_sent_y: i32,
}

impl Compositor {
    pub fn ensure_display_contracts<S: Sys>(&mut self, sys: &mut S) {
        if self.framebuffer_thing_id.is_none() {
            let mut targets = edge_targets(
                sys,
                self.fb.display_id,
                graph_kinds::EDGE_DISPLAY_FRONT_BUFFER,
            );
            self.framebuffer_thing_id = targets.pop();
        }

        if let Some(fb_id) = self.framebuffer_thing_id {
            if self.present_request_id.is_none() {
                if let Some(existing) = Self::find_present_request(sys, fb_id) {
                    self.present_request_id = Some(existing.id);
                } else {
                    let request = DisplayPresentRequest {
                        id: ThingId(0),
                        framebuffer_id: fb_id,
                        frame_index: 0,
                        requested_at_ns: 0,
                        presented_at_ns: None,
                        completed: true,
                    };
                    if let Some(id) = create_thing(sys, &request) {
                        self.present_request_id = Some(id);
                    }
                }
            }
        }
    }

    fn find_present_request<S: Sys>(sys: &mut S, fb_id: ThingId) -> Option<DisplayPresentRequest> {
        list_things_by_kind::<S, DisplayPresentRequest>(sys)
            .into_iter()
            .find(|req| req.framebuffer_id == fb_id)
    }

    pub fn publish_present_request<S: Sys>(&mut self, sys: &mut S) {
        let Some(req_id) = self.present_request_id else {
            return;
        };
        self.frame_counter = self.frame_counter.wrapping_add(1);
        let now = sys.time_monotonic_ns();
        let updates = [
            (
                graph_kinds::PROP_FRAME_INDEX,
                PropValue::U64(self.frame_counter),
            ),
            (graph_kinds::PROP_REQUESTED_AT_NS, PropValue::U64(now)),
            (graph_kinds::PROP_COMPLETED, PropValue::Bool(false)),
            (graph_kinds::PROP_PRESENTED_AT_NS, PropValue::U64(0)),
        ];
        let _ = update_props(sys, req_id, &updates);
    }
}
