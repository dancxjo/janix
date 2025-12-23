use abi::ThingId;
use alloc::string::ToString;
use thing_models::PropValue;
use thing_models::graph_kinds;
use thing_os::link_targets;
use thing_os::prelude::*;
use thing_os::{create_thing, list_things_by_kind, load_thing, update_props};

use crate::layout::StackedWindow;
use crate::render::cursor::{self, CursorKind, CursorSprites};
use crate::widget_layout::Rect;
use alloc::vec::Vec;
use thing_os::resident::mouse::MouseStreamMapped;
use crate::schemas::FramebufferInfo;

#[derive(Debug, Clone, Copy)]
pub struct BackgroundImage {
    pub ptr: *const u8,
    pub size: usize,
    pub width: i32,
    pub height: i32,
    pub bpp: u16,
}

#[derive(Debug, Clone)]
pub struct BackgroundCanvas {
    pub pixels: Vec<u32>,
    pub width: u32,
    pub height: u32,
    pub stride_pixels: u32,
}

impl BackgroundCanvas {
    pub fn as_ptr(&self) -> *const u8 {
        self.pixels.as_ptr() as *const u8
    }

    pub fn stride_bytes(&self) -> u32 {
        self.stride_pixels * 4
    }
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
    pub fb_info: FramebufferInfo,
    pub fb_ptr: *mut u8,
    pub display_id: ThingId, // We need to know which display we are on
    pub buffer_index: u32,
    
    pub cursor: CursorState,
    pub cursor_sprites: CursorSprites,
    pub last_mouse_seq: u64,
    pub last_mouse_event_id: Option<ThingId>,
    pub active_window: Option<ThingId>,
    pub drag: Option<DragState>,

    pub background_image: Option<BackgroundImage>,
    pub background_offset: (i32, i32),
    pub background_canvas: Option<BackgroundCanvas>,
    pub console_buffer: Option<ConsoleBuffer>,
    pub mapped_surfaces: alloc::collections::BTreeMap<ThingId, MappedSurface>,

    // Dirty rectangle tracking
    pub damage: Vec<Rect>,
    pub previous_damage: Vec<Rect>, // Kept for cursor trail cleaning, if needed

    pub mouse_stream: Option<MouseStreamMapped<()>>,
    pub mouse_head: u32,
    pub mouse_received: bool,

    pub frame_counter: u64,
    pub cached_layout: alloc::vec::Vec<StackedWindow>,
}

impl Compositor {
    pub fn new(fb_info: FramebufferInfo, fb_ptr: *mut u8, buffer_index: u32, display_id: ThingId) -> Self {
        let cx = fb_info.width as i32 / 2;
        let cy = fb_info.height as i32 / 2;
        Self {
            fb_info,
            fb_ptr,
            buffer_index,
            display_id,
            cursor: CursorState::new(cx, cy),
            cursor_sprites: cursor::build_cursor_sprites(),
            last_mouse_seq: 0,
            last_mouse_event_id: None,
            active_window: None,
            drag: None,
            background_image: None,
            background_offset: (0, 0),
            background_canvas: None,

            console_buffer: None,
            mapped_surfaces: alloc::collections::BTreeMap::new(),
            damage: Vec::new(),
            previous_damage: Vec::new(),
            mouse_stream: None,
            mouse_head: 0,
            mouse_received: false,
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

    pub fn ensure_window_active_from_layout(
        &mut self,
        window: &StackedWindow,
        stacked: &[StackedWindow],
    ) {
        if self.active_window == Some(window.id) {
            return;
        }

        if let Some(prev) = self.active_window {
            let _ = update_props(
                prev,
                &[(
                    graph_kinds::PROP_WINDOW_ACTIVE.to_string(),
                    PropValue::Bool(false),
                )],
            );
        }

        let max_z = stacked.iter().map(|w| w.z_index).max().unwrap_or(0);
        let new_z = max_z.saturating_add(1);
        let updates = [
            (
                graph_kinds::PROP_WINDOW_ACTIVE.to_string(),
                PropValue::Bool(true),
            ),
            (
                graph_kinds::PROP_Z_INDEX.to_string(),
                PropValue::I64(new_z as i64),
            ),
        ];
        let _ = update_props(window.id, &updates);
        self.active_window = Some(window.id);
    }

    pub fn add_damage(&mut self, rect: Rect) {
        self.damage.push(rect);
    }

    pub fn add_full_damage(&mut self) {
        self.damage.clear();
        self.damage
            .push(Rect::new(0, 0, self.fb_info.width as u32, self.fb_info.height as u32));
    }

    // No-op via this method, logic handled in state.rs via Intent
    pub fn present_frame(&mut self) {
        self.frame_counter = self.frame_counter.wrapping_add(1);
        // We don't update metadata on FramebufferInfo here because it's package-owned by driver.
        // We will issue PresentIntent in state.rs main loop.
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
