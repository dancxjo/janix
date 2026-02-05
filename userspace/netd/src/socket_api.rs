//! Socket API module for netd
//!
//! Provides a high-level socket API over IPC ports that allows applications
//! to perform TCP/UDP operations without directly managing the TCP/IP stack.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use smoltcp::iface::{Interface, SocketHandle, SocketSet};
use smoltcp::socket::tcp::{Socket as TcpSocket, SocketBuffer, State as TcpState};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{IpAddress, IpEndpoint, IpListenEndpoint, Ipv4Address};

use crate::ipc_device::IpcNicDevice;
use crate::dns;
use stem::{info, trace, warn};

// Socket API message types
pub const MSG_TCP_CONNECT: u16 = 0x0200;
pub const MSG_TCP_SEND: u16 = 0x0201;
pub const MSG_TCP_RECV: u16 = 0x0202;
pub const MSG_TCP_CLOSE: u16 = 0x0203;
pub const MSG_TCP_LISTEN: u16 = 0x0204;
pub const MSG_TCP_ACCEPT: u16 = 0x0205;

pub const MSG_UDP_BIND: u16 = 0x0300;
pub const MSG_UDP_SEND_TO: u16 = 0x0301;
pub const MSG_UDP_RECV_FROM: u16 = 0x0302;
pub const MSG_NET_JOIN_MULTICAST: u16 = 0x0400;

pub const MSG_DNS_QUERY: u16 = 0x0500;

// Response types
pub const RESP_OK: u16 = 0x0000;
pub const RESP_ERROR: u16 = 0x0001;
pub const RESP_HANDLE: u16 = 0x0002;
pub const RESP_DATA: u16 = 0x0003;
pub const RESP_ACCEPT: u16 = 0x0004;
pub const RESP_EMPTY: u16 = 0x0005;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SocketType {
    Tcp,
    Udp,
}

/// A managed socket
struct ManagedSocket {
    handle: SocketHandle,
    kind: SocketType,
    /// True if this is a listening socket (TCP only)
    is_listener: bool,
    /// Port for listeners (TCP) or local bind port (UDP)
    port: Option<u16>,
}

/// Socket API manager
pub struct SocketApi {
    /// Next handle ID to assign
    next_handle: u32,
    /// Map of API handles to managed sockets
    sockets: BTreeMap<u32, ManagedSocket>,
    /// Pending accepted connections (listen_handle -> Vec<(conn_handle, remote_ip, remote_port)>)
    pending_accepts: BTreeMap<u32, Vec<(u32, Ipv4Address, u16)>>,
    /// Socket handles pending removal from SocketSet (after TCP close completes)
    pending_removal: Vec<SocketHandle>,
    /// Reusable scratch buffer for receive operations
    recv_scratch: Vec<u8>,
}

impl SocketApi {
    pub fn new() -> Self {
        Self {
            next_handle: 1,
            sockets: BTreeMap::new(),
            pending_accepts: BTreeMap::new(),
            pending_removal: Vec::new(),
            recv_scratch: Vec::with_capacity(32768), // Large enough for most frames
        }
    }

    /// Allocate a new handle ID
    fn alloc_handle(&mut self) -> u32 {
        let h = self.next_handle;
        self.next_handle = self.next_handle.wrapping_add(1);
        if self.next_handle == 0 {
            self.next_handle = 1;
        }
        h
    }

    /// Handle a TCP_LISTEN request
    pub fn handle_listen<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        port: u16,
        _backlog: u16,
        rx_storage: &'a mut [u8],
        tx_storage: &'a mut [u8],
    ) -> Vec<u8> {
        info!("SOCKET_API: TCP_LISTEN on port {}", port);

        let rx_buffer = SocketBuffer::new(rx_storage);
        let tx_buffer = SocketBuffer::new(tx_storage);
        let mut socket = TcpSocket::new(rx_buffer, tx_buffer);

        // Set up as listening socket
        let endpoint = IpListenEndpoint::from(port);
        if let Err(e) = socket.listen(endpoint) {
            warn!("SOCKET_API: Failed to listen: {:?}", e);
            return encode_error();
        }

        let socket_handle = socket_set.add(socket);
        let api_handle = self.alloc_handle();

        self.sockets.insert(
            api_handle,
            ManagedSocket {
                handle: socket_handle,
                kind: SocketType::Tcp,
                is_listener: true,
                port: Some(port),
            },
        );
        self.pending_accepts.insert(api_handle, Vec::new());

        info!("SOCKET_API: Listening on port {}, handle={}", port, api_handle);
        encode_handle(api_handle)
    }

    /// Handle a TCP_CONNECT request
    pub fn handle_connect<'a, D: smoltcp::phy::Device>(
        &mut self,
        iface: &mut Interface,
        _device: &mut D,
        socket_set: &mut SocketSet<'a>,
        remote_ip: Ipv4Address,
        remote_port: u16,
        rx_storage: &'a mut [u8],
        tx_storage: &'a mut [u8],
    ) -> Vec<u8> {
        info!("SOCKET_API: TCP_CONNECT to {}:{}", remote_ip, remote_port);

        let rx_buffer = SocketBuffer::new(rx_storage);
        let tx_buffer = SocketBuffer::new(tx_storage);
        let mut socket = TcpSocket::new(rx_buffer, tx_buffer);

        let endpoint = IpEndpoint::new(IpAddress::Ipv4(remote_ip), remote_port);
        // Ephemeral port generation is handled by smoltcp if local_port is not specified (unspecified endpoint)
        let local_port = 49152 + (self.next_handle as u16 % 16384);

        if let Err(e) = socket.connect(iface.context(), endpoint, local_port) {
            warn!("SOCKET_API: Failed to connect: {:?}", e);
            return encode_error();
        }

        let socket_handle = socket_set.add(socket);
        let api_handle = self.alloc_handle();

        self.sockets.insert(
            api_handle,
            ManagedSocket {
                handle: socket_handle,
                kind: SocketType::Tcp,
                is_listener: false,
                port: Some(local_port),
            },
        );

        info!("SOCKET_API: Connected handle={} local_port={}", api_handle, local_port);
        encode_handle(api_handle)
    }

    /// Handle a DNS query
    pub fn handle_dns_query(
        &mut self,
        iface: &mut Interface,
        device: &mut IpcNicDevice,
        dns_server: Ipv4Address,
        hostname: &str,
    ) -> Vec<u8> {
        match dns::lookup_a(iface, device, dns_server, hostname) {
            Ok(ip) => {
                let mut v = Vec::with_capacity(6);
                v.extend_from_slice(&RESP_DATA.to_le_bytes());
                v.extend_from_slice(ip.as_bytes());
                v
            }
            Err(_) => encode_error(),
        }
    }

    /// Handle a TCP_ACCEPT request (non-blocking)
    pub fn handle_accept<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        listen_handle: u32,
        rx_storage: &'a mut [u8],
        tx_storage: &'a mut [u8],
    ) -> Vec<u8> {
        // Check if we have a pending accepted connection
        if let Some(pending) = self.pending_accepts.get_mut(&listen_handle) {
            if let Some((conn_handle, remote_ip, remote_port)) = pending.pop() {
                debug!(
                    "SOCKET_API: TCP_ACCEPT returning connection handle={} from {}:{}",
                    conn_handle, remote_ip, remote_port
                );
                return encode_accept(conn_handle, remote_ip, remote_port);
            }
        }

        // No pending connections - check if listener socket has a connection ready
        let (listener_socket_handle, listen_port) = match self.sockets.get(&listen_handle) {
            Some(s) if s.is_listener && s.kind == SocketType::Tcp => (s.handle, s.port.unwrap_or(80)),
            _ => return encode_error(),
        };

        let socket = socket_set.get_mut::<TcpSocket>(listener_socket_handle);
        
        // Check socket state - if it's established, we have a connection
        if socket.state() == TcpState::Established {
            let remote = socket.remote_endpoint();
            if let Some(ep) = remote {
                let remote_ip = match ep.addr {
                    IpAddress::Ipv4(ip) => ip,
                    _ => Ipv4Address::new(0, 0, 0, 0),
                };
                let remote_port = ep.port;

                debug!(
                    "SOCKET_API: Connection established from {}:{} on listener {}",
                    remote_ip, remote_port, listen_handle
                );

                // Create a new handle for this connection (reusing the same socket)
                let conn_handle = self.alloc_handle();
                
                // Update the managed socket entry: it's now a connection, not a listener
                if let Some(managed) = self.sockets.get_mut(&listen_handle) {
                    managed.is_listener = false;
                    managed.port = None;
                }
                
                // Move the socket to the new connection handle
                if let Some(mut old_managed) = self.sockets.remove(&listen_handle) {
                    old_managed.is_listener = false;
                    old_managed.port = None;
                    self.sockets.insert(conn_handle, old_managed);
                }
                
                // Remove old pending_accepts entry
                self.pending_accepts.remove(&listen_handle);
                
                // Create a new listener socket on the same port
                let rx_buffer = SocketBuffer::new(rx_storage);
                let tx_buffer = SocketBuffer::new(tx_storage);
                let mut new_listener = TcpSocket::new(rx_buffer, tx_buffer);
                
                let endpoint = IpListenEndpoint::from(listen_port);
                if let Err(e) = new_listener.listen(endpoint) {
                    warn!("SOCKET_API: Failed to respawn listener: {:?}", e);
                } else {
                    let new_socket_handle = socket_set.add(new_listener);
                    // Reuse the original listen_handle for the new listener
                    self.sockets.insert(
                        listen_handle,
                        ManagedSocket {
                            handle: new_socket_handle,
                            kind: SocketType::Tcp,
                            is_listener: true,
                            port: Some(listen_port),
                        },
                    );
                    self.pending_accepts.insert(listen_handle, Vec::new());
                    info!("SOCKET_API: Respawned listener on port {} handle={}", listen_port, listen_handle);
                }

                return encode_accept(conn_handle, remote_ip, remote_port);
            }
        }

        // No connection ready
        encode_empty()
    }

    /// Handle a UDP_BIND request
    pub fn handle_udp_bind<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        port: u16,
        rx_metadata_storage: &'a mut [smoltcp::socket::udp::PacketMetadata],
        rx_payload_storage: &'a mut [u8],
        tx_metadata_storage: &'a mut [smoltcp::socket::udp::PacketMetadata],
        tx_payload_storage: &'a mut [u8],
    ) -> Vec<u8> {
        info!("SOCKET_API: UDP_BIND on port {}", port);

        let rx_buffer = smoltcp::socket::udp::PacketBuffer::new(rx_metadata_storage, rx_payload_storage);
        let tx_buffer = smoltcp::socket::udp::PacketBuffer::new(tx_metadata_storage, tx_payload_storage);
        let mut socket = smoltcp::socket::udp::Socket::new(rx_buffer, tx_buffer);

        if let Err(e) = socket.bind(port) {
            warn!("SOCKET_API: Failed to bind UDP: {:?}", e);
            return encode_error();
        }

        let socket_handle = socket_set.add(socket);
        let api_handle = self.alloc_handle();

        self.sockets.insert(
            api_handle,
            ManagedSocket {
                handle: socket_handle,
                kind: SocketType::Udp,
                is_listener: false,
                port: Some(port),
            },
        );

        info!("SOCKET_API: Bound UDP on port {}, handle={}", port, api_handle);
        encode_handle(api_handle)
    }

    /// Handle a UDP_SEND_TO request
    pub fn handle_udp_send_to<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        handle: u32,
        remote_ip: Ipv4Address,
        remote_port: u16,
        data: &[u8],
    ) -> Vec<u8> {
        let managed = match self.sockets.get(&handle) {
            Some(s) if s.kind == SocketType::Udp => s,
            _ => return encode_error(),
        };

        let socket = socket_set.get_mut::<smoltcp::socket::udp::Socket>(managed.handle);
        let endpoint = IpEndpoint::new(IpAddress::Ipv4(remote_ip), remote_port);

        if !socket.can_send() {
            return encode_send_result(0);
        }

        match socket.send_slice(data, endpoint) {
            Ok(_) => {
                trace!("SOCKET_API: UDP_SEND_TO handle={} sent {} bytes to {}:{}", handle, data.len(), remote_ip, remote_port);
                encode_send_result(data.len() as u16)
            }
            Err(e) => {
                warn!("SOCKET_API: UDP_SEND_TO error: {:?}", e);
                encode_send_result(0)
            }
        }
    }

    /// Handle a UDP_RECV_FROM request
    pub fn handle_udp_recv_from<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        handle: u32,
    ) -> Vec<u8> {
        let managed = match self.sockets.get(&handle) {
            Some(s) if s.kind == SocketType::Udp => s,
            _ => return encode_error(),
        };

        let socket = socket_set.get_mut::<smoltcp::socket::udp::Socket>(managed.handle);

        if !socket.can_recv() {
            return encode_empty();
        }

        match socket.recv() {
            Ok((data, endpoint)) => {
                let remote_ip = match endpoint.endpoint.addr {
                    IpAddress::Ipv4(ip) => ip,
                    _ => Ipv4Address::new(0, 0, 0, 0),
                };
                let remote_port = endpoint.endpoint.port;
                
                trace!("SOCKET_API: UDP_RECV_FROM handle={} got {} bytes from {}:{}", handle, data.len(), remote_ip, remote_port);
                encode_udp_data(remote_ip, remote_port, data)
            }
            Err(e) => {
                warn!("SOCKET_API: UDP_RECV_FROM error: {:?}", e);
                encode_empty()
            }
        }
    }

    /// Handle a NET_JOIN_MULTICAST request
    pub fn handle_multicast_join<'a, D: smoltcp::phy::Device>(
        &mut self,
        iface: &mut Interface,
        device: &mut D,
        multicast_ip: Ipv4Address,
    ) -> Vec<u8> {
        info!("SOCKET_API: Joining multicast group {}", multicast_ip);
        let now = Instant::from_millis(stem::time::now().as_millis() as i64);
        iface.join_multicast_group(device, IpAddress::Ipv4(multicast_ip), now).ok();
        encode_ok()
    }

    /// Handle a TCP_SEND request
    pub fn handle_send<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        handle: u32,
        data: &[u8],
    ) -> Vec<u8> {
        let managed = match self.sockets.get(&handle) {
            Some(s) if s.kind == SocketType::Tcp => s,
            _ => return encode_error(),
        };

        let socket = socket_set.get_mut::<TcpSocket>(managed.handle);

        if !socket.can_send() {
            return encode_send_result(0);
        }

        match socket.send_slice(data) {
            Ok(sent) => {
                trace!("SOCKET_API: TCP_SEND handle={} sent {} bytes", handle, sent);
                encode_send_result(sent as u16)
            }
            Err(e) => {
                warn!("SOCKET_API: TCP_SEND error: {:?}", e);
                encode_send_result(0)
            }
        }
    }

    /// Handle a TCP_RECV request (non-blocking)
    pub fn handle_recv<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        handle: u32,
        max_len: u16,
    ) -> Vec<u8> {
        let managed = match self.sockets.get(&handle) {
            Some(s) if s.kind == SocketType::Tcp => s,
            _ => return encode_error(),
        };

        let socket = socket_set.get_mut::<TcpSocket>(managed.handle);

        // Use reusable scratch buffer
        let max_len = max_len as usize;
        if self.recv_scratch.len() < max_len {
            self.recv_scratch.resize(max_len, 0);
        }

        match socket.recv_slice(&mut self.recv_scratch[..max_len]) {
            Ok(len) => {
                trace!("SOCKET_API: TCP_RECV handle={} got {} bytes", handle, len);
                encode_data(&self.recv_scratch[..len])
            }
            Err(_) => {
                // If we can't receive, check if it's because the socket is empty or closed
                if socket.state() == TcpState::Established {
                    // Still established but no data
                    encode_empty()
                } else if socket.may_recv() {
                    // Still potentially receiving (e.g. FIN received but buffer not empty, 
                    // though recv_slice would have returned data in that case)
                    encode_empty()
                } else {
                    // Socket closed or EOF reached
                    encode_error()
                }
            }
        }
    }

    /// Handle a TCP_CLOSE request
    pub fn handle_close<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        handle: u32,
    ) -> Vec<u8> {
        if let Some(managed) = self.sockets.get(&handle) {
            if managed.kind == SocketType::Tcp {
                let socket = socket_set.get_mut::<TcpSocket>(managed.handle);
                socket.close();
                debug!("SOCKET_API: TCP_CLOSE handle={} (initiating close)", handle);
                self.pending_removal.push(managed.handle);
            } else {
                // UDP sockets can be removed immediately
                socket_set.remove(managed.handle);
                debug!("SOCKET_API: UDP close handle={} (removed)", handle);
            }
        }
        self.sockets.remove(&handle);
        self.pending_accepts.remove(&handle);
        encode_ok()
    }

    /// Garbage collect closed sockets from the SocketSet
    pub fn gc_closed_sockets<'a>(&mut self, socket_set: &mut SocketSet<'a>) {
        self.pending_removal.retain(|&socket_handle| {
            // Check if it's a TCP socket
            // In smoltcp 0.11, we can't easily check type without trying to get it
            // but we know pending_removal ONLY contains TCP handles that we called close() on.
            let socket = socket_set.get_mut::<TcpSocket>(socket_handle);
            if socket.state() == TcpState::Closed {
                socket_set.remove(socket_handle);
                debug!("SOCKET_API: GC removed explicitly closed TCP socket");
                false
            } else {
                true
            }
        });

        let mut tracked_handles = alloc::collections::BTreeSet::new();
        for managed in self.sockets.values() {
            tracked_handles.insert(managed.handle);
        }

        let all_handles: Vec<SocketHandle> = socket_set.iter().map(|(h, _)| h).collect();
        let mut to_remove = Vec::new();
        for handle in all_handles {
            if tracked_handles.contains(&handle) {
                continue;
            }
            
            // For now, only GC TCP sockets that reached Closed state
            // (e.g., remotely closed ones we haven't handled yet)
            // This is a bit tricky with smoltcp's type system in a loop,
            // so we'll stick to the explicit pending_removal for now as primary GC.
        }

        for handle in to_remove {
            socket_set.remove(handle);
            debug!("SOCKET_API: GC removed orphaned socket");
        }
    }

    /// Process an incoming API message
    pub fn process_message<'a>(
        &mut self,
        iface: &mut Interface,
        device: &mut IpcNicDevice,
        socket_set: &mut SocketSet<'a>,
        msg: &[u8],
        rx_storage: &'a mut [u8],
        tx_storage: &'a mut [u8],
        dns_server: Option<Ipv4Address>,
    ) -> Vec<u8> {
        if msg.len() < 2 {
            return encode_error();
        }

        let msg_type = u16::from_le_bytes([msg[0], msg[1]]);

        match msg_type {
            MSG_TCP_CONNECT => {
                if msg.len() < 8 {
                    return encode_error();
                }
                let ip = Ipv4Address::from_bytes(&msg[2..6]);
                let port = u16::from_le_bytes([msg[6], msg[7]]);
                self.handle_connect(iface, device, socket_set, ip, port, rx_storage, tx_storage)
            }
            MSG_TCP_LISTEN => {
                if msg.len() < 6 {
                    return encode_error();
                }
                let port = u16::from_le_bytes([msg[2], msg[3]]);
                let backlog = u16::from_le_bytes([msg[4], msg[5]]);
                self.handle_listen(socket_set, port, backlog, rx_storage, tx_storage)
            }
            MSG_TCP_ACCEPT => {
                if msg.len() < 6 {
                    return encode_error();
                }
                let listen_handle = u32::from_le_bytes([msg[2], msg[3], msg[4], msg[5]]);
                self.handle_accept(socket_set, listen_handle, rx_storage, tx_storage)
            }
            MSG_TCP_SEND => {
                if msg.len() < 6 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([msg[2], msg[3], msg[4], msg[5]]);
                let data = &msg[6..];
                self.handle_send(socket_set, handle, data)
            }
            MSG_TCP_RECV => {
                if msg.len() < 8 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([msg[2], msg[3], msg[4], msg[5]]);
                let max_len = u16::from_le_bytes([msg[6], msg[7]]);
                self.handle_recv(socket_set, handle, max_len)
            }
            MSG_TCP_CLOSE => {
                if msg.len() < 6 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([msg[2], msg[3], msg[4], msg[5]]);
                self.handle_close(socket_set, handle)
            }
            MSG_UDP_BIND => {
                if msg.len() < 4 {
                    return encode_error();
                }
                let port = u16::from_le_bytes([msg[2], msg[3]]);
                unsafe {
                    let (rx_meta, rx_payload) = split_packet_buffer(rx_storage);
                    let (tx_meta, tx_payload) = split_packet_buffer(tx_storage);
                    self.handle_udp_bind(socket_set, port, rx_meta, rx_payload, tx_meta, tx_payload)
                }
            }
            MSG_UDP_SEND_TO => {
                if msg.len() < 12 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([msg[2], msg[3], msg[4], msg[5]]);
                let ip = Ipv4Address::from_bytes(&msg[6..10]);
                let port = u16::from_le_bytes([msg[10], msg[11]]);
                let data = &msg[12..];
                self.handle_udp_send_to(socket_set, handle, ip, port, data)
            }
            MSG_UDP_RECV_FROM => {
                if msg.len() < 6 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([msg[2], msg[3], msg[4], msg[5]]);
                self.handle_udp_recv_from(socket_set, handle)
            }
            MSG_NET_JOIN_MULTICAST => {
                if msg.len() < 6 {
                    return encode_error();
                }
                let ip = Ipv4Address::from_bytes(&msg[2..6]);
                self.handle_multicast_join(iface, device, ip)
            }
            MSG_DNS_QUERY => {
                if let Some(dns_server) = dns_server {
                    if let Ok(hostname) = core::str::from_utf8(&msg[2..]) {
                        self.handle_dns_query(iface, device, dns_server, hostname)
                    } else {
                        encode_error()
                    }
                } else {
                    encode_error()
                }
            }
            _ => {
                warn!("SOCKET_API: Unknown message type 0x{:04x}", msg_type);
                encode_error()
            }
        }
    }
}

// Encoding helpers
fn encode_ok() -> Vec<u8> {
    RESP_OK.to_le_bytes().to_vec()
}

fn encode_error() -> Vec<u8> {
    RESP_ERROR.to_le_bytes().to_vec()
}

fn encode_handle(handle: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(6);
    v.extend_from_slice(&RESP_HANDLE.to_le_bytes());
    v.extend_from_slice(&handle.to_le_bytes());
    v
}

fn encode_send_result(bytes_sent: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(4);
    v.extend_from_slice(&RESP_OK.to_le_bytes());
    v.extend_from_slice(&bytes_sent.to_le_bytes());
    v
}

fn encode_data(data: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(2 + data.len());
    v.extend_from_slice(&RESP_DATA.to_le_bytes());
    v.extend_from_slice(data);
    v
}

fn encode_udp_data(remote_ip: Ipv4Address, remote_port: u16, data: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(2 + 4 + 2 + data.len());
    v.extend_from_slice(&RESP_DATA.to_le_bytes());
    v.extend_from_slice(remote_ip.as_bytes());
    v.extend_from_slice(&remote_port.to_le_bytes());
    v.extend_from_slice(data);
    v
}

fn encode_accept(conn_handle: u32, remote_ip: Ipv4Address, remote_port: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(12);
    v.extend_from_slice(&RESP_ACCEPT.to_le_bytes());
    v.extend_from_slice(&conn_handle.to_le_bytes());
    let ip_bytes = remote_ip.as_bytes();
    v.extend_from_slice(ip_bytes);
    v.extend_from_slice(&remote_port.to_le_bytes());
    v
}

fn encode_empty() -> Vec<u8> {
    RESP_EMPTY.to_le_bytes().to_vec()
}

/// Helper to split a large buffer into packet metadata and payload
unsafe fn split_packet_buffer(buf: &mut [u8]) -> (&mut [smoltcp::socket::udp::PacketMetadata], &mut [u8]) {
    use core::mem::size_of;
    let meta_count = 8;
    let meta_size = size_of::<smoltcp::socket::udp::PacketMetadata>() * meta_count;
    
    let (meta_bytes, payload) = buf.split_at_mut(meta_size);
    let meta_ptr = meta_bytes.as_mut_ptr() as *mut smoltcp::socket::udp::PacketMetadata;
    let meta = core::slice::from_raw_parts_mut(meta_ptr, meta_count);
    
    // Initialize metadata
    for m in meta.iter_mut() {
        *m = smoltcp::socket::udp::PacketMetadata::EMPTY;
    }
    
    (meta, payload)
}
