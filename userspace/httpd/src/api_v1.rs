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
use stem::syscall::graph::{prop_get, prop_set, intern};
use stem::thing::sys::get_edges;
use stem::thing::ThingId;
use stem::thing::HandleId; // for from_u64
use abi::types::Edge;

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
    
    let root = match root_id {
        Some(id) => id,
        None => return error_response(ApiError::bad_request("Missing required 'root' parameter")),
    };
    
    // BFS traversal
    let mut visited: alloc::vec::Vec<u64> = alloc::vec::Vec::new();
    let mut edges_out: alloc::vec::Vec<(u64, u64, u64)> = alloc::vec::Vec::new(); // (from, to, rel_sym)
    let mut queue: alloc::collections::VecDeque<(u64, u32)> = alloc::collections::VecDeque::new();
    let mut truncated = false;
    
    queue.push_back((root, 0));
    
    while let Some((node_id, node_depth)) = queue.pop_front() {
        if visited.contains(&node_id) {
            continue;
        }
        if visited.len() >= max_nodes {
            truncated = true;
            break;
        }
        visited.push(node_id);
        
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
                    if !visited.contains(&dst) {
                        queue.push_back((dst, node_depth + 1));
                    }
                }
            }
        }
    }
    
    // Build JSON response
    let mut json = JsonBuilder::new();
    json.start_object();
    
    json.key("root");
    json.number_value(root);
    
    // Nodes array
    json.key("nodes");
    json.start_array();
    for node_id in &visited {
        json.start_object();
        json.key("id");
        json.number_value(*node_id);
        
        // Get kind
        let kind_id = stem::syscall::graph::get_kind(*node_id).unwrap_or(0);
        json.key("kind");
        json.number_value(kind_id);
        
        // Label heuristic - try NAME first, then shortened ID
        let label = get_node_label(*node_id);
        json.key("label");
        json.string_value(&label);
        
        // Layout positions from existing LAYOUT_POS_X/Y (shared with Photosynthesis)
        let layout_x_sym = intern("layout.pos.x").unwrap_or(0);
        let layout_y_sym = intern("layout.pos.y").unwrap_or(0);
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
        // Only include edges where both endpoints are in visited set
        if visited.contains(from) && visited.contains(to) {
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
    json.number_value(visited.len() as u64);
    json.key("edges");
    json.number_value(edges_out.len() as u64);
    json.end_object();
    
    json.end_object();
    
    json_response("200 OK", &json.as_string().unwrap_or_default())
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
        ApiRoute::GetBytespace { thing_id, key } => handle_get_bytespace(thing_id, key, req),
        ApiRoute::GetBytespaceMetadata { thing_id, key } => handle_bytespace_meta(thing_id, key),
        ApiRoute::PutBytespace { thing_id, key } => handle_put_bytespace(thing_id, key, body),
        ApiRoute::ResolvePath { path } => handle_path_resolve(path),
        ApiRoute::Watch => handle_watch(req),
        ApiRoute::GetSubgraph { query } => handle_get_subgraph(query),
        ApiRoute::PatchLayout => handle_patch_layout(body),
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
