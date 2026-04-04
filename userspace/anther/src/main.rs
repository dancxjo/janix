#![feature(restricted_std)]
#![no_main]

extern crate alloc;
extern crate stem;

mod api_v1;
mod assets;
mod error;
mod gql_handler;
mod graph_api;
mod http;
mod net_client;
mod router;
mod ui;
mod upload;

use alloc::vec::Vec;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use net_client::NetClient;
use stem::{info, warn};

use crate::http::ResponseBody;

const SERVER_NAME: &str = "ThingOS-anther/0.1";
const MAX_HEADER_BYTES: usize = 16 * 1024;
const MAX_BODY_BYTES: usize = 8 * 1024 * 1024;

/// Adapter to let `write!` output directly into a `Vec<u8>` without
/// allocating intermediate `String`s.
struct VecWriter<'a>(&'a mut Vec<u8>);

impl<'a> core::fmt::Write for VecWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

/// Build HTTP headers — writes directly into a pre-sized Vec to avoid
/// per-header temporary String allocations.
fn build_headers(
    status: &str,
    content_type: &str,
    body_len: Option<usize>,
    keep_alive: bool,
) -> Vec<u8> {
    use core::fmt::Write;
    let mut response = Vec::with_capacity(256);

    // Status line + Server + Content-Type in one write
    write!(
        VecWriter(&mut response),
        "HTTP/1.1 {}\r\nServer: {}\r\nContent-Type: {}\r\n",
        status, SERVER_NAME, content_type
    ).ok();

    if let Some(len) = body_len {
        write!(VecWriter(&mut response), "Content-Length: {}\r\n", len).ok();
    } else {
        response.extend_from_slice(b"Transfer-Encoding: chunked\r\n");
    }

    if keep_alive {
        response.extend_from_slice(b"Connection: keep-alive\r\n");
    } else {
        response.extend_from_slice(b"Connection: close\r\n");
    }
    // CORS headers for local dev if needed
    response.extend_from_slice(b"Access-Control-Allow-Origin: *\r\n");
    response.extend_from_slice(b"\r\n");

    response
}

/// Build HTTP response with headers
fn build_response(
    status: &str,
    content_type: &str,
    body: ResponseBody,
    keep_alive: bool,
) -> (Vec<u8>, ResponseBody) {
    let headers = build_headers(status, content_type, body.len(), keep_alive);
    (headers, body)
}

/// Build HTTP redirect response
fn build_redirect(location: &str, is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    use core::fmt::Write;
    let body: &[u8] = if is_head { &[] } else { b"" };
    let mut response = Vec::with_capacity(192);
    write!(
        VecWriter(&mut response),
        "HTTP/1.1 302 Found\r\nServer: {}\r\nLocation: {}\r\nContent-Length: {}\r\n",
        SERVER_NAME, location, body.len()
    ).ok();
    if keep_alive {
        response.extend_from_slice(b"Connection: keep-alive\r\n");
    } else {
        response.extend_from_slice(b"Connection: close\r\n");
    }
    response.extend_from_slice(b"\r\n");

    (response, ResponseBody::Static(body))
}

/// Handle a single HTTP request and return a response
fn handle_request(
    req: &http::Request<'_>,
    body: &[u8],
    keep_alive: bool,
) -> (Vec<u8>, ResponseBody) {
    info!(
        "anther: {} {} {:?}",
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
        req.version
    );

    if req.method == http::Method::Post {
        info!("anther: Request body size: {} bytes", body.len());
    }

    // Validate path
    let safe_path = match http::decode_path(req.path) {
        Some(p) => p,
        None => {
            let body = b"400 Bad Request: Invalid path\n";
            return build_response(
                "400 Bad Request",
                "text/plain",
                ResponseBody::Static(body),
                keep_alive,
            );
        }
    };

    // Route the request
    route_request(req, safe_path, body, keep_alive)
}

/// Route a request to the appropriate handler
fn route_request(
    req: &http::Request<'_>,
    path: &str,
    body: &[u8],
    keep_alive: bool,
) -> (Vec<u8>, ResponseBody) {
    let method = req.method;
    let is_head = method == http::Method::Head;

    // Check if this is an API v1 request
    if router::is_api_v1_path(path) {
        if let Some(route) = router::match_route(method, path) {
            // For API routes, we need the request body (for POST/PUT/PATCH)
            let (status, api_resp) = api_v1::dispatch(route, req, body);
            // WatchStream needs SSE headers, not JSON
            let content_type = match &api_resp {
                ResponseBody::WatchStream { .. } => "text/event-stream",
                _ => "application/json",
            };
            return build_response(status, content_type, api_resp, keep_alive);
        }
    }

    // Legacy routes - only GET and HEAD allowed
    let allow_post_legacy =
        method == http::Method::Post && (path == "/upload" || path == "/ui/event");
    if method != http::Method::Get && method != http::Method::Head && !allow_post_legacy {
        let body = b"405 Method Not Allowed\n";
        return build_response(
            "405 Method Not Allowed",
            "text/plain",
            ResponseBody::Static(body),
            keep_alive,
        );
    }

    let asset_path = path.split('?').next().unwrap_or(path);
    if asset_path == "/graph.html" || asset_path == "/3d.html" {
        return build_redirect("/", is_head, keep_alive);
    }

    // Check for static assets first (strip query string)
    if let Some(asset) = assets::get_asset(asset_path) {
        let body: &[u8] = if is_head { &[] } else { asset.content };
        return build_response(
            "200 OK",
            asset.content_type,
            ResponseBody::Static(body),
            keep_alive,
        );
    }

    match path {
        "/health" => handle_health(is_head, keep_alive),
        "/top" => handle_top(is_head, keep_alive),
        "/ui" => handle_ui_index(is_head, keep_alive),
        "/ui/event" if req.method == http::Method::Post => handle_ui_event(body, keep_alive),
        "/upload" if req.method == http::Method::Post => {
            let (status, body) = upload::handle_upload_new(req, body);
            build_response(
                status,
                "application/json",
                ResponseBody::Owned(body),
                keep_alive,
            )
        }
        p if p.starts_with("/ui/") => handle_ui_route(req, p, is_head, keep_alive),
        "/graph" => handle_graph_index(is_head, keep_alive),
        p if p.starts_with("/graph/") => handle_graph_thing(p, is_head, keep_alive),
        _ => handle_404(is_head, keep_alive),
    }
}

fn handle_ui_index(is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    let json = ui::list_windows_json();
    let body = if is_head { Vec::new() } else { json };
    build_response(
        "200 OK",
        "application/json",
        ResponseBody::Owned(body),
        keep_alive,
    )
}

fn handle_ui_route(
    req: &http::Request<'_>,
    path: &str,
    is_head: bool,
    keep_alive: bool,
) -> (Vec<u8>, ResponseBody) {
    if req.method != http::Method::Get && req.method != http::Method::Head {
        let body = b"405 Method Not Allowed\n";
        return build_response(
            "405 Method Not Allowed",
            "text/plain",
            ResponseBody::Static(body),
            keep_alive,
        );
    }

    let path_only = path.split('?').next().unwrap_or(path);
    let segs: Vec<&str> = path_only.split('/').filter(|s| !s.is_empty()).collect();
    if segs.len() < 2 || segs[0] != "ui" {
        return handle_404(is_head, keep_alive);
    }

    if segs.len() == 2 && path_only.ends_with(".json") {
        let id_str = segs[1].trim_end_matches(".json");
        let id = match id_str.parse::<u64>() {
            Ok(v) => v,
            Err(_) => {
                let body = b"400 Bad Request: Invalid window ID\n";
                return build_response(
                    "400 Bad Request",
                    "text/plain",
                    ResponseBody::Static(body),
                    keep_alive,
                );
            }
        };
        return handle_ui_json(id, is_head, keep_alive);
    }

    if segs.len() == 3 && segs[2] == "json" {
        let window_id = match segs[1].parse::<u64>() {
            Ok(id) => id,
            Err(_) => {
                let body = b"400 Bad Request: Invalid window ID\n";
                return build_response(
                    "400 Bad Request",
                    "text/plain",
                    ResponseBody::Static(body),
                    keep_alive,
                );
            }
        };
        return handle_ui_json(window_id, is_head, keep_alive);
    }

    if segs.len() == 2 {
        let window_id = match segs[1].parse::<u64>() {
            Ok(id) => id,
            Err(_) => {
                let body = b"400 Bad Request: Invalid window ID\n";
                return build_response(
                    "400 Bad Request",
                    "text/plain",
                    ResponseBody::Static(body),
                    keep_alive,
                );
            }
        };
        let if_scene_gen = parse_u64_query(path, "gen");
        return match ui::render_window_html(window_id, if_scene_gen) {
            Ok(None) => {
                // 204 with empty body when scene generation has not changed.
                let empty: &[u8] = if is_head { &[] } else { b"" };
                build_response(
                    "204 No Content",
                    "text/plain",
                    ResponseBody::Static(empty),
                    keep_alive,
                )
            }
            Ok(Some(html)) => {
                let body = if is_head { Vec::new() } else { html };
                build_response(
                    "200 OK",
                    "text/html; charset=utf-8",
                    ResponseBody::Owned(body),
                    keep_alive,
                )
            }
            Err(ui::UiError::NotFound) => {
                let body = b"404 Not Found: UI window root not found\n";
                build_response(
                    "404 Not Found",
                    "text/plain",
                    ResponseBody::Static(body),
                    keep_alive,
                )
            }
            Err(ui::UiError::BadRequest(msg)) => {
                let body = alloc::format!("400 Bad Request: {}\n", msg).into_bytes();
                build_response(
                    "400 Bad Request",
                    "text/plain",
                    ResponseBody::Owned(body),
                    keep_alive,
                )
            }
            Err(ui::UiError::Internal) => {
                let body = b"500 Internal Server Error\n";
                build_response(
                    "500 Internal Server Error",
                    "text/plain",
                    ResponseBody::Static(body),
                    keep_alive,
                )
            }
        };
    }

    handle_404(is_head, keep_alive)
}

fn handle_ui_json(window_id: u64, is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    match ui::render_window_json(window_id) {
        Ok(bytes) => {
            let body = if is_head { Vec::new() } else { bytes };
            build_response(
                "200 OK",
                "application/json",
                ResponseBody::Owned(body),
                keep_alive,
            )
        }
        Err(ui::UiError::NotFound) => {
            let body = b"404 Not Found: UI window root not found\n";
            build_response(
                "404 Not Found",
                "text/plain",
                ResponseBody::Static(body),
                keep_alive,
            )
        }
        Err(_) => {
            let body = b"500 Internal Server Error\n";
            build_response(
                "500 Internal Server Error",
                "text/plain",
                ResponseBody::Static(body),
                keep_alive,
            )
        }
    }
}

fn handle_ui_event(body: &[u8], keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    match ui::ingest_event(body) {
        Ok(resp) => build_response(
            "200 OK",
            "application/json",
            ResponseBody::Owned(resp),
            keep_alive,
        ),
        Err(ui::UiError::BadRequest(msg)) => {
            let json = alloc::format!("{{\"ok\":false,\"error\":\"{}\"}}", msg).into_bytes();
            build_response(
                "400 Bad Request",
                "application/json",
                ResponseBody::Owned(json),
                keep_alive,
            )
        }
        Err(_) => {
            let json = b"{\"ok\":false,\"error\":\"internal\"}".to_vec();
            build_response(
                "500 Internal Server Error",
                "application/json",
                ResponseBody::Owned(json),
                keep_alive,
            )
        }
    }
}

fn parse_u64_query(path: &str, key: &str) -> Option<u64> {
    let q = path.split_once('?')?.1;
    for part in q.split('&') {
        let Some((k, v)) = part.split_once('=') else {
            continue;
        };
        if k == key {
            if let Ok(parsed) = v.parse::<u64>() {
                return Some(parsed);
            }
        }
    }
    None
}

/// GET /health
fn handle_health(is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    let body: &[u8] = if is_head { &[] } else { b"ok" };
    build_response(
        "200 OK",
        "text/plain",
        ResponseBody::Static(body),
        keep_alive,
    )
}

/// GET /top — JSON task list from the system graph
fn handle_top(is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    use abi::ids::HandleId;
    use stem::thing::sys::{describe_symbol, find, prop_get};
    use stem::thing::ThingId;

    let mut json = graph_api::JsonBuilder::new();
    json.start_object();

    // Find all proc.Thread nodes
    let mut ids = [ThingId::default(); 128];
    let count = find("proc.Thread", &mut ids).unwrap_or(0);

    json.key("count");
    json.number_value(count as u64);

    json.key("tasks");
    json.start_array();

    for i in 0..count {
        let id = ids[i];

        json.start_object();

        json.key("thing_id");
        json.number_value(id.to_u64_lossy());

        // proc.tid
        if let Ok(tid) = prop_get(id, "proc.tid") {
            json.key("tid");
            json.number_value(tid);
        }

        // proc.name (interned symbol → string), fall back to "name"
        let mut got_name = false;
        for key in &["proc.name", "name"] {
            if let Ok(name_sym) = prop_get(id, *key) {
                if name_sym != 0 && name_sym <= u32::MAX as u64 {
                    let mut buf = [0u8; 128];
                    if let Ok(len) = describe_symbol(name_sym as u32, &mut buf) {
                        if len > 0 {
                            if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                                json.key("name");
                                json.string_value(s);
                                got_name = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        if !got_name {
            json.key("name");
            json.string_value("?");
        }

        // proc.state (interned symbol → string)
        if let Ok(state_sym) = prop_get(id, "proc.state") {
            if state_sym != 0 && state_sym <= u32::MAX as u64 {
                let mut buf = [0u8; 64];
                if let Ok(len) = describe_symbol(state_sym as u32, &mut buf) {
                    if len > 0 {
                        if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                            json.key("state");
                            json.string_value(s);
                        }
                    }
                }
            }
        }

        // proc.priority (raw u64)
        if let Ok(pri) = prop_get(id, "proc.priority") {
            json.key("priority");
            json.number_value(pri);
        }

        // proc.is_user
        if let Ok(is_user) = prop_get(id, "proc.is_user") {
            json.key("is_user");
            json.bool_value(is_user != 0);
        }

        // proc.exit_code (only if non-zero)
        if let Ok(exit_code) = prop_get(id, "proc.exit_code") {
            if exit_code != 0 {
                json.key("exit_code");
                json.number_value(exit_code);
            }
        }

        json.end_object();
        json.buf.push(b',');
    }

    json.end_array();
    json.end_object();

    let body = if is_head {
        Vec::new()
    } else {
        json.into_bytes()
    };
    build_response(
        "200 OK",
        "application/json",
        ResponseBody::Owned(body),
        keep_alive,
    )
}

// Note: Index page is now served from embedded assets (assets.rs)

/// GET /graph
fn handle_graph_index(is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    // For now, return a simple JSON structure
    // TODO: enumerate actual graph roots/kinds
    let json = r#"{"message":"Graph index endpoint","note":"Use /graph/<thing_id> to query specific things"}"#;
    let body: &[u8] = if is_head { &[] } else { json.as_bytes() };
    build_response(
        "200 OK",
        "application/json",
        ResponseBody::Static(body),
        keep_alive,
    )
}

/// GET /graph/<thing_id> or /graph/<thing_id>/bytespace/<key>
fn handle_graph_thing(path: &str, is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    // Parse path like "/graph/123" or "/graph/123/bytespace/456"
    let path_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if path_parts.len() < 2 {
        let body = b"400 Bad Request: Invalid graph path\n";
        return build_response(
            "400 Bad Request",
            "text/plain",
            ResponseBody::Static(body),
            keep_alive,
        );
    }

    // Parse thing_id
    let thing_id = match path_parts[1].parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            let body = b"400 Bad Request: Invalid thing ID\n";
            return build_response(
                "400 Bad Request",
                "text/plain",
                ResponseBody::Static(body),
                keep_alive,
            );
        }
    };

    // Check if this is a bytespace request
    if path_parts.len() >= 4 && path_parts[2] == "bytespace" {
        return handle_bytespace(thing_id, path_parts[3], is_head, keep_alive);
    }

    // Get the thing as JSON
    match graph_api::thing_to_json(thing_id) {
        Ok(json) => {
            let body = if is_head {
                Vec::new()
            } else {
                json.into_bytes()
            };
            build_response(
                "200 OK",
                "application/json",
                ResponseBody::Owned(body),
                keep_alive,
            )
        }
        Err(graph_api::GraphError::NotFound) => {
            let body = b"404 Not Found: Thing does not exist\n";
            build_response(
                "404 Not Found",
                "text/plain",
                ResponseBody::Static(body),
                keep_alive,
            )
        }
        Err(_) => {
            let body = b"500 Internal Server Error\n";
            build_response(
                "500 Internal Server Error",
                "text/plain",
                ResponseBody::Static(body),
                keep_alive,
            )
        }
    }
}

/// GET /graph/<thing_id>/bytespace/<key>
fn handle_bytespace(
    _thing_id: u64,
    key_str: &str,
    _is_head: bool,
    keep_alive: bool,
) -> (Vec<u8>, ResponseBody) {
    // TODO: Add size limit query parameter support (?size=N or ?range=N-M)
    // TODO: Add permission check for bytespace access

    // Parse the key as a bytespace ID
    let bytespace_id = match key_str.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            let body = b"400 Bad Request: Invalid bytespace ID\n";
            return build_response(
                "400 Bad Request",
                "text/plain",
                ResponseBody::Static(body),
                keep_alive,
            );
        }
    };

    // Try to read the bytespace
    match graph_api::read_bytespace(bytespace_id, 1024 * 1024) {
        Ok(data) => {
            // Return the raw bytespace data
            build_response(
                "200 OK",
                "application/octet-stream",
                ResponseBody::Owned(data),
                keep_alive,
            )
        }
        Err(graph_api::GraphError::NotFound) => {
            let body = b"404 Not Found: Bytespace does not exist\n";
            build_response(
                "404 Not Found",
                "text/plain",
                ResponseBody::Static(body),
                keep_alive,
            )
        }
        Err(_) => {
            let body = b"500 Internal Server Error\n";
            build_response(
                "500 Internal Server Error",
                "text/plain",
                ResponseBody::Static(body),
                keep_alive,
            )
        }
    }
}

/// 404 Not Found
fn handle_404(is_head: bool, keep_alive: bool) -> (Vec<u8>, ResponseBody) {
    let body: &[u8] = if is_head { &[] } else { b"404 Not Found\n" };
    build_response(
        "404 Not Found",
        "text/plain",
        ResponseBody::Static(body),
        keep_alive,
    )
}

/// Run in stdio mode: read request from stdin, write response to stdout
fn run_stdio_mode() -> ! {
    info!("anther: Running in stdio mode");

    // Read request from stdin (simulated via a buffer for now)
    let test_request = "GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let req = http::parse_request(test_request).unwrap();
    let (headers, body) = handle_request(&req, &[], false);

    // In stdio mode, we'd write to stdout here
    info!(
        "anther: Response generated: headers={}, body_len={:?}",
        headers.len(),
        body.len()
    );

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

    info!(
        "anther: Listening on port {} (handle={})",
        port, listen_handle
    );

    // Global slot for passing conn_handle to the worker trampoline.
    // Protected by sequential spawning: we store before spawn, worker reads
    // before we can spawn again (single accept loop on the main thread).
    static CONN_HANDLE_SLOT: core::sync::atomic::AtomicU32 =
        core::sync::atomic::AtomicU32::new(0);

    extern "C" fn worker_trampoline() -> ! {
        let conn = CONN_HANDLE_SLOT.load(core::sync::atomic::Ordering::Acquire);
        handle_connection(conn);
        // Exit thread
        stem::syscall::exit(0);
    }

    // Main server loop — accept connections and spawn a thread per connection.
    // Uses stem::thread::spawn because std::thread::spawn hangs on ThingOS
    loop {
        if let Some(accept) = net.tcp_accept(listen_handle) {
            let conn = accept.conn_handle;
            info!("anther: Accepted connection, spawning thread for conn_handle={}", conn);
            CONN_HANDLE_SLOT.store(conn, core::sync::atomic::Ordering::Release);
            match stem::thread::spawn(worker_trampoline) {
                Ok(tid) => {
                    info!("anther: Thread spawned TID={} for conn_handle={}", tid, conn);
                }
                Err(e) => {
                    warn!("anther: Thread spawn FAILED for conn_handle={}: {:?}", conn, e);
                }
            }
            continue;
        }

        // 10ms polling interval keeps response port from filling up.
        stem::time::sleep_ms(10);
    }
}

/// Handle a single HTTP connection (runs in its own thread).
/// Creates a per-thread NetClient so IPC responses never interleave.
fn handle_connection(conn_handle: u32) {
    let tid = stem::syscall::get_tid().unwrap_or(0);
    info!("anther: Worker thread TID={} starting for conn_handle={}", tid, conn_handle);

    let net = match NetClient::connect() {
        Some(n) => {
            info!("anther: Worker TID={} connected to netd OK", tid);
            n
        }
        None => {
            warn!("anther: Worker TID={} failed to connect to netd, dropping connection", tid);
            return;
        }
    };
    let net = &net;
    let mut request_data = Vec::with_capacity(4096);
    let mut keep_alive = true;

    while keep_alive {
        let mut header_end = None;
        let mut scan_from = request_data.len().saturating_sub(3);
        let mut attempts = 0;
        let mut got_any_data = false;

        while attempts < 200 {
            if let Some(data) = net.tcp_recv(conn_handle, NetClient::MAX_RECV_LEN) {
                if !got_any_data {
                    info!("anther: Worker TID={} got first {} bytes on conn_handle={}",
                          tid, data.len(), conn_handle);
                    got_any_data = true;
                }
                request_data.extend_from_slice(&data);

                if let Some(pos) = find_header_end(&request_data, scan_from) {
                    header_end = Some(pos);
                    break;
                }

                if request_data.len() > MAX_HEADER_BYTES {
                    warn!("anther: Request header exceeded {} bytes", MAX_HEADER_BYTES);
                    break;
                }

                scan_from = request_data.len().saturating_sub(3);
                attempts = 0; // Reset on data
            } else {
                attempts += 1;
                stem::syscall::yield_now();
            }
        }

        let Some(header_end) = header_end else {
            if !request_data.is_empty() {
                warn!("anther: TID={} headers incomplete ({} bytes so far)", tid, request_data.len());
            } else {
                warn!("anther: TID={} conn_handle={} recv timed out with no data (200 attempts)",
                      tid, conn_handle);
            }
            break;
        };

        // Parse headers
        let request_str = match core::str::from_utf8(&request_data[..header_end]) {
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
        if content_length > MAX_BODY_BYTES {
            warn!(
                "anther: Request body too large ({} > {})",
                content_length, MAX_BODY_BYTES
            );
            break;
        }
        let mut body = Vec::with_capacity(content_length.min(4096));
        let mut pending_overread: Vec<u8> = Vec::new();

        if content_length > 0 {
            // Check if we already have the body in request_data
            if request_data.len() >= header_len + content_length {
                body.extend_from_slice(&request_data[header_len..header_len + content_length]);
            } else {
                body.reserve(content_length);
                body.extend_from_slice(&request_data[header_len..]);
                let mut body_attempts = 0;
                while body.len() < content_length && body_attempts < 100 {
                    if let Some(data) = net.tcp_recv(conn_handle, NetClient::MAX_RECV_LEN) {
                        let remaining = content_length - body.len();
                        let copy_len = remaining.min(data.len());
                        body.extend_from_slice(&data[..copy_len]);
                        // Preserve any over-read bytes so pipelined requests stay intact.
                        if copy_len < data.len() {
                            pending_overread.extend_from_slice(&data[copy_len..]);
                        }
                        body_attempts = 0;
                    } else {
                        body_attempts += 1;
                        stem::syscall::yield_now();
                    }
                }
            }

            if body.len() < content_length {
                warn!(
                    "anther: Body incomplete ({}/{} bytes received)",
                    body.len(),
                    content_length
                );
                break;
            }
        }

        keep_alive = req.is_keep_alive();
        let (headers, resp_body) = handle_request(&req, &body, keep_alive);
        if !pending_overread.is_empty() {
            request_data.extend_from_slice(&pending_overread);
        }

        // Send headers
        send_all(net, conn_handle, &headers);

        // Send body
        match resp_body {
            ResponseBody::Static(s) => {
                send_all(net, conn_handle, s);
            }
            ResponseBody::Owned(v) => {
                send_all(net, conn_handle, &v);
            }
            ResponseBody::Stream(mut stream) => {
                 use alloc::format;
                 let waker = noop_waker();
                 let mut cx = Context::from_waker(&waker);

                 loop {
                     match stream.poll_next(&mut cx) {
                         Poll::Ready(Ok(Some(delta))) => {
                             let finish_str = match delta.finish {
                                 Some(f) => match f {
                                     llm::FinishReason::Stop => "\"stop\"",
                                     llm::FinishReason::Length => "\"length\"",
                                     llm::FinishReason::Canceled => "\"canceled\"",
                                     llm::FinishReason::Error => "\"error\"",
                                 },
                                 None => "null",
                             };

                             let escaped_text = escape_json_string(&delta.text);
                             let json = format!("{{\"text\":\"{}\",\"finish\":{}}}", escaped_text, finish_str);

                             // Send as chunked encoding
                             let event_str = format!("data: {}\n\n", json);
                             send_chunk(net, conn_handle, event_str.as_bytes());

                             if delta.finish.is_some() {
                                 // Close stream
                                 send_chunk(net, conn_handle, &[]); // 0-length chunk to end
                                 break;
                             }
                         }
                         Poll::Ready(Ok(None)) => {
                             send_chunk(net, conn_handle, &[]); // 0-length chunk to end
                             break;
                         }
                         Poll::Ready(Err(_)) => {
                             // Send error event
                             let err_json = "{\"error\":\"Stream error\"}";
                             let event_str = format!("data: {}\n\n", err_json);
                             send_chunk(net, conn_handle, event_str.as_bytes());
                             send_chunk(net, conn_handle, &[]);
                             break;
                         }
                         Poll::Pending => {
                             stem::thread::yield_now();
                         }
                     }
                 }
            }
            ResponseBody::WatchStream { thing_id } => {
                use alloc::format;
                use abi::root::RootWatchFilter;
                use abi::types::{WatchSpec, WATCH_START_LATEST};

                // Reuse the existing props handler for consistent JSON output
                let fetch_props_json = |tid: u64| -> Option<Vec<u8>> {
                    let id_str = format!("{}", tid);
                    let (status, body) = api_v1::handle_get_thing_props(&id_str);
                    if status == "200 OK" {
                        match body {
                            ResponseBody::Owned(v) => Some(v),
                            _ => None,
                        }
                    } else {
                        None
                    }
                };

                // 1. Send initial props
                if let Some(initial_json) = fetch_props_json(thing_id) {
                    let event = format!("event: props\ndata: {}\n\n", core::str::from_utf8(&initial_json).unwrap_or("{}"));
                    send_chunk(net, conn_handle, event.as_bytes());
                }

                // 2. Open kernel watch filtered to this subject
                let filter = RootWatchFilter::subject(thing_id);
                let spec = WatchSpec {
                    query_ptr: 0,
                    query_len: 0,
                    mode: 1, // StreamOnly
                    _padding: 0,
                    start_seq: WATCH_START_LATEST,
                    filter_ptr: &filter as *const _ as u64,
                    filter_len: RootWatchFilter::SIZE as u64,
                };

                let watch_handle = match stem::syscall::root_watch_open(&spec) {
                    Ok(h) => h,
                    Err(_) => {
                        let err_event = "event: error\ndata: {\"error\":\"Failed to open watch\"}\n\n";
                        send_chunk(net, conn_handle, err_event.as_bytes());
                        send_chunk(net, conn_handle, &[]); // End chunked stream
                        break;
                    }
                };

                // 3. Stream loop — runs in its own thread so the accept
                //    loop is never blocked.
                let mut watch_buf = alloc::vec![0u8; 4096];
                let mut seq_out = 0u64;
                let mut stall_count = 0u32;

                loop {

                    match stem::syscall::root_watch_next(watch_handle, &mut seq_out, &mut watch_buf) {
                        Ok(_len) => {
                            // Something changed — re-fetch props and send
                            stall_count = 0;
                            if let Some(json) = fetch_props_json(thing_id) {
                                let event = format!("event: props\ndata: {}\n\n", core::str::from_utf8(&json).unwrap_or("{}"));
                                send_chunk(net, conn_handle, event.as_bytes());
                            }
                        }
                        Err(abi::errors::Errno::EAGAIN) => {
                            // No pending events — send keepalive every ~30s
                            stall_count += 1;
                            if stall_count % 600 == 0 {
                                send_chunk(net, conn_handle, b": keepalive\n\n");
                            }
                            stem::syscall::sleep_ms(50);
                        }
                        Err(abi::errors::Errno::EOVERFLOW) => {
                            continue; // Skip overflow events
                        }
                        Err(_) => {
                            break; // Fatal watch error
                        }
                    }
                }

                // 4. Cleanup
                let _ = stem::syscall::root_watch_close(watch_handle);
            }
        }

        // Drain processed request from buffer
        let total_processed = header_len + content_length;
        if total_processed < request_data.len() {
            request_data.drain(..total_processed);
        } else {
            request_data.clear();
        }
        // Prevent unbounded growth on long keep-alive sessions
        if request_data.is_empty() && request_data.capacity() > 8192 {
            request_data.shrink_to(4096);
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

fn send_all(net: &NetClient, conn_handle: u32, data: &[u8]) {
    const CHUNK_SIZE: usize = 8192;
    let mut sent = 0;
    let mut stall_count = 0;

    while sent < data.len() {
        let remaining = data.len() - sent;
        let chunk_len = remaining.min(CHUNK_SIZE);
        let chunk = &data[sent..sent + chunk_len];

        let n = net.tcp_send(conn_handle, chunk);

        if n == 0 {
            stall_count += 1;
            if stall_count >= 1000 {
                warn!("anther: Send stalled after {} bytes", sent);
                break;
            }
            stem::time::sleep_ms(5);
            continue;
        }

        sent += n;
        stall_count = 0;
    }
}

fn send_chunk(net: &NetClient, conn_handle: u32, data: &[u8]) {
    use alloc::format;
    // Chunk header: hex length \r\n — use send_all to prevent partial
    // writes from corrupting the chunked transfer encoding framing.
    let header = format!("{:x}\r\n", data.len());
    send_all(net, conn_handle, header.as_bytes());

    // Chunk data
    if !data.is_empty() {
        send_all(net, conn_handle, data);
    }

    // Chunk footer: \r\n
    send_all(net, conn_handle, b"\r\n");
}

/// Re-export the canonical escape function from the error module.
fn escape_json_string(s: &str) -> alloc::string::String {
    crate::error::escape_json_string(s)
}

fn find_header_end(buf: &[u8], start: usize) -> Option<usize> {
    if buf.len() < 2 {
        return None;
    }
    let mut i = start.min(buf.len().saturating_sub(1));
    while i + 1 < buf.len() {
        if i + 3 < buf.len()
            && buf[i] == b'\r'
            && buf[i + 1] == b'\n'
            && buf[i + 2] == b'\r'
            && buf[i + 3] == b'\n'
        {
            return Some(i + 4);
        }
        if buf[i] == b'\n' && buf[i + 1] == b'\n' {
            return Some(i + 2);
        }
        i += 1;
    }
    None
}

fn noop_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(core::ptr::null(), &VTABLE)
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &VTABLE)) }
}

// Magic value to signal stdio mode (for testing)
const STDIO_MODE_MAGIC: usize = 0xDEADBEEF;

#[stem::main]
fn main(arg: usize) -> ! {
    stem::info!("anther: Starting HTTP server (ThingOS anther v0.1)");

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
        let (headers, body) = handle_health(false, false);
        let headers_str = core::str::from_utf8(&headers).unwrap();
        assert!(headers_str.contains("200 OK"));
        match body {
            ResponseBody::Static(b) => assert_eq!(b, b"ok"),
            _ => panic!("Expected static body"),
        }
    }

    #[test]
    fn test_handle_graph_index() {
        let (headers, body) = handle_graph_index(false, false);
        let headers_str = core::str::from_utf8(&headers).unwrap();
        assert!(headers_str.contains("200 OK"));
        match body {
            ResponseBody::Static(b) => {
                 let s = core::str::from_utf8(b).unwrap();
                 assert!(s.contains("Graph index"));
            }
            _ => panic!("Expected static body"),
        }
    }

    #[test]
    fn test_handle_404() {
        let (headers, _body) = handle_404(false, false);
        let headers_str = core::str::from_utf8(&headers).unwrap();
        assert!(headers_str.contains("404 Not Found"));
    }

    #[test]
    fn test_build_response() {
        let response_body = ResponseBody::Static(b"test");
        let (headers, body) = build_response("200 OK", "text/plain", response_body, false);
        let headers_str = core::str::from_utf8(&headers).unwrap();
        assert!(headers_str.contains("HTTP/1.1 200 OK"));
        assert!(headers_str.contains("Content-Length: 4"));
        match body {
            ResponseBody::Static(b) => assert_eq!(b, b"test"),
            _ => panic!("Expected static body"),
        }
    }

    #[test]
    fn test_route_health() {
        let req = http::Request {
            method: http::Method::Get,
            path: "/health",
            version: http::HttpVersion::Http11,
            headers: [(None, None); 64],
            header_count: 0,
            header_len: 0,
        };
        let (headers, _body) = route_request(&req, "/health", &[], false);
        let headers_str = core::str::from_utf8(&headers).unwrap();
        assert!(headers_str.contains("200 OK"));
    }

    #[test]
    fn test_route_invalid_method() {
        let req = http::Request {
            method: http::Method::Post,
            path: "/health",
            version: http::HttpVersion::Http11,
            headers: [(None, None); 64],
            header_count: 0,
            header_len: 0,
        };
        let (headers, _body) = route_request(&req, "/health", &[], false);
        let headers_str = core::str::from_utf8(&headers).unwrap();
        assert!(headers_str.contains("405"));
    }
}
