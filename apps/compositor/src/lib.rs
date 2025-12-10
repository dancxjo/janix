#![no_std]

extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use core::cmp::{max, min};
use userland::prelude::*;
use userland_std::thing_models::MousePacketEvent;
use userland_std::{
    active_mode, default_mode, graph_kinds, is_console_mode_active, Mode, ModeSwitchEvent,
    PrimaryDisplayBuffer, Surface, Window, MODE_INDEX_CONSOLE,
};

const CLEAR_COLOR: u32 = 0xFF101014;
const FRAME_BG: u32 = 0xFF1D1E22;
const FRAME_BORDER: u32 = 0xFF3A3C45;
const TITLE_COLOR_ACTIVE: u32 = 0xFF4C8BF5;
const TITLE_COLOR_INACTIVE: u32 = 0xFF2B2D34;
const TITLE_TEXT_COLOR: u32 = 0xFFF0F0F0;
const CONTENT_BG: u32 = 0xFF121214;
const TEXT_COLOR: u32 = 0xFFE4E4E4;
const CURSOR_COLOR: u32 = 0xFFFFCC00;

const TITLE_BAR_HEIGHT: i32 = 26;
const FRAME_THICKNESS: i32 = 2;
const MIN_WINDOW_WIDTH: i32 = 80;
const MIN_WINDOW_HEIGHT: i32 = 60;
const LINE_HEIGHT: i32 = 10;
const GLYPH_WIDTH: i32 = 4;
const GLYPH_HEIGHT: i32 = 6;
const CHAR_ADVANCE: i32 = 6;
const FRAME_INTERVAL_NS: u64 = 16_000_000;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "compositor: starting");
    ensure_ui_schemas(sys);
    let _ = register_schema_for::<MousePacketEvent>(sys);

    let fb = loop {
        if let Some(fb) = active_framebuffer(sys) {
            break fb;
        }
        println(sys, "compositor: waiting for primary display");
        sys.sleep_for_ns(50_000_000);
    };

    let mut compositor = Compositor::new(fb);

    loop {
        compositor.tick(sys);
        sys.sleep_for_ns(FRAME_INTERVAL_NS);
    }
}

struct Compositor {
    fb: PrimaryDisplayBuffer,
    cursor: CursorState,
    last_mouse_seq: u64,
    active_window: Option<ThingId>,
    drag: Option<DragState>,
}

impl Compositor {
    fn new(fb: PrimaryDisplayBuffer) -> Self {
        let cursor = CursorState::new(fb.info.width as i32 / 2, fb.info.height as i32 / 2);
        Self {
            fb,
            cursor,
            last_mouse_seq: 0,
            active_window: None,
            drag: None,
        }
    }

    fn tick<S: Sys>(&mut self, sys: &mut S) {
        handle_mode_switches(sys);

        if is_console_mode_active(sys) {
            self.process_mouse_packets(sys, &[]);
            self.drag = None;
            return;
        }

        let mode = match active_mode(sys).or_else(|| default_mode(sys)) {
            Some(mode) => mode,
            None => {
                self.process_mouse_packets(sys, &[]);
                return;
            }
        };

        if mode.index == MODE_INDEX_CONSOLE {
            self.process_mouse_packets(sys, &[]);
            self.drag = None;
            return;
        }

        let place_id = mode.place_id.unwrap_or(ThingId(0));
        let windows = collect_windows_for_place(sys, place_id);
        let surface_map = collect_surfaces_for_windows(sys, &windows);
        self.sync_active_from_windows(&windows);
        self.process_mouse_packets(sys, &windows);
        self.render(&windows, &surface_map);
    }

    fn render(&self, windows: &[Window], surfaces: &BTreeMap<ThingId, Surface>) {
        let buffer = self.fb.ptr as *mut u32;
        let width = self.fb.info.width;
        let height = self.fb.info.height;
        let stride = self.fb.info.stride;

        fill_rect(buffer, stride, width, height, 0, 0, width as i32, height as i32, CLEAR_COLOR);

        let mut ordered = windows.to_vec();
        ordered.sort_by_key(|w| w.z_index);
        for window in ordered.iter() {
            let surface = surfaces.get(&window.id);
            draw_window(buffer, stride, width, height, window, surface);
        }

        draw_cursor(buffer, stride, width, height, self.cursor.x, self.cursor.y);
    }

    fn process_mouse_packets<S: Sys>(&mut self, sys: &mut S, windows: &[Window]) {
        let mut events: Vec<MousePacketEvent> = list_things_by_kind(sys);
        events.sort_by_key(|e| e.sequence_index);

        for event in events.into_iter() {
            if event.sequence_index <= self.last_mouse_seq {
                continue;
            }
            self.last_mouse_seq = event.sequence_index;
            self.apply_mouse_event(sys, &event, windows);
        }

        if let Some(drag) = &self.drag {
            if !windows.iter().any(|w| w.id == drag.window_id) {
                self.drag = None;
            }
        }
    }

    fn apply_mouse_event<S: Sys>(
        &mut self,
        sys: &mut S,
        event: &MousePacketEvent,
        windows: &[Window],
    ) {
        let previous = self.cursor.apply_packet(
            event,
            self.fb.info.width as i32,
            self.fb.info.height as i32,
        );

        if CursorState::left_pressed_changed(previous, event.buttons) {
            if CursorState::left_down(event.buttons) {
                self.handle_left_press(sys, windows);
            } else {
                self.handle_left_release();
            }
        } else if CursorState::left_down(event.buttons) {
            self.continue_drag(sys);
        }
    }

    fn handle_left_press<S: Sys>(&mut self, sys: &mut S, windows: &[Window]) {
        if let Some(window) = top_window_at(windows, self.cursor.x, self.cursor.y) {
            self.ensure_window_active(sys, window, windows);
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
        let Some(drag) = &mut self.drag else { return; };

        let max_x = max(self.fb.info.width as i32 - drag.width, 0);
        let max_y = max(self.fb.info.height as i32 - drag.height, 0);
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
            (graph_kinds::PROP_WINDOW_X, PropValue::I64(clamped_x as i64)),
            (graph_kinds::PROP_WINDOW_Y, PropValue::I64(clamped_y as i64)),
        ];
        let _ = update_props(sys, drag.window_id, &updates);
    }

    fn sync_active_from_windows(&mut self, windows: &[Window]) {
        if let Some(win) = windows.iter().find(|w| w.active) {
            self.active_window = Some(win.id);
        } else if self
            .active_window
            .map(|id| windows.iter().any(|w| w.id == id))
            .unwrap_or(false)
        {
            // Keep current active id if it still exists but graph hasn't updated yet.
        } else {
            self.active_window = None;
        }
    }

    fn ensure_window_active<S: Sys>(
        &mut self,
        sys: &mut S,
        window: &Window,
        windows: &[Window],
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

        let max_z = windows.iter().map(|w| w.z_index).max().unwrap_or(0);
        let new_z = max_z.saturating_add(1);
        let updates = [
            (graph_kinds::PROP_WINDOW_ACTIVE, PropValue::Bool(true)),
            (graph_kinds::PROP_Z_INDEX, PropValue::I64(new_z as i64)),
        ];
        let _ = update_props(sys, window.id, &updates);
        self.active_window = Some(window.id);
    }
}

fn handle_mode_switches<S: Sys>(sys: &mut S) {
    let events: Vec<ModeSwitchEvent> = list_things_by_kind(sys);
    if let Some(latest) = events.into_iter().max_by_key(|e| e.timestamp) {
        set_active_mode(sys, latest.mode_index);
    }
}

fn set_active_mode<S: Sys>(sys: &mut S, index: u8) {
    let modes: Vec<Mode> = list_things_by_kind(sys);
    for mode in modes {
        let active = mode.index == index;
        let _ = update_props(
            sys,
            mode.id,
            &[(graph_kinds::PROP_MODE_ACTIVE, PropValue::Bool(active))],
        );
    }
}

fn collect_windows_for_place<S: Sys>(sys: &mut S, place_id: ThingId) -> Vec<Window> {
    list_things_by_kind(sys)
        .into_iter()
        .filter(|w: &Window| w.place_id == place_id)
        .collect()
}

fn collect_surfaces_for_windows<S: Sys>(
    sys: &mut S,
    windows: &[Window],
) -> BTreeMap<ThingId, Surface> {
    let wanted: BTreeSet<ThingId> = windows.iter().map(|w| w.id).collect();
    let mut map = BTreeMap::new();
    for surface in list_things_by_kind::<S, Surface>(sys).into_iter() {
        if wanted.contains(&surface.window_id) {
            map.insert(surface.window_id, surface);
        }
    }
    map
}

fn draw_window(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    window: &Window,
    surface: Option<&Surface>,
) {
    let w = window.width.max(MIN_WINDOW_WIDTH);
    let h = window.height.max(MIN_WINDOW_HEIGHT);
    if w <= 0 || h <= 0 {
        return;
    }
    let x = window.x;
    let y = window.y;
    if x >= fb_width as i32 || y >= fb_height as i32 {
        return;
    }

    fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x,
        y,
        w,
        h,
        FRAME_BORDER,
    );

    fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x + FRAME_THICKNESS,
        y + FRAME_THICKNESS,
        w - FRAME_THICKNESS * 2,
        h - FRAME_THICKNESS * 2,
        FRAME_BG,
    );

    let title_color = if window.active {
        TITLE_COLOR_ACTIVE
    } else {
        TITLE_COLOR_INACTIVE
    };
    fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x + FRAME_THICKNESS,
        y + FRAME_THICKNESS,
        w - FRAME_THICKNESS * 2,
        TITLE_BAR_HEIGHT,
        title_color,
    );

    draw_text(
        buffer,
        stride,
        fb_width,
        fb_height,
        x + FRAME_THICKNESS + 6,
        y + FRAME_THICKNESS + 6,
        w - FRAME_THICKNESS * 2 - 12,
        TITLE_BAR_HEIGHT - 8,
        &window.title,
        TITLE_TEXT_COLOR,
    );

    let content_x = x + FRAME_THICKNESS + 4;
    let content_y = y + FRAME_THICKNESS + TITLE_BAR_HEIGHT + 4;
    let content_w = w - FRAME_THICKNESS * 2 - 8;
    let content_h = h - TITLE_BAR_HEIGHT - FRAME_THICKNESS * 2 - 8;
    if content_w > 0 && content_h > 0 {
        fill_rect(
            buffer,
            stride,
            fb_width,
            fb_height,
            content_x,
            content_y,
            content_w,
            content_h,
            CONTENT_BG,
        );

        if let Some(surface) = surface {
            draw_text(
                buffer,
                stride,
                fb_width,
                fb_height,
                content_x + 4,
                content_y + 4,
                content_w - 8,
                content_h - 8,
                &surface.text,
                TEXT_COLOR,
            );
        }
    }
}

fn draw_cursor(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    cx: i32,
    cy: i32,
) {
    let size = 12;
    let x = cx.clamp(0, fb_width as i32 - 1);
    let y = cy.clamp(0, fb_height as i32 - 1);
    fill_rect(buffer, stride, fb_width, fb_height, x - 1, y, size, 2, CURSOR_COLOR);
    fill_rect(buffer, stride, fb_width, fb_height, x, y - 1, 2, size, CURSOR_COLOR);
}

fn fill_rect(
    buffer: *mut u32,
    stride_bytes: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
) {
    if w <= 0 || h <= 0 {
        return;
    }
    let stride_pixels = (stride_bytes / 4) as usize;
    let start_row = y.max(0) as usize;
    let end_row = min(y.saturating_add(h), fb_height as i32).max(0) as usize;
    for row in start_row..end_row {
        let start_col = x.max(0) as usize;
        let end_col = min(x.saturating_add(w), fb_width as i32).max(0) as usize;
        for col in start_col..end_col {
            let idx = row * stride_pixels + col;
            unsafe {
                *buffer.add(idx) = color;
            }
        }
    }
}

fn draw_text(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    max_width: i32,
    max_height: i32,
    text: &str,
    color: u32,
) {
    let mut cx = x;
    let mut cy = y;
    for ch in text.bytes() {
        if ch == b'\n' {
            cy += LINE_HEIGHT;
            if cy + GLYPH_HEIGHT > y + max_height {
                break;
            }
            cx = x;
            continue;
        }

        if cx + GLYPH_WIDTH > x + max_width {
            cy += LINE_HEIGHT;
            if cy + GLYPH_HEIGHT > y + max_height {
                break;
            }
            cx = x;
        }

        if cy + GLYPH_HEIGHT > y + max_height {
            break;
        }

        draw_glyph(buffer, stride, fb_width, fb_height, cx, cy, ch, color);
        cx += CHAR_ADVANCE;
    }
}

fn draw_glyph(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    byte: u8,
    color: u32,
) {
    for dy in 0..GLYPH_HEIGHT {
        for dx in 0..GLYPH_WIDTH {
            let bit = (byte >> ((dx + dy) % 8)) & 1;
            if bit != 0 {
                fill_rect(
                    buffer,
                    stride,
                    fb_width,
                    fb_height,
                    x + dx,
                    y + dy,
                    1,
                    1,
                    color,
                );
            }
        }
    }
}

fn top_window_at<'a>(windows: &'a [Window], x: i32, y: i32) -> Option<&'a Window> {
    let mut best: Option<&Window> = None;
    for window in windows {
        let w = window.width.max(MIN_WINDOW_WIDTH);
        let h = window.height.max(MIN_WINDOW_HEIGHT);
        if x >= window.x
            && x < window.x + w
            && y >= window.y
            && y < window.y + h
        {
            match best {
                Some(current) if current.z_index > window.z_index => {}
                _ => best = Some(window),
            }
        }
    }
    best
}

fn active_framebuffer<S: Sys>(sys: &mut S) -> Option<PrimaryDisplayBuffer> {
    userland_std::open_primary_display_buffer(sys).ok()
}

struct CursorState {
    x: i32,
    y: i32,
    buttons: u8,
}

impl CursorState {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, buttons: 0 }
    }

    fn apply_packet(&mut self, packet: &MousePacketEvent, max_x: i32, max_y: i32) -> u8 {
        let previous = self.buttons;
        self.x = (self.x + packet.delta_x as i32).clamp(0, max_x.saturating_sub(1));
        self.y = (self.y - packet.delta_y as i32).clamp(0, max_y.saturating_sub(1));
        self.buttons = packet.buttons;
        previous
    }

    fn left_pressed_changed(previous: u8, current: u8) -> bool {
        (previous & 1) != (current & 1)
    }

    fn left_down(current: u8) -> bool {
        current & 1 != 0
    }
}

struct DragState {
    window_id: ThingId,
    grab_offset_x: i32,
    grab_offset_y: i32,
    width: i32,
    height: i32,
    last_sent_x: i32,
    last_sent_y: i32,
}
