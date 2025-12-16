#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use thing_os::prelude::*;
use runtime::UserlandSys;
use abi::{KernelRequest, KernelResponse, ThingId, PropValue};

mod window;
mod text;
mod graph_utils;
mod icons;
mod sky;

use window::{create_geographer_window, WindowContext};
use text::draw_text_simple;
use graph_utils::{Scene, GeoNode, GeoEdge, LinkThing};
use icons::{Icon, IconAtlas};
use sky::Sky;

#[unsafe(no_mangle)]
#[cfg(target_os = "none")]
fn main() {
    let mut sys = UserlandSys::new();
    geographer_main(&mut sys);
}

fn geographer_main(sys: &mut UserlandSys) -> ! {
    thing_os::println(sys, "geographer starting...");

    // 4. Create Window & Framebuffer
    let mut win_ctx = if let Some(ctx) = create_geographer_window(sys) {
        ctx
    } else {
        thing_os::println(sys, "geographer: failed to create window, exiting");
        loop { sys.sleep_for_ns(1_000_000_000); }
    };

    thing_os::println(sys, "geographer: window created");
    
    // Initialize Sky
    let sky = Sky::new();
    
    let mut last_graph_update = 0;
    let mut scene = Scene::new();
    
    let mut known_kinds: Vec<&'static str> = Vec::new();

    // Initialize Icon Atlas
    let atlas = IconAtlas::new(sys); 

    let mut graph_status = "Initializing...";

    loop {
        let now = sys.time_monotonic_ns();

        // 1. Update Graph Scene (every 2s)
        if now - last_graph_update > 2_000_000_000 {
            let mut new_scene = discover_scene(sys, &mut known_kinds);
            
            if new_scene.nodes.is_empty() && now > 10_000_000_000 { 
                 graph_status = "Graph Empty";
            } else {
                 graph_status = "Live";
            }

            // Layout
            layout_scene(&mut new_scene, &known_kinds, win_ctx.width as f32, win_ctx.height as f32);
            
            scene = new_scene;
            
            if last_graph_update == 0 {
                thing_os::println(sys, "geographer first graph update complete");
            }
            last_graph_update = now;
        }

        // 2. Render Phase
        sky.draw(
            unsafe { core::slice::from_raw_parts_mut(win_ctx.buffer_ptr, (win_ctx.height * win_ctx.stride) as usize) },
            win_ctx.stride,
            win_ctx.width, 
            win_ctx.height, 
            now
        );
        
        render_scene(&mut win_ctx, &scene, &atlas);
        render_hud(&mut win_ctx, &scene, graph_status);
        
        sys.sleep_for_ns(16_000_000);
    }
}

fn discover_scene(sys: &mut UserlandSys, known_kinds: &mut Vec<&'static str>) -> Scene {
    let mut scene = Scene::new();

    // A. Discover Kinds
    let mut cursor = ThingId(u64::MAX);
    loop {
         match sys.syscall(KernelRequest::ThingList {
            kind: "Kind",
            start_after: cursor,
         }) {
             KernelResponse::ThingListEntry { id: Some(next_id) } => {
                 if let Some((_, props)) = thing_os::get_thing(sys, next_id) {
                     for (k, v) in props {
                         if k == "name" {
                             if let PropValue::Str(s) = v {
                                 if !known_kinds.contains(&s.as_str()) {
                                     let leaked = Box::leak(s.into_boxed_str());
                                     known_kinds.push(leaked);
                                 }
                             }
                         }
                     }
                 }
                 cursor = next_id;
             }
             _ => break,
         }
    }
    // Ensure 'Kind' is in known_kinds so we see them too
    if !known_kinds.contains(&"Kind") { known_kinds.push("Kind"); }


    // B. For each Kind, list Things
    for &kind_static in known_kinds.iter() {
         if kind_static.is_empty() || kind_static == "Link" { continue; }

         let mut t_cursor = ThingId(u64::MAX);
         loop {
             match sys.syscall(KernelRequest::ThingList {
                kind: kind_static,
                start_after: t_cursor,
             }) {
                 KernelResponse::ThingListEntry { id: Some(tid) } => {
                     let mut name_val = String::new();
                     if let Some((_, props)) = thing_os::get_thing(sys, tid) {
                         for (k, v) in props {
                             if k == "name" {
                                 if let PropValue::Str(s) = v {
                                     name_val = s;
                                 }
                             }
                         }
                     }
                     
                     scene.nodes.insert(tid, GeoNode {
                         id: tid,
                         kind: kind_static.to_string(),
                         props: alloc::vec![(String::from("name"), PropValue::Str(name_val))],
                         x: 0.0, y: 0.0,
                     });
                     
                     t_cursor = tid;
                 }
                 _ => break,
             }
         }
    }

    // C. Discover Links
    let links: Vec<LinkThing> = thing_os::list_things_by_kind(sys);
    for l in links {
        scene.edges.push(GeoEdge {
            id: l.id,
            src: l.src,
            dst: l.dst,
            pred: l.pred,
        });
    }

    scene
}

fn layout_scene(scene: &mut Scene, known_kinds: &[&'static str], width: f32, height: f32) {
    let mut sorted_kinds: Vec<&str> = known_kinds.iter().copied().collect();
    sorted_kinds.sort();
    sorted_kinds.retain(|k| *k != "Link" && *k != "");

    if sorted_kinds.is_empty() { return; }

    let col_width = width / sorted_kinds.len() as f32;
    
    for kv in scene.nodes.values_mut() {
        let col_idx = sorted_kinds.iter().position(|&k| k == kv.kind).unwrap_or(0);
        let col_x_start = col_idx as f32 * col_width;
        
        let mut h = 0xcbf29ce484222325;
        h = (h ^ kv.id.0).wrapping_mul(0x100000001b3);
        
        let slots_x = 4;
        let slots_y = 10;
        
        let slot_idx = h % (slots_x * slots_y); // u64 arithmetic is cleaner
        let sx = (slot_idx % slots_x) as f32;
        let sy = (slot_idx / slots_x) as f32; // Integral division, effectively floor
        
        let cell_w = col_width / slots_x as f32;
        let cell_h = height / slots_y as f32;
        
        kv.x = col_x_start + sx * cell_w + cell_w * 0.5;
        kv.y = sy * cell_h + cell_h * 0.5 + 40.0;
    }
}

fn render_scene(ctx: &mut WindowContext, scene: &Scene, atlas: &IconAtlas) {
    let buffer = unsafe { core::slice::from_raw_parts_mut(ctx.buffer_ptr, (ctx.height * ctx.stride) as usize) };
    let width = ctx.width;
    let height = ctx.height;
    let stride = ctx.stride;
    
    for edge in &scene.edges {
        if let (Some(src), Some(dst)) = (scene.nodes.get(&edge.src), scene.nodes.get(&edge.dst)) {
           draw_line(buffer, stride, width, height, src.x as i32, src.y as i32, dst.x as i32, dst.y as i32, 0xFF606060);
        }
    }

    for node in scene.nodes.values() {
        let x = node.x as i32;
        let y = node.y as i32;
        
        let icon = atlas.get(&node.kind);
        let ix = x - (icon.width as i32 / 2);
        let iy = y - (icon.height as i32 / 2);
        
        draw_icon(buffer, stride, width, height, ix, iy, icon);
    }
}

fn render_hud(ctx: &mut WindowContext, scene: &Scene, status: &str) {
    let buffer = unsafe { core::slice::from_raw_parts_mut(ctx.buffer_ptr, (ctx.height * ctx.stride) as usize) };
    let stride = ctx.stride;
    let width = ctx.width;
    let height = ctx.height;
    
    let c = 0xFFFFFFFF;
    draw_text_simple(buffer, stride, width, height, 10, 10, "Geographer System Map", c);
    
    let stats = format!("Things: {}  Kinds: {}  Links: {}  Status: {}", 
        scene.nodes.len(), 
        0, 
        scene.edges.len(),
        status
    );
    draw_text_simple(buffer, stride, width, height, 10, 30, &stats, c);
}

fn draw_icon(buffer: &mut [u8], stride: u32, width: u32, height: u32, x: i32, y: i32, icon: &Icon) {
    let b_w = width as i32;
    let b_h = height as i32;
    
    for iy in 0..icon.height {
        let dy = y + iy as i32;
        if dy < 0 || dy >= b_h { continue; }
        
        for ix in 0..icon.width {
            let dx = x + ix as i32;
            if dx < 0 || dx >= b_w { continue; }
            
            let src_idx = (iy * icon.width + ix) as usize;
            let src_pixel = icon.pixels[src_idx];
            
            let alpha = (src_pixel >> 24) & 0xFF;
            if alpha == 0 { continue; }
            
            let dest_offset = (dy * stride as i32 + dx * 4) as usize;
            
            unsafe {
                let d_ptr = buffer.as_mut_ptr().add(dest_offset) as *mut u32;
                if alpha == 255 {
                    *d_ptr = src_pixel;
                } else {
                    let bg = *d_ptr;
                    let s_r = (src_pixel >> 16) & 0xFF;
                    let s_g = (src_pixel >> 8) & 0xFF;
                    let s_b = src_pixel & 0xFF;
                    
                    let d_r = (bg >> 16) & 0xFF;
                    let d_g = (bg >> 8) & 0xFF;
                    let d_b = bg & 0xFF;
                    
                    let inv_a = 255 - alpha;
                    let out_r = (s_r * alpha + d_r * inv_a) / 255;
                    let out_g = (s_g * alpha + d_g * inv_a) / 255;
                    let out_b = (s_b * alpha + d_b * inv_a) / 255;
                    
                    *d_ptr = (255 << 24) | (out_r << 16) | (out_g << 8) | out_b;
                }
            }
        }
    }
}

fn draw_line(buffer: &mut [u8], stride: u32, width: u32, height: u32, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
    let mut x = x1;
    let mut y = y1;
    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    
    loop {
        if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
             let offset = (y * stride as i32 + x * 4) as usize;
             unsafe {
                 let ptr = buffer.as_mut_ptr().add(offset) as *mut u32;
                 *ptr = color;
             }
        }
        if x == x2 && y == y2 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {}
