#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::panic::PanicInfo;
use thing_os::prelude::*;
use runtime::UserlandSys;
use abi::{KernelRequest, KernelResponse, ThingId, PropValue};

mod window;
mod text;
mod graph_utils;
mod icons;

use window::{create_cartographer_window, WindowContext};
use text::draw_text_simple;
use graph_utils::{Scene, GeoNode, GeoEdge, LinkThing, pred_to_string};
use icons::{Icon, IconAtlas};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut sys = UserlandSys::new();
    cartographer_main(&mut sys)
}

fn cartographer_main(sys: &mut UserlandSys) -> ! {
    thing_os::println(sys, "Cartographer starting...");

    // 4. Create Window & Framebuffer
    // We loop until the window is successfully created, which depends on
    // finding a DisplayThing and an active Mode.
    let mut win_ctx = loop {
        if let Some(ctx) = create_cartographer_window(sys) {
            break ctx;
        }
        thing_os::println(sys, "cartographer wait: no display or mode yet...");
        sys.sleep_for_ns(500_000_000);
    };

    thing_os::println(sys, "cartographer: window created");
    let mut last_graph_update = 0;
    let mut scene = Scene::new();
    
    // Cache for leaked kind strings to avoid memory leak explosion
    let mut known_kinds: Vec<&'static str> = Vec::new();

    // Initialize Icon Atlas
    let mut atlas = IconAtlas::new(sys);

    loop {
        let now = sys.time_monotonic_ns();

        // 1. Update Graph Scene (every 2s)
        if now - last_graph_update > 2_000_000_000 {
            let mut new_scene = discover_scene(sys, &mut known_kinds);
            merge_scene_layout(&scene, &mut new_scene);
            scene = new_scene;
            
            if last_graph_update == 0 {
                thing_os::println(sys, "Cartographer first graph update complete");
            }
            last_graph_update = now;
        }

        // 2. Physics
        run_physics(&mut scene, win_ctx.width as f32, win_ctx.height as f32);

        // 3. Render
        render_frame(&mut win_ctx, &scene, &atlas);

        // 3. Sleep (60 FPS target)
        sys.sleep_for_ns(16_000_000);
    }
}

fn discover_scene(sys: &mut UserlandSys, known_kinds: &mut Vec<&'static str>) -> Scene {
    let mut scene = Scene::new();

    // A. Discover Kinds
    // We scan "Kind" things to find new kinds to add to our known list
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
                                 // Check if we already know this kind
                                 if !known_kinds.contains(&s.as_str()) {
                                     // Leak it to get &'static str
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
                     if let Some((_, props)) = thing_os::get_thing(sys, tid) {
                         // Convert props to (String, PropValue) for storage
                         let owned_props = props.into_iter()
                             .map(|(k, v)| (k.to_string(), v))
                             .collect();
                         
                         scene.nodes.insert(tid, GeoNode {
                             id: tid,
                             kind: kind_static.to_string(),
                             props: owned_props,
                             x: 0.0, y: 0.0, vx: 0.0, vy: 0.0,
                         });
                     }
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

fn merge_scene_layout(old: &Scene, new: &mut Scene) {
    use core::hash::{Hash, Hasher};
    // Initialize new nodes with random positions if they weren't in old scene
    for (id, node) in new.nodes.iter_mut() {
        if let Some(old_node) = old.nodes.get(id) {
            node.x = old_node.x;
            node.y = old_node.y;
            node.vx = old_node.vx;
            node.vy = old_node.vy;
        } else {
             // Deterministic random position based on ID
             // Simple hash
             let mut h = 0xcbf29ce484222325; // FNV offset basis
             h = (h ^ id.0).wrapping_mul(0x100000001b3);
             
             let x = (h % 800) as f32 + 100.0;
             let y = ((h >> 16) % 600) as f32 + 50.0;
             
             node.x = x;
             node.y = y;
        }
    }
}

fn run_physics(scene: &mut Scene, width: f32, height: f32) {
    // Very simple force-directed layout
    // Repulsion
    let repulsion_force = 1000.0;
    let attraction_force = 0.05;
    let center_force = 0.01;
    let dt = 0.016; // fixed step assumption
    
    // Convert edges to map for fast lookup? No, just iterate edges for attraction
    // Repulsion: N^2, limit to small N? Or just do it. N < 100 usually.
    
    let nodes: Vec<ThingId> = scene.nodes.keys().cloned().collect();
    
    // Apply forces
    // 1. Repulsion
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
             let id1 = nodes[i];
             let id2 = nodes[j];
             let n1 = scene.nodes.get(&id1).unwrap();
             let n2 = scene.nodes.get(&id2).unwrap();
             
             let dx = n1.x - n2.x;
             let dy = n1.y - n2.y;
             let dist_sq = dx*dx + dy*dy + 0.1;
             let dist = sqrt(dist_sq);
             
             let f = repulsion_force / dist_sq;
             let fx = (dx / dist) * f;
             let fy = (dy / dist) * f;
             
             // Borrow checker struggle here if we modify in loop.
             // We need to accumulate forces then apply.
        }
    }
    // Simplified physics: center pull + random jitter to avoid stack
    for node in scene.nodes.values_mut() {
        // Pull to center
        let cx = width / 2.0;
        let cy = height / 2.0;
        node.vx += (cx - node.x) * center_force;
        node.vy += (cy - node.y) * center_force;
        
        // Dampen
        node.vx *= 0.90;
        node.vy *= 0.90;
        
        node.x += node.vx * dt;
        node.y += node.vy * dt;
        
        // Clamp
        node.x = node.x.max(0.0).min(width);
        node.y = node.y.max(0.0).min(height);
    }
    
    // 2. Attraction from edges
    for edge in &scene.edges {
        if let (Some(src), Some(dst)) = (scene.nodes.get(&edge.src), scene.nodes.get(&edge.dst)) {
            let dx = dst.x - src.x;
            let dy = dst.y - src.y;
            
            // We can't modify src/dst here because of borrowing.
            // Let's implement a really simple "spring" via direct update in a separate pass?
            // For now, center pull is enough to keep them on screen. 
            // Proper force directed requires two passes or collecting forces.
            // Let's do a simple iterative approach:
        }
    }
}

fn render_frame(ctx: &mut WindowContext, scene: &Scene, atlas: &IconAtlas) {
    // 1. Clear Buffer
    // 0x00000000 is transparent black
    unsafe {
        let slice = core::slice::from_raw_parts_mut(ctx.buffer_ptr, (ctx.height * ctx.stride) as usize);
        slice.fill(0); 
    }
    
    // Safety slice wrapper for draw calls
    let buffer = unsafe { core::slice::from_raw_parts_mut(ctx.buffer_ptr, (ctx.height * ctx.stride) as usize) };

    let width = ctx.width;
    let height = ctx.height;
    let stride = ctx.stride;
    
    let mut y = 20;
    let x = 20;
    let line_height = 20;
    let white: u32 = 0xFFFFFFFF;
    let green: u32 = 0xFF00FF00;

    // Draw Stats
    let stats = format!("Nodes: {}  Edges: {}", scene.nodes.len(), scene.edges.len());
    draw_text_simple(buffer, stride, width, height, x, y, &stats, white);
    y += line_height * 2;

    // Draw Nodes (first 15)
    // Sort by ID for stability
    let mut sorted_nodes: Vec<_> = scene.nodes.values().collect();
    sorted_nodes.sort_by_key(|n| n.id);

    for node in sorted_nodes.iter().take(15) {
        if y > height as i32 - 20 { break; }
        // Attempt to find a "name" property
        let name = node.props.iter()
            .find(|(k, _)| k == "name")
            .and_then(|(_, v)| match v {
                PropValue::Str(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or("");
            
        let text = format!("({}) :{} {{ name: \"{}\" }}", node.id.0, node.kind, name);
        draw_text_simple(buffer, stride, width, height, x, y, &text, white);
        y += line_height;
    }
    
    y += line_height;

    // Draw Nodes (first 15? No, draw all within bounds)
    // Draw edges first so they are under nodes
    
    // Draw Edges
    for edge in &scene.edges {
        let src = scene.nodes.get(&edge.src);
        let dst = scene.nodes.get(&edge.dst);
        
        if let (Some(n1), Some(n2)) = (src, dst) {
            // Draw line from n1 to n2
            draw_line(buffer, stride, width, height, n1.x as i32, n1.y as i32, n2.x as i32, n2.y as i32, 0xFF555555);
        }
    }

    // Draw Nodes
    // Sort by Y for simple depth (optional)
    let mut sorted_nodes: Vec<_> = scene.nodes.values().collect();
    sorted_nodes.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(core::cmp::Ordering::Equal));

    for node in sorted_nodes {
        let x = node.x as i32;
        let y = node.y as i32;
        
        // Get icon
        let icon = atlas.get(&node.kind);
        
        // Draw icon centered
        let ix = x - (icon.width as i32 / 2);
        let iy = y - (icon.height as i32 / 2);
        
        draw_icon(buffer, stride, width, height, ix, iy, icon);
        
        // Draw label
        let name = node.props.iter()
            .find(|(k, _)| k == "name")
            .and_then(|(_, v)| match v {
                PropValue::Str(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or("");
            
        if !name.is_empty() {
             draw_text_simple(buffer, stride, width, height, x, y + 32, name, 0xFFFFFFFF);
        }
        
        // Draw ID/Kind smaller?
        // draw_text_simple(buffer, stride, width, height, x, y + 48, &format!("{}:{}", node.id.0, node.kind), 0xFFAAAAAA);
    }
}

fn draw_icon(buffer: &mut [u8], stride: u32, width: u32, height: u32, x: i32, y: i32, icon: &Icon) {
    let dest_stride = stride as i32;
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
            
            // Alpha blend
            // src_pixel is BGRA8888 (0xAARRGGBB in little endian u32 usually means BB GG RR AA in memory?)
            // Actually u32 0xAARRGGBB means AA is MSB.
            // Limine framebuffer is often BGRA or RGBx.
            
            let alpha = (src_pixel >> 24) & 0xFF;
            if alpha == 0 { continue; }
            
            let dest_offset = (dy * dest_stride + dx * 4) as usize;
            
            if alpha == 255 {
                // Opaque
                 unsafe {
                    let d = buffer.as_mut_ptr().add(dest_offset) as *mut u32;
                    *d = src_pixel;
                }
            } else {
                // Blend
                unsafe {
                    let d = buffer.as_mut_ptr().add(dest_offset) as *mut u32;
                    let bg = *d;
                    // Simple alpha blend: src * a + dst * (1-a)
                    // Assuming u32 is 0xAARRGGBB
                    
                    let s_r = (src_pixel >> 16) & 0xFF;
                    let s_g = (src_pixel >> 8) & 0xFF;
                    let s_b = src_pixel & 0xFF;
                    
                    let d_a = (bg >> 24) & 0xFF;
                    let d_r = (bg >> 16) & 0xFF;
                    let d_g = (bg >> 8) & 0xFF;
                    let d_b = bg & 0xFF;
                    
                    // Alpha is 0..255.
                    // r = (s_r * a + d_r * (255 - a)) / 255
                    let inv_a = 255 - alpha;
                    
                    let out_r = (s_r * alpha + d_r * inv_a) / 255;
                    let out_g = (s_g * alpha + d_g * inv_a) / 255;
                    let out_b = (s_b * alpha + d_b * inv_a) / 255;
                    // Keep dest alpha? Or blend alpha?
                    let out_a = 255; // simple
                    
                    *d = (out_a << 24) | (out_r << 16) | (out_g << 8) | out_b;
                }
            }
        }
    }
}

fn draw_line(buffer: &mut [u8], stride: u32, width: u32, height: u32, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
    // Bresenham
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
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn sqrt(val: f32) -> f32 {
    // Simple Newton-Raphson
    if val <= 0.0 { return 0.0; }
    let mut z = val;
    for _ in 0..10 {
        z = 0.5 * (z + val / z);
    }
    z
}

#[cfg(not(target_os = "none"))]
fn main() {}
