//! Socket API module for netd
//!
//! Provides a high-level socket API over IPC ports that allows applications
//! to perform TCP/UDP operations without directly managing the TCP/IP stack.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::vec::Vec;
use smoltcp::iface::{Interface, SocketHandle, SocketSet};
use smoltcp::socket::tcp::{Socket as TcpSocket, SocketBuffer, State as TcpState};
use smoltcp::time::Instant;
use smoltcp::wire::{IpAddress, IpEndpoint, IpListenEndpoint, Ipv4Address};

use crate::dns;
use crate::ipc_device::IpcNicDevice;
use stem::net;
use stem::net::conn_graph;
use stem::thing::ThingId;
use stem::{debug, info, trace, warn};

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

pub fn is_known_msg_type(msg_type: u16) -> bool {
    matches!(
        msg_type,
        MSG_TCP_CONNECT
            | MSG_TCP_SEND
            | MSG_TCP_RECV
            | MSG_TCP_CLOSE
            | MSG_TCP_LISTEN
            | MSG_TCP_ACCEPT
            | MSG_UDP_BIND
            | MSG_UDP_SEND_TO
            | MSG_UDP_RECV_FROM
            | MSG_NET_JOIN_MULTICAST
            | MSG_DNS_QUERY
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SocketType {
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Copy)]
struct EndpointV4 {
    ip: Ipv4Address,
    port: u16,
}

#[derive(Debug)]
struct ManagedSocket {
    handle: SocketHandle,
    kind: SocketType,
    is_listener: bool,
    local: Option<EndpointV4>,
    remote: Option<EndpointV4>,
    owner_tid: u64,
    socket_node: ThingId,
    connection_node: Option<ThingId>,
    bytes_tx: u64,
    bytes_rx: u64,
    packets_tx: u64,
    packets_rx: u64,
    last_error_sym: u64,
    last_seen_ms: u64,
    /// True when graph nodes need (re-)initialization
    graph_dirty: bool,
    /// Stable key for graph node creation (consumed on first flush)
    socket_key: Option<u64>,
}

impl ManagedSocket {
    fn proto_str(&self) -> &'static str {
        match self.kind {
            SocketType::Tcp => "tcp",
            SocketType::Udp => "udp",
        }
    }
}

/// Socket API manager
pub struct SocketApi {
    /// Next handle ID to assign
    next_handle: u32,
    /// Monotonic socket identity for graph-stable keys
    next_socket_id: u64,
    /// Map of API handles to managed sockets
    sockets: BTreeMap<u32, ManagedSocket>,
    /// Pending accepted connections (listen_handle -> Vec<(conn_handle, remote_ip, remote_port)>)
    pending_accepts: BTreeMap<u32, Vec<(u32, Ipv4Address, u16)>>,
    /// Socket handles pending removal from SocketSet (after TCP close completes)
    pending_removal: Vec<SocketHandle>,
    /// Reusable scratch buffer for receive operations
    recv_scratch: Vec<u8>,
    /// Next graph stats flush timestamp
    next_graph_flush_ms: u64,
}

impl SocketApi {
    pub fn new() -> Self {
        Self {
            next_handle: 1,
            next_socket_id: 1,
            sockets: BTreeMap::new(),
            pending_accepts: BTreeMap::new(),
            pending_removal: Vec::new(),
            recv_scratch: Vec::with_capacity(32768), // Large enough for most frames
            // Defer first graph flush to avoid blocking Root IPC during early boot
            next_graph_flush_ms: (stem::time::now().as_millis() as u64).saturating_add(60_000),
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

    fn alloc_socket_id(&mut self) -> u64 {
        let id = self.next_socket_id;
        self.next_socket_id = self.next_socket_id.wrapping_add(1).max(1);
        id
    }

    fn now_ms() -> u64 {
        stem::time::now().as_millis() as u64
    }

    fn endpoint_ip_string(ep: EndpointV4) -> alloc::string::String {
        let b = ep.ip.as_bytes();
        format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3])
    }

    fn socket_key(owner_tid: u64, socket_id: u64, kind: SocketType) -> u64 {
        let proto_tag = match kind {
            SocketType::Tcp => 1u64,
            SocketType::Udp => 2u64,
        };
        conn_graph::hash64_u64s(&[owner_tid, socket_id, proto_tag])
    }

    fn connection_key(owner_tid: u64, local: EndpointV4, remote: EndpointV4, start_ms: u64) -> u64 {
        let l = local.ip.as_bytes();
        let r = remote.ip.as_bytes();
        let local_ip = u32::from_le_bytes([l[0], l[1], l[2], l[3]]) as u64;
        let remote_ip = u32::from_le_bytes([r[0], r[1], r[2], r[3]]) as u64;
        conn_graph::hash64_u64s(&[
            1, // tcp
            owner_tid,
            local_ip,
            local.port as u64,
            remote_ip,
            remote.port as u64,
            start_ms,
        ])
    }

    fn tcp_state_label(state: TcpState) -> &'static str {
        match state {
            TcpState::Closed => "closed",
            TcpState::Listen => "listen",
            TcpState::SynSent => "syn-sent",
            TcpState::SynReceived => "syn-received",
            TcpState::Established => "established",
            TcpState::FinWait1 => "fin-wait-1",
            TcpState::FinWait2 => "fin-wait-2",
            TcpState::CloseWait => "close-wait",
            TcpState::Closing => "closing",
            TcpState::LastAck => "last-ack",
            TcpState::TimeWait => "time-wait",
        }
    }

    fn socket_state_label(managed: &ManagedSocket, tcp_state: Option<TcpState>) -> &'static str {
        if managed.is_listener {
            return "listening";
        }
        match managed.kind {
            SocketType::Udp => {
                if managed.remote.is_some() {
                    "connected"
                } else if managed.local.is_some() {
                    "bound"
                } else {
                    "created"
                }
            }
            SocketType::Tcp => match tcp_state {
                Some(TcpState::Closed) => "closed",
                Some(TcpState::Listen) => "listening",
                Some(TcpState::Established)
                | Some(TcpState::FinWait1)
                | Some(TcpState::FinWait2)
                | Some(TcpState::CloseWait)
                | Some(TcpState::Closing)
                | Some(TcpState::LastAck)
                | Some(TcpState::TimeWait) => "connected",
                Some(TcpState::SynSent) | Some(TcpState::SynReceived) => "bound",
                None => {
                    if managed.local.is_some() {
                        "bound"
                    } else {
                        "created"
                    }
                }
            },
        }
    }

    fn set_last_error(managed: &mut ManagedSocket, text: &str) {
        managed.last_error_sym = stem::thing::sys::intern(text).ok().unwrap_or(0) as u64;
    }

    fn new_managed_socket(
        &mut self,
        socket_handle: SocketHandle,
        kind: SocketType,
        is_listener: bool,
        local: Option<EndpointV4>,
        owner_tid: u64,
        api_handle: u32,
        now_ms: u64,
    ) -> ManagedSocket {
        let socket_id = self.alloc_socket_id();
        let socket_key = Self::socket_key(owner_tid, socket_id, kind);

        // DEFERRED: All graph operations are lazy-initialized during flush_graph.
        // This prevents blocking Root service IPC from stalling the network I/O
        // hot path. The socket_node will be populated on the first flush_graph pass.
        let managed = ManagedSocket {
            handle: socket_handle,
            kind,
            is_listener,
            local,
            remote: None,
            owner_tid,
            socket_node: ThingId::default(), // Lazily populated in flush_graph
            connection_node: None,
            bytes_tx: 0,
            bytes_rx: 0,
            packets_tx: 0,
            packets_rx: 0,
            last_error_sym: 0,
            last_seen_ms: now_ms,
            graph_dirty: true, // Needs graph initialization
            socket_key: Some(socket_key),
        };

        managed
    }

    fn sync_local_edge(managed: &ManagedSocket) {
        if let Some(local) = managed.local {
            let ip = Self::endpoint_ip_string(local);
            if let Ok(local_id) =
                conn_graph::ensure_endpoint_node(managed.proto_str(), &ip, local.port)
            {
                net::ensure_edge(managed.socket_node, net::preds::SOCKET_HAS_LOCAL, local_id).ok();
            }
        }
    }

    fn sync_remote_edge(managed: &ManagedSocket) {
        if let Some(remote) = managed.remote {
            let ip = Self::endpoint_ip_string(remote);
            if let Ok(remote_id) =
                conn_graph::ensure_endpoint_node(managed.proto_str(), &ip, remote.port)
            {
                net::ensure_edge(
                    managed.socket_node,
                    net::preds::SOCKET_HAS_REMOTE,
                    remote_id,
                )
                .ok();
                if let Some(conn_id) = managed.connection_node {
                    net::ensure_edge(conn_id, net::preds::CONNECTION_PEER, remote_id).ok();
                }
            }
        }
    }

    fn ensure_tcp_connection(managed: &mut ManagedSocket, now_ms: u64, initial_state: &str) {
        if managed.kind != SocketType::Tcp || managed.connection_node.is_some() {
            return;
        }
        let (Some(local), Some(remote)) = (managed.local, managed.remote) else {
            return;
        };

        let conn_key = Self::connection_key(managed.owner_tid, local, remote, now_ms);
        let Ok(conn_id) = conn_graph::ensure_connection_node(conn_key) else {
            return;
        };

        conn_graph::set_sym_if_changed(conn_id, net::props::CONN_STATE, initial_state).ok();
        conn_graph::set_if_changed(conn_id, net::props::CONN_BYTES_TX, managed.bytes_tx).ok();
        conn_graph::set_if_changed(conn_id, net::props::CONN_BYTES_RX, managed.bytes_rx).ok();
        conn_graph::set_if_changed(conn_id, net::props::CONN_PACKETS_TX, managed.packets_tx).ok();
        conn_graph::set_if_changed(conn_id, net::props::CONN_PACKETS_RX, managed.packets_rx).ok();
        conn_graph::set_if_changed(conn_id, net::props::CONN_LAST_SEEN, now_ms).ok();

        net::ensure_edge(
            managed.socket_node,
            net::preds::SOCKET_HAS_CONNECTION,
            conn_id,
        )
        .ok();
        managed.connection_node = Some(conn_id);
        Self::sync_remote_edge(managed);
    }

    fn flush_managed_socket_tcp<'a>(
        managed: &mut ManagedSocket,
        socket_set: &mut SocketSet<'a>,
        now_ms: u64,
    ) {
        let socket = socket_set.get_mut::<TcpSocket>(managed.handle);
        let state = socket.state();
        conn_graph::set_sym_if_changed(
            managed.socket_node,
            net::props::SOCK_STATE,
            Self::socket_state_label(managed, Some(state)),
        )
        .ok();

        if let Some(conn_id) = managed.connection_node {
            conn_graph::set_sym_if_changed(
                conn_id,
                net::props::CONN_STATE,
                Self::tcp_state_label(state),
            )
            .ok();
            conn_graph::set_if_changed(conn_id, net::props::CONN_BYTES_TX, managed.bytes_tx).ok();
            conn_graph::set_if_changed(conn_id, net::props::CONN_BYTES_RX, managed.bytes_rx).ok();
            conn_graph::set_if_changed(conn_id, net::props::CONN_PACKETS_TX, managed.packets_tx)
                .ok();
            conn_graph::set_if_changed(conn_id, net::props::CONN_PACKETS_RX, managed.packets_rx)
                .ok();
            conn_graph::set_if_changed(conn_id, net::props::CONN_LAST_SEEN, now_ms).ok();
            if managed.last_error_sym != 0 {
                conn_graph::set_if_changed(
                    conn_id,
                    net::props::CONN_LAST_ERROR,
                    managed.last_error_sym,
                )
                .ok();
            }
        }
    }

    fn flush_managed_socket_udp(managed: &ManagedSocket, now_ms: u64) {
        conn_graph::set_sym_if_changed(
            managed.socket_node,
            net::props::SOCK_STATE,
            Self::socket_state_label(managed, None),
        )
        .ok();
        if let Some(conn_id) = managed.connection_node {
            conn_graph::set_if_changed(conn_id, net::props::CONN_BYTES_TX, managed.bytes_tx).ok();
            conn_graph::set_if_changed(conn_id, net::props::CONN_BYTES_RX, managed.bytes_rx).ok();
            conn_graph::set_if_changed(conn_id, net::props::CONN_PACKETS_TX, managed.packets_tx)
                .ok();
            conn_graph::set_if_changed(conn_id, net::props::CONN_PACKETS_RX, managed.packets_rx)
                .ok();
            conn_graph::set_if_changed(conn_id, net::props::CONN_LAST_SEEN, now_ms).ok();
            if managed.last_error_sym != 0 {
                conn_graph::set_if_changed(
                    conn_id,
                    net::props::CONN_LAST_ERROR,
                    managed.last_error_sym,
                )
                .ok();
            }
        }
    }

    pub fn flush_graph<'a>(&mut self, socket_set: &mut SocketSet<'a>, now_ms: u64) {
        if now_ms < self.next_graph_flush_ms {
            return;
        }
        self.next_graph_flush_ms = now_ms.saturating_add(10_000);

        let handles: Vec<u32> = self.sockets.keys().copied().collect();
        for api_handle in handles {
            let Some(managed) = self.sockets.get_mut(&api_handle) else {
                continue;
            };

            // Lazily initialize graph nodes for new sockets
            if managed.graph_dirty {
                if let Some(key) = managed.socket_key {
                    if let Ok(node) = conn_graph::ensure_socket_node(key) {
                        managed.socket_node = node;
                        conn_graph::set_sym_if_changed(
                            node,
                            net::props::SOCK_PROTO,
                            managed.proto_str(),
                        )
                        .ok();
                        conn_graph::set_sym_if_changed(
                            node,
                            net::props::SOCK_STATE,
                            if managed.is_listener {
                                "listening"
                            } else if managed.local.is_some() {
                                "bound"
                            } else {
                                "created"
                            },
                        )
                        .ok();
                        conn_graph::set_if_changed(node, net::props::SOCK_FD, api_handle as u64)
                            .ok();
                        conn_graph::set_if_changed(
                            node,
                            net::props::SOCK_PID,
                            managed.owner_tid,
                        )
                        .ok();
                        conn_graph::set_if_changed(
                            node,
                            net::props::SOCK_CREATED_AT,
                            managed.last_seen_ms,
                        )
                        .ok();

                        if managed.owner_tid != 0 {
                            if let Some(owner_node) =
                                conn_graph::find_thread_node_by_tid(managed.owner_tid)
                                    .ok()
                                    .flatten()
                            {
                                net::ensure_edge(
                                    owner_node,
                                    net::preds::PROC_OWNS_SOCKET,
                                    node,
                                )
                                .ok();
                            }
                        }
                    }
                }
                managed.graph_dirty = false;
                managed.socket_key = None; // Consumed
            }

            // Only sync graph edges if the socket node was successfully initialized
            if managed.socket_node != ThingId::default() {
                Self::sync_local_edge(managed);
                Self::sync_remote_edge(managed);
                if managed.kind == SocketType::Tcp {
                    Self::flush_managed_socket_tcp(managed, socket_set, now_ms);
                } else {
                    Self::flush_managed_socket_udp(managed, now_ms);
                }
            }
        }
    }

    /// Handle a TCP_LISTEN request
    pub fn handle_listen<'a>(
        &mut self,
        socket_set: &mut SocketSet<'a>,
        owner_tid: u64,
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
        let now_ms = Self::now_ms();

        let managed = self.new_managed_socket(
            socket_handle,
            SocketType::Tcp,
            true,
            Some(EndpointV4 {
                ip: Ipv4Address::new(0, 0, 0, 0),
                port,
            }),
            owner_tid,
            api_handle,
            now_ms,
        );

        self.sockets.insert(api_handle, managed);
        self.pending_accepts.insert(api_handle, Vec::new());

        info!(
            "SOCKET_API: Listening on port {}, handle={}",
            port, api_handle
        );
        encode_handle(api_handle)
    }

    /// Handle a TCP_CONNECT request
    pub fn handle_connect<'a, D: smoltcp::phy::Device>(
        &mut self,
        iface: &mut Interface,
        _device: &mut D,
        socket_set: &mut SocketSet<'a>,
        owner_tid: u64,
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
        let now_ms = Self::now_ms();

        let mut managed = self.new_managed_socket(
            socket_handle,
            SocketType::Tcp,
            false,
            Some(EndpointV4 {
                ip: Ipv4Address::new(0, 0, 0, 0),
                port: local_port,
            }),
            owner_tid,
            api_handle,
            now_ms,
        );
        managed.remote = Some(EndpointV4 {
            ip: remote_ip,
            port: remote_port,
        });
        Self::sync_remote_edge(&managed);
        Self::ensure_tcp_connection(&mut managed, now_ms, "syn-sent");

        self.sockets.insert(api_handle, managed);

        info!(
            "SOCKET_API: Connected handle={} local_port={}",
            api_handle, local_port
        );
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
        let (listener_socket_handle, listen_port, owner_tid) =
            match self.sockets.get(&listen_handle) {
                Some(s) if s.is_listener && s.kind == SocketType::Tcp => {
                    (s.handle, s.local.map(|e| e.port).unwrap_or(80), s.owner_tid)
                }
                _ => return encode_error(),
            };

        let socket = socket_set.get_mut::<TcpSocket>(listener_socket_handle);

        // Diagnostic: log the listener socket state on every accept attempt
        let state = socket.state();
        if state != TcpState::Listen {
            info!(
                "SOCKET_API: TCP_ACCEPT handle={} socket state={:?}",
                listen_handle, state
            );
        }

        // Check socket state - if it's established, we have a connection
        if socket.state() == TcpState::Established {
            let remote = socket.remote_endpoint();
            if let Some(ep) = remote {
                let IpAddress::Ipv4(remote_ip) = ep.addr;
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
                    managed.remote = Some(EndpointV4 {
                        ip: remote_ip,
                        port: remote_port,
                    });
                    managed.last_seen_ms = Self::now_ms();
                    Self::sync_remote_edge(managed);
                    Self::ensure_tcp_connection(managed, managed.last_seen_ms, "established");
                    conn_graph::set_sym_if_changed(
                        managed.socket_node,
                        net::props::SOCK_STATE,
                        "connected",
                    )
                    .ok();
                }

                // Move the socket to the new connection handle
                if let Some(old_managed) = self.sockets.remove(&listen_handle) {
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
                    let now_ms = Self::now_ms();
                    let listener_managed = self.new_managed_socket(
                        new_socket_handle,
                        SocketType::Tcp,
                        true,
                        Some(EndpointV4 {
                            ip: Ipv4Address::new(0, 0, 0, 0),
                            port: listen_port,
                        }),
                        owner_tid,
                        listen_handle,
                        now_ms,
                    );
                    self.sockets.insert(listen_handle, listener_managed);
                    self.pending_accepts.insert(listen_handle, Vec::new());
                    info!(
                        "SOCKET_API: Respawned listener on port {} handle={}",
                        listen_port, listen_handle
                    );
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
        owner_tid: u64,
        port: u16,
        rx_metadata_storage: &'a mut [smoltcp::socket::udp::PacketMetadata],
        rx_payload_storage: &'a mut [u8],
        tx_metadata_storage: &'a mut [smoltcp::socket::udp::PacketMetadata],
        tx_payload_storage: &'a mut [u8],
    ) -> Vec<u8> {
        info!("SOCKET_API: UDP_BIND on port {}", port);

        let rx_buffer =
            smoltcp::socket::udp::PacketBuffer::new(rx_metadata_storage, rx_payload_storage);
        let tx_buffer =
            smoltcp::socket::udp::PacketBuffer::new(tx_metadata_storage, tx_payload_storage);
        let mut socket = smoltcp::socket::udp::Socket::new(rx_buffer, tx_buffer);

        if let Err(e) = socket.bind(port) {
            warn!("SOCKET_API: Failed to bind UDP: {:?}", e);
            return encode_error();
        }

        let socket_handle = socket_set.add(socket);
        let api_handle = self.alloc_handle();
        let now_ms = Self::now_ms();

        let managed = self.new_managed_socket(
            socket_handle,
            SocketType::Udp,
            false,
            Some(EndpointV4 {
                ip: Ipv4Address::new(0, 0, 0, 0),
                port,
            }),
            owner_tid,
            api_handle,
            now_ms,
        );

        self.sockets.insert(api_handle, managed);

        info!(
            "SOCKET_API: Bound UDP on port {}, handle={}",
            port, api_handle
        );
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

        let result = match socket.send_slice(data, endpoint) {
            Ok(_) => {
                trace!(
                    "SOCKET_API: UDP_SEND_TO handle={} sent {} bytes to {}:{}",
                    handle,
                    data.len(),
                    remote_ip,
                    remote_port
                );
                encode_send_result(data.len() as u16)
            }
            Err(e) => {
                warn!("SOCKET_API: UDP_SEND_TO error: {:?}", e);
                encode_send_result(0)
            }
        };

        if let Some(m) = self.sockets.get_mut(&handle) {
            if result.len() >= 4 && u16::from_le_bytes([result[2], result[3]]) != 0 {
                m.bytes_tx = m.bytes_tx.saturating_add(data.len() as u64);
                m.packets_tx = m.packets_tx.saturating_add(1);
                m.last_seen_ms = Self::now_ms();
            }
            m.remote = Some(EndpointV4 {
                ip: remote_ip,
                port: remote_port,
            });
            Self::sync_remote_edge(m);
        }

        result
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
                let IpAddress::Ipv4(remote_ip) = endpoint.endpoint.addr;
                let remote_port = endpoint.endpoint.port;

                trace!(
                    "SOCKET_API: UDP_RECV_FROM handle={} got {} bytes from {}:{}",
                    handle,
                    data.len(),
                    remote_ip,
                    remote_port
                );

                if let Some(m) = self.sockets.get_mut(&handle) {
                    m.bytes_rx = m.bytes_rx.saturating_add(data.len() as u64);
                    m.packets_rx = m.packets_rx.saturating_add(1);
                    m.last_seen_ms = Self::now_ms();
                    m.remote = Some(EndpointV4 {
                        ip: remote_ip,
                        port: remote_port,
                    });
                    Self::sync_remote_edge(m);
                }

                encode_udp_data(remote_ip, remote_port, data)
            }
            Err(e) => {
                warn!("SOCKET_API: UDP_RECV_FROM error: {:?}", e);
                if let Some(m) = self.sockets.get_mut(&handle) {
                    Self::set_last_error(m, &format!("udp_recv:{:?}", e));
                }
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
        iface
            .join_multicast_group(device, IpAddress::Ipv4(multicast_ip), now)
            .ok();
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

        let response = match socket.send_slice(data) {
            Ok(sent) => {
                trace!("SOCKET_API: TCP_SEND handle={} sent {} bytes", handle, sent);
                encode_send_result(sent as u16)
            }
            Err(e) => {
                warn!("SOCKET_API: TCP_SEND error: {:?}", e);
                encode_send_result(0)
            }
        };

        if let Some(m) = self.sockets.get_mut(&handle) {
            if response.len() >= 4 {
                let sent = u16::from_le_bytes([response[2], response[3]]) as u64;
                if sent != 0 {
                    m.bytes_tx = m.bytes_tx.saturating_add(sent);
                    m.packets_tx = m.packets_tx.saturating_add(1);
                    m.last_seen_ms = Self::now_ms();
                }
            }
        }

        response
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
                if let Some(m) = self.sockets.get_mut(&handle) {
                    m.bytes_rx = m.bytes_rx.saturating_add(len as u64);
                    m.packets_rx = m.packets_rx.saturating_add(1);
                    m.last_seen_ms = Self::now_ms();
                }
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
    pub fn handle_close<'a>(&mut self, socket_set: &mut SocketSet<'a>, handle: u32) -> Vec<u8> {
        let now_ms = Self::now_ms();
        if let Some(managed) = self.sockets.get(&handle) {
            conn_graph::set_sym_if_changed(managed.socket_node, net::props::SOCK_STATE, "closed")
                .ok();
            conn_graph::set_if_changed(managed.socket_node, net::props::SOCK_CLOSED_AT, now_ms)
                .ok();
            if let Some(conn_id) = managed.connection_node {
                conn_graph::set_sym_if_changed(conn_id, net::props::CONN_STATE, "closed").ok();
                conn_graph::set_if_changed(conn_id, net::props::CONN_LAST_SEEN, now_ms).ok();
            }

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

        let mut tracked_handles = BTreeSet::new();
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
        caller_tid: Option<u64>,
        rx_storage: &'a mut [u8],
        tx_storage: &'a mut [u8],
        dns_server: Option<Ipv4Address>,
    ) -> Vec<u8> {
        if msg.len() < 4 {
            return encode_error();
        }

        let msg_type = u16::from_le_bytes([msg[0], msg[1]]);
        let payload_len = u16::from_le_bytes([msg[2], msg[3]]) as usize;
        let owner_tid = caller_tid.unwrap_or(0);

        if msg.len() < 4 + payload_len {
            warn!(
                "SOCKET_API: Message too short for declared payload_len (type=0x{:04x}, len={}, expected={})",
                msg_type,
                msg.len(),
                4 + payload_len
            );
            return encode_error();
        }

        // The actual payload starts at index 4 (after type and length)
        let body = &msg[4..4 + payload_len];

        match msg_type {
            MSG_TCP_CONNECT => {
                if body.len() < 6 {
                    return encode_error();
                }
                let ip = Ipv4Address::from_bytes(&body[0..4]);
                let port = u16::from_le_bytes([body[4], body[5]]);
                self.handle_connect(
                    iface, device, socket_set, owner_tid, ip, port, rx_storage, tx_storage,
                )
            }
            MSG_TCP_LISTEN => {
                if body.len() < 4 {
                    return encode_error();
                }
                let port = u16::from_le_bytes([body[0], body[1]]);
                let backlog = u16::from_le_bytes([body[2], body[3]]);
                self.handle_listen(socket_set, owner_tid, port, backlog, rx_storage, tx_storage)
            }
            MSG_TCP_ACCEPT => {
                if body.len() < 4 {
                    return encode_error();
                }
                let listen_handle = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
                self.handle_accept(socket_set, listen_handle, rx_storage, tx_storage)
            }
            MSG_TCP_SEND => {
                if body.len() < 4 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
                let data = &body[4..];
                self.handle_send(socket_set, handle, data)
            }
            MSG_TCP_RECV => {
                if body.len() < 6 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
                let max_len = u16::from_le_bytes([body[4], body[5]]);
                self.handle_recv(socket_set, handle, max_len)
            }
            MSG_TCP_CLOSE => {
                if body.len() < 4 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
                self.handle_close(socket_set, handle)
            }
            MSG_UDP_BIND => {
                if body.len() < 2 {
                    return encode_error();
                }
                let port = u16::from_le_bytes([body[0], body[1]]);
                unsafe {
                    let (rx_meta, rx_payload) = split_packet_buffer(rx_storage);
                    let (tx_meta, tx_payload) = split_packet_buffer(tx_storage);
                    self.handle_udp_bind(
                        socket_set, owner_tid, port, rx_meta, rx_payload, tx_meta, tx_payload,
                    )
                }
            }
            MSG_UDP_SEND_TO => {
                if body.len() < 10 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
                let ip = Ipv4Address::from_bytes(&body[4..8]);
                let port = u16::from_le_bytes([body[8], body[9]]);
                let data = &body[10..];
                self.handle_udp_send_to(socket_set, handle, ip, port, data)
            }
            MSG_UDP_RECV_FROM => {
                if body.len() < 4 {
                    return encode_error();
                }
                let handle = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
                self.handle_udp_recv_from(socket_set, handle)
            }
            MSG_NET_JOIN_MULTICAST => {
                if body.len() < 4 {
                    return encode_error();
                }
                let ip = Ipv4Address::from_bytes(&body[0..4]);
                self.handle_multicast_join(iface, device, ip)
            }
            MSG_DNS_QUERY => {
                if let Some(dns_server) = dns_server {
                    if let Ok(hostname) = core::str::from_utf8(body) {
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
unsafe fn split_packet_buffer(
    buf: &mut [u8],
) -> (&mut [smoltcp::socket::udp::PacketMetadata], &mut [u8]) {
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
