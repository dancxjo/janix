use abi::ids::ThingId;
use abi::types::AlignedRelBufLarge;
use alloc::string::String;
use alloc::vec::Vec;
use models::*;
use thing_std::graph::{relationships_from, symbol_intern};
use thing_std::symbol_resolve;
use thing_std::SyscallGraphClient;

use crate::layout::{layout_widgets, PlacedWidget, TITLE_BAR_HEIGHT};
use crate::painter::Painter;
use crate::scene::Rect;
use crate::shadow::{ShadowMask, ShadowParams};
use crate::text::draw_text_on_painter;

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

/// Render all window scenes using the Painter API.
pub fn render_window_scenes(painter: &mut dyn Painter, scenes: &[WindowScene]) {
    for scene in scenes {
        render_window(painter, scene);
    }
}

/// Compute the screen bounding rect for clipping.
#[inline]
fn screen_rect(painter: &dyn Painter) -> Rect {
    let (w, h) = painter.screen_size();
    Rect { x: 0, y: 0, w, h }
}

fn render_window(painter: &mut dyn Painter, scene: &WindowScene) {
    let win = &scene.window;
    let rect = Rect {
        x: win.x,
        y: win.y,
        w: win.width,
        h: win.height,
    };

    // Clip to screen
    let screen = screen_rect(painter);
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

    // Set clip for this window
    painter.set_clip(crate::painter::Clip::from_rect(clip));

    let t0 = thing_std::monotonic_now();
    #[cfg(feature = "shadows")]
    if win.style.shadow != 0 {
        painter.draw_shadow_mask(
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
    let t1 = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: shadow {} ms", (t1-t0)/1_000_000));

    // Paint panel background with gradient and title stripe
    painter.fill_panel(rect, win.style.radius, win.style.bg_rgba, Some(TITLE_BAR_HEIGHT));
    let t2 = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: paint_panel {} ms", (t2-t1)/1_000_000));

    // Draw window title in the title bar
    if !scene.title_text.is_empty() {
        let text_x = rect.x + TITLE_PADDING_X;
        let text_y = rect.y + (TITLE_BAR_HEIGHT - TITLE_FONT_SIZE as i32) / 2;
        draw_text_on_painter(painter, text_x, text_y, &scene.title_text, TITLE_TEXT_COLOR, TITLE_FONT_SIZE);
    }
    let t2b = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: title {} ms", (t2b-t2)/1_000_000));

    // Layout phase: compute widget positions (no graph syscalls here)
    let placed = layout_widgets(&scene.layout, &scene.children, rect);

    // Paint phase: draw widgets at computed positions
    for pw in placed {
        match pw.widget {
            WidgetKind::Label(ref label, _) => {
                paint_label(painter, pw.rect, label);
            }
            WidgetKind::Button(ref button, _) => {
                paint_button(painter, pw.rect, button);
            }
        }
    }
    let t3 = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: widgets {} ms", (t3-t2b)/1_000_000));

    // Draw border
    painter.stroke_rounded_rect(rect, win.style.radius, 1, 0xFF404040);
    let t4 = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: border {} ms", (t4-t3)/1_000_000));

    thing_std::log_info(&alloc::format!("BLOOM: render_window TOTAL {} ms", (t4-t0)/1_000_000));
}

fn paint_label(painter: &mut dyn Painter, rect: Rect, label: &Label) {
    let screen = screen_rect(painter);
    let clip = rect.intersect(screen);
    if clip.is_empty() {
        return;
    }

    // Simple colored lines as placeholder for label
    let color = label.style.color_rgba;
    let base_y = rect.y + (rect.h as i32 / 2);

    for offset in 0..2 {
        let y = base_y + offset;
        if y < clip.y || y >= clip.y + clip.h as i32 {
            continue;
        }
        for cx in clip.x..(clip.x + clip.w as i32) {
            // Draw a colored line - we need a blend_line method but for now use fill_rect
        }
    }
    // Draw as a thin colored bar for now
    painter.fill_rect(
        Rect { x: clip.x, y: base_y, w: clip.w, h: 2 },
        color,
    );
}

fn paint_button(painter: &mut dyn Painter, rect: Rect, button: &Button) {
    let screen = screen_rect(painter);
    let clip = rect.intersect(screen);
    if clip.is_empty() {
        return;
    }

    // Draw button panel
    painter.fill_panel(rect, button.style.radius, button.style.bg_rgba, None);

    // Draw a centered glyph line
    let line_y = rect.y + rect.h as i32 / 2;
    let (screen_w, screen_h) = painter.screen_size();
    if line_y < 0 || line_y >= screen_h as i32 {
        return;
    }
    let start_x = rect.x + rect.w as i32 / 2 - 6;
    let glyph_rect = Rect {
        x: start_x.max(0),
        y: line_y,
        w: 12.min(screen_w),
        h: 1,
    };
    painter.fill_rect(glyph_rect, 0x55000000);
}
