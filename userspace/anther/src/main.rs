#![no_std]
#![no_main]

extern crate alloc;

mod http;
mod api_v1;
mod upload;
mod router;
mod assets;
mod graph_api;
mod error;
mod net_client;
mod gql_handler;

use alloc::vec::Vec;
use stem::{info, warn, error};
use net_client::NetClient;

const SERVER_NAME: &str = "ThingOS-anther/0.1";

pub enum ResponseBody {
    Static(&'static [u8]),
    Owned(Vec<u8>),
}

impl ResponseBody {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            ResponseBody::Static(s) => s,
            ResponseBody::Owned(o) => o.as_slice(),
        }
    }
}

/// Build HTTP headers
fn build_headers(status: &str, content_type: &str, body_len: usize, keep_alive: bool) -> Vec<u8> {
    use alloc::format;
    let mut response = Vec::new();
    
    // Status line
    let status_line = format!("HTTP/1.1 {}\r\n", status);
    response.extend_from_slice(status_line.as_bytes());
    
    // Headers
    let server_hdr = format!("Server: {}\r\n", SERVER_NAME);
    response.extend_from_slice(server_hdr.as_bytes());
    
    let content_type_hdr = format!("Content-Type: {}\r\n", content_type);
    response.extend_from_slice(content_type_hdr.as_bytes());
    
    let content_len_hdr = format!("Content-Length: {}\r\n", body_len);
    response.extend_from_slice(content_len_hdr.as_bytes());
    
    if keep_alive {
        response.extend_from_slice(b"Connection: keep-alive\r\n");
    } else {
        response.extend_from_slice(b"Connection: close\r\n");
    }
    response.extend_from_slice(b"\r\n");
    
    response
}

/// Build HTTP response with headers
fn build_response(status: &str, content_type: &str, body: ResponseBody, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    let headers = build_headers(status, content_type, body.as_slice().len(), keep_alive);
    (headers, body)
}

/// Build HTTP redirect response
fn build_redirect(location: &str, is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    use alloc::format;
    let body: &[u8] = if is_head { &[] } else { b"" };
    let headers = build_headers("302 Found", "text/html", body.len(), keep_alive);
    
    let mut response = headers;
    // We need to insert the Location header before the final \r\n
    // This is a bit messy, let's fix build_headers to accept extra headers
    // Actually, let's just build it manually here for now
    let mut response = Vec::new();
    response.extend_from_slice(b"HTTP/1.1 302 Found\r\n");
    response.extend_from_slice(format!("Server: {}\r\n", SERVER_NAME).as_bytes());
    response.extend_from_slice(format!("Location: {}\r\n", location).as_bytes());
    response.extend_from_slice(format!("Content-Length: {}\r\n", body.len()).as_bytes());
    if keep_alive {
        response.extend_from_slice(b"Connection: keep-alive\r\n");
    } else {
        response.extend_from_slice(b"Connection: close\r\n");
    }
    response.extend_from_slice(b"\r\n");

    (response, ResponseBody::Static(body))
}

/// Handle a single HTTP request and return a response
fn handle_request(req: &http::Request<'_>, body: &[u8], keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    info!("anther: {} {} {:?}", 
          match req.method {
              http::Method::Get => "GET",
              http::Method::Head => "HEAD",
              http::Method::Post => "POST",
              http::Method::Put => "PUT",
              http::Method::Patch => "PATCH",
              http::Method::Delete => "DELETE",
              http::Method::Other => "OTHER",
          },
          req.path,
          req.version);
    
    if req.method == http::Method::Post {
        info!("anther: Request body size: {} bytes", body.len());
    }
    
    // Validate path
    let safe_path = match http::decode_path(req.path) {
        Some(p) => p,
        None => {
            let body = b"400 Bad Request: Invalid path\n";
            return build_response("400 Bad Request", "text/plain", ResponseBody::Static(body), keep_alive);
        }
    };
    
    // Route the request
    route_request(req, safe_path, body, keep_alive)
}

/// Route a request to the appropriate handler
fn route_request(req: &http::Request<'_>, path: &str, body: &[u8], keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    let method = req.method;
    let is_head = method == http::Method::Head;
    
    // Check if this is an API v1 request
    if router::is_api_v1_path(path) {
        if let Some(route) = router::match_route(method, path) {
            // For API routes, we need the request body (for POST/PUT/PATCH)
            let (status, api_resp) = api_v1::dispatch(route, req, body);
            return build_response(status, "application/json", ResponseBody::Owned(api_resp), keep_alive);
        }
    }
    
    // Legacy routes - only GET and HEAD allowed
    if method != http::Method::Get && method != http::Method::Head {
        let body = b"405 Method Not Allowed\n";
        return build_response("405 Method Not Allowed", "text/plain", ResponseBody::Static(body), keep_alive);
    }
    
    let asset_path = path.split('?').next().unwrap_or(path);
    if asset_path == "/graph.html" || asset_path == "/3d.html" {
        return build_redirect("/", is_head, keep_alive);
    }

    // Check for static assets first (strip query string)
    if let Some(asset) = assets::get_asset(asset_path) {
        let body: &[u8] = if is_head { &[] } else { asset.content };
        return build_response("200 OK", asset.content_type, ResponseBody::Static(body), keep_alive);
    }
    
    match path {
        "/health" => handle_health(is_head, keep_alive),
        "/upload" if req.method == http::Method::Post => {
            let (status, body) = upload::handle_upload_new(req, body);
            build_response(status, "application/json", ResponseBody::Owned(body), keep_alive)
        },
        "/graph" => handle_graph_index(is_head, keep_alive),
        p if p.starts_with("/graph/") => handle_graph_thing(p, is_head, keep_alive),
        _ => handle_404(is_head, keep_alive),
    }
}

/// GET /health
fn handle_health(is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    let body: &[u8] = if is_head { &[] } else { b"ok" };
    build_response("200 OK", "text/plain", ResponseBody::Static(body), keep_alive)
}

// Note: Index page is now served from embedded assets (assets.rs)

/// GET /graph
fn handle_graph_index(is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    // For now, return a simple JSON structure
    // TODO: enumerate actual graph roots/kinds
    let json = r#"{"message":"Graph index endpoint","note":"Use /graph/<thing_id> to query specific things"}"#;
    let body: &[u8] = if is_head { &[] } else { json.as_bytes() };
    build_response("200 OK", "application/json", ResponseBody::Static(body), keep_alive)
}

/// GET /graph/<thing_id> or /graph/<thing_id>/bytespace/<key>
fn handle_graph_thing(path: &str, is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    // Parse path like "/graph/123" or "/graph/123/bytespace/456"
    let path_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    
    if path_parts.len() < 2 {
        let body = b"400 Bad Request: Invalid graph path\n";
        return build_response("400 Bad Request", "text/plain", ResponseBody::Static(body), keep_alive);
    }
    
    // Parse thing_id
    let thing_id = match path_parts[1].parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            let body = b"400 Bad Request: Invalid thing ID\n";
            return build_response("400 Bad Request", "text/plain", ResponseBody::Static(body), keep_alive);
        }
    };
    
    // Check if this is a bytespace request
    if path_parts.len() >= 4 && path_parts[2] == "bytespace" {
        return handle_bytespace(thing_id, path_parts[3], is_head, keep_alive);
    }
    
    // Get the thing as JSON
    match graph_api::thing_to_json(thing_id) {
        Ok(json) => {
            let body = if is_head { Vec::new() } else { json.into_bytes() };
            build_response("200 OK", "application/json", ResponseBody::Owned(body), keep_alive)
        }
        Err(graph_api::GraphError::NotFound) => {
            let body = b"404 Not Found: Thing does not exist\n";
            build_response("404 Not Found", "text/plain", ResponseBody::Static(body), keep_alive)
        }
        Err(_) => {
            let body = b"500 Internal Server Error\n";
            build_response("500 Internal Server Error", "text/plain", ResponseBody::Static(body), keep_alive)
        }
    }
}

/// GET /graph/<thing_id>/bytespace/<key>
fn handle_bytespace(_thing_id: u64, key_str: &str, _is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    // TODO: Add size limit query parameter support (?size=N or ?range=N-M)
    // TODO: Add permission check for bytespace access
    
    // Parse the key as a bytespace ID
    let bytespace_id = match key_str.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            let body = b"400 Bad Request: Invalid bytespace ID\n";
            return build_response("400 Bad Request", "text/plain", ResponseBody::Static(body), keep_alive);
        }
    };
    
    // Try to read the bytespace
    match graph_api::read_bytespace(bytespace_id, 1024 * 1024) {
        Ok(data) => {
            // Return the raw bytespace data
            build_response("200 OK", "application/octet-stream", ResponseBody::Owned(data), keep_alive)
        }
        Err(graph_api::GraphError::NotFound) => {
            let body = b"404 Not Found: Bytespace does not exist\n";
            build_response("404 Not Found", "text/plain", ResponseBody::Static(body), keep_alive)
        }
        Err(_) => {
            let body = b"500 Internal Server Error\n";
            build_response("500 Internal Server Error", "text/plain", ResponseBody::Static(body), keep_alive)
        }
    }
}

/// 404 Not Found
fn handle_404(is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    let body: &[u8] = if is_head { &[] } else { b"404 Not Found\n" };
    build_response("404 Not Found", "text/plain", ResponseBody::Static(body), keep_alive)
}

/// Run in stdio mode: read request from stdin, write response to stdout
fn run_stdio_mode() -> ! {
    info!("anther: Running in stdio mode");
    
    // Read request from stdin (simulated via a buffer for now)
    let test_request = "GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let req = http::parse_request(test_request).unwrap();
    let (headers, body) = handle_request(&req, &[], false);
    
    // In stdio mode, we'd write to stdout here
    info!("anther: Response generated: headers={}, body={}", headers.len(), body.as_slice().len());
    
    // Exit after one request in stdio mode
    stem::syscall::exit(0);
}

/// Run in server mode - listen for TCP connections and serve HTTP
fn run_server_mode(port: u16) -> ! {
    info!("anther: Starting server mode on port {}...", port);

    // Wait for network stack to be ready
    let net = loop {
        match NetClient::connect() {
            Some(n) => break n,
            None => {
                info!("anther: Waiting for network stack...");
                stem::time::sleep_ms(500);
            }
        }
    };

    info!("anther: Connected to network stack");

    // Start listening
    let listen_handle = loop {
        match net.tcp_listen(port) {
            Some(h) => break h,
            None => {
                warn!("anther: Failed to listen on port {}, retrying...", port);
                stem::time::sleep_ms(1000);
            }
        }
    };

    info!("anther: Listening on port {} (handle={})", port, listen_handle);

    // Main server loop
    loop {
        // Try to accept a connection
        if let Some(accept) = net.tcp_accept(listen_handle) {
            // Handle this connection
            handle_connection(&net, accept.conn_handle);
        }

        // Reduced delay to avoid busy-waiting, but still yields
        stem::time::sleep_ms(1);
    }
}

/// Handle a single HTTP connection
fn handle_connection(net: &NetClient, conn_handle: u32) {
    let mut request_data = Vec::with_capacity(4096);
    let mut keep_alive = true;

    while keep_alive {
        let mut header_found = false;
        let mut attempts = 0;
        
        while attempts < 100 {
            if let Some(data) = net.tcp_recv(conn_handle, 4096) {
                request_data.extend_from_slice(&data);
                
                // Check if we have a complete request (ends with \r\n\r\n)
                if request_data.windows(4).any(|w| w == b"\r\n\r\n") {
                    header_found = true;
                    break;
                }
                attempts = 0; // Reset on data
            } else {
                attempts += 1;
                stem::time::sleep_ms(10);
            }
        }

        if !header_found {
            if !request_data.is_empty() {
                warn!("anther: Request headers incomplete or timed out");
            }
            break;
        }

        // Parse headers
        let request_str = match core::str::from_utf8(&request_data) {
            Ok(s) => s,
            Err(_) => {
                warn!("anther: Invalid UTF-8 in request");
                break;
            }
        };

        let req = match http::parse_request(request_str) {
            Ok(r) => r,
            Err(_) => {
                warn!("anther: Failed to parse request");
                break;
            }
        };

        let header_len = req.header_len;
        let content_length = http::parse_content_length(&req).unwrap_or(0);
        let mut body = Vec::new();

        if content_length > 0 {
            // Check if we already have the body in request_data
            if request_data.len() >= header_len + content_length {
                body.extend_from_slice(&request_data[header_len..header_len + content_length]);
            } else {
                body.extend_from_slice(&request_data[header_len..]);
                let mut body_attempts = 0;
                while body.len() < content_length && body_attempts < 20 {
                    if let Some(data) = net.tcp_recv(conn_handle, 4096) {
                        body.extend_from_slice(&data);
                        body_attempts = 0;
                    } else {
                        body_attempts += 1;
                        stem::syscall::yield_now();
                    }
                }
            }
            
            if body.len() < content_length {
                warn!("anther: Body incomplete ({}/{} bytes received)", body.len(), content_length);
                break;
            }
        }

        keep_alive = req.is_keep_alive();
        let (headers, resp_body) = handle_request(&req, &body, keep_alive);
        
        // Send headers
        net.tcp_send(conn_handle, &headers);
        
        // Send body in chunks
        let response_body_slice = resp_body.as_slice();
        const CHUNK_SIZE: usize = 8192;
        let mut sent = 0;
        let mut stall_count = 0;
        
        while sent < response_body_slice.len() {
            let remaining = response_body_slice.len() - sent;
            let chunk_len = remaining.min(CHUNK_SIZE);
            let chunk = &response_body_slice[sent..sent + chunk_len];
            
            let n = net.tcp_send(conn_handle, chunk);
            
            if n == 0 {
                stall_count += 1;
                if stall_count >= 100 {
                    warn!("anther: Send stalled after {} bytes", sent);
                    keep_alive = false;
                    break;
                }
                stem::syscall::yield_now();
                continue;
            }
            
            sent += n;
            stall_count = 0;
        }

        // Drain processed request from buffer
        let total_processed = header_len + content_length;
        if total_processed < request_data.len() {
            request_data.drain(..total_processed);
        } else {
            request_data.clear();
        }
        
        if !keep_alive {
            break;
        }
    }

    // Minimal flush delay
    stem::time::sleep_ms(5);
    net.tcp_close(conn_handle);
    // Minimal post-close delay
    stem::time::sleep_ms(10);
}

// Magic value to signal stdio mode (for testing)
const STDIO_MODE_MAGIC: usize = 0xDEADBEEF;

#[stem::main]
fn main(arg: usize) -> ! {
    info!("anther: Starting HTTP server (ThingOS anther v0.1)");
    
    // Default to server mode when spawned as a service (arg=0)
    // stdio mode is only for testing (requires magic value)
    if arg == STDIO_MODE_MAGIC {
        run_stdio_mode();
    } else {
        // Default: run as a persistent server (like other services)
        run_server_mode(80);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_health() {
        let response = handle_health(false);
        let response_str = core::str::from_utf8(&response).unwrap();
        assert!(response_str.contains("200 OK"));
        assert!(response_str.contains("ok"));
    }

    #[test]
    fn test_handle_index() {
        let response = handle_index(false);
        let response_str = core::str::from_utf8(&response).unwrap();
        assert!(response_str.contains("200 OK"));
        assert!(response_str.contains("ThingOS"));
    }

    #[test]
    fn test_handle_404() {
        let response = handle_404(false);
        let response_str = core::str::from_utf8(&response).unwrap();
        assert!(response_str.contains("404 Not Found"));
    }

    #[test]
    fn test_build_response() {
        let response = build_response("200 OK", "text/plain", b"test");
        let response_str = core::str::from_utf8(&response).unwrap();
        assert!(response_str.contains("HTTP/1.1 200 OK"));
        assert!(response_str.contains("Content-Length: 4"));
        assert!(response_str.contains("test"));
    }

    #[test]
    fn test_route_health() {
        let req = http::Request {
            method: http::Method::Get,
            path: "/health",
            version: http::HttpVersion::Http11,
            headers: [(None, None); 64],
            header_count: 0,
        };
        let response = route_request(&req, "/health", &[]);
        let response_str = core::str::from_utf8(&response).unwrap();
        assert!(response_str.contains("200 OK"));
    }

    #[test]
    fn test_route_invalid_method() {
        let req = http::Request {
            method: http::Method::Post,
            path: "/health",
            version: http::HttpVersion::Http11,
            headers: [(None, None); 64],
            header_count: 0,
        };
        let response = route_request(&req, "/health", &[]);
        let response_str = core::str::from_utf8(&response).unwrap();
        assert!(response_str.contains("405"));
    }
}
