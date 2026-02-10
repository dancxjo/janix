use crate::errors::Errno;
use crate::thing::ThingId;
use crate::thing::sys::{create_node, find, get_edges, intern, link, prop_get, prop_set};
use abi::types::Edge;

pub mod kinds {
    pub const ROOT: &str = "net.Root";
    pub const INTERFACE: &str = "net.Interface";
    pub const ADDRESS: &str = "net.Address";
    pub const ROUTE: &str = "net.Route";
    pub const NEIGHBOR: &str = "net.Neighbor";
}

pub mod preds {
    pub const ROOT_HAS_IFACE: &str = "net.root_has_iface";
    pub const IFACE_HAS_ADDR: &str = "net.iface_has_addr";
    pub const ROOT_HAS_ROUTE: &str = "net.root_has_route";
    pub const ROUTE_VIA_IFACE: &str = "net.route_via_iface";
    pub const IFACE_KNOWS_NEIGHBOR: &str = "net.iface_knows_neighbor";
}

pub mod props {
    pub const ROOT_INSTANCE: &str = "net.root.instance";
    pub const GEN: &str = "net.gen";

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
        if edge.to.to_u64_lossy() == dst.to_u64_lossy() && edge.predicate.to_u64_lossy() == rel_id
        {
            return Ok(true);
        }
    }
    Ok(false)
}
