use crate::errors::Errno;
use crate::thing::sys::{create_node, find, get_edges, intern, link, prop_get, prop_set};
use crate::thing::ThingId;
use abi::types::Edge;

pub mod kinds {
    pub const ROOT: &str = "net.Root";
    pub const INTERFACE: &str = "net.Interface";
    pub const ADDRESS: &str = "net.Address";
    pub const ROUTE: &str = "net.Route";
    pub const NEIGHBOR: &str = "net.Neighbor";
    pub const SOCKET: &str = "net.Socket";
    pub const CONNECTION: &str = "net.Connection";
    pub const ENDPOINT: &str = "net.Endpoint";
}

pub mod preds {
    pub const ROOT_HAS_IFACE: &str = "net.root_has_iface";
    pub const IFACE_HAS_ADDR: &str = "net.iface_has_addr";
    pub const ROOT_HAS_ROUTE: &str = "net.root_has_route";
    pub const ROUTE_VIA_IFACE: &str = "net.route_via_iface";
    pub const IFACE_KNOWS_NEIGHBOR: &str = "net.iface_knows_neighbor";
    pub const PROC_OWNS_SOCKET: &str = "proc.owns_socket";
    pub const SOCKET_HAS_LOCAL: &str = "net.socket_has_local";
    pub const SOCKET_HAS_REMOTE: &str = "net.socket_has_remote";
    pub const SOCKET_HAS_CONNECTION: &str = "net.socket_has_connection";
    pub const CONNECTION_PEER: &str = "net.connection_peer";
    pub const CONNECTION_FOR_SERVICE: &str = "net.connection_for_service";
}

pub mod props {
    pub const ROOT_INSTANCE: &str = "net.root.instance";
    pub const GEN: &str = "net.gen";
    pub const NET_ID_KEY: &str = "net.id_key";

    pub const IFACE_STABLE_KEY: &str = "net.iface.stable_key";
    pub const IFACE_NAME: &str = "net.iface.name";
    pub const IFACE_MAC: &str = "net.iface.mac";
    pub const IFACE_MTU: &str = "net.iface.mtu";
    pub const IFACE_LINK_UP: &str = "net.iface.link_up";
    pub const IFACE_DRIVER: &str = "net.iface.driver";
    pub const IFACE_SPEED_MBPS: &str = "net.iface.speed_mbps";

    pub const ADDR_IFACE_ID: &str = "net.addr.iface_id";
    pub const ADDR_FAMILY: &str = "net.addr.family";
    pub const ADDR_IP: &str = "net.addr.ip";
    pub const ADDR_PREFIX: &str = "net.addr.prefix";
    pub const ADDR_ACTIVE: &str = "net.addr.active";

    pub const ROUTE_IFACE_ID: &str = "net.route.iface_id";
    pub const ROUTE_DST: &str = "net.route.dst";
    pub const ROUTE_GATEWAY: &str = "net.route.gateway";
    pub const ROUTE_METRIC: &str = "net.route.metric";
    pub const ROUTE_ACTIVE: &str = "net.route.active";

    pub const ENDPOINT_PORT: &str = "net.endpoint.port";
    pub const ENDPOINT_PROTO: &str = "net.endpoint.proto";
    pub const ENDPOINT_IP: &str = "net.endpoint.ip";

    pub const SOCK_KEY: &str = "net.sock.key";
    pub const SOCK_PROTO: &str = "net.sock.proto";
    pub const SOCK_STATE: &str = "net.sock.state";
    pub const SOCK_FD: &str = "net.sock.fd";
    pub const SOCK_PID: &str = "net.sock.pid";
    pub const SOCK_CREATED_AT: &str = "net.sock.created_at";
    pub const SOCK_CLOSED_AT: &str = "net.sock.closed_at";

    pub const CONN_KEY: &str = "net.conn.key";
    pub const CONN_STATE: &str = "net.conn.state";
    pub const CONN_BYTES_TX: &str = "net.conn.bytes_tx";
    pub const CONN_BYTES_RX: &str = "net.conn.bytes_rx";
    pub const CONN_PACKETS_TX: &str = "net.conn.packets_tx";
    pub const CONN_PACKETS_RX: &str = "net.conn.packets_rx";
    pub const CONN_LAST_SEEN: &str = "net.conn.last_seen";
    pub const CONN_LAST_ERROR: &str = "net.conn.last_error";
    pub const CONN_RTT_MS: &str = "net.conn.rtt_ms";
}

pub fn ensure_net_root() -> Result<ThingId, Errno> {
    upsert_node_by_key(kinds::ROOT, props::ROOT_INSTANCE, 1)
}

pub fn upsert_node_by_key(kind: &str, key: &str, key_value: u64) -> Result<ThingId, Errno> {
    if let Some(id) = find_node_by_key(kind, key, key_value)? {
        return Ok(id);
    }

    let id = create_node(kind)?;
    prop_set(id, key, key_value)?;
    Ok(id)
}

pub fn find_node_by_key(kind: &str, key: &str, key_value: u64) -> Result<Option<ThingId>, Errno> {
    let mut ids = [ThingId::default(); 128];
    let count = find(kind, &mut ids)?;
    for id in ids.iter().copied().take(count) {
        if prop_get(id, key).unwrap_or(u64::MAX) == key_value {
            return Ok(Some(id));
        }
    }
    Ok(None)
}

pub fn ensure_edge(src: ThingId, rel: &str, dst: ThingId) -> Result<(), Errno> {
    if edge_exists(src, rel, dst)? {
        return Ok(());
    }
    link(src, rel, dst)
}

pub fn edge_exists(src: ThingId, rel: &str, dst: ThingId) -> Result<bool, Errno> {
    let rel_id = intern(rel)? as u64;
    let mut edges = [Edge::default(); 128];
    let count = get_edges(src, &mut edges)?;
    for edge in edges.iter().take(count) {
        if edge.to.to_u64_lossy() == dst.to_u64_lossy() && edge.predicate.to_u64_lossy() == rel_id {
            return Ok(true);
        }
    }
    Ok(false)
}

pub mod conn_graph {
    use super::{ensure_edge, kinds, props};
    use crate::errors::Errno;
    use crate::thing::sys::{find, intern, prop_get, prop_set};
    use crate::thing::ThingId;
    use alloc::format;

    pub fn hash64_bytes(bytes: &[u8]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }

    pub fn hash64_u64s(words: &[u64]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for &w in words {
            for b in w.to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
        h
    }

    pub fn ensure_socket_node(key: u64) -> Result<ThingId, Errno> {
        super::upsert_node_by_key(kinds::SOCKET, props::SOCK_KEY, key)
    }

    pub fn ensure_connection_node(key: u64) -> Result<ThingId, Errno> {
        super::upsert_node_by_key(kinds::CONNECTION, props::CONN_KEY, key)
    }

    pub fn ensure_endpoint_node(proto: &str, ip: &str, port: u16) -> Result<ThingId, Errno> {
        let key = hash64_bytes(format!("{}|{}|{}", proto, ip, port).as_bytes());
        let id = super::upsert_node_by_key(kinds::ENDPOINT, props::NET_ID_KEY, key)?;
        set_sym_if_changed(id, props::ENDPOINT_PROTO, proto).ok();
        set_sym_if_changed(id, props::ENDPOINT_IP, ip).ok();
        set_if_changed(id, props::ENDPOINT_PORT, port as u64).ok();
        Ok(id)
    }

    pub fn find_thread_node_by_tid(tid: u64) -> Result<Option<ThingId>, Errno> {
        let mut ids = [ThingId::default(); 256];
        let count = find("proc.Thread", &mut ids)?;
        for id in ids.iter().copied().take(count) {
            if prop_get(id, "proc.tid").unwrap_or(0) == tid {
                return Ok(Some(id));
            }
        }
        Ok(None)
    }

    pub fn set_if_changed(id: ThingId, key: &str, value: u64) -> Result<bool, Errno> {
        let current = prop_get(id, key).ok();
        if current == Some(value) {
            return Ok(false);
        }
        prop_set(id, key, value)?;
        Ok(true)
    }

    pub fn set_sym_if_changed(id: ThingId, key: &str, value: &str) -> Result<bool, Errno> {
        let sym = intern(value)? as u64;
        set_if_changed(id, key, sym)
    }

    pub fn link_if_present(src: ThingId, rel: &str, dst: Option<ThingId>) -> Result<(), Errno> {
        if let Some(dst) = dst {
            ensure_edge(src, rel, dst)?;
        }
        Ok(())
    }
}
