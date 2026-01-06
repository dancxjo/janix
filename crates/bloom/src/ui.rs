use abi::ids::ThingId;
use abi::types::AlignedRelBufLarge;
use alloc::string::String;
use alloc::vec::Vec;
use models::*;
use thing_std::graph::{relationships_from, symbol_intern};
use thing_std::symbol_resolve;
use thing_std::SyscallGraphClient;

use crate::scene_cache::BytespaceMappingCache;
use crate::layout::{layout_widgets, TITLE_BAR_HEIGHT};
use crate::painter::Painter;
use crate::scene::Rect;
use crate::text::draw_text_on_painter;
// use crate::assets::map_bytespace; // Removed
use abi::draw_cmd::DrawCmd;

#[cfg(feature = "shadows")]
use crate::shadow::{ShadowMask, ShadowParams};

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
    Canvas(Canvas),
    DrawList(DrawList),
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
                    if let Ok(canvas) = Canvas::read(client, crel.target) {
                        children.push(WidgetKind::Canvas(canvas));
                        continue;
                    }
                    if let Ok(dl) = DrawList::read(client, crel.target) {
                        children.push(WidgetKind::DrawList(dl));
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
pub fn render_window_scenes(painter: &mut dyn Painter, scenes: &[WindowScene], mapping_cache: &mut BytespaceMappingCache) {
    for scene in scenes {
        render_window(painter, scene, mapping_cache);
    }
}

/// Compute the screen bounding rect for clipping.
#[inline]
fn screen_rect(painter: &dyn Painter) -> Rect {
    let (w, h) = painter.screen_size();
    Rect { x: 0, y: 0, w, h }
}

fn render_window(painter: &mut dyn Painter, scene: &WindowScene, mapping_cache: &mut BytespaceMappingCache) {
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
        painter.draw_text(text_x, text_y, &scene.title_text, TITLE_TEXT_COLOR, TITLE_FONT_SIZE);
    }
    let t2b = thing_std::monotonic_now();
    thing_std::log_info(&alloc::format!("BLOOM: title {} ms", (t2b-t2)/1_000_000));

    // Layout phase: compute widget positions (no graph syscalls here)
    let placed = layout_widgets(&scene.layout, &scene.children, rect);

    // Paint phase: draw widgets at computed positions
    for pw in placed {
        match pw.widget {
            WidgetKind::Label(ref label, ref text) => {
                paint_label(painter, pw.rect, label, text);
            }
            WidgetKind::Button(ref button, ref text) => {
                paint_button(painter, pw.rect, button, text);
            }
            WidgetKind::Canvas(ref canvas) => {
                paint_canvas(painter, pw.rect, canvas, mapping_cache);
            }
            WidgetKind::DrawList(ref dl) => {
                paint_drawlist(painter, pw.rect, dl, mapping_cache);
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

fn paint_label(painter: &mut dyn Painter, rect: Rect, label: &Label, text: &str) {
    let screen = screen_rect(painter);
    let clip = rect.intersect(screen);
    if clip.is_empty() {
        return;
    }

    // Draw text centered vertically in the label rect
    let font_size = (label.style.size as f32).max(14.0);
    // Unifont is fixed 16px tall, so center based on that
    let text_y = rect.y + (rect.h as i32 - 16) / 2;
    let text_x = rect.x;
    
    painter.draw_text(text_x, text_y, text, label.style.color_rgba, font_size);
}

fn paint_button(painter: &mut dyn Painter, rect: Rect, button: &Button, text: &str) {
    let screen = screen_rect(painter);
    let clip = rect.intersect(screen);
    if clip.is_empty() {
        return;
    }

    // Draw button panel background
    painter.fill_panel(rect, button.style.radius, button.style.bg_rgba, None);

    // Calculate text color with contrast against background
    // Extract background luminance (simple brightness check)
    let bg_r = ((button.style.bg_rgba >> 16) & 0xFF) as u32;
    let bg_g = ((button.style.bg_rgba >> 8) & 0xFF) as u32;
    let bg_b = (button.style.bg_rgba & 0xFF) as u32;
    let luminance = (bg_r * 299 + bg_g * 587 + bg_b * 114) / 1000;
    let text_color = if luminance > 128 { 0xFF222222 } else { 0xFFFFFFFF };
    
    // Center text in button - measure text width first
    let text_width = crate::text::measure_text_width(text, 14.0);
    let text_x = rect.x + (rect.w as i32 - text_width) / 2;
    // Unifont is 16px tall
    let text_y = rect.y + (rect.h as i32 - 16) / 2;
    
    painter.draw_text(text_x, text_y, text, text_color, 14.0);
}

fn paint_canvas(painter: &mut dyn Painter, rect: Rect, canvas: &Canvas, mapping_cache: &mut BytespaceMappingCache) {
    let screen = screen_rect(painter);
    let clip = rect.intersect(screen);
    if clip.is_empty() {
        return;
    }

    // Delegate to painter (which handles mapping or recording)
    let size = (canvas.width * canvas.height * 4) as usize;
    painter.blit_asset(rect.x, rect.y, canvas.bytespace, canvas.width, canvas.height, canvas.width, size, mapping_cache);
}

fn paint_drawlist(painter: &mut dyn Painter, rect: Rect, dl: &DrawList, mapping_cache: &mut BytespaceMappingCache) {
    let screen = screen_rect(painter);
    let clip = rect.intersect(screen);
    if clip.is_empty() {
        return;
    }

    // Map command buffer via cache
    // Heuristic size for now, ideally DrawList would have a size field or we'd map a fix amount
    let size = 64 * 1024; 
    let (ptr, len) = if let Some(b) = mapping_cache.get_or_map_ro(dl.bytespace, size) {
        (b.as_ptr(), b.len())
    } else {
        return;
    };
    
    // SAFETY: The bytespace memory is stable (OS managed) and will not move or be unmapped 
    // even if we mutate the mapping_cache (BTreeMap) to add new mappings.
    // We need to drop the borrow on mapping_cache so we can pass it mutably to blit_asset.
    let buf = unsafe { core::slice::from_raw_parts(ptr, len) };
    
    // Deserialize commands
    let mut cursor = 0;
    let mut count = 0;
    
    // Set clip to the widget rect for safety
    let old_clip = painter.clip();
    let widget_clip = old_clip.rect.intersect(rect);
    painter.set_clip(crate::painter::Clip::from_rect(widget_clip));
    
    loop {
        if count >= dl.cmd_count {
            break;
        }
        
        // Peek/Deserialize next command using postcard or manual
        // Since we defined DrawCmd as repr(C) we might just cast, but it has a String variant...
        // Wait, DrawCmd::Text has 'len' then bytes. It's not standard Deserialize compatible if we do custom layout.
        // Actually I defined it using Serde. 
        // Let's use postcard for simplicity if possible, OR manual if we want zero-copy text.
        // My ABI definition was:
        // Text { x, y, color, len }
        // The implementation plan implies a custom binary format for Text.
        // standard Deserialize might expect structure.
        
        // Let's assume standard postcard serialization for the enum variants, 
        // BUT for Text specifically, how do we handle the trailing bytes?
        // Postcard handles `&str` by copying.
        
        // Use take_from_bytes to get the command and the remaining slice
        if let Ok((cmd, remaining)) = postcard::take_from_bytes(&buf[cursor..]) {
            // Calculate how many bytes were consumed by the command itself
            let used = buf[cursor..].len() - remaining.len();
            cursor += used;
            count += 1;
            
            match cmd {
                DrawCmd::FillRect { x, y, w, h, color } => {
                    painter.fill_rect(Rect { x: rect.x + x as i32, y: rect.y + y as i32, w: w as u32, h: h as u32 }, color);
                }
                DrawCmd::FillRoundedRect { x, y, w, h, radius, color } => {
                     painter.fill_rounded_rect(Rect { x: rect.x + x as i32, y: rect.y + y as i32, w: w as u32, h: h as u32 }, radius, color);
                }
                DrawCmd::StrokeRoundedRect { x, y, w, h, radius, thickness, color } => {
                     painter.stroke_rounded_rect(Rect { x: rect.x + x as i32, y: rect.y + y as i32, w: w as u32, h: h as u32 }, radius, thickness, color);
                }
                DrawCmd::Clear { color } => {
                    painter.fill_rect(rect, color);
                }
                DrawCmd::Text { x, y, color, len } => {
                    // Following bytes are text
                    let text_bytes = &buf[cursor..cursor + len as usize];
                    cursor += len as usize;
                    if let Ok(text) = core::str::from_utf8(text_bytes) {
                        painter.draw_text(rect.x + x as i32, rect.y + y as i32, text, color, 14.0);
                    }
                }
                DrawCmd::Shadow { .. } => {
                    // Start with no-op placeholder
                }
                DrawCmd::Blit { x, y, w, h, asset } => {
                     // Assume standard asset format (stride=width, 32bpp)
                     let stride = w as u32;
                     let len = (w as usize) * (h as usize) * 4;
                     painter.blit_asset(rect.x + x as i32, rect.y + y as i32, abi::ids::ThingId::from_parts(0, asset), w as u32, h as u32, stride, len, mapping_cache);
                }
                DrawCmd::End => break,
            }
        } else {
            break;
        }
    }
    
    // Restore clip
    painter.set_clip(old_clip);
}
