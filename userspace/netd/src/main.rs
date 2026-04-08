#![no_std]
#![no_main]

//! # Network Service (netd) — Phase 3: /net/ VFS provider
//!
//! Replaces the graph-based driver IPC and port-based socket API with:
//! - Driver access via `/dev/net/virtio0/{rx,tx,mac,mtu}` VFS files (issue #540)
//! - Application socket API via `/net/` VFS tree (issue #541)
//! Provides networking capabilities using smoltcp TCP/IP stack.

#[macro_use]
extern crate alloc;
extern crate stem;

mod dhcp;
mod dns;
mod socket_api;
mod vfs_device;
mod vfs_provider;

use abi::syscall::vfs_flags::{O_NONBLOCK, O_RDONLY, O_WRONLY};
use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
use smoltcp::wire::EthernetAddress;
use socket_api::SocketApi;
use stem::syscall::port::PortHandle;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::{info, warn};
use vfs_device::VfsNicDevice;
use vfs_provider::NetVfsProvider;

/// Path prefix for the virtio NIC VFS provider (published by virtio_netd).
const VIRTIO0_PATH: &str = "/dev/net/virtio0";

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("NETD: Starting network service (Phase 3 — /net/ VFS provider)");

    info!(
        "NETD: Waiting for virtio_netd VFS provider at {}...",
        VIRTIO0_PATH
    );
    let (rx_fd, tx_fd, events_fd, mac, iface_mtu, initial_link_up) = open_nic_device();
    let mtu = iface_mtu as usize;

    info!(
        "NETD: Driver online — MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}  MTU {}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5], mtu
    );

    let mut device = VfsNicDevice::new(rx_fd, tx_fd, events_fd, mac, mtu, initial_link_up);
    let config = Config::new(EthernetAddress(mac).into());
    let mut iface = Interface::new(config, &mut device, VfsNicDevice::now());

    let mut net_provider = loop {
        match NetVfsProvider::new(mac, mtu, initial_link_up) {
            Some(provider) => break provider,
            None => {
                warn!("NETD: Failed to mount /net/, retrying...");
                stem::time::sleep_ms(200);
            }
        }
    };

    info!("NETD: Running DHCP...");
    let dhcp_config = match dhcp::run_dhcp(&mut iface, &mut device) {
        Ok(cfg) => {
            info!(
                "NETD: DHCP — IP: {}, GW: {}, DNS: {}",
                cfg.ip, cfg.gateway, cfg.dns
            );
            cfg
        }
        Err(e) => {
            warn!("NETD: DHCP failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    net_provider.set_ip_config(dhcp_config.ip, dhcp_config.prefix_len, dhcp_config.gateway);
    info!("NETD: Network ready — entering VFS service loop");

    let mut socket_api = SocketApi::new();
    let mut sockets_storage: [SocketStorage; 256] = [SocketStorage::EMPTY; 256];
    let mut socket_set = SocketSet::new(&mut sockets_storage[..]);
    let mut last_link_state = device.link_up();

    loop {
        let mut did_work = false;

        let now = VfsNicDevice::now();
        if iface.poll(now, &mut device, &mut socket_set) {
            did_work = true;
        }

        let before_len = socket_api.socket_count();
        net_provider.drain_rpcs(&mut iface, &mut device, &mut socket_set, &mut socket_api);
        if socket_api.socket_count() != before_len {
            did_work = true;
        }

        let now = VfsNicDevice::now();
        if iface.poll(now, &mut device, &mut socket_set) {
            did_work = true;
        }

        let current_link = device.link_up();
        if current_link != last_link_state {
            last_link_state = current_link;
            net_provider.link_up = current_link;
            did_work = true;
            info!(
                "NETD: Link state changed → {}",
                if current_link { "UP" } else { "DOWN" }
            );
        }

        socket_api.gc_closed_sockets(&mut socket_set);

        if !did_work {
            let _ = stem::syscall::port::port_wait(
                &[net_provider.req_read_port()],
                abi::syscall::port_wait::READABLE,
            );
        }
    }
}

const SOCKET_API_ENVELOPE_HEADER_LEN: usize = 16;
const SOCKET_API_MAX_PAYLOAD_LEN: usize = 4096;

enum EnvelopeDecode<'a> {
    Complete {
        client_response_port: PortHandle,
        caller_tid: Option<u64>,
        msg_body: &'a [u8],
        msg_type: u16,
        consumed: usize,
    },
    NeedMore,
    Malformed,
}

fn decode_socket_api_envelope(packet: &[u8]) -> EnvelopeDecode<'_> {
    if packet.len() < SOCKET_API_ENVELOPE_HEADER_LEN {
        return EnvelopeDecode::NeedMore;
    }

    let response_port =
        u32::from_le_bytes([packet[0], packet[1], packet[2], packet[3]]) as PortHandle;
    let caller_tid = u64::from_le_bytes([
        packet[4], packet[5], packet[6], packet[7], packet[8], packet[9], packet[10], packet[11],
    ]);
    let msg_type = u16::from_le_bytes([packet[12], packet[13]]);
    let payload_len = u16::from_le_bytes([packet[14], packet[15]]) as usize;

    if payload_len > SOCKET_API_MAX_PAYLOAD_LEN {
        warn!(
            "NETD: Malformed Socket API message: payload too large (len={}, port={}, tid=0x{:016x}, type=0x{:04x}, payload_len={}, hex={:02x?})",
            packet.len(),
            response_port,
            caller_tid,
            msg_type,
            payload_len,
            &packet[..core::cmp::min(packet.len(), SOCKET_API_ENVELOPE_HEADER_LEN)]
        );
        return EnvelopeDecode::Malformed;
    }

    if packet.len() < SOCKET_API_ENVELOPE_HEADER_LEN + payload_len {
        return EnvelopeDecode::NeedMore;
    }

    if socket_api::is_known_msg_type(msg_type) {
        return EnvelopeDecode::Complete {
            client_response_port: response_port,
            caller_tid: Some(caller_tid),
            msg_body: &packet[12..SOCKET_API_ENVELOPE_HEADER_LEN + payload_len],
            msg_type,
            consumed: SOCKET_API_ENVELOPE_HEADER_LEN + payload_len,
        };
    }

    if msg_type == 0 {
        warn!(
            "NETD: Protocol desync! Received RESP_OK (0) as request type. Full header: {:02x?}",
            &packet[..SOCKET_API_ENVELOPE_HEADER_LEN]
        );
    }

    warn!(
        "NETD: Malformed Socket API message: len={}, port={}, tid=0x{:016x}, type=0x{:04x}, payload_len={}, hex={:02x?}",
        packet.len(),
        response_port,
        caller_tid,
        msg_type,
        payload_len,
        &packet[..core::cmp::min(packet.len(), SOCKET_API_ENVELOPE_HEADER_LEN)]
    );
    EnvelopeDecode::Malformed
}

/// Open the virtio NIC device files, retrying until the VFS provider is ready.
fn open_nic_device() -> (u32, u32, u32, [u8; 6], u32, bool) {
    let rx_path = alloc::format!("{}/rx", VIRTIO0_PATH);
    let tx_path = alloc::format!("{}/tx", VIRTIO0_PATH);
    let events_path = alloc::format!("{}/events", VIRTIO0_PATH);
    let mac_path = alloc::format!("{}/mac", VIRTIO0_PATH);
    let mtu_path = alloc::format!("{}/mtu", VIRTIO0_PATH);

    loop {
        let rx_fd = match vfs_open(&rx_path, O_RDONLY | O_NONBLOCK) {
            Ok(fd) => fd,
            Err(_) => {
                stem::time::sleep_ms(100);
                continue;
            }
        };

        let tx_fd = match vfs_open(&tx_path, O_WRONLY) {
            Ok(fd) => fd,
            Err(e) => {
                warn!("NETD: Failed to open {}: {:?}", tx_path, e);
                let _ = vfs_close(rx_fd);
                stem::time::sleep_ms(100);
                continue;
            }
        };

        let events_fd = match vfs_open(&events_path, O_RDONLY | O_NONBLOCK) {
            Ok(fd) => fd,
            Err(e) => {
                warn!("NETD: Failed to open {}: {:?}", events_path, e);
                let _ = vfs_close(rx_fd);
                let _ = vfs_close(tx_fd);
                stem::time::sleep_ms(100);
                continue;
            }
        };

        let mac = read_mac_file(&mac_path).unwrap_or([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
        let mtu = read_u32_file(&mtu_path).unwrap_or(1500);

        info!(
            "NETD: Opened VFS NIC device (rx={}, tx={}, events={}, mtu={})",
            rx_fd, tx_fd, events_fd, mtu
        );
        return (rx_fd, tx_fd, events_fd, mac, mtu, true);
    }
}

fn read_mac_file(path: &str) -> Option<[u8; 6]> {
    let mut buf = [0u8; 24];
    let n = read_file_bytes(path, &mut buf)?;
    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    parse_mac(s)
}

fn parse_mac(s: &str) -> Option<[u8; 6]> {
    let mut mac = [0u8; 6];
    let mut count = 0usize;
    for (i, hex) in s.split(':').enumerate() {
        if i >= 6 {
            return None;
        }
        mac[i] = u8::from_str_radix(hex.trim(), 16).ok()?;
        count += 1;
    }
    if count != 6 {
        return None;
    }
    Some(mac)
}

fn read_u32_file(path: &str) -> Option<u32> {
    let mut buf = [0u8; 16];
    let n = read_file_bytes(path, &mut buf)?;
    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    s.parse().ok()
}

fn read_file_bytes(path: &str, buf: &mut [u8]) -> Option<usize> {
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let result = vfs_read(fd, buf).ok();
    let _ = vfs_close(fd);
    result
}
