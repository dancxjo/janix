#![feature(restricted_std)]
#![no_main]

//! # Fetch Service (fetchd)
//!
//! Displays the system's IP address in a window.
//! Watches the network stack service for IP configuration updates.

extern crate alloc;

use abi::ids::HandleId;
use abi::schema::{keys, kinds, rels};
use alloc::format;
use core::time::Duration;
use stem::info;
use stem::ui::UiBuilder;
use stem::thing::sys::{create_node, find, link, prop_get, prop_set};
use stem::thing::ThingId;

/// Graph kind for the network stack service (published by netd)
const KIND_NET_STACK: &str = "svc.net.Stack";

fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
    use stem::thing::sys::{bytespace_create, bytespace_write};
    if value.is_empty() {
        prop_set(id, key_name, 0).ok();
        return;
    }
    let bs_id = bytespace_create(value.len(), 0, 0).expect("create bytespace");
    bytespace_write(bs_id, 0, value.as_bytes()).ok();
    prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

/// Holds the node IDs for the fetchd UI tree so we can update text without recreating nodes.
struct FetchdUiNodes {
    ip_node: ThingId,
    status_node: ThingId,
}

/// Build the initial UI tree for the network window. Returns the node IDs for later updates.
fn render_window_init(
    window_id: ThingId,
    ip_text: &str,
    status_text: &str,
) -> Option<FetchdUiNodes> {
    let panel = UiBuilder::create_panel(window_id);
    let _label = UiBuilder::create_text(panel, "IP Address");
    let ip = UiBuilder::create_text(panel, ip_text);
    let status = UiBuilder::create_text(panel, status_text);

    Some(FetchdUiNodes {
        ip_node: ip,
        status_node: status,
    })
}

/// Update just the text content on existing nodes and bump scene gen (no new nodes created).
fn render_window_update(
    window_id: ThingId,
    nodes: &FetchdUiNodes,
    ip_text: &str,
    status_text: &str,
) {
    set_string_prop(nodes.ip_node, keys::UI_TEXT, ip_text);
    set_string_prop(nodes.status_node, keys::UI_TEXT, status_text);
    // Bump scene gen so Bloom re-renders
    let gen = prop_get(window_id, keys::UI_SCENE_GEN).unwrap_or(0);
    prop_set(window_id, keys::UI_SCENE_GEN, gen.wrapping_add(1)).ok();
}

/// Unpack IP address from u64 to dotted decimal string
fn ip_to_string(packed: u64) -> alloc::string::String {
    let a = (packed & 0xFF) as u8;
    let b = ((packed >> 8) & 0xFF) as u8;
    let c = ((packed >> 16) & 0xFF) as u8;
    let d = ((packed >> 24) & 0xFF) as u8;
    format!("{}.{}.{}.{}", a, b, c, d)
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("FETCHD: Starting IP address display...");

    let mut window_id: Option<ThingId> = None;
    let mut ui_nodes: Option<FetchdUiNodes> = None;

    // Wait for UI Root (Compositor) - like Bloom / Photosynthesis pattern
    info!("FETCHD: Waiting for UI Root (Compositor)...");
    let ui_crown = {
        let kind = stem::thing::sys::intern(kinds::UI_CROWN).unwrap_or(0) as u64;
        let mut found_id = 0;
        loop {
            let mut ids = [ThingId::default(); 1];
            if let Ok(n) = stem::thing::sys::find(kind, &mut ids) {
                if n > 0 {
                    found_id = ids[0].to_u64_lossy();
                    break;
                }
            }
            stem::time::sleep_ms(100);
        }
        ThingId::from_u64(found_id)
    };
    info!("FETCHD: Found UI Root: {}", ui_crown.to_u64_lossy());

    if ui_crown.to_u64_lossy() != 0 {
        // Create Window
        let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
        link(win, rels::CHILD_OF, ui_crown).expect("link window");
        link(ui_crown, rels::HAS_CHILD, win).expect("link window has_child");
        window_id = Some(win);

        // Window Style: Light Background
        prop_set(win, keys::UI_BG_COLOR, 0xFFE8EEF4).ok(); // Light blue-gray
        set_string_prop(win, keys::UI_TITLE, "Network");

        // Window Layout: Bottom-left corner (mirroring clock at bottom-right)
        prop_set(win, keys::UI_WIDTH, 360).ok();
        prop_set(win, keys::UI_HEIGHT, 140).ok();
        prop_set(win, keys::UI_X, 20).ok(); // Left edge offset
        prop_set(win, keys::UI_Y, 0).ok();
        prop_set(win, keys::UI_INSET_BOTTOM, 30).ok(); // Match clock's bottom offset

        // Initial scene publish — create the tree once
        match render_window_init(win, "-.-.-.--", "Waiting for network...") {
            Some(nodes) => {
                ui_nodes = Some(nodes);
            }
            None => {
                info!("FETCHD: initial scene publish failed");
            }
        }
    }

    info!("FETCHD: Entering main loop, watching for network stack...");

    let mut last_ip: u64 = 0;

    loop {
        // Look for the network stack service
        let mut buf = [ThingId::default(); 16];
        let ip_text;
        let status_text;

        match find(KIND_NET_STACK, &mut buf) {
            Ok(count) if count > 0 => {
                let stack_id = pick_best_net_stack(&buf[..count]).unwrap_or(buf[0]);

                // Get IP address from graph
                match prop_get(stack_id, "net.ip") {
                    Ok(ip_packed) if ip_packed != 0 => {
                        if ip_packed != last_ip {
                            last_ip = ip_packed;
                            info!("FETCHD: Got IP address: {}", ip_to_string(ip_packed));
                        }
                        ip_text = ip_to_string(ip_packed);
                        status_text = alloc::string::String::from("Connected");
                    }
                    _ => {
                        ip_text = alloc::string::String::from("-.-.-.--");
                        status_text = alloc::string::String::from("Awaiting DHCP...");
                    }
                }
            }
            _ => {
                ip_text = alloc::string::String::from("-.-.-.--");
                status_text = alloc::string::String::from("Network offline");
            }
        }

        // Update existing UI nodes (no new graph nodes created)
        if let (Some(win), Some(ref nodes)) = (window_id, &ui_nodes) {
            render_window_update(win, nodes, &ip_text, &status_text);
        }

        // Poll every second
        stem::sleep(Duration::from_secs(1));
    }
}

fn pick_best_net_stack(nodes: &[ThingId]) -> Option<ThingId> {
    // Prefer a stack that already has a non-zero IP; otherwise prefer one with socket API handle.
    let mut socket_ready: Option<ThingId> = None;
    for &id in nodes {
        if prop_get(id, "net.ip").ok().unwrap_or(0) != 0 {
            return Some(id);
        }
        if prop_get(id, keys::WRITE_PORT_HANDLE).ok().unwrap_or(0) != 0 && socket_ready.is_none() {
            socket_ready = Some(id);
        }
    }
    socket_ready
}
