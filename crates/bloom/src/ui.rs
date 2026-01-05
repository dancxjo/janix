use abi::ids::ThingId;
use abi::types::AlignedRelBufLarge;
use alloc::string::String;
use alloc::vec::Vec;
use models::*;
use thing_std::graph::{relationships_from, symbol_intern};
use thing_std::symbol_resolve;
use thing_std::SyscallGraphClient;

use crate::layout::{layout_widgets, PlacedWidget, TITLE_BAR_HEIGHT};
use crate::pixels::blend_pixel;
use crate::scene::Rect;
#[cfg(feature = "shadows")]
use crate::shadow::{draw_shadow_from_mask, ShadowMask, ShadowParams};
use crate::text::draw_text;

/// Font size for window titles
const TITLE_FONT_SIZE: f32 = 14.0;
/// Title text color (dark gray for contrast)
const TITLE_TEXT_COLOR: u32 = 0xFF333333;
/// Horizontal padding for title text
const TITLE_PADDING_X: i32 = 8;

#[derive(Clone)]
pub enum WidgetKind {
    Label(Label, String),
    Button(Button, String),
}

#[derive(Clone)]
pub struct WindowScene {
    pub id: ThingId,
    pub window: Window,
    pub layout: Layout,
    pub children: Vec<WidgetKind>,
    pub title_text: String,
}

pub fn window_ids_in_graph(graph_id: ThingId) -> Vec<ThingId> {
    let pred_contains = symbol_intern("predicate.contains");
    let mut aligned_buf = AlignedRelBufLarge::default();
    let buf = &mut aligned_buf.inner;
    let mut cursor = 0u64;
    let mut out = Vec::new();

    loop {
        match relationships_from(graph_id, cursor, buf) {
            Ok((count, total)) => {
                if count == 0 {
                    break;
                }
                for rel in buf.iter().take(count as usize) {
                    if rel.kind == pred_contains {
                        out.push(rel.target);
                    }
                }
                if cursor + count as u64 >= total {
                    break;
                }
                cursor += count as u64;
            }
            Err(_) => break,
        }
    }
    out
}

pub fn read_window_scene(client: &mut SyscallGraphClient, window_id: ThingId) -> Option<WindowScene> {
    let window = Window::read(client, window_id).ok()?;
    let layout = Layout::read(client, window.content_root).ok()?;

    // Resolve title text from symbol
    let title_text = symbol_resolve(window.title)
        .map(|v| String::from_utf8_lossy(&v).into_owned())
        .unwrap_or_else(|| String::from("Window"));

    let rel_child = symbol_intern("child");
    let mut aligned_child = AlignedRelBufLarge::default();
    let child_buf = &mut aligned_child.inner;
    let mut children = Vec::new();
    let mut child_cursor = 0u64;

    loop {
        match relationships_from(window.content_root, child_cursor, child_buf) {
            Ok((child_count, child_total)) => {
                if child_count == 0 {
                    break;
                }
                for crel in child_buf.iter().take(child_count as usize) {
                    if crel.kind != rel_child {
                        continue;
                    }
                    if let Ok(label) = Label::read(client, crel.target) {
                        let text = symbol_resolve(label.text)
                            .map(|v| String::from_utf8_lossy(&v).into_owned())
                            .unwrap_or_else(|| String::from("Label"));
                        children.push(WidgetKind::Label(label, text));
                        continue;
                    }
                    if let Ok(button) = Button::read(client, crel.target) {
                        let text = symbol_resolve(button.text)
                            .map(|v| String::from_utf8_lossy(&v).into_owned())
                            .unwrap_or_else(|| String::from("Button"));
                        children.push(WidgetKind::Button(button, text));
                        continue;
                    }
                }
                if child_cursor + child_count as u64 >= child_total {
                    break;
                }
                child_cursor += child_count as u64;
            }
            Err(_) => break,
        }
    }

    Some(WindowScene {
        id: window_id,
        window,
        layout,
        children,
        title_text,
    })
}

pub fn collect_window_scenes(client: &mut SyscallGraphClient) -> Vec<WindowScene> {
    let mut out = Vec::new();
    let windows_graph = if let Some(id) = thing_std::graph::thing_find("graph.windows") {
        thing_std::log_info(&alloc::format!("BLOOM: found graph.windows id={}", id.low()));
        id
    } else {
        thing_std::log_info("BLOOM: graph.windows NOT FOUND");
        return out;
    };

    for window_id in window_ids_in_graph(windows_graph) {
        if let Some(scene) = read_window_scene(client, window_id) {
            out.push(scene);
        }
    }

    out
}

pub fn render_window_scenes(dest: *mut u32, screen_w: u32, screen_h: u32, scenes: &[WindowScene]) {
    for scene in scenes {
        render_window(dest, screen_w, screen_h, scene);
    }
}

/// Compute the screen bounding rect for clipping.
#[inline]
fn screen_rect(screen_w: u32, screen_h: u32) -> Rect {
    Rect { x: 0, y: 0, w: screen_w, h: screen_h }
}

fn render_window(dest: *mut u32, screen_w: u32, screen_h: u32, scene: &WindowScene) {
    let win = &scene.window;
    let rect = Rect {
        x: win.x,
        y: win.y,
        w: win.width,
        h: win.height,
    };

    // Clip to screen
    let screen = screen_rect(screen_w, screen_h);
    let clip = rect.intersect(screen);

    thing_std::log_info(&alloc::format!(
        "BLOOM: render_window rect=({},{},{},{}) clip=({},{},{},{})",
        rect.x, rect.y, rect.w, rect.h,
        clip.x, clip.y, clip.w, clip.h
    ));

    if clip.is_empty() {
        thing_std::log_info("BLOOM: window clipped away, skipping");
        return;
    }

    let t0 = thing_std::monotonic_now();
    #[cfg(feature = "shadows")]
    if win.style.shadow != 0 {
        unsafe {
            draw_shadow_from_mask(
                dest,
                screen_w,
                screen_h,
                rect.x,
                rect.y,
                ShadowMask::RoundedRect {
                    width: rect.w,
                    height: rect.h,
                    radius: win.style.radius,
                },
                ShadowParams {
                    offset_x: 2,
                    offset_y: 3,
                    blur_radius: 4,
                    color: 0x44000000,
                },
            );
        }
    }
    let t1 = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: shadow {} ms", (t1-t0)/1_000_000));

    paint_panel(dest, screen_w, screen_h, rect, clip, win.style.bg_rgba, win.style.radius, Some(TITLE_BAR_HEIGHT));
    let t2 = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: paint_panel {} ms", (t2-t1)/1_000_000));

    // Draw window title in the title bar
    if !scene.title_text.is_empty() {
        let text_x = rect.x + TITLE_PADDING_X;
        let text_y = rect.y + (TITLE_BAR_HEIGHT - TITLE_FONT_SIZE as i32) / 2;
        draw_text(dest, screen_w, screen_h, text_x, text_y, &scene.title_text, TITLE_TEXT_COLOR, TITLE_FONT_SIZE);
    }
    let t2b = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: title {} ms", (t2b-t2)/1_000_000));

    // Layout phase: compute widget positions (no graph syscalls here)
    let placed = layout_widgets(&scene.layout, &scene.children, rect);

    // Paint phase: draw widgets at computed positions
    for pw in placed {
        match pw.widget {
            WidgetKind::Label(ref label, _) => {
                paint_label(dest, screen_w, screen_h, pw.rect, label);
            }
            WidgetKind::Button(ref button, _) => {
                paint_button(dest, screen_w, screen_h, pw.rect, button);
            }
        }
    }
    let t3 = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: widgets {} ms", (t3-t2b)/1_000_000));

    draw_border(dest, screen_w, screen_h, rect, clip, win.style.radius);
    let t4 = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: border {} ms", (t4-t3)/1_000_000));

    thing_std::log_info(&alloc::format!("BLOOM: render_window TOTAL {} ms", (t4-t0)/1_000_000));
}

fn draw_border(dest: *mut u32, screen_w: u32, _screen_h: u32, rect: Rect, clip: Rect, radius: u16) {
    let border_color: u32 = 0xFF404040;
    let thickness: i32 = 1;
    let outer_r = radius;
    let inner_r = radius.saturating_sub(thickness as u16);

    for y in clip.y..(clip.y + clip.h as i32) {
        let local_y = y - rect.y;
        for x in clip.x..(clip.x + clip.w as i32) {
            let local_x = x - rect.x;

            if !in_round(local_x, local_y, rect.w, rect.h, outer_r) { continue; }

            // inside inner rounded rect? then not border
            let inner_x = local_x - thickness;
            let inner_y = local_y - thickness;
            let inner_w = rect.w.saturating_sub((thickness * 2) as u32);
            let inner_h = rect.h.saturating_sub((thickness * 2) as u32);

            if inner_x >= 0 && inner_y >= 0
                && inner_x < inner_w as i32 && inner_y < inner_h as i32
                && in_round(inner_x, inner_y, inner_w, inner_h, inner_r)
            {
                continue;
            }

            let idx = (y as u32 * screen_w + x as u32) as usize;
            unsafe { *dest.add(idx) = border_color; }
        }
    }
}

fn paint_panel(
    dest: *mut u32,
    screen_w: u32,
    _screen_h: u32,
    rect: Rect,
    clip: Rect,
    base: u32,
    radius: u16,
    stripe_height: Option<i32>,
) {
    let light = adjust_color(base, 6);
    let dark  = adjust_color(base, -8);

    // Precompute gradient per row using integer math: grad in [-4..+4]
    // This yields a gentle center-bright panel.
    let h = rect.h.max(1) as i32;

    for dy in 0..clip.h as i32 {
        let y = clip.y + dy;
        let local_y = y - rect.y;
        // integer "tent" shape centered vertically
        let dist = (local_y - h / 2).abs();
        let grad = ((h / 2 - dist) * 4 / (h / 2).max(1)).clamp(0, 4) as i16;
        let grad = grad - 2; // shift to ~[-2..+2], tweak as desired

        let row_color = adjust_color(base, grad);

        for dx in 0..clip.w as i32 {
            let x = clip.x + dx;
            let local_x = x - rect.x;

            if !in_round(local_x, local_y, rect.w, rect.h, radius) { continue; }

            let idx = (y as u32 * screen_w + x as u32) as usize;
            unsafe { *dest.add(idx) = row_color; }
        }
    }

    // Bevel: only draw within clip edges, not whole rect
    // Top + bottom lines
    for dx in 0..clip.w as i32 {
        let x = clip.x + dx;
        let local_x = x - rect.x;

        let top_y = rect.y;
        if top_y >= clip.y && top_y < (clip.y + clip.h as i32) {
            if in_round(local_x, 0, rect.w, rect.h, radius) {
                let idx = (top_y as u32 * screen_w + x as u32) as usize;
                unsafe { *dest.add(idx) = light; }
            }
        }

        let bottom_y = rect.y + rect.h as i32 - 1;
        if bottom_y >= clip.y && bottom_y < (clip.y + clip.h as i32) {
            if in_round(local_x, rect.h as i32 - 1, rect.w, rect.h, radius) {
                let idx = (bottom_y as u32 * screen_w + x as u32) as usize;
                unsafe { *dest.add(idx) = dark; }
            }
        }
    }

    // Optional stripes: only if stripes overlap the clip
    if let Some(hs) = stripe_height {
        let title_h = hs.min(rect.h as i32).max(0);
        let stripe_a = adjust_color(base, -3);

        let y0 = rect.y.max(clip.y);
        let y1 = (rect.y + title_h).min(clip.y + clip.h as i32);

        for y in y0..y1 {
            let local_y = y - rect.y;
            if local_y % 2 != 0 { continue; }

            for x in clip.x..(clip.x + clip.w as i32) {
                let local_x = x - rect.x;
                if !in_round(local_x, local_y, rect.w, rect.h, radius) { continue; }
                let idx = (y as u32 * screen_w + x as u32) as usize;
                unsafe { *dest.add(idx) = stripe_a; }
            }
        }
    }
}


fn paint_label(dest: *mut u32, screen_w: u32, screen_h: u32, rect: Rect, label: &Label) {
    let screen = screen_rect(screen_w, screen_h);
    let clip = rect.intersect(screen);
    if clip.is_empty() {
        return;
    }

    let color = label.style.color_rgba;
    let base_y = rect.y + (rect.h as i32 / 2);

    for offset in 0..2 {
        let y = base_y + offset;
        if y < clip.y || y >= clip.y + clip.h as i32 {
            continue;
        }
        for cx in clip.x..(clip.x + clip.w as i32) {
            let idx = (y as u32 * screen_w + cx as u32) as usize;
            unsafe {
                let dst = *dest.add(idx);
                *dest.add(idx) = blend_pixel(color, dst);
            }
        }
    }
}

fn paint_button(dest: *mut u32, screen_w: u32, screen_h: u32, rect: Rect, button: &Button) {
    let screen = screen_rect(screen_w, screen_h);
    let clip = rect.intersect(screen);
    if clip.is_empty() {
        return;
    }

    paint_panel(
        dest,
        screen_w,
        screen_h,
        rect,
        clip,
        button.style.bg_rgba,
        button.style.radius,
        None,
    );

    // Minimal glyph: centered short line to read as punctuation
    let line_y = rect.y + rect.h as i32 / 2;
    if line_y < 0 || line_y >= screen_h as i32 {
        return;
    }
    let start_x = rect.x + rect.w as i32 / 2 - 6;
    let glyph_start = start_x.max(0);
    let glyph_end = (start_x + 12).min(screen_w as i32);

    for x in glyph_start..glyph_end {
        let idx = (line_y as u32 * screen_w + x as u32) as usize;
        unsafe {
            let dst = *dest.add(idx);
            *dest.add(idx) = blend_pixel(0x55000000, dst);
        }
    }
}

fn adjust_color(color: u32, delta: i16) -> u32 {
    let a = (color >> 24) & 0xFF;
    let r = (((color >> 16) & 0xFF) as i16 + delta).clamp(0, 255) as u32;
    let g = (((color >> 8) & 0xFF) as i16 + delta).clamp(0, 255) as u32;
    let b = ((color & 0xFF) as i16 + delta).clamp(0, 255) as u32;
    (a << 24) | (r << 16) | (g << 8) | b
}

fn in_round(x: i32, y: i32, w: u32, h: u32, radius: u16) -> bool {
    if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
        return false;
    }
    if radius == 0 {
        return true;
    }
    let r = radius as i32;
    let w_i = w as i32;
    let h_i = h as i32;
    let cx = if x < r {
        r - 1
    } else if x >= w_i - r {
        w_i - r
    } else {
        x
    };
    let cy = if y < r {
        r - 1
    } else if y >= h_i - r {
        h_i - r
    } else {
        y
    };
    let dx = x - cx;
    let dy = y - cy;
    dx * dx + dy * dy <= r * r
}
