#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use abi::schema::{keys, kinds, rels};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::syscall::port::{port_create, port_recv, port_send, PortHandle};
use stem::{info, warn, error};
use stem::petals::{AlignItems, Color, Flex, FontKey, JustifyContent, Scene, Styled, Text, Window};
use core::time::Duration;

mod dns_packet;

const KIND_NET_STACK: &str = "svc.net.Stack";
const KIND_DEV_HOST: &str = "dev.Host";

// Socket API message types (from netd)
const MSG_UDP_BIND: u16 = 0x0300;
const MSG_UDP_SEND_TO: u16 = 0x0301;
const MSG_UDP_RECV_FROM: u16 = 0x0302;
const MSG_NET_JOIN_MULTICAST: u16 = 0x0400;

// Response types
const RESP_HANDLE: u16 = 0x0002;
const RESP_DATA: u16 = 0x0003;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("NECTAR: Starting mDNS daemon...");

    // 1. Wait for network stack and get MAC address
    let (api_port, mac) = loop {
        if let Some(res) = find_net_stack() {
            break res;
        }
        stem::time::sleep_ms(500);
    };

    // 2. Resolve/Generate Hostname
    let hostname = get_or_generate_hostname(mac);
    info!("NECTAR: Hostname resolved to '{}'", hostname);

    // 3. Publish hostname to graph
    publish_hostname(&hostname);

    // 4. Setup UI
    let mut window_id: Option<ThingId> = None;
    let mut ui_crown = ThingId::default();
    
    // Attempt to find UI crown for UI display
    let mut i = 0;
    while i < 10 {
        let mut ui_crowns = [ThingId::default(); 1];
        if let Ok(count) = thingsys::find(kinds::UI_CROWN, &mut ui_crowns) {
            if count > 0 {
                ui_crown = ui_crowns[0];
                break;
            }
        }
        stem::time::sleep_ms(100);
        i += 1;
    }

    if ui_crown.to_u64_lossy() != 0 {
        info!("NECTAR: Creating UI_WINDOW node...");
        let win = thingsys::create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
        thingsys::link(win, rels::CHILD_OF, ui_crown).expect("link window");
        thingsys::link(ui_crown, rels::HAS_CHILD, win).expect("link window has_child");
        window_id = Some(win);

        thingsys::prop_set(win, keys::UI_BG_COLOR, 0xFF181824).ok(); // Dark slate
        set_string_prop(win, keys::UI_TITLE, "Hostname");

        thingsys::prop_set(win, keys::UI_WIDTH, 360).ok();
        thingsys::prop_set(win, keys::UI_HEIGHT, 100).ok();
        thingsys::prop_set(win, keys::UI_X, 0).ok();
        thingsys::prop_set(win, keys::UI_Y, 20).ok(); // 20px from top
        thingsys::prop_set(win, keys::UI_INSET_RIGHT, 20).ok(); // 20px from right

        info!("NECTAR: Publishing scene for hostname '{}' on window {:?}...", hostname, win);
        let scene = build_scene(win, &hostname);
        match stem::petals::publish_window(&scene) {
            Ok(_) => info!("NECTAR: Scene published successfully."),
            Err(e) => warn!("NECTAR: Failed to publish scene: {:?}", e),
        }
    } else {
        warn!("NECTAR: Could not find UI_CROWN. UI disabled.");
    }

    // 5. Initialize Networking
    let (resp_write, resp_read) = port_create(1024).expect("Failed to create response port");
    
    // Bind to 5353
    let handle = match udp_bind(api_port, resp_write, resp_read, 5353) {
        Ok(h) => {
            info!("NECTAR: Bound to UDP 5353, handle={}", h);
            h
        }
        Err(_) => {
            error!("NECTAR: Failed to bind to 5353. mDNS features disabled.");
            0 // Dummy handle
        }
    };

    if handle != 0 {
        // Join mDNS multicast group 224.0.0.251
        let mdns_ip = [224, 0, 0, 251];
        if let Err(_) = net_join_multicast(api_port, resp_write, resp_read, mdns_ip) {
            warn!("NECTAR: Failed to join multicast group.");
        } else {
            info!("NECTAR: Joined multicast group 224.0.0.251");
        }
    }

    info!("NECTAR: mDNS responder active.");

    // 6. Main Loop
    let mut buf = [0u8; 2048];
    let mut last_sync_check = stem::monotonic_ns();
    let mut hostname = hostname;

    loop {
        if handle != 0 {
            // Poll for UDP packets
            match udp_recv_from(api_port, resp_write, resp_read, handle, &mut buf) {
                Ok(Some((remote_ip, remote_port, len))) => {
                    let packet = &buf[..len];
                    handle_mdns_packet(api_port, resp_write, resp_read, handle, &hostname, remote_ip, remote_port, packet);
                }
                Ok(None) => {}
                Err(e) => warn!("NECTAR: recv error: {:?}", e),
            }
        }

        // Periodic sync check (every second)
        let now = stem::monotonic_ns();
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
                        Err(e) => warn!("NECTAR: Failed to update UI: {:?}", e),
                    }
                }
            }
        }

        stem::time::sleep_ms(10);
    }
}

fn find_net_stack() -> Option<(PortHandle, [u8; 6])> {
    let mut buf = [ThingId::default(); 1];
    let count = thingsys::find(KIND_NET_STACK, &mut buf).ok()?;
    if count == 0 { return None; }
    
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
                            .font(FontKey::new("DSEG7Classic-Regular").size(32))
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

fn publish_hostname(hostname: &str) {
    let mut host_buf = [ThingId::default(); 1];
    if let Ok(count) = thingsys::find(kinds::DEV_HOST, &mut host_buf) {
        let host_id = if count > 0 {
            host_buf[0]
        } else {
            thingsys::create_node(kinds::DEV_HOST).expect("Failed to create dev.Host node")
        };
        
        if let Ok(sym) = thingsys::intern(hostname) {
            thingsys::prop_set(host_id, keys::NAME, sym as u64).ok();
            info!("NECTAR: Published hostname '{}' to graph", hostname);
        }
    }
}

// Networking helpers (Client side of Socket API)

fn udp_bind(api: PortHandle, resp_w: PortHandle, resp_r: PortHandle, port: u16) -> Result<u32, ()> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&(resp_w as u32).to_le_bytes());
    msg.extend_from_slice(&MSG_UDP_BIND.to_le_bytes());
    msg.extend_from_slice(&port.to_le_bytes());
    
    port_send(api, &msg).map_err(|_| ())?;
    
    let mut resp = [0u8; 128];
    let mut len = 0;
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
            error!("NECTAR: udp_bind failed - resp_type=0x{:04x}", resp_type);
            Err(())
        }
    } else {
        error!("NECTAR: udp_bind timeout/failed");
        Err(())
    }
}

fn udp_recv_from(api: PortHandle, resp_w: PortHandle, resp_r: PortHandle, handle: u32, buf: &mut [u8]) -> Result<Option<([u8; 4], u16, usize)>, ()> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&(resp_w as u32).to_le_bytes());
    msg.extend_from_slice(&MSG_UDP_RECV_FROM.to_le_bytes());
    msg.extend_from_slice(&handle.to_le_bytes());
    
    port_send(api, &msg).map_err(|_| ())?;
    
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

fn udp_send_to(api: PortHandle, resp_w: PortHandle, resp_r: PortHandle, handle: u32, ip: [u8; 4], port: u16, data: &[u8]) -> Result<(), ()> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&(resp_w as u32).to_le_bytes());
    msg.extend_from_slice(&MSG_UDP_SEND_TO.to_le_bytes());
    msg.extend_from_slice(&handle.to_le_bytes());
    msg.extend_from_slice(&ip);
    msg.extend_from_slice(&port.to_le_bytes());
    msg.extend_from_slice(data);
    
    port_send(api, &msg).map_err(|_| ())?;
    
    // We don't necessarily need to wait for response for simple send, but netd sends one
    let mut resp = [0u8; 64];
    for _ in 0..10 {
        if port_recv(resp_r, &mut resp).is_ok() { break; }
        stem::time::sleep_ms(5);
    }
    
    Ok(())
}

fn net_join_multicast(api: PortHandle, resp_w: PortHandle, resp_r: PortHandle, ip: [u8; 4]) -> Result<(), ()> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&(resp_w as u32).to_le_bytes());
    msg.extend_from_slice(&MSG_NET_JOIN_MULTICAST.to_le_bytes());
    msg.extend_from_slice(&ip);
    
    port_send(api, &msg).map_err(|_| ())?;
    
    let mut resp = [0u8; 64];
    for _ in 0..10 {
        if port_recv(resp_r, &mut resp).is_ok() { break; }
        stem::time::sleep_ms(5);
    }
    
    Ok(())
}

fn handle_mdns_packet(api: PortHandle, resp_w: PortHandle, resp_r: PortHandle, handle: u32, hostname: &str, remote_ip: [u8; 4], remote_port: u16, data: &[u8]) {
    use dns_packet::*;
    
    let packet = match DnsPacket::parse(data) {
        Some(p) => p,
        None => return,
    };
    
    // Only handle queries
    if packet.flags & 0x8000 != 0 { return; }
    
    let mut should_respond = false;
    let local_name = format!("{}.local", hostname);

    for q in packet.questions {
        if q.name == local_name || q.name == "_" {
             should_respond = true;
             break;
        }
    }
    
    if should_respond {
        info!("NECTAR: Responding to mDNS query from {:?}:{}", remote_ip, remote_port);
        
        let mut resp = DnsPacket::new_response(packet.transaction_id);
        
        // A record for hostname.local
        // For now we don't know our own IP here easily, but we can get it from the graph!
        let mut ip = [0u8; 4];
        let mut net_buf = [ThingId::default(); 1];
        if let Ok(1) = thingsys::find(KIND_NET_STACK, &mut net_buf) {
            if let Ok(ip_packed) = thingsys::prop_get(net_buf[0], "net.ip") {
                ip = [
                    (ip_packed & 0xFF) as u8,
                    ((ip_packed >> 8) & 0xFF) as u8,
                    ((ip_packed >> 16) & 0xFF) as u8,
                    ((ip_packed >> 24) & 0xFF) as u8,
                ];
            }
        }

        if ip != [0, 0, 0, 0] {
            resp.answers.push(DnsResourceRecord {
                name: local_name.clone(),
                rtype: 1, // A
                rclass: 1, // IN
                ttl: 120,
                data: ip.to_vec(),
            });

            // TXT record with some metadata
            resp.answers.push(DnsResourceRecord {
                name: local_name.clone(),
                rtype: 16, // TXT
                rclass: 1, // IN
                ttl: 120,
                data: b"\x0bos=thing-os\x0erole=nectar-os".to_vec(),
            });

            let encoded = resp.encode();
            // mDNS responses are usually sent to the multicast group
            let mdns_multicast = [224, 0, 0, 251];
            let _ = udp_send_to(api, resp_w, resp_r, handle, mdns_multicast, 5353, &encoded);
        }
    }
}
