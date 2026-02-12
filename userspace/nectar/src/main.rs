#![feature(restricted_std)]
#![no_main]

extern crate alloc;
extern crate stem;

mod discovery_state;
mod dns_packet;

use abi::ids::HandleId;
use abi::schema::{keys, kinds, rels};
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use discovery_state::reconcile_desired;
use dns_packet::{
    encode_ptr_rdata, encode_srv_rdata, encode_txt_rdata, parse_a, parse_aaaa, parse_ptr_target,
    parse_srv, parse_txt_kvs, DnsPacket, DnsResourceRecord,
};
use stem::info;
use stem::petals::Petals;
use stem::syscall::port::{port_create, port_recv, port_send_all, port_wait, PortHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;

const MSG_UDP_BIND: u16 = 0x0300;
const MSG_UDP_SEND_TO: u16 = 0x0301;
const MSG_UDP_RECV_FROM: u16 = 0x0302;
const MSG_NET_JOIN_MULTICAST: u16 = 0x0400;

const RESP_OK: u16 = 0x0000;
const RESP_HANDLE: u16 = 0x0002;
const RESP_DATA: u16 = 0x0003;
const RESP_EMPTY: u16 = 0x0005;

const MDNS_PORT: u16 = 5353;
const DEFAULT_ANNOUNCE_TTL: u32 = 120;

#[derive(Clone, Debug, PartialEq, Eq)]
struct DesiredService {
    service_type: String,
    domain: String,
    instance_name: String,
    port: u16,
    txt_items: Vec<String>,
    hostname: String,
}

impl DesiredService {
    fn service_type_fqdn(&self) -> String {
        let mut out = self.service_type.trim_end_matches('.').to_string();
        if !out.ends_with(&self.domain) {
            out.push('.');
            out.push_str(self.domain.trim_end_matches('.'));
        }
        out
    }

    fn instance_fqdn(&self) -> String {
        format!("{}.{}", self.instance_name, self.service_type_fqdn())
    }

    fn host_fqdn(&self) -> String {
        format!("{}.{}", self.hostname, self.domain.trim_end_matches('.'))
    }

    fn stable_key(&self) -> String {
        self.instance_fqdn()
    }
}

struct DiscoveryGraph {
    id_cache: BTreeMap<u64, ThingId>,
    expiry: BTreeMap<u64, u64>,
}

impl DiscoveryGraph {
    fn new() -> Self {
        Self {
            id_cache: BTreeMap::new(),
            expiry: BTreeMap::new(),
        }
    }

    fn observe_service_type(&mut self, type_fqdn: &str, ttl: u32) {
        let key = format!("type:{}", canonical_name(type_fqdn));
        let id = self.upsert_node_by_key(kinds::NET_SERVICE_TYPE, &key);
        let (name, domain) = split_service_type(type_fqdn);
        set_string_prop(id, keys::NET_SVC_TYPE_NAME, &name);
        set_string_prop(id, keys::NET_SVC_TYPE_DOMAIN, &domain);
        self.touch(id, ttl, keys::NET_LAST_SEEN, keys::NET_EXPIRES_AT);
    }

    fn observe_instance(&mut self, instance_fqdn: &str, type_fqdn: &str, ttl: u32) {
        self.observe_service_type(type_fqdn, ttl);
        let instance_key = format!("instance:{}", canonical_name(instance_fqdn));
        let instance_id = self.upsert_node_by_key(kinds::NET_SERVICE_INSTANCE, &instance_key);
        let type_id = self.upsert_node_by_key(
            kinds::NET_SERVICE_TYPE,
            &format!("type:{}", canonical_name(type_fqdn)),
        );
        let instance_name = instance_fqdn
            .split('.')
            .next()
            .unwrap_or(instance_fqdn)
            .to_string();
        set_string_prop(instance_id, keys::NET_INSTANCE_NAME, &instance_name);
        set_string_prop(instance_id, keys::NET_INSTANCE_FQDN, instance_fqdn);
        ensure_edge(instance_id, rels::NET_INSTANCE_IS_A, type_id);
        self.touch(
            instance_id,
            ttl,
            keys::NET_INSTANCE_LAST_SEEN,
            keys::NET_INSTANCE_EXPIRES_AT,
        );
    }

    fn observe_srv(
        &mut self,
        instance_fqdn: &str,
        target_host: &str,
        port: u16,
        ttl: u32,
        priority: u16,
        weight: u16,
    ) {
        let type_fqdn =
            infer_type_fqdn_from_instance(instance_fqdn).unwrap_or("_unknown._tcp.local");
        self.observe_instance(instance_fqdn, type_fqdn, ttl);
        let instance_id = self.upsert_node_by_key(
            kinds::NET_SERVICE_INSTANCE,
            &format!("instance:{}", canonical_name(instance_fqdn)),
        );
        let host_id = self.observe_host(target_host, ttl);

        let endpoint_key = format!(
            "endpoint:{}:{}:{}",
            canonical_name(instance_fqdn),
            port,
            "tcp"
        );
        let endpoint_id = self.upsert_node_by_key(kinds::NET_ENDPOINT, &endpoint_key);
        thingsys::prop_set(endpoint_id, keys::NET_ENDPOINT_PORT, port as u64).ok();
        set_string_prop(endpoint_id, keys::NET_ENDPOINT_PROTO, "tcp");
        thingsys::prop_set(endpoint_id, keys::NET_ENDPOINT_PRIORITY, priority as u64).ok();
        thingsys::prop_set(endpoint_id, keys::NET_ENDPOINT_WEIGHT, weight as u64).ok();

        ensure_edge(instance_id, rels::NET_INSTANCE_REACHABLE_AT, endpoint_id);
        ensure_edge(host_id, rels::NET_HOST_ADVERTISES, instance_id);
        self.touch(endpoint_id, ttl, keys::NET_LAST_SEEN, keys::NET_EXPIRES_AT);
    }

    fn observe_txt(
        &mut self,
        instance_fqdn: &str,
        raw_txt: &[u8],
        _kvs: &[(String, String)],
        ttl: u32,
    ) {
        let type_fqdn =
            infer_type_fqdn_from_instance(instance_fqdn).unwrap_or("_unknown._tcp.local");
        self.observe_instance(instance_fqdn, type_fqdn, ttl);

        let txt_hash = hash64_bytes(raw_txt);
        let txt_key = format!("txt:{}:{:016x}", canonical_name(instance_fqdn), txt_hash);
        let txt_id = self.upsert_node_by_key(kinds::NET_TXT_RECORD, &txt_key);
        thingsys::prop_set(txt_id, keys::NET_TXT_HASH, txt_hash).ok();
        set_bytes_prop(txt_id, keys::BYTESPACE, raw_txt);

        let instance_id = self.upsert_node_by_key(
            kinds::NET_SERVICE_INSTANCE,
            &format!("instance:{}", canonical_name(instance_fqdn)),
        );
        ensure_edge(instance_id, rels::NET_INSTANCE_HAS_TXT, txt_id);
        self.touch(txt_id, ttl, keys::NET_LAST_SEEN, keys::NET_EXPIRES_AT);
    }

    fn observe_addr(&mut self, host: &str, ip: &str, ttl: u32) {
        let host_id = self.observe_host(host, ttl);
        let family = if ip.contains(':') { "ipv6" } else { "ipv4" };
        let key = format!("addr:{}:{}", canonical_name(host), canonical_name(ip));
        let addr_id = self.upsert_node_by_key(kinds::NET_ADDRESS, &key);
        set_string_prop(addr_id, keys::NET_ADDR_FAMILY, family);
        set_string_prop(addr_id, keys::NET_ADDR_IP, ip);
        ensure_edge(host_id, rels::NET_HOST_HAS_ADDR, addr_id);
        self.touch(
            addr_id,
            ttl,
            keys::NET_ADDR_LAST_SEEN,
            keys::NET_ADDR_EXPIRES_AT,
        );
    }

    fn observe_host(&mut self, host: &str, ttl: u32) -> ThingId {
        let host_key = format!("host:{}", canonical_name(host));
        let host_id = self.upsert_node_by_key(kinds::NET_HOST, &host_key);
        set_string_prop(host_id, keys::NET_HOST_NAME, host);
        self.touch(
            host_id,
            ttl,
            keys::NET_HOST_LAST_SEEN,
            keys::NET_HOST_EXPIRES_AT,
        );
        host_id
    }

    fn sweep_stale(&mut self, now_s: u64) {
        let mut expired = Vec::new();
        for (&id, &expires_at) in &self.expiry {
            if expires_at <= now_s {
                expired.push(id);
            }
        }
        for id in expired {
            thingsys::prop_set(ThingId::from_u64(id), keys::NET_STALE, 1).ok();
            self.expiry.remove(&id);
        }
    }

    fn touch(&mut self, id: ThingId, ttl: u32, last_seen_key: &str, expires_at_key: &str) {
        let now_s = stem::time::now_unix_seconds();
        let expires = now_s.saturating_add(ttl as u64);
        thingsys::prop_set(id, last_seen_key, now_s).ok();
        thingsys::prop_set(id, expires_at_key, expires).ok();
        thingsys::prop_set(id, keys::NET_LAST_SEEN, now_s).ok();
        thingsys::prop_set(id, keys::NET_EXPIRES_AT, expires).ok();
        thingsys::prop_set(id, keys::NET_STALE, 0).ok();
        self.expiry.insert(id.to_u64_lossy(), expires);
    }

    fn upsert_node_by_key(&mut self, kind: &str, key: &str) -> ThingId {
        let id_key = hash64(key.as_bytes());
        if let Some(id) = self.id_cache.get(&id_key).copied() {
            return id;
        }

        let mut ids = [ThingId::default(); 128];
        if let Ok(count) = thingsys::find(kind, &mut ids) {
            for id in ids.iter().copied().take(count) {
                if thingsys::prop_get(id, keys::NET_ID_KEY).unwrap_or(0) == id_key {
                    self.id_cache.insert(id_key, id);
                    return id;
                }
            }
        }

        let id = thingsys::create_node(kind).expect("create node");
        thingsys::prop_set(id, keys::NET_ID_KEY, id_key).ok();
        self.id_cache.insert(id_key, id);
        id
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("NECTAR: Started.");

    let nectar_svc_id =
        ensure_singleton_node("svc.Nectar", keys::NET_ID_KEY, hash64(b"svc.Nectar"));
    if thingsys::prop_get(nectar_svc_id, keys::NET_NECTAR_PUBLISH_DISCOVERY_TO_GRAPH).is_err() {
        thingsys::prop_set(
            nectar_svc_id,
            keys::NET_NECTAR_PUBLISH_DISCOVERY_TO_GRAPH,
            1,
        )
        .ok();
    }

    let mut window_id = None;
    let mut hostname = String::from("thing-os");
    let mut last_ip: Option<[u8; 4]> = None;
    window_id = maybe_create_window(window_id, &hostname, last_ip);

    let (api, mac, net_stack) = loop {
        if let Some(res) = find_netd_and_mac() {
            break res;
        }
        stem::time::sleep_ms(1000);
    };

    let (resp_w, resp_r) = port_create(64).expect("port_create");
    let udp_handle = match udp_bind(api, resp_w, resp_r, MDNS_PORT) {
        Ok(h) => h,
        Err(_) => {
            stem::error!("NECTAR: Failed to bind to 5353.");
            0
        }
    };

    if udp_handle != 0 {
        net_join_multicast(api, resp_w, resp_r, [224, 0, 0, 251]).ok();
    }

    hostname = get_or_generate_hostname(mac);
    let mut buf = [ThingId::default(); 1];
    if let Ok(1) = thingsys::find(kinds::DEV_HOST, &mut buf) {
        set_string_prop(buf[0], keys::NAME, &hostname);
    }
    window_id = maybe_create_window(window_id, &hostname, last_ip);

    let mut graph = DiscoveryGraph::new();
    let advertise_root = ensure_singleton_node(
        kinds::NET_ADVERTISE_ROOT,
        keys::NET_ADVERTISE_ROOT_INSTANCE,
        1,
    );

    let mut active_ads: BTreeMap<String, DesiredService> = BTreeMap::new();
    let mut last_sync_check = stem::monotonic_ns();
    let mut last_ad_reconcile = stem::monotonic_ns();
    let mut last_ad_announce = stem::monotonic_ns();
    let mut last_sweep = stem::monotonic_ns();

    loop {
        let now_ns = stem::monotonic_ns();
        let publish_to_graph =
            thingsys::prop_get(nectar_svc_id, keys::NET_NECTAR_PUBLISH_DISCOVERY_TO_GRAPH)
                .unwrap_or(1)
                != 0;

        if udp_handle != 0 {
            let mut pkt_buf = [0u8; 1500];
            for _ in 0..4 {
                match udp_recv_from(api, resp_w, resp_r, udp_handle, &mut pkt_buf) {
                    Ok(Some((src_ip, _src_port, len))) => {
                        if let Some(pkt) = DnsPacket::parse(&pkt_buf[..len]) {
                            if publish_to_graph {
                                ingest_packet_into_graph(&pkt, &pkt_buf[..len], &mut graph);
                            }
                            if pkt.is_query() {
                                let host_ip = net_ipv4_from_stack(net_stack);
                                let ads = active_ads.values().cloned().collect::<Vec<_>>();
                                maybe_respond_to_query(
                                    api, resp_w, resp_r, udp_handle, &pkt, &ads, host_ip, src_ip,
                                );
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(_) => break,
                }
            }
        }

        if now_ns.saturating_sub(last_sync_check) > 1_000_000_000 {
            last_sync_check = now_ns;
            window_id = maybe_create_window(window_id, &hostname, last_ip);
            let current_hostname = get_or_generate_hostname(mac);
            let current_ip = net_ipv4_from_stack(net_stack);
            if current_hostname != hostname || current_ip != last_ip {
                hostname = current_hostname;
                last_ip = current_ip;
                if let Some(win) = window_id {
                    let _ = render_window(win, &hostname, last_ip);
                }
            }
        }

        if now_ns.saturating_sub(last_sweep) > 5_000_000_000 {
            last_sweep = now_ns;
            if publish_to_graph {
                graph.sweep_stale(stem::time::now_unix_seconds());
            }
        }

        if now_ns.saturating_sub(last_ad_reconcile) > 2_000_000_000 {
            last_ad_reconcile = now_ns;
            let desired = load_desired_advertisements(advertise_root, &hostname);
            let desired_keys = desired
                .iter()
                .map(DesiredService::stable_key)
                .collect::<Vec<_>>();
            let active_keys = active_ads.keys().cloned().collect::<Vec<_>>();
            let (to_publish, to_unpublish) = reconcile_desired(&desired_keys, &active_keys);

            let desired_map = desired
                .into_iter()
                .map(|d| (d.stable_key(), d))
                .collect::<BTreeMap<_, _>>();

            let host_ip = net_ipv4_from_stack(net_stack);
            for key in &to_publish {
                if let Some(svc) = desired_map.get(key) {
                    send_announcement(
                        api,
                        resp_w,
                        resp_r,
                        udp_handle,
                        svc,
                        host_ip,
                        DEFAULT_ANNOUNCE_TTL,
                    );
                    active_ads.insert(key.clone(), svc.clone());
                }
            }

            for key in &to_unpublish {
                if let Some(svc) = active_ads.get(key) {
                    send_announcement(api, resp_w, resp_r, udp_handle, svc, host_ip, 0);
                }
                active_ads.remove(key);
            }

            for (k, svc) in desired_map {
                active_ads.insert(k, svc);
            }
        }

        if now_ns.saturating_sub(last_ad_announce) > 10_000_000_000 {
            last_ad_announce = now_ns;
            let host_ip = net_ipv4_from_stack(net_stack);
            for svc in active_ads.values() {
                send_announcement(
                    api,
                    resp_w,
                    resp_r,
                    udp_handle,
                    svc,
                    host_ip,
                    DEFAULT_ANNOUNCE_TTL,
                );
            }
        }

        stem::yield_now();
    }
}

fn ingest_packet_into_graph(packet: &DnsPacket, raw: &[u8], graph: &mut DiscoveryGraph) {
    for rr in packet.all_records() {
        match rr.rtype {
            12 => {
                if let Some(target) = parse_ptr_target(rr, raw) {
                    let type_fqdn = canonical_name(&rr.name);
                    graph.observe_service_type(&type_fqdn, rr.ttl);
                    graph.observe_instance(&canonical_name(&target), &type_fqdn, rr.ttl);
                }
            }
            33 => {
                if let Some(srv) = parse_srv(rr, raw) {
                    graph.observe_srv(
                        &canonical_name(&rr.name),
                        &canonical_name(&srv.target),
                        srv.port,
                        rr.ttl,
                        srv.priority,
                        srv.weight,
                    );
                }
            }
            16 => {
                let kvs = parse_txt_kvs(rr);
                graph.observe_txt(&canonical_name(&rr.name), &rr.data, &kvs, rr.ttl);
            }
            1 => {
                if let Some(ip) = parse_a(rr) {
                    let ip_s = format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
                    graph.observe_addr(&canonical_name(&rr.name), &ip_s, rr.ttl);
                }
            }
            28 => {
                if let Some(ip6) = parse_aaaa(rr) {
                    let ip_s = format_ipv6(&ip6);
                    graph.observe_addr(&canonical_name(&rr.name), &ip_s, rr.ttl);
                }
            }
            _ => {}
        }
    }
}

fn maybe_respond_to_query(
    api: PortHandle,
    resp_w: PortHandle,
    resp_r: PortHandle,
    udp_handle: u32,
    query: &DnsPacket,
    ads: &[DesiredService],
    host_ip: Option<[u8; 4]>,
    _src_ip: [u8; 4],
) {
    if ads.is_empty() || udp_handle == 0 {
        return;
    }

    let mut resp = DnsPacket::new_response(query.transaction_id);
    resp.flags = 0x8400;
    let mut seen = BTreeSet::new();

    for q in &query.questions {
        let qname = canonical_name(&q.name);
        let qtype = q.qtype;
        for svc in ads {
            let svc_type = canonical_name(&svc.service_type_fqdn());
            let instance = canonical_name(&svc.instance_fqdn());
            let host = canonical_name(&svc.host_fqdn());

            if (qtype == 12 || qtype == 255) && qname == svc_type {
                let k = format!("ptr:{}:{}", svc_type, instance);
                if seen.insert(k) {
                    resp.answers.push(DnsResourceRecord {
                        name: svc_type.clone(),
                        rtype: 12,
                        rclass: 0x0001,
                        ttl: DEFAULT_ANNOUNCE_TTL,
                        data: encode_ptr_rdata(&instance),
                    });
                }
            }
            if (qtype == 33 || qtype == 255) && qname == instance {
                let k = format!("srv:{}:{}", instance, svc.port);
                if seen.insert(k) {
                    resp.answers.push(DnsResourceRecord {
                        name: instance.clone(),
                        rtype: 33,
                        rclass: 0x0001,
                        ttl: DEFAULT_ANNOUNCE_TTL,
                        data: encode_srv_rdata(0, 0, svc.port, &host),
                    });
                }
            }
            if (qtype == 16 || qtype == 255) && qname == instance {
                let k = format!("txt:{}", instance);
                if seen.insert(k) {
                    resp.answers.push(DnsResourceRecord {
                        name: instance.clone(),
                        rtype: 16,
                        rclass: 0x0001,
                        ttl: DEFAULT_ANNOUNCE_TTL,
                        data: encode_txt_rdata(&svc.txt_items),
                    });
                }
            }
            if (qtype == 1 || qtype == 255) && qname == host {
                if let Some(ip) = host_ip {
                    let k = format!("a:{}", host);
                    if seen.insert(k) {
                        resp.answers.push(DnsResourceRecord {
                            name: host.clone(),
                            rtype: 1,
                            rclass: 0x0001,
                            ttl: DEFAULT_ANNOUNCE_TTL,
                            data: ip.to_vec(),
                        });
                    }
                }
            }
        }
    }

    if !resp.answers.is_empty() {
        let data = resp.encode();
        let _ = udp_send_to(
            api,
            resp_w,
            resp_r,
            udp_handle,
            [224, 0, 0, 251],
            MDNS_PORT,
            &data,
        );
    }
}

fn send_announcement(
    api: PortHandle,
    resp_w: PortHandle,
    resp_r: PortHandle,
    udp_handle: u32,
    svc: &DesiredService,
    host_ip: Option<[u8; 4]>,
    ttl: u32,
) {
    if udp_handle == 0 {
        return;
    }

    let service_type = svc.service_type_fqdn();
    let instance = svc.instance_fqdn();
    let host = svc.host_fqdn();

    let mut packet = DnsPacket::new_response(0);
    packet.answers.push(DnsResourceRecord {
        name: service_type.clone(),
        rtype: 12,
        rclass: 0x0001,
        ttl,
        data: encode_ptr_rdata(&instance),
    });
    packet.answers.push(DnsResourceRecord {
        name: instance.clone(),
        rtype: 33,
        rclass: 0x0001,
        ttl,
        data: encode_srv_rdata(0, 0, svc.port, &host),
    });
    packet.answers.push(DnsResourceRecord {
        name: instance,
        rtype: 16,
        rclass: 0x0001,
        ttl,
        data: encode_txt_rdata(&svc.txt_items),
    });

    if let Some(ip) = host_ip {
        packet.answers.push(DnsResourceRecord {
            name: host,
            rtype: 1,
            rclass: 0x0001,
            ttl,
            data: ip.to_vec(),
        });
    }

    let data = packet.encode();
    let _ = udp_send_to(
        api,
        resp_w,
        resp_r,
        udp_handle,
        [224, 0, 0, 251],
        MDNS_PORT,
        &data,
    );
}

fn load_desired_advertisements(root: ThingId, default_hostname: &str) -> Vec<DesiredService> {
    let mut out = Vec::new();
    let rel_id = thingsys::intern(rels::NET_WANTS_ADVERTISED).unwrap_or(0) as u64;

    let mut edges = [abi::types::Edge::default(); 256];
    if let Ok(count) = thingsys::get_edges(root, &mut edges) {
        for edge in edges.iter().take(count) {
            if edge.predicate.to_u64_lossy() != rel_id {
                continue;
            }

            let id = edge.to;
            let instance_name = read_string_prop(id, keys::NET_DESIRED_INSTANCE_NAME)
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "thingos".to_string());
            let service_type = read_string_prop(id, keys::NET_DESIRED_SERVICE_TYPE)
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "_http._tcp".to_string());
            let domain = read_string_prop(id, keys::NET_DESIRED_DOMAIN)
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "local".to_string());
            let hostname = read_string_prop(id, keys::NET_DESIRED_HOSTNAME)
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| default_hostname.to_string());
            let port = thingsys::prop_get(id, keys::NET_DESIRED_PORT)
                .ok()
                .and_then(|v| u16::try_from(v).ok())
                .unwrap_or(80);

            let txt_text = read_string_prop(id, keys::NET_DESIRED_TXT).unwrap_or_default();
            let txt_items = parse_desired_txt(&txt_text);

            out.push(DesiredService {
                service_type,
                domain,
                instance_name,
                port,
                txt_items,
                hostname,
            });
        }
    }

    out
}

fn parse_desired_txt(txt: &str) -> Vec<String> {
    txt.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn find_netd_and_mac() -> Option<(PortHandle, [u8; 6], ThingId)> {
    let mut buf = [ThingId::default(); 16];
    let n = thingsys::find("svc.net.Stack", &mut buf).ok()?;
    if n == 0 {
        return None;
    }
    let net_id = choose_best_net_stack(&buf[..n]).unwrap_or(buf[0]);
    let api_port = thingsys::prop_get(net_id, keys::WRITE_PORT_HANDLE).ok()? as PortHandle;
    let mac_packed = thingsys::prop_get(net_id, "net.mac").ok()?;
    let mac = [
        (mac_packed & 0xFF) as u8,
        ((mac_packed >> 8) & 0xFF) as u8,
        ((mac_packed >> 16) & 0xFF) as u8,
        ((mac_packed >> 24) & 0xFF) as u8,
        ((mac_packed >> 32) & 0xFF) as u8,
        ((mac_packed >> 40) & 0xFF) as u8,
    ];
    Some((api_port, mac, net_id))
}

fn choose_best_net_stack(nodes: &[ThingId]) -> Option<ThingId> {
    let mut best: Option<ThingId> = None;
    for &id in nodes {
        let socket_api = thingsys::prop_get(id, keys::WRITE_PORT_HANDLE).ok().unwrap_or(0);
        let ip = thingsys::prop_get(id, "net.ip").ok().unwrap_or(0);
        if socket_api == 0 {
            continue;
        }
        if ip != 0 {
            return Some(id);
        }
        if best.is_none() {
            best = Some(id);
        }
    }
    best
}

fn net_ipv4_from_stack(net_id: ThingId) -> Option<[u8; 4]> {
    let packed = thingsys::prop_get(net_id, "net.ip").ok()?;
    if packed == 0 {
        return None;
    }
    Some([
        (packed & 0xFF) as u8,
        ((packed >> 8) & 0xFF) as u8,
        ((packed >> 16) & 0xFF) as u8,
        ((packed >> 24) & 0xFF) as u8,
    ])
}

fn get_or_generate_hostname(mac: [u8; 6]) -> String {
    let mut host_buf = [ThingId::default(); 1];
    if let Ok(1) = thingsys::find(kinds::DEV_HOST, &mut host_buf) {
        let host_id = host_buf[0];
        if let Some(name) = read_string_prop(host_id, keys::NAME).filter(|s| !s.is_empty()) {
            return name;
        }
    }

    let descriptors = [
        "quiet", "vibrant", "ancient", "sunny", "dewy", "wild", "silver", "golden", "mossy",
        "blooming",
    ];
    let plants = [
        "moss", "fern", "ivy", "clover", "petal", "thistle", "willow", "cedar", "birch", "maple",
    ];
    let seed = mac.iter().fold(0u32, |acc, &x| acc.wrapping_add(x as u32));
    let d_idx = (seed as usize) % descriptors.len();
    let p_idx = (seed as usize / descriptors.len()) % plants.len();
    format!("{}-{}", descriptors[d_idx], plants[p_idx])
}

fn maybe_create_window(current: Option<ThingId>, hostname: &str, ip: Option<[u8; 4]>) -> Option<ThingId> {
    if current.is_some() {
        return current;
    }

    let mut crowns = [ThingId::default(); 1];
    let crown = match thingsys::find(kinds::UI_CROWN, &mut crowns) {
        Ok(n) if n > 0 => crowns[0],
        _ => return None,
    };

    let win = match thingsys::create_node(kinds::UI_WINDOW) {
        Ok(id) => id,
        Err(_) => return None,
    };

    thingsys::link(win, rels::CHILD_OF, crown).ok();
    thingsys::link(crown, rels::HAS_CHILD, win).ok();
    thingsys::prop_set(win, keys::UI_BG_COLOR, 0xFFF0EDE8).ok();
    thingsys::prop_set(win, keys::UI_WIDTH, 360).ok();
    thingsys::prop_set(win, keys::UI_HEIGHT, 120).ok();
    thingsys::prop_set(win, keys::UI_X, 0).ok();
    thingsys::prop_set(win, keys::UI_Y, 0).ok();
    thingsys::prop_set(win, keys::UI_INSET_RIGHT, 16).ok();
    thingsys::prop_set(win, keys::UI_INSET_TOP, 16).ok();
    thingsys::prop_set(win, keys::UI_MANUAL_POSITION, 1).ok();
    let _ = render_window(win, hostname, ip);
    Some(win)
}

fn render_window(window_id: ThingId, hostname: &str, ip: Option<[u8; 4]>) -> Result<(), stem::errors::Error> {
    let ip_str = match ip {
        Some([a, b, c, d]) => format!("{}.{}.{}.{}", a, b, c, d),
        None => "Acquiring…".into(),
    };
    let mut ui = Petals::begin_window(window_id);
    let root = ui.column(|ui| {
        let label = ui.text("Hostname")?;
        let _ = ui.set_font_name(label, "NotoSans-Regular");
        let _ = ui.set_font_size(label, 14);
        let _ = ui.set_color(label, 0xFF505050);
        let value = ui.text(hostname)?;
        let _ = ui.set_font_name(value, "NotoSans-Regular");
        let _ = ui.set_font_size(value, 24);
        let _ = ui.set_color(value, 0xFF1A5080);

        let ip_label = ui.text("IP Address")?;
        let _ = ui.set_font_name(ip_label, "NotoSans-Regular");
        let _ = ui.set_font_size(ip_label, 14);
        let _ = ui.set_color(ip_label, 0xFF505050);
        let ip_value = ui.text(&ip_str)?;
        let _ = ui.set_font_name(ip_value, "NotoSans-Regular");
        let _ = ui.set_font_size(ip_value, 24);
        let _ = ui.set_color(ip_value, 0xFF1A5080);
        Ok(())
    })?;
    let _ = ui.set_gap(root, 6);
    let _ = ui.set_padding(root, 16);
    ui.finish()?;
    Ok(())
}

fn ensure_singleton_node(kind: &str, key_name: &str, key_value: u64) -> ThingId {
    let mut ids = [ThingId::default(); 64];
    if let Ok(count) = thingsys::find(kind, &mut ids) {
        for id in ids.iter().copied().take(count) {
            if thingsys::prop_get(id, key_name).unwrap_or(0) == key_value {
                return id;
            }
        }
    }
    let id = thingsys::create_node(kind).expect("singleton node");
    thingsys::prop_set(id, key_name, key_value).ok();
    id
}

fn ensure_edge(src: ThingId, rel: &str, dst: ThingId) {
    stem::net::ensure_edge(src, rel, dst).ok();
}

fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
    if value.is_empty() {
        thingsys::prop_set(id, key_name, 0).ok();
        return;
    }
    let bs_id = thingsys::bytespace_create(value.len(), 0, 0).expect("create bytespace");
    thingsys::bytespace_write(bs_id, 0, value.as_bytes()).ok();
    thingsys::prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

fn set_bytes_prop(id: ThingId, key_name: &str, value: &[u8]) {
    if value.is_empty() {
        thingsys::prop_set(id, key_name, 0).ok();
        return;
    }
    let bs_id = thingsys::bytespace_create(value.len(), 0, 0).expect("create bytespace");
    thingsys::bytespace_write(bs_id, 0, value).ok();
    thingsys::prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

fn read_string_prop(id: ThingId, key_name: &str) -> Option<String> {
    let raw = thingsys::prop_get(id, key_name).ok()?;
    if raw == 0 {
        return None;
    }
    let bs = ThingId::from_u64(raw);
    let len = thingsys::bytespace_info(bs).ok()?;
    if len == 0 {
        return Some(String::new());
    }
    let mut buf = Vec::with_capacity(len);
    buf.resize(len, 0);
    let n = thingsys::bytespace_read(bs, 0, &mut buf).ok()?;
    buf.truncate(n);
    Some(String::from_utf8_lossy(&buf).into_owned())
}

fn append_socket_api_header(msg: &mut Vec<u8>, resp_w: PortHandle, msg_type: u16, payload_len: u16) {
    msg.extend_from_slice(&(resp_w as u32).to_le_bytes());
    let caller_tid = stem::syscall::get_tid().unwrap_or(0);
    msg.extend_from_slice(&caller_tid.to_le_bytes());
    msg.extend_from_slice(&msg_type.to_le_bytes());
    msg.extend_from_slice(&payload_len.to_le_bytes());
}

fn send_socket_api_msg(api: PortHandle, msg: &[u8]) -> Result<(), ()> {
    let deadline_ms = (stem::time::now().as_millis() as u64).saturating_add(5_000);
    loop {
        match port_send_all(api, msg) {
            Ok(n) if n == msg.len() => return Ok(()),
            Ok(n) => {
                stem::warn!(
                    "NECTAR: partial Socket API write (wrote {} of {} bytes), dropping request",
                    n,
                    msg.len()
                );
                return Err(());
            }
            Err(abi::errors::Errno::EAGAIN) => {
                if (stem::time::now().as_millis() as u64) >= deadline_ms {
                    break;
                }
                let _ = port_wait(&[api], abi::syscall::port_wait::WRITABLE);
            }
            Err(_) => return Err(()),
        }
    }
    stem::warn!(
        "NECTAR: timeout waiting for Socket API port space (need {} bytes)",
        msg.len()
    );
    Err(())
}

fn udp_bind(api: PortHandle, resp_w: PortHandle, resp_r: PortHandle, port: u16) -> Result<u32, ()> {
    let mut msg = Vec::new();
    append_socket_api_header(&mut msg, resp_w, MSG_UDP_BIND, 2);
    msg.extend_from_slice(&port.to_le_bytes());
    send_socket_api_msg(api, &msg)?;
    let mut resp = [0u8; 128];
    for _ in 0..100 {
        if let Ok(len) = port_recv(resp_r, &mut resp) {
            if len >= 6 && u16::from_le_bytes([resp[0], resp[1]]) == RESP_HANDLE {
                return Ok(u32::from_le_bytes([resp[2], resp[3], resp[4], resp[5]]));
            }
        }
        stem::time::sleep_ms(10);
    }
    Err(())
}

fn net_join_multicast(
    api: PortHandle,
    resp_w: PortHandle,
    resp_r: PortHandle,
    ip: [u8; 4],
) -> Result<(), ()> {
    let mut msg = Vec::new();
    append_socket_api_header(&mut msg, resp_w, MSG_NET_JOIN_MULTICAST, 4);
    msg.extend_from_slice(&ip);
    send_socket_api_msg(api, &msg)?;
    let mut resp = [0u8; 64];
    for _ in 0..10 {
        if port_recv(resp_r, &mut resp).is_ok() {
            break;
        }
        stem::time::sleep_ms(5);
    }
    Ok(())
}

fn udp_recv_from(
    api: PortHandle,
    resp_w: PortHandle,
    resp_r: PortHandle,
    handle: u32,
    buf: &mut [u8],
) -> Result<Option<([u8; 4], u16, usize)>, ()> {
    let mut msg = Vec::new();
    append_socket_api_header(&mut msg, resp_w, MSG_UDP_RECV_FROM, 4);
    msg.extend_from_slice(&handle.to_le_bytes());
    let _ = send_socket_api_msg(api, &msg);

    let mut resp = [0u8; 2048];
    match port_recv(resp_r, &mut resp) {
        Ok(len) if len >= 2 => {
            let resp_type = u16::from_le_bytes([resp[0], resp[1]]);
            if resp_type == RESP_EMPTY {
                return Ok(None);
            }
            if resp_type == RESP_DATA && len >= 8 {
                let mut ip = [0u8; 4];
                ip.copy_from_slice(&resp[2..6]);
                let port = u16::from_le_bytes([resp[6], resp[7]]);
                let data_len = core::cmp::min(len - 8, buf.len());
                buf[..data_len].copy_from_slice(&resp[8..8 + data_len]);
                return Ok(Some((ip, port, data_len)));
            }
            Ok(None)
        }
        Err(abi::errors::Errno::EAGAIN) => Ok(None),
        Err(_) => Err(()),
        _ => Ok(None),
    }
}

fn udp_send_to(
    api: PortHandle,
    resp_w: PortHandle,
    resp_r: PortHandle,
    handle: u32,
    ip: [u8; 4],
    port: u16,
    data: &[u8],
) -> Result<(), ()> {
    let mut msg = Vec::new();
    append_socket_api_header(&mut msg, resp_w, MSG_UDP_SEND_TO, 6 + data.len() as u16);
    msg.extend_from_slice(&handle.to_le_bytes());
    msg.extend_from_slice(&ip);
    msg.extend_from_slice(&port.to_le_bytes());
    msg.extend_from_slice(data);

    if msg.len() > 4096 {
        stem::warn!("NECTAR: UDP IPC message too large (len={}), would be truncated", msg.len());
        return Err(());
    }

    send_socket_api_msg(api, &msg)?;

    let mut resp = [0u8; 64];
    for _ in 0..20 {
        if let Ok(len) = port_recv(resp_r, &mut resp) {
            if len >= 2 && u16::from_le_bytes([resp[0], resp[1]]) == RESP_OK {
                return Ok(());
            }
        }
        stem::time::sleep_ms(2);
    }
    Err(())
}

fn split_service_type(type_fqdn: &str) -> (String, String) {
    let clean = canonical_name(type_fqdn);
    if let Some((left, right)) = clean.rsplit_once('.') {
        (left.to_string(), right.to_string())
    } else {
        (clean, "local".to_string())
    }
}

fn infer_type_fqdn_from_instance(instance_fqdn: &str) -> Option<&str> {
    let clean = instance_fqdn.trim_end_matches('.');
    clean.split_once('.').map(|(_, rest)| rest)
}

fn canonical_name(name: &str) -> String {
    name.trim().trim_end_matches('.').to_ascii_lowercase()
}

fn hash64(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn hash64_bytes(data: &[u8]) -> u64 {
    hash64(data)
}

fn format_ipv6(bytes: &[u8; 16]) -> String {
    let mut parts = [0u16; 8];
    for i in 0..8 {
        parts[i] = u16::from_be_bytes([bytes[i * 2], bytes[i * 2 + 1]]);
    }
    format!(
        "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
        parts[0], parts[1], parts[2], parts[3], parts[4], parts[5], parts[6], parts[7]
    )
}
