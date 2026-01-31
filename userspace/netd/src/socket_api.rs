//! Socket API module for netd
//!
//! Provides a high-level socket API over IPC ports that allows applications
//! to perform TCP operations without directly managing the TCP/IP stack.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use smoltcp::iface::{Interface, SocketHandle, SocketSet};
use smoltcp::socket::tcp::{Socket as TcpSocket, SocketBuffer, State as TcpState};
use smoltcp::time::Duration;
use smoltcp::wire::{IpAddress, IpEndpoint, IpListenEndpoint, Ipv4Address};

use crate::ipc_device::IpcNicDevice;
use stem::{info, warn};

// Socket API message types
pub const MSG_TCP_CONNECT: u16 = 0x0200;
pub const MSG_TCP_SEND: u16 = 0x0201;
pub const MSG_TCP_RECV: u16 = 0x0202;
pub const MSG_TCP_CLOSE: u16 = 0x0203;
pub const MSG_TCP_LISTEN: u16 = 0x0204;
pub const MSG_TCP_ACCEPT: u16 = 0x0205;

// Response types
pub const RESP_OK: u16 = 0x0000;
pub const RESP_ERROR: u16 = 0x0001;
pub const RESP_HANDLE: u16 = 0x0002;
pub const RESP_DATA: u16 = 0x0003;
pub const RESP_ACCEPT: u16 = 0x0004;
pub const RESP_EMPTY: u16 = 0x0005;

/// A managed TCP socket
struct ManagedSocket {
    handle: SocketHandle,
    /// True if this is a listening socket
    is_listener: bool,
    /// Port for listeners
    listen_port: Option<u16>,
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
}

impl SocketApi {
    pub fn new() -> Self {
        Self {
            next_handle: 1,
            sockets: BTreeMap::new(),
            pending_accepts: BTreeMap::new(),
            pending_removal: Vec::new(),
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
                is_listener: true,
                listen_port: Some(port),
            },
        );
        self.pending_accepts.insert(api_handle, Vec::new());

        info!("SOCKET_API: Listening on port {}, handle={}", port, api_handle);
        encode_handle(api_handle)
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
                info!(
                    "SOCKET_API: TCP_ACCEPT returning connection handle={} from {}:{}",
                    conn_handle, remote_ip, remote_port
                );
                return encode_accept(conn_handle, remote_ip, remote_port);
            }
        }

        // No pending connections - check if listener socket has a connection ready
        let (listener_socket_handle, listen_port) = match self.sockets.get(&listen_handle) {
            Some(s) if s.is_listener => (s.handle, s.listen_port.unwrap_or(80)),
            _ => return encode_error(),
        };

        let socket = socket_set.get_mut::<TcpSocket>(listener_socket_handle);
        
        // Check socket state - if it's established, we have a connection
        if socket.state() == TcpState::Established {
            // The listener socket itself became the connection
            // We need to:
            // 1. Mark this listener as no longer a listener (it's now a connection)
            // 2. Create a new listener socket on the same port
            // 3. Return a NEW handle for the connection
            
            let remote = socket.remote_endpoint();
            if let Some(ep) = remote {
                let remote_ip = match ep.addr {
                    IpAddress::Ipv4(ip) => ip,
                    _ => Ipv4Address::new(0, 0, 0, 0),
                };
                let remote_port = ep.port;

                info!(
                    "SOCKET_API: Connection established from {}:{} on listener {}",
                    remote_ip, remote_port, listen_handle
                );

                // Create a new handle for this connection (reusing the same socket)
                let conn_handle = self.alloc_handle();
                
                // Update the managed socket entry: it's now a connection, not a listener
                if let Some(managed) = self.sockets.get_mut(&listen_handle) {
                    managed.is_listener = false;
                    managed.listen_port = None;
                }
                
                // Move the socket to the new connection handle
                if let Some(mut old_managed) = self.sockets.remove(&listen_handle) {
                    old_managed.is_listener = false;
                    old_managed.listen_port = None;
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
                    // Connection still works, but no more accepts possible
                } else {
                    let new_socket_handle = socket_set.add(new_listener);
                    // Reuse the original listen_handle for the new listener
                    self.sockets.insert(
                        listen_handle,
                        ManagedSocket {
                            handle: new_socket_handle,
                            is_listener: true,
                            listen_port: Some(listen_port),
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

    /// Handle a TCP_SEND request
    pub fn handle_send<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        handle: u32,
        data: &[u8],
    ) -> Vec<u8> {
        let managed = match self.sockets.get(&handle) {
            Some(s) => s,
            None => return encode_error(),
        };

        let socket = socket_set.get_mut::<TcpSocket>(managed.handle);

        if !socket.can_send() {
            return encode_send_result(0);
        }

        match socket.send_slice(data) {
            Ok(sent) => {
                info!("SOCKET_API: TCP_SEND handle={} sent {} bytes", handle, sent);
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
            Some(s) => s,
            None => return encode_error(),
        };

        let socket = socket_set.get_mut::<TcpSocket>(managed.handle);

        if !socket.can_recv() {
            return encode_data(&[]);
        }

        let mut buf = Vec::new();
        buf.resize(max_len as usize, 0u8);

        match socket.recv_slice(&mut buf) {
            Ok(len) => {
                buf.truncate(len);
                info!("SOCKET_API: TCP_RECV handle={} got {} bytes", handle, len);
                encode_data(&buf)
            }
            Err(e) => {
                warn!("SOCKET_API: TCP_RECV error: {:?}", e);
                encode_data(&[])
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
            // Just close the socket - don't remove it yet
            // The socket needs to remain in the set so smoltcp can:
            // 1. Flush remaining TX data
            // 2. Complete the TCP FIN handshake
            let socket = socket_set.get_mut::<TcpSocket>(managed.handle);
            socket.close();
            info!("SOCKET_API: TCP_CLOSE handle={} (initiating close)", handle);
            
            // Track this socket for later removal once it reaches Closed state
            self.pending_removal.push(managed.handle);
        }
        // Remove from our tracking map so future operations fail
        self.sockets.remove(&handle);
        self.pending_accepts.remove(&handle);
        encode_ok()
    }

    /// Garbage collect closed sockets from the SocketSet
    /// Call this periodically from the main loop to reclaim socket slots
    pub fn gc_closed_sockets<'a>(&mut self, socket_set: &mut SocketSet<'a>) {
        // Retain only sockets that are NOT yet in Closed state
        self.pending_removal.retain(|&socket_handle| {
            let socket = socket_set.get_mut::<TcpSocket>(socket_handle);
            if socket.state() == TcpState::Closed {
                // Socket is fully closed, remove it from the set
                socket_set.remove(socket_handle);
                info!("SOCKET_API: GC removed closed socket");
                false // Remove from pending_removal
            } else {
                true // Keep in pending_removal, check again later
            }
        });
    }

    /// Process an incoming API message
    pub fn process_message<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        msg: &[u8],
        rx_storage: &'a mut [u8],
        tx_storage: &'a mut [u8],
    ) -> Vec<u8> {
        if msg.len() < 2 {
            return encode_error();
        }

        let msg_type = u16::from_le_bytes([msg[0], msg[1]]);

        match msg_type {
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
