use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use stem::errors::Errno;
use stem::net;
use stem::thing::ThingId;
use stem::thing::sys::{find, get_edges, intern, prop_get, prop_set};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfaceSnapshot {
    pub stable_key: u64,
    pub name: String,
    pub mac_packed: u64,
    pub mtu: u32,
    pub link_up: bool,
    pub driver: String,
    pub speed_mbps: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddressSnapshot {
    pub family: String,
    pub ip: String,
    pub prefix: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteSnapshot {
    pub dst: String,
    pub gateway: Option<String>,
    pub metric: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetSnapshot {
    pub iface: IfaceSnapshot,
    pub addrs: Vec<AddressSnapshot>,
    pub routes: Vec<RouteSnapshot>,
}

pub struct NetGraphMirror {
    root_id: ThingId,
    iface_id: Option<ThingId>,
    last_snapshot: Option<NetSnapshot>,
    last_refresh_ms: u64,
}

impl NetGraphMirror {
    pub fn new() -> Result<Self, Errno> {
        let root_id = ensure_net_root()?;
        Ok(Self {
            root_id,
            iface_id: None,
            last_snapshot: None,
            last_refresh_ms: 0,
        })
    }

    pub fn root_id(&self) -> ThingId {
        self.root_id
    }

    pub fn apply(&mut self, snapshot: &NetSnapshot, now_ms: u64) -> Result<(), Errno> {
        let changed = self
            .last_snapshot
            .as_ref()
            .map(|prev| prev != snapshot)
            .unwrap_or(true);
        let periodic_refresh = now_ms.saturating_sub(self.last_refresh_ms) >= 3000;
        if !changed && !periodic_refresh {
            return Ok(());
        }

        let iface_id = mirror_iface(self.root_id, &snapshot.iface)?;
        let addr_changed = mirror_addrs(iface_id, &snapshot.addrs)?;
        let route_changed = mirror_routes(self.root_id, iface_id, &snapshot.routes)?;
        let iface_changed = self.iface_id.map(|id| id != iface_id).unwrap_or(true) || changed;
        if iface_changed || addr_changed || route_changed {
            bump_gen(self.root_id).ok();
        }

        self.iface_id = Some(iface_id);
        self.last_snapshot = Some(snapshot.clone());
        self.last_refresh_ms = now_ms;
        Ok(())
    }
}

pub fn ensure_net_root() -> Result<ThingId, Errno> {
    net::ensure_net_root()
}

pub fn mirror_iface(root: ThingId, snap: &IfaceSnapshot) -> Result<ThingId, Errno> {
    let iface_id = net::upsert_node_by_key(
        net::kinds::INTERFACE,
        net::props::IFACE_STABLE_KEY,
        snap.stable_key,
    )?;

    net::ensure_edge(root, net::preds::ROOT_HAS_IFACE, iface_id)?;

    set_if_changed_sym(iface_id, net::props::IFACE_NAME, &snap.name).ok();
    set_if_changed(iface_id, net::props::IFACE_MAC, snap.mac_packed).ok();
    set_if_changed(iface_id, net::props::IFACE_MTU, snap.mtu as u64).ok();
    set_if_changed(
        iface_id,
        net::props::IFACE_LINK_UP,
        if snap.link_up { 1 } else { 0 },
    )
    .ok();
    set_if_changed_sym(iface_id, net::props::IFACE_DRIVER, &snap.driver).ok();
    if let Some(speed) = snap.speed_mbps {
        set_if_changed(iface_id, net::props::IFACE_SPEED_MBPS, speed as u64).ok();
    }

    Ok(iface_id)
}

pub fn mirror_addrs(iface_id: ThingId, addrs: &[AddressSnapshot]) -> Result<bool, Errno> {
    let mut desired_nodes = Vec::with_capacity(addrs.len());
    let mut changed = false;

    for addr in addrs {
        let family_sym = intern(&addr.family)? as u64;
        let ip_sym = intern(&addr.ip)? as u64;
        let node = upsert_addr_node(iface_id, family_sym, ip_sym, addr.prefix)?;
        desired_nodes.push(node);

        changed |= set_if_changed(node, net::props::ADDR_IFACE_ID, iface_id.to_u64_lossy())?;
        changed |= set_if_changed(node, net::props::ADDR_FAMILY, family_sym)?;
        changed |= set_if_changed(node, net::props::ADDR_IP, ip_sym)?;
        changed |= set_if_changed(node, net::props::ADDR_PREFIX, addr.prefix as u64)?;
        changed |= set_if_changed(node, net::props::ADDR_ACTIVE, 1)?;
        net::ensure_edge(iface_id, net::preds::IFACE_HAS_ADDR, node)?;
    }

    let mut edges = [abi::types::Edge::default(); 128];
    let count = get_edges(iface_id, &mut edges)?;
    let rel_id = intern(net::preds::IFACE_HAS_ADDR)? as u64;
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() != rel_id {
            continue;
        }
        let addr_id = edge.to;
        if !contains_id(&desired_nodes, addr_id) {
            changed |= set_if_changed(addr_id, net::props::ADDR_ACTIVE, 0).unwrap_or(false);
        }
    }

    Ok(changed)
}

pub fn mirror_routes(
    root: ThingId,
    iface_id: ThingId,
    routes: &[RouteSnapshot],
) -> Result<bool, Errno> {
    let mut desired_nodes = Vec::with_capacity(routes.len());
    let mut changed = false;

    for route in routes {
        let dst_sym = intern(&route.dst)? as u64;
        let gw_sym = match route.gateway.as_deref() {
            Some(gw) => intern(gw)? as u64,
            None => 0,
        };

        let node = upsert_route_node(iface_id, dst_sym, gw_sym)?;
        desired_nodes.push(node);

        changed |= set_if_changed(node, net::props::ROUTE_IFACE_ID, iface_id.to_u64_lossy())?;
        changed |= set_if_changed(node, net::props::ROUTE_DST, dst_sym)?;
        changed |= set_if_changed(node, net::props::ROUTE_GATEWAY, gw_sym)?;
        if let Some(metric) = route.metric {
            changed |= set_if_changed(node, net::props::ROUTE_METRIC, metric as u64)?;
        }
        changed |= set_if_changed(node, net::props::ROUTE_ACTIVE, 1)?;
        net::ensure_edge(root, net::preds::ROOT_HAS_ROUTE, node)?;
        net::ensure_edge(node, net::preds::ROUTE_VIA_IFACE, iface_id)?;
    }

    let mut edges = [abi::types::Edge::default(); 128];
    let count = get_edges(root, &mut edges)?;
    let rel_id = intern(net::preds::ROOT_HAS_ROUTE)? as u64;
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() != rel_id {
            continue;
        }
        let route_id = edge.to;
        if !contains_id(&desired_nodes, route_id) {
            changed |= set_if_changed(route_id, net::props::ROUTE_ACTIVE, 0).unwrap_or(false);
        }
    }

    Ok(changed)
}

fn upsert_addr_node(
    iface_id: ThingId,
    family_sym: u64,
    ip_sym: u64,
    prefix: u8,
) -> Result<ThingId, Errno> {
    let mut ids = [ThingId::default(); 128];
    let count = find(net::kinds::ADDRESS, &mut ids)?;
    for id in ids.iter().copied().take(count) {
        if prop_get(id, net::props::ADDR_IFACE_ID).unwrap_or(0) != iface_id.to_u64_lossy() {
            continue;
        }
        if prop_get(id, net::props::ADDR_FAMILY).unwrap_or(0) != family_sym {
            continue;
        }
        if prop_get(id, net::props::ADDR_IP).unwrap_or(0) != ip_sym {
            continue;
        }
        if prop_get(id, net::props::ADDR_PREFIX).unwrap_or(0) == prefix as u64 {
            return Ok(id);
        }
    }

    let id = stem::thing::sys::create_node(net::kinds::ADDRESS)?;
    prop_set(id, net::props::ADDR_IFACE_ID, iface_id.to_u64_lossy()).ok();
    prop_set(id, net::props::ADDR_FAMILY, family_sym).ok();
    prop_set(id, net::props::ADDR_IP, ip_sym).ok();
    prop_set(id, net::props::ADDR_PREFIX, prefix as u64).ok();
    Ok(id)
}

fn upsert_route_node(iface_id: ThingId, dst_sym: u64, gateway_sym: u64) -> Result<ThingId, Errno> {
    let mut ids = [ThingId::default(); 128];
    let count = find(net::kinds::ROUTE, &mut ids)?;
    for id in ids.iter().copied().take(count) {
        if prop_get(id, net::props::ROUTE_IFACE_ID).unwrap_or(0) != iface_id.to_u64_lossy() {
            continue;
        }
        if prop_get(id, net::props::ROUTE_DST).unwrap_or(0) != dst_sym {
            continue;
        }
        if prop_get(id, net::props::ROUTE_GATEWAY).unwrap_or(0) == gateway_sym {
            return Ok(id);
        }
    }

    let id = stem::thing::sys::create_node(net::kinds::ROUTE)?;
    prop_set(id, net::props::ROUTE_IFACE_ID, iface_id.to_u64_lossy()).ok();
    prop_set(id, net::props::ROUTE_DST, dst_sym).ok();
    prop_set(id, net::props::ROUTE_GATEWAY, gateway_sym).ok();
    Ok(id)
}

fn bump_gen(root: ThingId) -> Result<(), Errno> {
    let current = prop_get(root, net::props::GEN).unwrap_or(0);
    prop_set(root, net::props::GEN, current.wrapping_add(1))
}

fn set_if_changed(id: ThingId, key: &str, value: u64) -> Result<bool, Errno> {
    let current = prop_get(id, key).ok();
    if current == Some(value) {
        return Ok(false);
    }
    prop_set(id, key, value)?;
    Ok(true)
}

fn set_if_changed_sym(id: ThingId, key: &str, value: &str) -> Result<bool, Errno> {
    let sym = intern(value)? as u64;
    set_if_changed(id, key, sym)
}

fn contains_id(ids: &[ThingId], target: ThingId) -> bool {
    ids.iter()
        .any(|id| id.to_u64_lossy() == target.to_u64_lossy())
}

pub fn stable_iface_key_from_mac(mac: [u8; 6]) -> u64 {
    (mac[0] as u64)
        | ((mac[1] as u64) << 8)
        | ((mac[2] as u64) << 16)
        | ((mac[3] as u64) << 24)
        | ((mac[4] as u64) << 32)
        | ((mac[5] as u64) << 40)
}

pub fn default_routes_from_gateway(gateway: [u8; 4]) -> Vec<RouteSnapshot> {
    if gateway == [0, 0, 0, 0] {
        return Vec::new();
    }
    vec![RouteSnapshot {
        dst: "0.0.0.0/0".into(),
        gateway: Some(format!(
            "{}.{}.{}.{}",
            gateway[0], gateway[1], gateway[2], gateway[3]
        )),
        metric: None,
    }]
}
