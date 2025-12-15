#![no_std]
#![no_main]

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

use window::{create_cartographer_window, WindowContext};
use text::draw_text_simple;
use graph_utils::{Scene, GeoNode, GeoEdge, LinkThing, pred_to_string};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut sys = UserlandSys::new();
    cartographer_main(&mut sys)
}

fn cartographer_main(sys: &mut UserlandSys) -> ! {
    thing_os::println(sys, "Cartographer starting...");

    // Initialize Window
    let mut win_ctx = match create_cartographer_window(sys) {
        Some(ctx) => ctx,
        None => {
            thing_os::println(sys, "Cartographer failed to create window/buffer");
            sys.exit_thread();
        }
    };

    let mut last_graph_update = 0;
    let mut scene = Scene::new();
    
    // Cache for leaked kind strings to avoid memory leak explosion
    let mut known_kinds: Vec<&'static str> = Vec::new();

    loop {
        let now = sys.time_monotonic_ns();

        // 1. Update Graph Scene (every 2s)
        if now - last_graph_update > 2_000_000_000 {
            scene = discover_scene(sys, &mut known_kinds);
            if last_graph_update == 0 {
                thing_os::println(sys, "Cartographer first graph update complete");
            }
            last_graph_update = now;
        }

        // 2. Render
        render_frame(&mut win_ctx, &scene);

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

fn render_frame(ctx: &mut WindowContext, scene: &Scene) {
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
    let white = 0xFFFFFFFF;
    let green = 0xFF00FF00;

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

    // Draw Edges (first 15)
    let mut sorted_edges: Vec<_> = scene.edges.iter().collect();
    sorted_edges.sort_by_key(|e| e.id);

    for edge in sorted_edges.iter().take(15) {
        if y > height as i32 - 20 { break; }
        let pred_name = pred_to_string(edge.pred);
        let text = format!("({}) -[:{}]-> ({})", edge.src.0, pred_name, edge.dst.0);
        draw_text_simple(buffer, stride, width, height, x, y, &text, green);
        y += line_height;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
