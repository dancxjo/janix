#![no_std]
#![no_main]

//! # Fetch Service (fetchd)
//!
//! Displays the system's IP address in a window.
//! Watches the network stack service for IP configuration updates.

extern crate alloc;

use abi::schema::{keys, kinds, rels};
use alloc::format;
use core::time::Duration;
use stem::info;
use stem::petals::{AlignItems, Color, Flex, FontKey, JustifyContent, Scene, Styled, Text, Window};
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

fn build_scene(window_id: ThingId, ip_text: &str, status_text: &str) -> Scene {
    Scene::new().window(
        Window::new(window_id)
            .title("Network")
            .initial_size(360, 140)
            .root(
                Flex::column()
                    .gap(12)
                    .padding(20)
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center)
                    .push(
                        Text::new("IP Address")
                            .font(FontKey::new("NotoSans-Regular").size(16))
                            .color(Color::rgb(180, 180, 180)),
                    )
                    .push(
                        Text::new(ip_text)
                            .font(FontKey::new("DSEG7Classic-Regular").size(48))
                            .color(Color::rgb(64, 240, 128)),
                    )
                    .push(
                        Text::new(status_text)
                            .font(FontKey::new("NotoSans-Regular").size(12))
                            .color(Color::rgb(140, 140, 140)),
                    ),
            ),
    )
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

    // Wait for UI Root (Compositor) - like Bloom / Photosynthesis pattern
    info!("FETCHD: Waiting for UI Root (Compositor)...");
    let mut ui_crown = ThingId::default();
    while ui_crown.to_u64_lossy() == 0 {
        let mut ui_crowns = [ThingId::default(); 1];
        if let Ok(1) = find(kinds::UI_CROWN, &mut ui_crowns) {
            ui_crown = ui_crowns[0];
        } else {
            stem::sleep(Duration::from_millis(100));
        }
    }
    info!("FETCHD: Found UI Root: {}", ui_crown.to_u64_lossy());

    if ui_crown.to_u64_lossy() != 0 {
        // Create Window
        let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
        link(win, rels::CHILD_OF, ui_crown).expect("link window");
        link(ui_crown, rels::HAS_CHILD, win).expect("link window has_child");
        window_id = Some(win);

        // Window Style: Dark Background
        prop_set(win, keys::UI_BG_COLOR, 0xFF101820).ok(); // Dark teal-black
        set_string_prop(win, keys::UI_TITLE, "Network");

        // Window Layout: Bottom-left corner (mirroring clock at bottom-right)
        prop_set(win, keys::UI_WIDTH, 360).ok();
        prop_set(win, keys::UI_HEIGHT, 140).ok();
        prop_set(win, keys::UI_X, 20).ok();  // Left edge offset
        prop_set(win, keys::UI_Y, 0).ok();
        prop_set(win, keys::UI_INSET_BOTTOM, 30).ok(); // Match clock's bottom offset

        // Initial scene publish
        let scene = build_scene(win, "-.-.-.--", "Waiting for network...");
        if let Err(e) = stem::petals::publish_window(&scene) {
            info!("FETCHD: initial scene publish failed: {:?}", e);
        }
    }

    info!("FETCHD: Entering main loop, watching for network stack...");

    let mut last_ip: u64 = 0;

    loop {
        // Look for the network stack service
        let mut buf = [ThingId::default(); 1];
        let ip_text;
        let status_text;

        match find(KIND_NET_STACK, &mut buf) {
            Ok(count) if count > 0 => {
                let stack_id = buf[0];
                
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

        // Update window if we have one
        if let Some(win) = window_id {
            let scene = build_scene(win, &ip_text, &status_text);
            if let Err(e) = stem::petals::publish_window(&scene) {
                info!("FETCHD: scene publish failed: {:?}", e);
            }
        }

        // Poll every second
        stem::sleep(Duration::from_secs(1));
    }
}
