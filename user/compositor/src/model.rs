use thing_os::prelude::*;
use thing_os::link_targets;
use thing_os::thing_models::DisplayPresentRequest;
use thing_os::{PrimaryDisplayBuffer, graph_kinds};

use crate::layout::StackedWindow;
use crate::render::cursor::{self, CursorKind, CursorSprites};
use crate::widget_layout::Rect;
use alloc::vec::Vec;
use thing_os::resident::mouse::MouseStreamMapped;

#[derive(Debug, Clone, Copy)]
pub struct BackgroundImage {
    pub ptr: *const u8,
    pub size: usize,
    pub width: i32,
    pub height: i32,
    pub bpp: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ConsoleBuffer {
    pub id: ThingId,
    pub ptr: *const u8,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: abi::PixelFormat,
}


#[derive(Debug, Clone, Copy)]
pub struct MappedSurface {
    pub ptr: *const u8,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: abi::PixelFormat,
    pub size: usize,
}

#[derive(Debug)]
pub struct Compositor {
    pub fb: PrimaryDisplayBuffer,
    pub cursor: CursorState,
    pub cursor_sprites: CursorSprites,
    pub last_mouse_seq: u64,
    pub last_mouse_event_id: Option<ThingId>,
    pub active_window: Option<ThingId>,
    pub drag: Option<DragState>,
    framebuffer_thing_id: Option<ThingId>,
    present_request_id: Option<ThingId>,
    pub background_image: Option<BackgroundImage>,
    pub background_offset: (i32, i32),
    pub console_buffer: Option<ConsoleBuffer>,
    pub mapped_surfaces: alloc::collections::BTreeMap<ThingId, MappedSurface>,
    // Dirty rectangle tracking
    pub damage: Vec<Rect>,
    pub previous_damage: Vec<Rect>,
    pub mouse_stream: Option<MouseStreamMapped<()>>,
    pub mouse_head: u32,
    pub frame_counter: u64,
    pub cached_layout: alloc::vec::Vec<StackedWindow>,
}

impl Compositor {
    pub fn new(fb: PrimaryDisplayBuffer) -> Self {
        let cx = fb.info.width as i32 / 2;
        let cy = fb.info.height as i32 / 2;
        Self {
            fb,
            cursor: CursorState::new(cx, cy),
            cursor_sprites: cursor::build_cursor_sprites(),
            last_mouse_seq: 0,
            last_mouse_event_id: None,
            active_window: None,
            drag: None,
            framebuffer_thing_id: None,
            present_request_id: None,
            background_image: None,
            background_offset: (0, 0),

            console_buffer: None,
            mapped_surfaces: alloc::collections::BTreeMap::new(),
            damage: Vec::new(),
            previous_damage: Vec::new(),
            mouse_stream: None,
            mouse_head: 0,
            frame_counter: 0,
            cached_layout: alloc::vec::Vec::new(),
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

    pub fn add_damage(&mut self, rect: Rect) {
        self.damage.push(rect);
    }

    pub fn add_full_damage(&mut self) {
        self.damage.clear();
        self.damage.push(Rect::new(
            0, 
            0, 
            self.fb.info.width, 
            self.fb.info.height
        ));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CursorState {
    pub x: i32,
    pub y: i32,
    pub buttons: u64,
    pub visible: bool,
    pub kind: CursorKind,
}

impl CursorState {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            buttons: 0,
            visible: true,
            kind: CursorKind::Arrow,
        }
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
            let mut targets = link_targets(
                sys,
                self.fb.display_id,
                graph_kinds::LINK_DISPLAY_FRONT_BUFFER,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::StackedWindow;
    use crate::test_support::{FramebufferFixture, MockSys, list_responses, success};
    use abi::{KernelRequest, KernelResponse, PropValue, ThingId, graph_kinds};
    use thing_os::thing_models::DisplayPresentRequest;

    fn stacked_window(id: u64, z_index: i64, active: bool) -> StackedWindow {
        StackedWindow {
            id: ThingId(id),
            x: 0,
            y: 0,
            width: 50,
            height: 50,
            z_index,
            active,
        }
    }

    #[test]
    fn sync_active_tracks_layout_state() {
        let fb = FramebufferFixture::new(40, 40);
        let mut comp = Compositor::new(fb.fb);
        let stacked = vec![stacked_window(1, 0, true), stacked_window(2, 1, false)];
        comp.sync_active_from_layout(&stacked);
        assert_eq!(comp.active_window, Some(ThingId(1)));

        let stacked = vec![stacked_window(2, 1, false)];
        comp.sync_active_from_layout(&stacked);
        assert_eq!(comp.active_window, None);
    }

    #[test]
    fn ensure_window_active_updates_previous_and_bumps_z() {
        let fb = FramebufferFixture::new(60, 60);
        let mut comp = Compositor::new(fb.fb);
        comp.active_window = Some(ThingId(1));
        let stacked = vec![stacked_window(1, 1, true), stacked_window(2, 3, false)];
        let mut sys = MockSys::with_responses(vec![success(), success()]);

        comp.ensure_window_active_from_layout(&mut sys, &stacked[1], &stacked);
        assert_eq!(comp.active_window, Some(ThingId(2)));

        let requests = sys.drain_requests();
        assert_eq!(requests.len(), 2);
        if let KernelRequest::ThingUpdate { id, props } = &requests[0] {
            assert_eq!(*id, ThingId(1));
            let active = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_WINDOW_ACTIVE)
                .map(|p| p.1.clone());
            assert_eq!(active, Some(PropValue::Bool(false)));
        } else {
            panic!("expected ThingUpdate");
        }

        if let KernelRequest::ThingUpdate { id, props } = &requests[1] {
            assert_eq!(*id, ThingId(2));
            let active = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_WINDOW_ACTIVE)
                .map(|p| p.1.clone());
            let z_index = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_Z_INDEX)
                .map(|p| p.1.clone());
            assert_eq!(active, Some(PropValue::Bool(true)));
            assert_eq!(z_index, Some(PropValue::I64(4)));
        } else {
            panic!("expected ThingUpdate for new active window");
        }
    }

    #[test]
    fn ensure_display_contracts_reuses_present_request() {
        let fb = FramebufferFixture::new(80, 80);
        let mut comp = Compositor::new(fb.fb);
        let fb_id = ThingId(10);
        let present = DisplayPresentRequest {
            id: ThingId(77),
            framebuffer_id: fb_id,
            frame_index: 0,
            requested_at_ns: 0,
            presented_at_ns: None,
            completed: true,
        };

        let mut responses = vec![
            KernelResponse::LinkTarget {
                target: Some(fb_id),
            },
            KernelResponse::LinkTarget { target: None },
        ];
        responses.extend(list_responses(vec![present.clone()], |p| p.id));
        let mut sys = MockSys::with_responses(responses);

        comp.ensure_display_contracts(&mut sys);
        assert_eq!(comp.framebuffer_thing_id, Some(fb_id));
        assert_eq!(comp.present_request_id, Some(present.id));
    }

    #[test]
    fn ensure_display_contracts_creates_request_when_missing() {
        let fb = FramebufferFixture::new(80, 80);
        let mut comp = Compositor::new(fb.fb);
        let fb_id = ThingId(5);

        let responses = vec![
            KernelResponse::LinkTarget {
                target: Some(fb_id),
            },
            KernelResponse::LinkTarget { target: None },
            KernelResponse::ThingListEntry { id: None },
            KernelResponse::ThingCreated { id: ThingId(44) },
        ];
        let mut sys = MockSys::with_responses(responses);
        comp.ensure_display_contracts(&mut sys);
        assert_eq!(comp.present_request_id, Some(ThingId(44)));
        assert_eq!(comp.framebuffer_thing_id, Some(fb_id));
    }

    #[test]
    fn publish_present_request_updates_frame_state() {
        let fb = FramebufferFixture::new(100, 100);
        let mut comp = Compositor::new(fb.fb);
        comp.present_request_id = Some(ThingId(30));
        let mut sys = MockSys::with_responses(vec![success()]);
        sys.set_time(40);

        comp.publish_present_request(&mut sys);
        assert_eq!(comp.frame_counter, 1);

        let requests = sys.drain_requests();
        assert_eq!(requests.len(), 1);
        if let KernelRequest::ThingUpdate { props, .. } = &requests[0] {
            let frame_index = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_FRAME_INDEX)
                .map(|p| p.1.clone());
            let requested_at = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_REQUESTED_AT_NS)
                .map(|p| p.1.clone());
            let completed = props
                .iter()
                .find(|p| p.0 == graph_kinds::PROP_COMPLETED)
                .map(|p| p.1.clone());

            assert_eq!(frame_index, Some(PropValue::U64(1)));
            assert_eq!(requested_at, Some(PropValue::U64(41)));
            assert_eq!(completed, Some(PropValue::Bool(false)));
        } else {
            panic!("expected ThingUpdate for present request");
        }
    }
}
