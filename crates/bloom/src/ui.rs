use abi::ids::ThingId;
use abi::types::AlignedRelBufLarge;
use alloc::string::String;
use alloc::vec::Vec;
use models::*;
use thing_std::graph::{relationships_from, symbol_intern};
use thing_std::symbol_resolve;
use thing_std::SyscallGraphClient;

use crate::pixels::blend_pixel;
use crate::scene::Rect;
use crate::shadow::{draw_shadow_from_mask, ShadowMask, ShadowParams};

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

fn render_window(dest: *mut u32, screen_w: u32, screen_h: u32, scene: &WindowScene) {
    let win = &scene.window;
    let rect = Rect {
        x: win.x,
        y: win.y,
        w: win.width,
        h: win.height,
    };
    if rect.w == 0 || rect.h == 0 {
        return;
    }

    // Shared shadow with cursor
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
                    color: 0x44000000, // soft, low-opacity
                },
            );
        }
    }

    paint_panel(
        dest,
        screen_w,
        screen_h,
        rect,
        win.style.bg_rgba,
        win.style.radius,
        Some(22),
    );

    // Simple Column/Row layout
    let mut widgets = Vec::new();
    layout_widgets(&scene.layout, &scene.children, rect, &mut widgets);

    for (widget_rect, widget) in widgets {
        match widget {
            WidgetKind::Label(label, _) => {
                paint_label(dest, screen_w, screen_h, widget_rect, &label);
            }
            WidgetKind::Button(button, _) => {
                paint_button(dest, screen_w, screen_h, widget_rect, &button);
            }
        }
    }
}

fn layout_widgets(
    layout: &Layout,
    children: &[WidgetKind],
    window_rect: Rect,
    out: &mut Vec<(Rect, WidgetKind)>,
) {
    let padding = layout.padding as i32;
    let gap = layout.gap as i32;
    let usable_w = window_rect.w.saturating_sub((padding as u32) * 2);
    let start_x = window_rect.x + padding;
    let mut cursor_y = window_rect.y + padding;

    for child in children {
        let (h, w_guess) = match child {
            WidgetKind::Label(label, text) => {
                let h = label.style.size as i32 + 6;
                let w = text_width_guess(text, label.style.size).min(usable_w as i32);
                (h.max(14), w.max(24))
            }
            WidgetKind::Button(_button, text) => {
                let h = 32;
                let w = text_width_guess(text, 16).max(64);
                (h, w.min(usable_w as i32))
            }
        };

        let rect = match layout.kind {
            LayoutKind::Column => Rect {
                x: start_x,
                y: cursor_y,
                w: usable_w,
                h: h as u32,
            },
            LayoutKind::Row => Rect {
                x: start_x,
                y: window_rect.y + padding,
                w: w_guess as u32,
                h: h as u32,
            },
        };
        cursor_y += h + gap;
        out.push((rect, child.clone()));
    }
}

fn paint_panel(
    dest: *mut u32,
    screen_w: u32,
    screen_h: u32,
    rect: Rect,
    color: u32,
    radius: u16,
    stripe_height: Option<i32>,
) {
    let base = color;
    let light = adjust_color(base, 6);
    let dark = adjust_color(base, -8);

    // Optional feathered gradient: +2% top, -2% bottom
    let height = rect.h.max(1) as i32;
    for dy in 0..rect.h as i32 {
        let y = rect.y + dy;
        if y < 0 || y >= screen_h as i32 {
            continue;
        }
        let t = dy as f32 / height as f32;
        let grad = ((1.0 - 2.0 * (t - 0.5).abs()) * 4.0) as i16; // +/-4 brightness ticks
        for dx in 0..rect.w as i32 {
            let x = rect.x + dx;
            if x < 0 || x >= screen_w as i32 {
                continue;
            }
            if !in_round(dx, dy, rect.w, rect.h, radius) {
                continue;
            }
            let final_color = adjust_color(base, grad);
            let idx = (y as u32 * screen_w + x as u32) as usize;
            unsafe {
                *dest.add(idx) = final_color;
            }
        }
    }

    // 1px bevel: top/left light, bottom/right dark
    for dx in 0..rect.w as i32 {
        let x = rect.x + dx;
        if x < 0 || x >= screen_w as i32 {
            continue;
        }
        let top_y = rect.y;
        let bottom_y = rect.y + rect.h as i32 - 1;
        if top_y >= 0 && top_y < screen_h as i32 && in_round(dx, 0, rect.w, rect.h, radius) {
            let idx = (top_y as u32 * screen_w + x as u32) as usize;
            unsafe {
                *dest.add(idx) = light;
            }
        }
        if bottom_y >= 0
            && bottom_y < screen_h as i32
            && in_round(dx, rect.h as i32 - 1, rect.w, rect.h, radius)
        {
            let idx = (bottom_y as u32 * screen_w + x as u32) as usize;
            unsafe {
                *dest.add(idx) = dark;
            }
        }
    }
    for dy in 0..rect.h as i32 {
        let y = rect.y + dy;
        if y < 0 || y >= screen_h as i32 {
            continue;
        }
        let left_x = rect.x;
        let right_x = rect.x + rect.w as i32 - 1;
        if left_x >= 0 && left_x < screen_w as i32 && in_round(0, dy, rect.w, rect.h, radius) {
            let idx = (y as u32 * screen_w + left_x as u32) as usize;
            unsafe {
                *dest.add(idx) = light;
            }
        }
        if right_x >= 0
            && right_x < screen_w as i32
            && in_round(rect.w as i32 - 1, dy, rect.w, rect.h, radius)
        {
            let idx = (y as u32 * screen_w + right_x as u32) as usize;
            unsafe {
                *dest.add(idx) = dark;
            }
        }
    }

    if let Some(h) = stripe_height {
        let title_h = h.min(rect.h as i32).max(0);
        let stripe_a = adjust_color(base, -3);
        for dy in 0..title_h {
            let y = rect.y + dy;
            if y < 0 || y >= screen_h as i32 {
                continue;
            }
            if dy % 2 != 0 {
                continue;
            }
            for dx in 0..rect.w as i32 {
                if !in_round(dx, dy, rect.w, rect.h, radius) {
                    continue;
                }
                let x = rect.x + dx;
                if x < 0 || x >= screen_w as i32 {
                    continue;
                }
                let idx = (y as u32 * screen_w + x as u32) as usize;
                unsafe {
                    *dest.add(idx) = stripe_a;
                }
            }
        }
    }
}

fn paint_label(dest: *mut u32, screen_w: u32, screen_h: u32, rect: Rect, label: &Label) {
    let color = label.style.color_rgba;
    let base_y = rect.y + (rect.h as i32 / 2);
    for offset in 0..2 {
        let y = base_y + offset;
        if y < 0 || y >= screen_h as i32 {
            continue;
        }
        for dx in 0..rect.w as i32 {
            let x = rect.x + dx;
            if x < 0 || x >= screen_w as i32 {
                continue;
            }
            let idx = (y as u32 * screen_w + x as u32) as usize;
            unsafe {
                let dst = *dest.add(idx);
                *dest.add(idx) = blend_pixel(color, dst);
            }
        }
    }
}

fn paint_button(dest: *mut u32, screen_w: u32, screen_h: u32, rect: Rect, button: &Button) {
    paint_panel(
        dest,
        screen_w,
        screen_h,
        rect,
        button.style.bg_rgba,
        button.style.radius,
        None,
    );
    // Minimal glyph: centered short line to read as punctuation
    let line_y = rect.y + rect.h as i32 / 2;
    let start_x = rect.x + rect.w as i32 / 2 - 6;
    for dx in 0..12 {
        let x = start_x + dx;
        if x < 0 || x >= screen_w as i32 || line_y < 0 || line_y >= screen_h as i32 {
            continue;
        }
        let idx = (line_y as u32 * screen_w + x as u32) as usize;
        unsafe {
            let dst = *dest.add(idx);
            *dest.add(idx) = blend_pixel(0x55000000, dst);
        }
    }
}

fn text_width_guess(text: &str, size: u16) -> i32 {
    let avg = (size as i32 / 2).max(6);
    (text.len() as i32 * avg).min(320)
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
