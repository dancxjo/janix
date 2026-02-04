//! anther: HTTP/1.1 server for Thing-OS
//!
//! A minimal HTTP server that serves static responses and graph-backed endpoints.

#![no_std]
#![no_main]

extern crate alloc;

mod api_v1;
mod assets;
mod error;
mod gql_handler;
mod graph_api;
mod http;
mod net_client;
mod router;


use alloc::vec::Vec;
use net_client::NetClient;
use stem::{info, warn};

const SERVER_NAME: &str = "ThingOS-anther/0.1";

/// Build HTTP response with headers
fn build_response(status: &str, content_type: &str, body: &[u8]) -> Vec<u8> {
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
    
    let content_len_hdr = format!("Content-Length: {}\r\n", body.len());
    response.extend_from_slice(content_len_hdr.as_bytes());
    
    response.extend_from_slice(b"Connection: close\r\n");
    response.extend_from_slice(b"\r\n");
    
    // Body
    response.extend_from_slice(body);
    
    response
}

/// Build HTTP redirect response
fn build_redirect(location: &str, is_head: bool) -> Vec<u8> {
    use alloc::format;
    let body: &[u8] = if is_head { &[] } else { b"" };
    let mut response = Vec::new();

    let status_line = "HTTP/1.1 302 Found\r\n";
    response.extend_from_slice(status_line.as_bytes());

    let server_hdr = format!("Server: {}\r\n", SERVER_NAME);
    response.extend_from_slice(server_hdr.as_bytes());

    let location_hdr = format!("Location: {}\r\n", location);
    response.extend_from_slice(location_hdr.as_bytes());

    let content_len_hdr = format!("Content-Length: {}\r\n", body.len());
    response.extend_from_slice(content_len_hdr.as_bytes());

    response.extend_from_slice(b"Connection: close\r\n");
    response.extend_from_slice(b"\r\n");
    response.extend_from_slice(body);

    response
}

/// Handle a single HTTP request and return a response
fn handle_request(request_str: &str, body: &[u8]) -> Vec<u8> {
    // Parse the request
    let req = match http::parse_request(request_str) {
        Ok(r) => r,
        Err(e) => {
            warn!("anther: Parse error: {:?}", e);
            let body = b"400 Bad Request\n";
            return build_response("400 Bad Request", "text/plain", body);
        }
    };
    
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
            return build_response("400 Bad Request", "text/plain", body);
        }
    };
    
    // Route the request
    route_request(&req, safe_path, body)
}

/// Route a request to the appropriate handler
fn route_request(req: &http::Request<'_>, path: &str, body: &[u8]) -> Vec<u8> {
    let method = req.method;
    let is_head = method == http::Method::Head;
    
    // Check if this is an API v1 request
    if router::is_api_v1_path(path) {
        if let Some(route) = router::match_route(method, path) {
            // For API routes, we need the request body (for POST/PUT/PATCH)
            return api_v1::dispatch(route, req, body);
        }
    }
    
    // Legacy routes - only GET and HEAD allowed
    if method != http::Method::Get && method != http::Method::Head {
        let body = b"405 Method Not Allowed\n";
        return build_response("405 Method Not Allowed", "text/plain", body);
    }
    
    // TODO: Authentication - add token/capability check here
    // TODO: Per-route permission gating
    
    let asset_path = path.split('?').next().unwrap_or(path);
    if asset_path == "/graph.html" || asset_path == "/3d.html" {
        return build_redirect("/", is_head);
    }

    // Check for static assets first (strip query string)
    if let Some(asset) = assets::get_asset(asset_path) {
        let body: &[u8] = if is_head { &[] } else { asset.content };
        return build_response("200 OK", asset.content_type, body);
    }
    
    match path {
        "/health" => handle_health(is_head),
        "/graph" => handle_graph_index(is_head),
        p if p.starts_with("/graph/") => handle_graph_thing(p, is_head),
        _ => handle_404(is_head),
    }
}

/// GET /health
fn handle_health(is_head: bool) -> Vec<u8> {
    let body: &[u8] = if is_head { &[] } else { b"ok" };
    build_response("200 OK", "text/plain", body)
}

// Note: Index page is now served from embedded assets (assets.rs)

/// GET /graph
fn handle_graph_index(is_head: bool) -> Vec<u8> {
    // For now, return a simple JSON structure
    // TODO: enumerate actual graph roots/kinds
    let json = r#"{"message":"Graph index endpoint","note":"Use /graph/<thing_id> to query specific things"}"#;
    let body: &[u8] = if is_head { &[] } else { json.as_bytes() };
    build_response("200 OK", "application/json", body)
}

/// GET /graph/<thing_id> or /graph/<thing_id>/bytespace/<key>
fn handle_graph_thing(path: &str, is_head: bool) -> Vec<u8> {
    // Parse path like "/graph/123" or "/graph/123/bytespace/456"
    let path_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    
    if path_parts.len() < 2 {
        let body = b"400 Bad Request: Invalid graph path\n";
        return build_response("400 Bad Request", "text/plain", body);
    }
    
    // Parse thing_id
    let thing_id = match path_parts[1].parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            let body = b"400 Bad Request: Invalid thing ID\n";
            return build_response("400 Bad Request", "text/plain", body);
        }
    };
    
    // Check if this is a bytespace request
    if path_parts.len() >= 4 && path_parts[2] == "bytespace" {
        return handle_bytespace(thing_id, path_parts[3], is_head);
    }
    
    // Get the thing as JSON
    match graph_api::thing_to_json(thing_id) {
        Ok(json) => {
            let body = if is_head { b"" } else { json.as_bytes() };
            build_response("200 OK", "application/json", body)
        }
        Err(graph_api::GraphError::NotFound) => {
            let body = b"404 Not Found: Thing does not exist\n";
            build_response("404 Not Found", "text/plain", body)
        }
        Err(_) => {
            let body = b"500 Internal Server Error\n";
            build_response("500 Internal Server Error", "text/plain", body)
        }
    }
}

/// GET /graph/<thing_id>/bytespace/<key>
fn handle_bytespace(_thing_id: u64, key_str: &str, _is_head: bool) -> Vec<u8> {
    // TODO: Add size limit query parameter support (?size=N or ?range=N-M)
    // TODO: Add permission check for bytespace access
    
    // Parse the key as a bytespace ID
    let bytespace_id = match key_str.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            let body = b"400 Bad Request: Invalid bytespace ID\n";
            return build_response("400 Bad Request", "text/plain", body);
        }
    };
    
    // Try to read the bytespace
    match graph_api::read_bytespace(bytespace_id, 1024 * 1024) {
        Ok(data) => {
            // Return the raw bytespace data
            build_response("200 OK", "application/octet-stream", &data)
        }
        Err(graph_api::GraphError::NotFound) => {
            let body = b"404 Not Found: Bytespace does not exist\n";
            build_response("404 Not Found", "text/plain", body)
        }
        Err(_) => {
            let body = b"500 Internal Server Error\n";
            build_response("500 Internal Server Error", "text/plain", body)
        }
    }
}

/// 404 Not Found
fn handle_404(is_head: bool) -> Vec<u8> {
    let body: &[u8] = if is_head { &[] } else { b"404 Not Found\n" };
    build_response("404 Not Found", "text/plain", body)
}

/// Run in stdio mode: read request from stdin, write response to stdout
fn run_stdio_mode() -> ! {
    info!("anther: Running in stdio mode");
    
    // Read request from stdin (simulated via a buffer for now)
    // In a real implementation, we'd use SYS_STREAM_READ or similar
    let test_request = "GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = handle_request(test_request, &[]);
    
    // In stdio mode, we'd write to stdout here
    // For now, just log it
    info!("anther: Response generated: {} bytes", response.len());
    
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
            info!(
                "anther: Connection from {}.{}.{}.{}:{}",
                accept.remote_ip[0],
                accept.remote_ip[1],
                accept.remote_ip[2],
                accept.remote_ip[3],
                accept.remote_port
            );

            // Handle this connection
            handle_connection(&net, accept.conn_handle);
        }

        // Small delay to avoid busy-waiting
        stem::time::sleep_ms(10);
    }
}

/// Handle a single HTTP connection
fn handle_connection(net: &NetClient, conn_handle: u32) {
    // Read the request (with timeout)
    let mut request_data = Vec::new();
    let mut attempts = 0;
    
    while attempts < 100 {
        if let Some(data) = net.tcp_recv(conn_handle, 4096) {
            request_data.extend_from_slice(&data);
            
            // Check if we have a complete request (ends with \r\n\r\n)
            if request_data.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }
        attempts += 1;
        stem::time::sleep_ms(10);
    }

    if request_data.is_empty() {
        warn!("anther: No request received, closing connection");
        net.tcp_close(conn_handle);
        return;
    }

    // Parse and handle the request
    let request_str = match core::str::from_utf8(&request_data) {
        Ok(s) => s,
        Err(_) => {
            warn!("anther: Invalid UTF-8 in request");
            net.tcp_close(conn_handle);
            return;
        }
    };

    // Parse headers to find Content-Length
    let req_headers = match http::parse_request(request_str) {
        Ok(r) => r,
        Err(_) => {
            warn!("anther: Failed to parse request for body extraction");
            net.tcp_close(conn_handle);
            return;
        }
    };

    let content_length = http::parse_content_length(&req_headers).unwrap_or(0);
    let mut body = Vec::new();

    if content_length > 0 {
        // We might already have some body in request_data
        let header_end = request_data.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
        body.extend_from_slice(&request_data[header_end..]);

        let mut body_attempts = 0;
        while body.len() < content_length && body_attempts < 100 {
            if let Some(data) = net.tcp_recv(conn_handle, 4096) {
                body.extend_from_slice(&data);
                body_attempts = 0; // Reset on success
            } else {
                // Wait briefly or check for timeout
                body_attempts += 1;
                stem::time::sleep_ms(10);
            }
        }
        
        if body.len() < content_length {
            warn!("anther: Body incomplete ({}/{} bytes received after timeout)", body.len(), content_length);
            net.tcp_close(conn_handle);
            return;
        }

        // Truncate if we read too much
        if body.len() > content_length {
            body.truncate(content_length);
        }
    }

    let response = handle_request(request_str, &body);
    info!("anther: Sending response ({} bytes)", response.len());

    // Send response in chunks
    // Use larger chunks (8KB) to reduce overhead, the TX buffer is 32KB
    const CHUNK_SIZE: usize = 8192;
    let mut sent = 0;
    let mut stall_count = 0;
    // Allow longer stall time - 5 seconds total for large responses
    // This handles cases where smoltcp TX buffer fills and needs time to drain
    const MAX_STALLS: usize = 500;
    const STALL_SLEEP_MS: u64 = 5;  // Shorter sleep for faster retry
    
    while sent < response.len() {
        let remaining = response.len() - sent;
        let chunk_len = remaining.min(CHUNK_SIZE);
        let chunk = &response[sent..sent + chunk_len];
        
        let n = net.tcp_send(conn_handle, chunk);
        
        if n == 0 {
            // Socket TX buffer full - wait briefly for netd to drain
            stall_count += 1;
            if stall_count >= MAX_STALLS {
                warn!("anther: Send stalled after {} bytes (max retries)", sent);
                break;
            }
            // Brief yield to let netd/smoltcp process
            stem::time::sleep_ms(STALL_SLEEP_MS);
            continue;
        }
        
        sent += n;
        // Reset stall counter on ANY successful send
        stall_count = 0;
        // No inter-chunk delay - let the TX buffer fill naturally
    }

    // Brief flush delay before close (2ms per KB, min 10ms, max 100ms)
    let flush_delay_ms = ((response.len() / 1024) * 2).clamp(10, 100) as u64;
    stem::time::sleep_ms(flush_delay_ms);

    // Close connection - this initiates the TCP FIN handshake
    net.tcp_close(conn_handle);
    
    // Post-close delay to allow smoltcp to complete FIN/ACK exchange
    // The connection needs time for: FIN -> FIN-ACK -> ACK sequence
    stem::time::sleep_ms(50);
    
    info!("anther: Connection closed (sent {}/{} bytes)", sent, response.len());
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
