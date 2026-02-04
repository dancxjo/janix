//! API v1 Endpoint Handlers
//!
//! Implements all /api/v1/... endpoints for the Graph REST API.

#![allow(dead_code)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::error::{ApiError, ApiErrorCode};
use crate::graph_api::{self, GraphError, JsonBuilder};
use crate::http::{self, Request};
use crate::router::ApiRoute;
use stem::error;
use stem::syscall::graph::{prop_get, prop_set, intern, link, find};
use stem::thing::sys::{get_edges, get_props};
use stem::thing::ThingId;
use stem::thing::HandleId; // for from_u64
use abi::types::Edge;
use abi::schema::{keys, kinds, rels};

// ============================================================================
// API Limits (constants)
// ============================================================================

/// Maximum size of JSON request body (POST/PATCH)
pub const MAX_JSON_BODY: usize = 64 * 1024;

/// Maximum bytespace upload size
pub const MAX_BYTESPACE_UPLOAD: usize = 4 * 1024 * 1024;

/// Maximum bytespace stream without Range header
pub const MAX_BYTESPACE_STREAM: usize = 1024 * 1024;

/// API version string
pub const API_VERSION: &str = "1.0.0";

/// Maximum subgraph nodes
pub const MAX_SUBGRAPH_NODES: usize = 500;

/// Default subgraph depth
pub const DEFAULT_SUBGRAPH_DEPTH: u32 = 2;

// ============================================================================
// Response Helpers
// ============================================================================

/// Build HTTP response with JSON content type
pub fn json_response(status: &str, body: &str) -> Vec<u8> {
    crate::build_response(status, "application/json", body.as_bytes())
}

/// Build error response
pub fn error_response(err: ApiError) -> Vec<u8> {
    json_response(err.http_status(), &err.to_json())
}

// ============================================================================
// Endpoint Handlers
// ============================================================================

/// GET /api/v1/
/// Returns API capabilities and schema versions
pub fn handle_discovery() -> Vec<u8> {
    let mut json = JsonBuilder::new();
    json.start_object();
    
    json.key("api_version");
    json.string_value(API_VERSION);
    
    json.key("schema_version");
    json.string_value("0.2.0");
    
    json.key("limits");
    json.start_object();
    json.key("max_json_body");
    json.number_value(MAX_JSON_BODY as u64);
    json.key("max_bytespace_upload");
    json.number_value(MAX_BYTESPACE_UPLOAD as u64);
    json.key("max_bytespace_stream");
    json.number_value(MAX_BYTESPACE_STREAM as u64);
    json.end_object();
    json.buf.push(b',');
    
    json.key("endpoints");
    json.start_array();
    json.string_value("/api/v1/things");
    json.string_value("/api/v1/things/{id}");
    json.string_value("/api/v1/things/{id}/bytespaces/{key}");
    json.string_value("/api/v1/things/{id}/launch");
    json.string_value("/api/v1/path/{path}");
    json.string_value("/api/v1/watch");
    json.end_array();
    
    json.end_object();
    
    json_response("200 OK", &json.as_string().unwrap_or_default())
}

/// GET /api/v1/things/{id}
/// Returns full Thing representation with props and links
pub fn handle_get_thing(id_str: &str) -> Vec<u8> {
    let id = match parse_thing_id(id_str) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    
    match graph_api::thing_to_json(id) {
        Ok(json) => json_response("200 OK", &json),
        Err(GraphError::NotFound) => {
            error_response(ApiError::not_found(format!("Thing {} not found", id)))
        }
        Err(_) => {
            error_response(ApiError::internal("Failed to fetch thing"))
        }
    }
}

/// POST /api/v1/things
/// Create a new thing
pub fn handle_create_thing(_body: &[u8]) -> Vec<u8> {
    // TODO: Parse JSON body for kind_id and initial props
    // For now, return not implemented
    error_response(ApiError::new(
        ApiErrorCode::InternalError,
        "Create thing not yet implemented",
    ))
}

/// DELETE /api/v1/things/{id}
pub fn handle_delete_thing(id_str: &str) -> Vec<u8> {
    let _id = match parse_thing_id(id_str) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    
    // TODO: Implement thing deletion when kernel supports it
    error_response(ApiError::new(
        ApiErrorCode::InternalError,
        "Delete thing not yet implemented",
    ))
}

/// PATCH /api/v1/things/{id}
/// Update thing properties
pub fn handle_patch_thing(id_str: &str, _body: &[u8]) -> Vec<u8> {
    let _id = match parse_thing_id(id_str) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    
    // TODO: Parse JSON body for set/clear operations
    error_response(ApiError::new(
        ApiErrorCode::InternalError,
        "Patch thing not yet implemented",
    ))
}

/// GET /api/v1/things/{id}/props
/// Returns all properties of a Thing as JSON
pub fn handle_get_thing_props(id_str: &str) -> Vec<u8> {
    let id = match parse_thing_id(id_str) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    
    // Check if the thing exists by getting its kind
    let kind_id = match stem::syscall::graph::get_kind(id) {
        Ok(k) if k != 0 => k,
        _ => return error_response(ApiError::not_found(format!("Thing {} not found", id))),
    };

    const INITIAL_PROP_CAP: usize = 128;
    const MAX_PROP_CAP: usize = 4096;

    let mut props_buf = alloc::vec![abi::types::GraphProp::default(); INITIAL_PROP_CAP];
    let mut prop_count = get_props(ThingId::from_u64(id), &mut props_buf).unwrap_or(0);
    let mut cap = INITIAL_PROP_CAP;

    while prop_count == cap && cap < MAX_PROP_CAP {
        cap = (cap * 2).min(MAX_PROP_CAP);
        props_buf.resize(cap, abi::types::GraphProp::default());
        prop_count = get_props(ThingId::from_u64(id), &mut props_buf).unwrap_or(0);
        if prop_count < cap {
            break;
        }
    }

    let truncated = prop_count == cap && cap == MAX_PROP_CAP;
    let prop_count = core::cmp::min(prop_count, cap);
    props_buf.truncate(prop_count);

    use alloc::collections::BTreeMap;
    let mut symbol_cache: BTreeMap<u32, String> = BTreeMap::new();
    
    let mut json = JsonBuilder::new();
    json.start_object();
    
    // Thing ID
    json.key("thing_id");
    json.number_value(id);
    
    // Kind
    json.key("kind_id");
    json.number_value(kind_id);
    
    // Resolve kind name
    json.key("kind_name");
    let kind_name = get_symbol_name_cached(kind_id as u32, &mut symbol_cache);
    json.string_value(&kind_name);
    
    // Properties object
    json.key("props");
    json.start_object();

    for prop in &props_buf {
        let key_name = get_symbol_name_cached(prop.key, &mut symbol_cache);
        let val = prop.value;
        json.key(&key_name);

        let is_layout_key = key_name.starts_with("layout.") || key_name.starts_with("ui.layout.");
        let is_float = is_layout_key
            && (key_name.ends_with(".x")
                || key_name.ends_with(".y")
                || key_name.ends_with(".w")
                || key_name.ends_with(".h")
                || key_name.ends_with(".z"));

        if is_float {
            let f = f32::from_bits(val as u32);
            json.float_value(f);
        } else if key_name.contains("text")
            || key_name.contains("name")
            || key_name.contains("path")
            || key_name.contains("title")
            || key_name.contains("label")
            || key_name.contains("tag")
            || key_name.contains("source")
            || key_name.contains("icon")
        {
            if val <= u32::MAX as u64 {
                let mut buf = [0u8; 128];
                if let Ok(len) = stem::thing::sys::describe_symbol(val as u32, &mut buf) {
                    if len > 0 {
                        if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                            json.string_value(s);
                            continue;
                        }
                    }
                }
            }
            json.number_value(val);
        } else {
            json.number_value(val);
        }
    }

    json.end_object();
    json.buf.push(b',');

    json.key("prop_count");
    json.number_value(prop_count as u64);

    json.key("truncated");
    if truncated {
        json.buf.extend_from_slice(b"true,");
    } else {
        json.buf.extend_from_slice(b"false,");
    }

    let launch_info = get_launch_info(id, kind_id);
    json.key("launchable");
    if launch_info.launchable {
        json.buf.extend_from_slice(b"true,");
    } else {
        json.buf.extend_from_slice(b"false,");
    }
    if let Some(name) = launch_info.name.as_ref() {
        json.key("launch_name");
        json.string_value(name);
    }
    if let Some(reason) = launch_info.reason.as_ref() {
        json.key("launch_reason");
        json.string_value(reason);
    }

    json.end_object();
    
    json_response("200 OK", &json.as_string().unwrap_or_default())
}

/// GET /api/v1/things/{id}/bytespaces/{key}
pub fn handle_get_bytespace(thing_id_str: &str, key: &str, req: &Request<'_>) -> Vec<u8> {
    let _thing_id = match parse_thing_id(thing_id_str) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    
    let bytespace_id = match parse_thing_id(key) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    // Check for Range header
    let range = http::parse_range_header(req);
    let (offset, limit) = match range {
        Some(r) => {
            let end = r.end.unwrap_or(MAX_BYTESPACE_STREAM + r.start);
            (r.start, end.saturating_sub(r.start).min(MAX_BYTESPACE_STREAM))
        }
        None => (0, MAX_BYTESPACE_STREAM),
    };
    
    match graph_api::read_bytespace_ranged(bytespace_id, offset, limit) {
        Ok(data) => {
            crate::build_response("200 OK", "application/octet-stream", &data)
        }
        Err(GraphError::NotFound) => {
            error_response(ApiError::not_found(format!("Bytespace {} not found", key)))
        }
        Err(_) => {
            error_response(ApiError::internal("Failed to read bytespace"))
        }
    }
}

/// GET /api/v1/things/{id}/bytespaces/{key}/meta
pub fn handle_bytespace_meta(thing_id_str: &str, key: &str) -> Vec<u8> {
    let _thing_id = match parse_thing_id(thing_id_str) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    
    let bytespace_id = match parse_thing_id(key) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    
    match graph_api::bytespace_meta(bytespace_id) {
        Ok(meta) => {
            let mut json = JsonBuilder::new();
            json.start_object();
            json.key("size");
            json.number_value(meta.size as u64);
            json.key("content_type");
            json.string_value("application/octet-stream");
            json.end_object();
            
            json_response("200 OK", &json.as_string().unwrap_or_default())
        }
        Err(GraphError::NotFound) => {
            error_response(ApiError::not_found(format!("Bytespace {} not found", key)))
        }
        Err(_) => {
            error_response(ApiError::internal("Failed to get bytespace info"))
        }
    }
}

/// PUT /api/v1/things/{id}/bytespaces/{key}
pub fn handle_put_bytespace(thing_id_str: &str, _key: &str, body: &[u8]) -> Vec<u8> {
    let _thing_id = match parse_thing_id(thing_id_str) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    
    if body.len() > MAX_BYTESPACE_UPLOAD {
        return error_response(ApiError::payload_too_large(
            format!("Bytespace size {} exceeds limit {}", body.len(), MAX_BYTESPACE_UPLOAD)
        ));
    }
    
    // TODO: Implement bytespace create/write
    error_response(ApiError::new(
        ApiErrorCode::InternalError,
        "Put bytespace not yet implemented",
    ))
}

/// GET /api/v1/path/{path}
pub fn handle_path_resolve(path: &str) -> Vec<u8> {
    // TODO: Implement path resolution via graph find
    let _ = path;
    error_response(ApiError::new(
        ApiErrorCode::InternalError,
        "Path resolution not yet implemented",
    ))
}

/// GET /api/v1/watch
/// Returns SSE stream of graph events
pub fn handle_watch(_req: &Request<'_>) -> Vec<u8> {
    // TODO: Implement SSE streaming (requires persistent connection)
    error_response(ApiError::new(
        ApiErrorCode::InternalError,
        "Watch SSE not yet implemented",
    ))
}

/// GET /api/v1/things/{id}/launch
/// Returns whether the Thing is launchable as a process.
pub fn handle_get_launch_info(id_str: &str) -> Vec<u8> {
    let id = match parse_thing_id(id_str) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    let kind_id = match stem::syscall::graph::get_kind(id) {
        Ok(k) if k != 0 => k,
        _ => return error_response(ApiError::not_found(format!("Thing {} not found", id))),
    };

    let launch_info = get_launch_info(id, kind_id);

    let mut json = JsonBuilder::new();
    json.start_object();
    json.key("thing_id");
    json.number_value(id);
    json.key("launchable");
    if launch_info.launchable {
        json.buf.extend_from_slice(b"true,");
    } else {
        json.buf.extend_from_slice(b"false,");
    }
    if let Some(name) = launch_info.name.as_ref() {
        json.key("launch_name");
        json.string_value(name);
    }
    if let Some(reason) = launch_info.reason.as_ref() {
        json.key("launch_reason");
        json.string_value(reason);
    }
    json.end_object();

    json_response("200 OK", &json.as_string().unwrap_or_default())
}

/// POST /api/v1/things/{id}/launch
/// Launch a boot.Module via spawn_process and record a LAUNCHED edge.
pub fn handle_launch(id_str: &str) -> Vec<u8> {
    let id = match parse_thing_id(id_str) {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    let kind_id = match stem::syscall::graph::get_kind(id) {
        Ok(k) if k != 0 => k,
        _ => return error_response(ApiError::not_found(format!("Thing {} not found", id))),
    };

    let launch_info = get_launch_info(id, kind_id);
    if !launch_info.launchable {
        let reason = launch_info
            .reason
            .unwrap_or_else(|| "Thing is not executable".into());
        return error_response(ApiError::unprocessable(reason));
    }

    let name = match launch_info.name {
        Some(n) => n,
        None => return error_response(ApiError::unprocessable("Missing module name")),
    };

    let host_id = match find_host_id() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    let pid = match stem::syscall::spawn_process(&name, 0) {
        Ok(pid) => pid,
        Err(e) => {
            return error_response(ApiError::internal(format!(
                "spawn_process failed: {:?}",
                e
            )))
        }
    };

    let launched_at = stem::time::monotonic_ns();

    let mut graph_linked = false;
    let mut graph_error: Option<String> = None;

    if let Ok(rel_id) = intern(rels::LAUNCHED) {
        if let Err(e) = link(host_id, rel_id, id) {
            graph_error = Some(format!("LAUNCHED link failed: {:?}", e));
        } else {
            graph_linked = true;
        }
    } else {
        graph_error = Some("Failed to intern LAUNCHED".into());
    }

    if let Ok(key_id) = intern(keys::LAUNCH_AT) {
        let _ = prop_set(id, key_id, launched_at);
    }

    let mut json = JsonBuilder::new();
    json.start_object();
    json.key("thing_id");
    json.number_value(id);
    json.key("launch_name");
    json.string_value(&name);
    json.key("pid");
    json.number_value(pid);
    json.key("host_id");
    json.number_value(host_id);
    json.key("launched_at");
    json.number_value(launched_at);
    json.key("graph_linked");
    if graph_linked {
        json.buf.extend_from_slice(b"true,");
    } else {
        json.buf.extend_from_slice(b"false,");
    }
    if let Some(err) = graph_error.as_ref() {
        json.key("graph_error");
        json.string_value(err);
    }
    json.end_object();

    json_response("200 OK", &json.as_string().unwrap_or_default())
}

/// Handle route not found
pub fn handle_not_found() -> Vec<u8> {
    error_response(ApiError::not_found("API endpoint not found"))
}

/// Handle method not allowed
pub fn handle_method_not_allowed() -> Vec<u8> {
    error_response(ApiError::method_not_allowed("Method not allowed for this endpoint"))
}

// ============================================================================
// Subgraph Endpoint
// ============================================================================

/// GET /api/v1/subgraph?root=...&depth=...&max_nodes=...
pub fn handle_get_subgraph(query: &str) -> Vec<u8> {
    // Parse query parameters
    let mut root_id: Option<u64> = None;
    let mut depth: u32 = DEFAULT_SUBGRAPH_DEPTH;
    let mut max_nodes: usize = MAX_SUBGRAPH_NODES;
    
    for part in query.split('&') {
        if let Some((key, value)) = part.split_once('=') {
            match key {
                "root" => root_id = value.parse().ok(),
                "depth" => depth = value.parse().unwrap_or(DEFAULT_SUBGRAPH_DEPTH).min(10),
                "max_nodes" => max_nodes = value.parse().unwrap_or(MAX_SUBGRAPH_NODES).min(MAX_SUBGRAPH_NODES),
                _ => {}
            }
        }
    }
    
    // Pre-intern all symbols we'll need - avoids syscalls in hot loops
    let layout_x_sym = intern("layout.pos.x").unwrap_or(0);
    let layout_y_sym = intern("layout.pos.y").unwrap_or(0);
    let name_syms: [u64; 4] = [
        intern("name").unwrap_or(0),
        intern("ui.title").unwrap_or(0),
        intern("asset.name").unwrap_or(0),
        intern("file.name").unwrap_or(0),
    ];
    
    // BFS traversal - use BTreeSet for O(log n) membership checks
    use alloc::collections::BTreeSet;
    let mut visited_set: BTreeSet<u64> = BTreeSet::new();
    let mut visited_order: alloc::vec::Vec<u64> = alloc::vec::Vec::new();  // Preserve order for JSON
    let mut edges_out: alloc::vec::Vec<(u64, u64, u64)> = alloc::vec::Vec::new(); // (from, to, rel_sym)
    let mut queue: alloc::collections::VecDeque<(u64, u32)> = alloc::collections::VecDeque::new();
    let mut truncated = false;
    
    // Symbol cache for relationship names (avoids redundant describe_symbol calls)
    use alloc::collections::BTreeMap;
    let mut symbol_cache: BTreeMap<u32, String> = BTreeMap::new();
    
    // If root is specified, use it. Otherwise, auto-discover from interesting kinds (like Photosynthesis)
    if let Some(root) = root_id {
        queue.push_back((root, 0));
    } else {
        // Auto-discovery mode: seed from diverse system nodes to show full graph variety
        use stem::thing::sys::find;
        use abi::schema::kinds;
        
        // Comprehensive list of interesting kinds to seed from
        // This ensures the subgraph shows all types of nodes, not just mem.Range
        let interesting_kinds = [
            // Core system
            kinds::SVC_ROOT,              // System root node
            kinds::PROC_KERNEL,           // Kernel process
            kinds::SVC_SCHEDULER,         // Scheduler service
            // Device/Hardware
            kinds::DEV_HOST,              // Host device
            kinds::DEV_BUS_PCI,           // PCI bus
            kinds::DEV_PCI_FUNCTION,      // PCI devices
            kinds::DEV_NET_NIC,           // Network cards
            kinds::DEV_STORAGE_DISK,      // Disks
            kinds::DEV_DISPLAY_GPU,       // GPU
            kinds::DEV_CPU,               // CPUs
            // UI
            kinds::UI_CROWN,              // UI crown (desktop)
            kinds::UI_WINDOW,             // Windows
            // Fonts
            kinds::FONT_FAMILY,           // Font families
            kinds::FONT_FACE,             // Font faces
            // Content/Files
            kinds::CONTENT_SOURCE,        // Content sources
            kinds::CONTENT_FILE,          // Files
            kinds::BOOT_MODULE,           // Boot modules
            // Services
            "svc.net.Stack",              // Network stack
            "svc.net.Driver",             // Network driver
            // Assets
            kinds::ASSET,                 // Assets
        ];
        
        for kind_name in &interesting_kinds {
            let mut ids = [ThingId::default(); 32];
            if let Ok(count) = find(*kind_name, &mut ids) {
                for i in 0..count {
                    let id = ids[i].to_u64_lossy();
                    if !visited_set.contains(&id) {
                        queue.push_back((id, 0));
                    }
                }
            }
        }
    }
    
    while let Some((node_id, node_depth)) = queue.pop_front() {
        if visited_set.contains(&node_id) {
            continue;
        }
        if visited_order.len() >= max_nodes {
            truncated = true;
            break;
        }
        visited_set.insert(node_id);
        visited_order.push(node_id);
        
        // Get outgoing edges if we haven't reached max depth
        if node_depth < depth {
            let mut edge_buf = [Edge::default(); 64];
            let thing_id = ThingId::from_u64(node_id);
            if let Ok(count) = get_edges(thing_id, &mut edge_buf) {
                for i in 0..count {
                    let e = &edge_buf[i];
                    let dst = e.to.to_u64_lossy();
                    let rel = e.predicate.to_u64_lossy();
                    edges_out.push((node_id, dst, rel));
                    if !visited_set.contains(&dst) {
                        queue.push_back((dst, node_depth + 1));
                    }
                }
            }
        }
    }
    
    // Build JSON response
    let mut json = JsonBuilder::new();
    json.start_object();
    
    // Root field - either the specified root or null for auto-discovery
    json.key("root");
    if let Some(root) = root_id {
        json.number_value(root);
    } else {
        json.buf.extend_from_slice(b"null,");
    }
    
    // Nodes array
    json.key("nodes");
    json.start_array();
    for node_id in &visited_order {
        json.start_object();
        json.key("id");
        json.number_value(*node_id);
        
        // Get kind
        let kind_id = stem::syscall::graph::get_kind(*node_id).unwrap_or(0);
        json.key("kind");
        json.number_value(kind_id);
        
        // Resolve kind to string name (cached)
        let kind_name = get_symbol_name_cached(kind_id as u32, &mut symbol_cache);
        json.key("kind_name");
        json.string_value(&kind_name);
        
        // Label heuristic - try NAME first, then shortened ID (using pre-interned symbols)
        let label = get_node_label_fast(*node_id, &name_syms);
        json.key("label");
        json.string_value(&label);

        // Explicit name property if available
        if name_syms[0] != 0 {
            if let Ok(val) = prop_get(*node_id, name_syms[0]) {
                if val != 0 {
                    if let Some(name) = resolve_symbol_value(val) {
                        json.key("name");
                        json.string_value(&name);
                    }
                }
            }
        }
        
        // Layout positions from existing LAYOUT_POS_X/Y (shared with Photosynthesis)
        if layout_x_sym != 0 && layout_y_sym != 0 {
            if let (Ok(x_bits), Ok(y_bits)) = (prop_get(*node_id, layout_x_sym), prop_get(*node_id, layout_y_sym)) {
                if x_bits != 0 || y_bits != 0 {
                    // Stored as f32 bits
                    let x = f32::from_bits(x_bits as u32);
                    let y = f32::from_bits(y_bits as u32);
                    json.key("x");
                    json.float_value(x);
                    json.key("y");
                    json.float_value(y);
                }
            }
        }
        
        json.end_object();
        json.buf.push(b',');
    }
    json.end_array();
    json.buf.push(b',');
    
    // Edges array
    json.key("edges");
    json.start_array();
    for (from, to, rel_sym) in &edges_out {
        // Only include edges where both endpoints are in visited set (O(log n) check)
        if visited_set.contains(from) && visited_set.contains(to) {
            json.start_object();
            
            // Edge ID
            json.key("id");
            let edge_id = format!("e:{}->{}:{}", from, to, rel_sym);
            json.string_value(&edge_id);
            
            json.key("from");
            json.number_value(*from);
            
            json.key("to");
            json.number_value(*to);
            
            json.key("rel");
            json.number_value(*rel_sym);
            
            // Resolve relationship name (cached)
            let rel_name = get_symbol_name_cached(*rel_sym as u32, &mut symbol_cache);
            json.key("rel_name");
            json.string_value(&rel_name);
            
            json.end_object();
            json.buf.push(b',');
        }
    }
    json.end_array();
    json.buf.push(b',');
    
    // Truncated flag
    json.key("truncated");
    if truncated {
        json.buf.extend_from_slice(b"true,");
    } else {
        json.buf.extend_from_slice(b"false,");
    }
    
    // Stats
    json.key("stats");
    json.start_object();
    json.key("depth");
    json.number_value(depth as u64);
    json.key("nodes");
    json.number_value(visited_order.len() as u64);
    json.key("edges");
    json.number_value(edges_out.len() as u64);
    json.end_object();
    
    json.end_object();
    
    json_response("200 OK", &json.as_string().unwrap_or_default())
}

/// Fast label lookup using pre-interned symbols
fn get_node_label_fast(node_id: u64, name_syms: &[u64; 4]) -> String {
    for &sym in name_syms {
        if sym == 0 { continue; }
        if let Ok(val) = prop_get(node_id, sym) {
            if val != 0 {
                // Try to resolve as interned string
                let mut buf = [0u8; 64];
                if let Ok(len) = stem::thing::sys::describe_symbol(val as u32, &mut buf) {
                    if len > 0 {
                        if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                            return String::from(s);
                        }
                    }
                }
            }
        }
    }
    
    // Fallback: shortened ID
    let id_str = format!("{}", node_id);
    if id_str.len() > 8 {
        format!("...{}", &id_str[id_str.len()-8..])
    } else {
        id_str
    }
}

// ============================================================================
// Views System
// ============================================================================

struct ViewDef {
    id: &'static str,
    title: &'static str,
    description: &'static str,
    query: &'static str,
}

const AVAILABLE_VIEWS: &[ViewDef] = &[
    ViewDef {
        id: "task_monitor",
        title: "Task Monitor",
        description: "Processes, threads, and scheduling state",
        query: "VIEW task_monitor",
    },
    ViewDef {
        id: "device_census",
        title: "Device Census",
        description: "Hardware devices and drivers",
        query: "VIEW device_census",
    },
    ViewDef {
        id: "graph_islands",
        title: "Graph Islands",
        description: "Disconnected components analysis",
        query: "VIEW graph_islands",
    },
];

/// GET /api/v1/views
pub fn handle_list_views() -> Vec<u8> {
    let mut json = JsonBuilder::new();
    json.start_array();
    
    for view in AVAILABLE_VIEWS {
        json.start_object();
        json.key("id");
        json.string_value(view.id);
        json.key("title");
        json.string_value(view.title);
        json.key("description");
        json.string_value(view.description);
        json.key("query");
        json.string_value(view.query);
        json.end_object();
        json.buf.push(b',');
    }
    
    json.end_array();
    json_response("200 OK", &json.as_string().unwrap_or_default())
}

/// GET /api/v1/views/{id}
pub fn handle_get_view(id: &str, query: &str) -> Vec<u8> {
    let json_body = handle_get_view_body(id, query);
    json_response("200 OK", &json_body)
}

/// Helper to get just the JSON body for a view
pub fn handle_get_view_body(id: &str, query: &str) -> String {
    match id {
        "task_monitor" => handle_view_task_monitor(query),
        "device_census" => handle_view_device_census(query),
        "graph_islands" => handle_view_graph_islands(query),
        _ => {
            let mut json = JsonBuilder::new();
            json.start_object();
            json.key("error");
            json.string_value(&alloc::format!("View '{}' not found", id));
            json.end_object();
            json.as_string().unwrap_or_default()
        },
    }
}

fn handle_view_task_monitor(_query: &str) -> String {
    // Task Monitor: Scheduler -> CPUs -> Tasks -> Threads
    // Layout: LR
    
    let mut json = JsonBuilder::new();
    json.start_object();
    
    json.key("view");
    json.start_object();
    json.key("id");
    json.string_value("task_monitor");
    json.key("title");
    json.string_value("Task Monitor");
    json.end_object();
    json.buf.push(b',');

    // Hints
    json.key("hints");
    json.start_object();
    json.key("layout");
    json.start_object();
    json.key("engine");
    json.string_value("elk");
    json.key("direction");
    json.string_value("RIGHT"); // Left-to-Right layout
    json.end_object();
    json.end_object();
    json.buf.push(b',');

    // Collect seeds
    let mut seeds = Vec::new();
    let mut ids = [0u64; 32];
    
    // Debug logging
    error!("VIEW[task_monitor]: collecting seeds...");

    if let Ok(kind) = intern(kinds::SVC_SCHEDULER) {
        if let Ok(count) = find(kind, &mut ids) {
            if count > 0 { seeds.push(ids[0]); }
        } else { error!("find(SVC_SCHEDULER) failed"); }
    } else { error!("intern(SVC_SCHEDULER) failed"); }

    if let Ok(kind) = intern(kinds::PROC_KERNEL) {
        if let Ok(count) = find(kind, &mut ids) {
             if count > 0 { seeds.push(ids[0]); }
        }
    }
    if let Ok(kind) = intern(kinds::DEV_CPU) {
        if let Ok(count) = find(kind, &mut ids) {
            for i in 0..count {
                seeds.push(ids[i]);
            }
        }
    }

    error!("VIEW[task_monitor]: seeds count = {}", seeds.len());

    let (nodes, edges) = traverse_view(seeds, 3, 200);
    error!("VIEW[task_monitor]: nodes={} edges={}", nodes.len(), edges.len());
    
    write_graph_json(&mut json, nodes, edges);
    json.end_object();
    json.as_string().unwrap_or_default()
}

fn handle_view_device_census(_query: &str) -> String {
    // Device Census: Host -> Buses -> Devices -> Drivers
    // Layout: TB
    
    let mut json = JsonBuilder::new();
    json.start_object();
    
    json.key("view");
    json.start_object();
    json.key("id");
    json.string_value("device_census");
    json.key("title");
    json.string_value("Device Census");
    json.end_object();
    json.buf.push(b',');

    // Hints
    json.key("hints");
    json.start_object();
    json.key("layout");
    json.start_object();
    json.key("engine");
    json.string_value("elk");
    json.key("direction");
    json.string_value("DOWN"); // Top-Down layout
    json.end_object();
    json.end_object();
    json.buf.push(b',');

    let mut seeds = Vec::new();
    let mut ids = [0u64; 32];
    
    if let Ok(kind) = intern(kinds::DEV_HOST) {
        if let Ok(count) = find(kind, &mut ids) {
            if count > 0 { seeds.push(ids[0]); }
        }
    }
    
    let (nodes, edges) = traverse_view(seeds, 4, 300);
    write_graph_json(&mut json, nodes, edges);
    json.end_object();
    json.as_string().unwrap_or_default()
}

fn handle_view_graph_islands(query: &str) -> String {
    let mut root = 0;
    for part in query.split('&') {
        if let Some((key, value)) = part.split_once('=') {
            if key == "root" {
                root = value.parse().unwrap_or(0);
            }
        }
    }
    
    let mut seeds = Vec::new();
    if root != 0 {
        seeds.push(root);
    } else {
        let mut ids = [0u64; 1];
        if let Ok(kind) = intern(kinds::SVC_ROOT) {
            if let Ok(count) = find(kind, &mut ids) {
               if count > 0 { seeds.push(ids[0]); }
            }
        }
    }
    
    let mut json = JsonBuilder::new();
    json.start_object();
    json.key("view");
    json.start_object();
    json.key("id");
    json.string_value("graph_islands");
    json.key("title");
    json.string_value("Graph Islands");
    json.end_object();
    json.buf.push(b',');
    
    json.key("hints");
    json.start_object();
    json.end_object();
    json.buf.push(b',');

    let (nodes, edges) = traverse_view(seeds, 3, 200);
    write_graph_json(&mut json, nodes, edges);
    json.end_object();
    json.as_string().unwrap_or_default()
}

struct GraphNode {
    id: u64,
    kind: u64,
    label: String,
    x: Option<f32>,
    y: Option<f32>,
}

struct GraphEdge {
    from: u64,
    to: u64,
    rel: u64,
}

fn traverse_view(seeds: Vec<u64>, depth: u32, max_nodes: usize) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    let layout_x_sym = intern("layout.pos.x").unwrap_or(0);
    let layout_y_sym = intern("layout.pos.y").unwrap_or(0);
    let name_syms: [u64; 4] = [
        intern("name").unwrap_or(0),
        intern("ui.title").unwrap_or(0),
        intern("asset.name").unwrap_or(0),
        intern("file.name").unwrap_or(0),
    ];
    
    use alloc::collections::BTreeSet;
    let mut visited_set: BTreeSet<u64> = BTreeSet::new();
    let mut visited_order: Vec<u64> = Vec::new();
    let mut edges_out: Vec<GraphEdge> = Vec::new();
    let mut queue: alloc::collections::VecDeque<(u64, u32)> = alloc::collections::VecDeque::new();
    
    for seed in seeds {
        if !visited_set.contains(&seed) {
            queue.push_back((seed, 0));
        }
    }
    
    while let Some((node_id, node_depth)) = queue.pop_front() {
        if visited_set.contains(&node_id) {
            continue;
        }
        if visited_order.len() >= max_nodes {
            break;
        }
        visited_set.insert(node_id);
        visited_order.push(node_id);
        
        if node_depth < depth {
            let mut edge_buf = [Edge::default(); 64];
            let thing_id = ThingId::from_u64(node_id);
            if let Ok(count) = get_edges(thing_id, &mut edge_buf) {
                for i in 0..count {
                    let e = &edge_buf[i];
                    let dst = e.to.to_u64_lossy();
                    let rel = e.predicate.to_u64_lossy();
                    
                    edges_out.push(GraphEdge { from: node_id, to: dst, rel });
                    if !visited_set.contains(&dst) {
                        queue.push_back((dst, node_depth + 1));
                    }
                }
            }
        }
    }
    
    let mut nodes = Vec::new();
    for node_id in visited_order {
        let kind_id = stem::syscall::graph::get_kind(node_id).unwrap_or(0);
        let label = get_node_label_fast(node_id, &name_syms);
        
        let mut x = None;
        let mut y = None;
        
        if layout_x_sym != 0 && layout_y_sym != 0 {
            if let (Ok(x_bits), Ok(y_bits)) = (prop_get(node_id, layout_x_sym), prop_get(node_id, layout_y_sym)) {
                if x_bits != 0 || y_bits != 0 {
                    x = Some(f32::from_bits(x_bits as u32));
                    y = Some(f32::from_bits(y_bits as u32));
                }
            }
        }
        
        nodes.push(GraphNode { id: node_id, kind: kind_id, label, x, y });
    }
    
    // Filter edges to ensure both ends exist in our node set
    let valid_edges = edges_out.into_iter()
        .filter(|e| visited_set.contains(&e.from) && visited_set.contains(&e.to))
        .collect();
        
    (nodes, valid_edges)
}

fn write_graph_json(json: &mut JsonBuilder, nodes: Vec<GraphNode>, edges: Vec<GraphEdge>) {
    use alloc::collections::BTreeMap;
    let mut symbol_cache: BTreeMap<u32, String> = BTreeMap::new();

    json.key("nodes");
    json.start_array();
    for node in nodes {
        json.start_object();
        json.key("id");
        json.number_value(node.id);
        json.key("kind");
        json.number_value(node.kind);
        
        let kind_name = get_symbol_name_cached(node.kind as u32, &mut symbol_cache);
        json.key("kind_name");
        json.string_value(&kind_name);
        
        json.key("label");
        json.string_value(&node.label);
        
        if let Some(v) = node.x {
            json.key("x");
            json.float_value(v);
        }
        if let Some(v) = node.y {
            json.key("y");
            json.float_value(v);
        }
        json.end_object();
        json.buf.push(b',');
    }
    json.end_array();
    json.buf.push(b',');
    
    json.key("edges");
    json.start_array();
    for edge in edges {
        json.start_object();
        let edge_id = format!("e:{}->{}:{}", edge.from, edge.to, edge.rel);
        json.key("id");
        json.string_value(&edge_id);
        json.key("from");
        json.number_value(edge.from);
        json.key("to");
        json.number_value(edge.to);
        json.key("rel");
        json.number_value(edge.rel);
        
        let rel_name = get_symbol_name_cached(edge.rel as u32, &mut symbol_cache);
        json.key("rel_name");
        json.string_value(&rel_name);
        
        json.end_object();
        json.buf.push(b',');
    }
    
    json.end_array();
}

/// Cached symbol name resolution
fn get_symbol_name_cached(sym_id: u32, cache: &mut alloc::collections::BTreeMap<u32, String>) -> String {
    if sym_id == 0 {
        return String::from("unknown");
    }
    if let Some(cached) = cache.get(&sym_id) {
        return cached.clone();
    }
    let name = get_symbol_name(sym_id);
    cache.insert(sym_id, name.clone());
    name
}

fn get_node_label(node_id: u64) -> String {
    // Try common name keys
    let name_keys = ["name", "ui.title", "asset.name", "file.name"];
    for key in name_keys {
        if let Ok(sym) = intern(key) {
            if let Ok(val) = prop_get(node_id, sym) {
                if val != 0 {
                    // Try to resolve as interned string
                    let mut buf = [0u8; 64];
                    if let Ok(len) = stem::thing::sys::describe_symbol(val as u32, &mut buf) {
                        if len > 0 {
                            if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                                return String::from(s);
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fallback: shortened ID
    let id_str = format!("{}", node_id);
    if id_str.len() > 8 {
        format!("...{}", &id_str[id_str.len()-8..])
    } else {
        id_str
    }
}

fn get_symbol_name(sym_id: u32) -> String {
    if sym_id == 0 {
        return String::from("unknown");
    }
    let mut buf = [0u8; 128];
    if let Ok(len) = stem::thing::sys::describe_symbol(sym_id, &mut buf) {
        if len > 0 {
            if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                return String::from(s);
            }
        }
    }
    format!("sym:{}", sym_id)
}

// ============================================================================
// Layout Endpoint
// ============================================================================

/// PATCH /api/v1/layout
/// Bulk update node positions (shared with Photosynthesis via LAYOUT_POS_X/Y)
pub fn handle_patch_layout(body: &[u8]) -> Vec<u8> {
    // Simple JSON parsing for layout updates
    // Expected: { "space": "graph_ui_v1", "nodes": [{ "id": "...", "x": 1.0, "y": 2.0 }] }
    
    let body_str = match core::str::from_utf8(body) {
        Ok(s) => s,
        Err(_) => return error_response(ApiError::bad_request("Invalid UTF-8 in body")),
    };
    
    // Get layout key symbols (use existing Photosynthesis keys)
    let layout_x_sym = match intern("layout.pos.x") {
        Ok(s) => s,
        Err(_) => return error_response(ApiError::internal("Failed to intern layout.pos.x")),
    };
    let layout_y_sym = match intern("layout.pos.y") {
        Ok(s) => s,
        Err(_) => return error_response(ApiError::internal("Failed to intern layout.pos.y")),
    };
    
    // Parse nodes from JSON (simple extraction)
    let mut saved = 0u64;
    
    // Find "nodes" array and parse each entry
    if let Some(nodes_start) = body_str.find("\"nodes\":") {
        let rest = &body_str[nodes_start..];
        if let Some(arr_start) = rest.find('[') {
            let arr_content = &rest[arr_start + 1..];
            // Parse each node object
            let mut pos = 0;
            while pos < arr_content.len() {
                // Find next object
                if let Some(obj_start) = arr_content[pos..].find('{') {
                    let obj_pos = pos + obj_start;
                    if let Some(obj_end) = arr_content[obj_pos..].find('}') {
                        let obj = &arr_content[obj_pos..obj_pos + obj_end + 1];
                        
                        // Extract id, x, y from object
                        if let (Some(id), Some(x), Some(y)) = (
                            extract_json_string(obj, "id").and_then(|s| s.parse::<u64>().ok()),
                            extract_json_number(obj, "x"),
                            extract_json_number(obj, "y"),
                        ) {
                            // Store as f32 bits in u64
                            let x_bits = (x as f32).to_bits() as u64;
                            let y_bits = (y as f32).to_bits() as u64;
                            
                            if prop_set(id, layout_x_sym, x_bits).is_ok() 
                               && prop_set(id, layout_y_sym, y_bits).is_ok() {
                                saved += 1;
                            }
                        }
                        
                        pos = obj_pos + obj_end + 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
    
    let mut json = JsonBuilder::new();
    json.start_object();
    json.key("saved");
    json.number_value(saved);
    json.end_object();
    
    json_response("200 OK", &json.as_string().unwrap_or_default())
}

// Simple JSON value extractors
fn extract_json_string<'a>(obj: &'a str, key: &str) -> Option<&'a str> {
    let key_pattern = format!("\"{}\":", key);
    let start = obj.find(&key_pattern)? + key_pattern.len();
    let rest = obj[start..].trim_start();
    if rest.starts_with('"') {
        let inner = &rest[1..];
        let end = inner.find('"')?;
        Some(&inner[..end])
    } else {
        // Might be a number without quotes
        let end = rest.find(|c: char| c == ',' || c == '}' || c.is_whitespace())?;
        Some(&rest[..end])
    }
}

fn extract_json_number(obj: &str, key: &str) -> Option<f64> {
    let key_pattern = format!("\"{}\":", key);
    let start = obj.find(&key_pattern)? + key_pattern.len();
    let rest = obj[start..].trim_start();
    let end = rest.find(|c: char| c == ',' || c == '}' || c.is_whitespace())?;
    rest[..end].parse().ok()
}

// ============================================================================
// Launch Helpers
// ============================================================================

struct LaunchInfo {
    launchable: bool,
    name: Option<String>,
    reason: Option<String>,
}

impl LaunchInfo {
    fn not(reason: impl Into<String>) -> Self {
        Self {
            launchable: false,
            name: None,
            reason: Some(reason.into()),
        }
    }
}

fn get_launch_info(thing_id: u64, kind_id: u64) -> LaunchInfo {
    let boot_kind = match intern(kinds::BOOT_MODULE) {
        Ok(id) => id,
        Err(_) => return LaunchInfo::not("Failed to resolve boot.Module kind"),
    };

    if kind_id != boot_kind {
        return LaunchInfo {
            launchable: false,
            name: None,
            reason: None,
        };
    }

    let name_key = match intern(keys::NAME) {
        Ok(id) => id,
        Err(_) => return LaunchInfo::not("Failed to resolve module name key"),
    };
    let bytespace_key = match intern(keys::BYTESPACE) {
        Ok(id) => id,
        Err(_) => return LaunchInfo::not("Failed to resolve bytespace key"),
    };

    let name_val = prop_get(thing_id, name_key).unwrap_or(0);
    let name = resolve_symbol_value(name_val);
    if name.is_none() {
        return LaunchInfo::not("Missing module name");
    }

    let bytespace_id = prop_get(thing_id, bytespace_key).unwrap_or(0);
    if bytespace_id == 0 {
        return LaunchInfo::not("Missing bytespace");
    }

    match bytespace_is_elf(bytespace_id) {
        Ok(true) => LaunchInfo {
            launchable: true,
            name,
            reason: None,
        },
        Ok(false) => LaunchInfo::not("Bytespace is not an ELF"),
        Err(_) => LaunchInfo::not("Failed to read bytespace"),
    }
}

fn bytespace_is_elf(bytespace_id: u64) -> Result<bool, GraphError> {
    let data = graph_api::read_bytespace_ranged(bytespace_id, 0, 4)?;
    Ok(data.len() >= 4 && data[0] == 0x7F && data[1] == b'E' && data[2] == b'L' && data[3] == b'F')
}

fn resolve_symbol_value(value: u64) -> Option<String> {
    if value == 0 || value > u32::MAX as u64 {
        return None;
    }
    let mut buf = [0u8; 256];
    if let Ok(len) = stem::thing::sys::describe_symbol(value as u32, &mut buf) {
        if len > 0 {
            if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                return Some(String::from(s));
            }
        }
    }
    None
}

fn find_host_id() -> Result<u64, ApiError> {
    let host_kind = intern(kinds::DEV_HOST)
        .map_err(|_| ApiError::internal("Failed to resolve dev.Host kind"))?;
    let mut ids = [0u64; 4];
    let count = find(host_kind, &mut ids)
        .map_err(|_| ApiError::internal("Failed to find dev.Host"))?;
    if count == 0 {
        return Err(ApiError::not_found("dev.Host not found"));
    }
    Ok(ids[0])
}

// ============================================================================
// GQL Query Endpoints
// ============================================================================

/// POST /api/v1/query
/// Execute a GQL query from request body
pub fn handle_execute_gql_query(body: &[u8]) -> Vec<u8> {
    // Convert body to string
    let query = match core::str::from_utf8(body) {
        Ok(s) => s,
        Err(_) => return error_response(ApiError::bad_request("Invalid UTF-8 in request body")),
    };
    
    // Execute the query
    let result = crate::gql_handler::handle_gql_post(query);
    
    // Return as JSON
    crate::build_response("200 OK", "application/json", &result)
}

/// GET /api/v1/query?q=...
/// Execute a GQL query from query parameter
pub fn handle_execute_gql_query_get(query_string: &str) -> Vec<u8> {
    // Parse query parameter
    let mut gql_query = "";
    for part in query_string.split('&') {
        if let Some((key, value)) = part.split_once('=') {
            if key == "q" {
                gql_query = value;
                break;
            }
        }
    }
    
    if gql_query.is_empty() {
        return error_response(ApiError::bad_request("Missing 'q' query parameter"));
    }
    
    // URL decode would go here if needed
    // For now, assume it's already decoded
    
    // Execute the query
    let result = crate::gql_handler::handle_gql_get(gql_query);
    
    // Return as JSON
    crate::build_response("200 OK", "application/json", &result)
}


// ============================================================================
// Dispatch
// ============================================================================

/// Dispatch a request to the appropriate API handler
pub fn dispatch(route: ApiRoute<'_>, req: &Request<'_>, body: &[u8]) -> Vec<u8> {
    match route {
        ApiRoute::Discovery => handle_discovery(),
        ApiRoute::GetThing { id } => handle_get_thing(id),
        ApiRoute::CreateThing => handle_create_thing(body),
        ApiRoute::DeleteThing { id } => handle_delete_thing(id),
        ApiRoute::PatchThing { id } => handle_patch_thing(id, body),
        ApiRoute::GetThingProps { id } => handle_get_thing_props(id),
        ApiRoute::GetBytespace { thing_id, key } => handle_get_bytespace(thing_id, key, req),
        ApiRoute::GetBytespaceMetadata { thing_id, key } => handle_bytespace_meta(thing_id, key),
        ApiRoute::PutBytespace { thing_id, key } => handle_put_bytespace(thing_id, key, body),
        ApiRoute::GetLaunchInfo { id } => handle_get_launch_info(id),
        ApiRoute::Launch { id } => handle_launch(id),
        ApiRoute::ResolvePath { path } => handle_path_resolve(path),
        ApiRoute::Watch => handle_watch(req),
        ApiRoute::GetSubgraph { query } => handle_get_subgraph(query),
        ApiRoute::PatchLayout => handle_patch_layout(body),
        ApiRoute::ExecuteGqlQuery => handle_execute_gql_query(body),
        ApiRoute::ExecuteGqlQueryGet { query } => handle_execute_gql_query_get(query),
        ApiRoute::ListViews => handle_list_views(),
        ApiRoute::GetView { id, query } => handle_get_view(id, query),
        ApiRoute::NotFound => handle_not_found(),
        ApiRoute::MethodNotAllowed => handle_method_not_allowed(),
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn parse_thing_id(s: &str) -> Result<u64, ApiError> {
    s.parse::<u64>()
        .map_err(|_| ApiError::bad_request(format!("Invalid thing ID: {}", s)))
}
