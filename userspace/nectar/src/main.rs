#![no_std]
#![no_main]

extern crate alloc;
extern crate stem;

use abi::ids::HandleId;
use abi::schema::{keys, kinds};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use stem::info;
use stem::petals::{AlignItems, Color, Flex, FontKey, JustifyContent, Scene, Text, Window, Styled};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::syscall::port::{port_recv, port_send, port_create, PortHandle};

const MSG_UDP_BIND: u16 = 0x0110;
const MSG_UDP_SEND_TO: u16 = 0x0111;
const MSG_NET_JOIN_MULTICAST: u16 = 0x0105;

const RESP_HANDLE: u16 = 0x8001;
const RESP_DATA: u16 = 0x8002;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("NECTAR: Started.");

    // 1. Discovery - Find UI_CROWN
    let mut ui_crown_id = None;
    let mut buf = [ThingId::default(); 1];
    for _ in 0..50 {
        if let Ok(n) = thingsys::find("ui.Crown", &mut buf) {
            if n > 0 {
                ui_crown_id = Some(buf[0]);
                info!("NECTAR: Found UI_CROWN: {:?}", ui_crown_id);
                break;
            }
        }
        stem::time::sleep_ms(100);
    }

    let mut window_id = None;
    let mut hostname = String::from("thing-os");

    // Pre-initialize UI if possible
    if let Some(crown_id) = ui_crown_id {
        if let Ok(win) = thingsys::create_node("ui.Window") {
            window_id = Some(win);
            info!("NECTAR: Created UI_WINDOW node: {:?}", win);
            thingsys::link(crown_id, "ui.HasWindow", win).ok();
            
            // Initial scene
            let scene = build_scene(win, &hostname);
            match stem::petals::publish_window(&scene) {
                Ok(_) => info!("NECTAR: Initial UI scene published."),
                Err(e) => stem::warn!("NECTAR: Failed to publish initial UI: {:?}", e),
            }
        }
    }

    info!("NECTAR: Networking setup starting...");
    let (api, mac) = loop {
        if let Some(res) = find_netd_and_mac() {
            info!("NECTAR: Found netd and MAC.");
            break res;
        }
        stem::time::sleep_ms(1000);
    };

    let (resp_w, resp_r) = port_create(64).expect("port_create");
    info!("NECTAR: Attempting to bind UDP 5353...");
    let udp_handle = match udp_bind(api, resp_w, resp_r, 5353) {
        Ok(h) => {
            info!("NECTAR: Bound to UDP 5353, handle={}", h);
            h
        }
        Err(_) => {
            stem::error!("NECTAR: Failed to bind to 5353. Continuing without mDNS responder.");
            0
        }
    };

    if udp_handle != 0 {
        net_join_multicast(api, resp_w, resp_r, [224, 0, 0, 251]).ok();
        info!("NECTAR: Joined multicast group 224.0.0.251");
    }

    info!("NECTAR: Starting mDNS daemon...");
    
    // Initial hostname sync
    hostname = get_or_generate_hostname(mac);
    info!("NECTAR: Hostname resolved to '{}'", hostname);
    
    // Update graph/UI
    let mut host_buf = [ThingId::default(); 1];
    if let Ok(1) = thingsys::find(kinds::DEV_HOST, &mut host_buf) {
        set_string_prop(host_buf[0], keys::NAME, &hostname);
        info!("NECTAR: Published hostname '{}' to graph", hostname);
    }
    
    if let Some(win) = window_id {
        let scene = build_scene(win, &hostname);
        match stem::petals::publish_window(&scene) {
            Ok(_) => info!("NECTAR: Publishing scene for hostname '{}' on window {:?}...", hostname, win),
            Err(e) => stem::warn!("NECTAR: Failed to update UI: {:?}", e),
        }
        info!("NECTAR: Scene published successfully.");
    }

    if udp_handle != 0 {
        info!("NECTAR: mDNS responder active.");
    }

    let mut last_sync_check = stem::monotonic_ns();
    let mut last_heartbeat = stem::monotonic_ns();

    loop {
        let now = stem::monotonic_ns();
        
        // mDNS responder logic would go here (UDP recv/reply)
        let mut buf = [0u8; 1500];
        match udp_recv_from(resp_w, resp_r, &mut buf) {
            Ok(Some((_ip, _port, _len))) => {
                // info!("NECTAR: Received mDNS query from {}.{}.{}.{}:{}", ip[0], ip[1], ip[2], ip[3], port);
                // TODO: Parse mDNS and reply if hostname matches
            }
            Ok(None) => {}
            Err(_) => {}
        }

        // Periodic heartbeat (every 5 seconds)
        if now - last_heartbeat > 5_000_000_000 {
            last_heartbeat = now;
            info!("NECTAR: Heartbeat - active and monitoring graph...");
        }

        // Periodic sync check (every second)
        if now - last_sync_check > 1_000_000_000 {
            last_sync_check = now;
            let current_hostname = get_or_generate_hostname(mac);
            if current_hostname != hostname {
                info!("NECTAR: Synchronizing hostname: '{}' -> '{}'", hostname, current_hostname);
                hostname = current_hostname;
                
                // Update UI
                if let Some(win) = window_id {
                    let scene = build_scene(win, &hostname);
                    match stem::petals::publish_window(&scene) {
                        Ok(_) => info!("NECTAR: UI updated with new hostname."),
                        Err(e) => stem::warn!("NECTAR: Failed to update UI: {:?}", e),
                    }
                }
            }
        }

        stem::yield_now();
    }
}

fn find_netd_and_mac() -> Option<(PortHandle, [u8; 6])> {
    let mut buf = [ThingId::default(); 1];
    let n = thingsys::find("svc.net.Stack", &mut buf).ok()?;
    if n == 0 { return None; }
    let net_id = buf[0];
    
    let api_port = thingsys::prop_get(net_id, "net.socket_api").ok()? as PortHandle;
    let mac_packed = thingsys::prop_get(net_id, "net.mac").ok()?;
    
    let mac = [
        (mac_packed & 0xFF) as u8,
        ((mac_packed >> 8) & 0xFF) as u8,
        ((mac_packed >> 16) & 0xFF) as u8,
        ((mac_packed >> 24) & 0xFF) as u8,
        ((mac_packed >> 32) & 0xFF) as u8,
        ((mac_packed >> 40) & 0xFF) as u8,
    ];
    
    Some((api_port, mac))
}

fn get_or_generate_hostname(mac: [u8; 6]) -> String {
    // Check if hostname is already set in dev.Host.name
    let mut host_buf = [ThingId::default(); 1];
    if let Ok(1) = thingsys::find(kinds::DEV_HOST, &mut host_buf) {
        let host_id = host_buf[0];
        if let Ok(sym) = thingsys::prop_get(host_id, keys::NAME) {
            // Check if it's non-zero
            if sym != 0 {
                let mut desc_buf = [0u8; 256];
                if let Ok(len) = thingsys::describe_thing(host_id, &mut desc_buf) {
                    let s = core::str::from_utf8(&desc_buf[..len]).unwrap_or("");
                    if let Some(rest) = s.split_once("name: \"").map(|(_, r)| r) {
                        if let Some(end) = rest.find('"') {
                            return String::from(&rest[..end]);
                        }
                    }
                }
            }
        }
    }

    // Generate plant-themed hostname
    let descriptors = ["quiet", "vibrant", "ancient", "sunny", "dewy", "wild", "silver", "golden", "mossy", "blooming"];
    let plants = ["moss", "fern", "ivy", "clover", "petal", "thistle", "willow", "cedar", "birch", "maple"];

    // Use MAC for stable hash
    let seed = mac.iter().fold(0u32, |acc, &x| acc.wrapping_add(x as u32));
    let d_idx = (seed as usize) % descriptors.len();
    let p_idx = (seed as usize / descriptors.len()) % plants.len();

    format!("{}-{}", descriptors[d_idx], plants[p_idx])
}

fn build_scene(window_id: ThingId, hostname: &str) -> Scene {
    Scene::new().window(
        Window::new(window_id)
            .title("Hostname")
            .initial_size(360, 100)
            .root(
                Flex::column()
                    .gap(8)
                    .padding(16)
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center)
                    .push(
                        Text::new("Hostname")
                            .font(FontKey::new("NotoSans-Regular").size(14))
                            .color(Color::rgb(180, 180, 180)),
                    )
                    .push(
                        Text::new(hostname)
                            .font(FontKey::new("NotoSans-Regular").size(32)) // Use NotoSans as fallback if DSEG7 is missing
                            .color(Color::rgb(100, 200, 255)), // Light blue
                    ),
            ),
    )
}

fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
    use stem::thing::sys::{bytespace_create, bytespace_write};
    if value.is_empty() {
        thingsys::prop_set(id, key_name, 0).ok();
        return;
    }
    let bs_id = bytespace_create(value.len(), 0, 0).expect("create bytespace");
    bytespace_write(bs_id, 0, value.as_bytes()).ok();
    thingsys::prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

fn udp_bind(api: PortHandle, resp_w: PortHandle, resp_r: PortHandle, port: u16) -> Result<u32, ()> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&(resp_w as u32).to_le_bytes());
    msg.extend_from_slice(&MSG_UDP_BIND.to_le_bytes());
    msg.extend_from_slice(&port.to_le_bytes());
    
    info!("NECTAR: udp_bind sending to api={}", api);
    port_send(api, &msg).map_err(|_| ())?;
    
    info!("NECTAR: udp_bind waiting for response...");
    let mut resp = [0u8; 128];
    let mut len = 0;
    // Wait up to 1 second for response
    for _ in 0..100 {
        match port_recv(resp_r, &mut resp) {
            Ok(l) if l >= 2 => {
                len = l;
                break;
            }
            _ => stem::time::sleep_ms(10),
        }
    }
    
    if len >= 2 {
        let resp_type = u16::from_le_bytes([resp[0], resp[1]]);
        if resp_type == RESP_HANDLE {
            Ok(u32::from_le_bytes([resp[2], resp[3], resp[4], resp[5]]))
        } else {
            stem::error!("NECTAR: udp_bind failed - resp_type=0x{:04x}", resp_type);
            Err(())
        }
    } else {
        stem::error!("NECTAR: udp_bind timeout/failed");
        Err(())
    }
}

fn net_join_multicast(api: PortHandle, resp_w: PortHandle, resp_r: PortHandle, ip: [u8; 4]) -> Result<(), ()> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&(resp_w as u32).to_le_bytes());
    msg.extend_from_slice(&MSG_NET_JOIN_MULTICAST.to_le_bytes());
    msg.extend_from_slice(&ip);
    
    port_send(api, &msg).map_err(|_| ())?;
    
    let mut resp = [0u8; 64];
    // Wait for response acknowledgement
    for _ in 0..10 {
        if port_recv(resp_r, &mut resp).is_ok() { break; }
        stem::time::sleep_ms(5);
    }
    
    Ok(())
}

fn udp_recv_from(resp_w: PortHandle, resp_r: PortHandle, buf: &mut [u8]) -> Result<Option<([u8; 4], u16, usize)>, ()> {
    // Request is not needed for non-blocking recv if netd pushes data or we just poll the bounded port
    // However, netd's socket API typically requires a RECV msg to trigger a read if it's not and unsolicited push.
    // Based on anther's net_client, it seems they just poll port_recv.
    
    let mut resp = [0u8; 2048];
    match port_recv(resp_r, &mut resp) {
        Ok(len) if len >= 2 => {
            let resp_type = u16::from_le_bytes([resp[0], resp[1]]);
            if resp_type == RESP_DATA && len >= 8 {
                let mut ip = [0u8; 4];
                ip.copy_from_slice(&resp[2..6]);
                let port = u16::from_le_bytes([resp[6], resp[7]]);
                let data_len = len - 8;
                buf[..data_len].copy_from_slice(&resp[8..len]);
                Ok(Some((ip, port, data_len)))
            } else {
                Ok(None)
            }
        }
        Err(abi::errors::Errno::EAGAIN) => Ok(None),
        Err(_) => Err(()),
        _ => Ok(None),
    }
}
