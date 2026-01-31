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
