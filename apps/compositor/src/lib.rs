#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use userland::prelude::*;
use userland_std::{Mode, ModeSwitchEvent, Surface, Window, graph_kinds};

const CLEAR_COLOR: u32 = 0xFF202025; // BGRA
const TITLE_COLOR_ACTIVE: u32 = 0xFF3A7BD5;
const TITLE_COLOR_INACTIVE: u32 = 0xFF2A2A2F;
const WINDOW_BG: u32 = 0xFF0E0E10;
const TEXT_COLOR: u32 = 0xFFFFFFFF;
const CURSOR_COLOR: u32 = 0xFFFFCC00;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "compositor: starting");
    ensure_ui_schemas(sys);
    let fb = match userland_std::open_primary_display_buffer(sys) {
        Ok(fb) => fb,
        Err(_) => {
            println(sys, "compositor: no framebuffer; exiting");
            sys.exit_thread();
        }
    };

    loop {
        handle_mode_switches(sys);
        render_once(sys, &fb);
        sys.sleep_for_ns(16_000_000); // ~60fps
    }
}

fn handle_mode_switches<S: Sys>(sys: &mut S) {
    let events: Vec<ModeSwitchEvent> = list_things_by_kind(sys);
    let latest = events.into_iter().max_by_key(|e| e.timestamp);
    if let Some(ev) = latest {
        set_active_mode(sys, ev.mode_index);
    }
}

fn set_active_mode<S: Sys>(sys: &mut S, index: u8) {
    let mut modes: Vec<Mode> = list_things_by_kind(sys);
    for mode in modes.iter_mut() {
        let active = mode.index == index;
        let _ = update_props(
            sys,
            mode.id,
            &[(graph_kinds::PROP_MODE_ACTIVE, PropValue::Bool(active))],
        );
    }
}

fn render_once<S: Sys>(sys: &mut S, fb: &userland_std::PrimaryDisplayBuffer) {
    let buffer = fb.ptr as *mut u32;
    let width = fb.info.width;
    let height = fb.info.height;
    let stride = fb.info.stride;

    // Clear
    fill_rect(
        buffer,
        stride,
        width,
        height,
        0,
        0,
        width as i32,
        height as i32,
        CLEAR_COLOR,
    );

    let mode = active_mode(sys).or_else(|| default_mode(sys));
    if mode.is_none() {
        return;
    }
    let mode = mode.unwrap();

    let windows: Vec<Window> = list_things_by_kind(sys)
        .into_iter()
        .filter(|w: &Window| w.place_id == mode.place_id.unwrap_or(ThingId(0)))
        .collect();

    let surfaces: Vec<Surface> = list_things_by_kind(sys);

    // Basic z-order: lower z first
    let mut windows = windows;
    windows.sort_by_key(|w| w.z_index);

    for win in windows.iter() {
        draw_window(buffer, stride, width, height, win, &surfaces);
    }

    // Draw simple cursor at center for now.
    draw_cursor(
        buffer,
        stride,
        width,
        height,
        width as i32 / 2,
        height as i32 / 2,
    );
}

fn active_mode<S: Sys>(sys: &mut S) -> Option<Mode> {
    list_things_by_kind::<S, Mode>(sys)
        .into_iter()
        .find(|m| m.active)
}

fn default_mode<S: Sys>(sys: &mut S) -> Option<Mode> {
    list_things_by_kind::<S, Mode>(sys)
        .into_iter()
        .min_by_key(|m| m.index)
}

fn draw_window(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    win: &Window,
    surfaces: &[Surface],
) {
    let title_h = 24;
    let x = win.x;
    let y = win.y;
    let w = win.width.max(32);
    let h = win.height.max(title_h + 8);

    fill_rect(buffer, stride, fb_width, fb_height, x, y, w, h, WINDOW_BG);

    let title_color = if win.active {
        TITLE_COLOR_ACTIVE
    } else {
        TITLE_COLOR_INACTIVE
    };
    fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x,
        y,
        w,
        title_h,
        title_color,
    );

    // Simple text rendering as tiny bars; enough to see window titles.
    draw_text(
        buffer,
        stride,
        fb_width,
        fb_height,
        x + 6,
        y + 6,
        &win.title,
        TEXT_COLOR,
    );

    if let Some(surface) = surfaces.iter().find(|s| s.window_id == win.id) {
        draw_text(
            buffer,
            stride,
            fb_width,
            fb_height,
            x + 8,
            y + title_h + 4,
            &surface.text,
            TEXT_COLOR,
        );
    }
}

fn draw_cursor(buffer: *mut u32, stride: u32, width: u32, height: u32, cx: i32, cy: i32) {
    let size = 10;
    let x = cx.clamp(0, width as i32 - size);
    let y = cy.clamp(0, height as i32 - size);
    fill_rect(
        buffer,
        stride,
        width,
        height,
        x,
        y,
        size,
        size,
        CURSOR_COLOR,
    );
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
    let mut row = y.max(0) as usize;
    let max_row = fb_height as usize;
    while row < y.saturating_add(h) as usize && row < max_row {
        let mut col = x.max(0) as usize;
        let max_col = fb_width as usize;
        while col < x.saturating_add(w) as usize && col < max_col {
            let idx = row * stride_pixels + col;
            unsafe {
                *buffer.add(idx) = color;
            }
            col += 1;
        }
        row += 1;
    }
}

fn draw_text(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    text: &str,
    color: u32,
) {
    let mut cx = x;
    let mut cy = y;
    for ch in text.bytes() {
        if ch == b'\n' {
            cy += 10;
            cx = x;
            continue;
        }
        draw_glyph(buffer, stride, fb_width, fb_height, cx, cy, ch, color);
        cx += 6;
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
    // Tiny 4x6 pseudo glyph based on the byte pattern for simplicity.
    for dy in 0..6 {
        for dx in 0..4 {
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
