//! phloem: Graph Shell Daemon
//!
//! A telnet server for OpenGQL-subset interaction with the system graph.

#![no_std]
#![no_main]

extern crate alloc;

mod net_client;
mod gql;
mod executor;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use net_client::NetClient;
use stem::{info, warn};

const PORT: u16 = 2323;
const SESSION_TIMEOUT_MS: u64 = 300_000; // 5 minutes

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("phloem: Starting Graph Shell Daemon on port {}", PORT);
    run_server_mode(PORT);
}

fn run_server_mode(port: u16) -> ! {
    // Wait for network stack
    let net = loop {
        match NetClient::connect() {
            Some(n) => break n,
            None => {
                stem::time::sleep_ms(500);
            }
        }
    };
    info!("phloem: Connected to network stack");

    // Listen
    let listen_handle = loop {
        match net.tcp_listen(port) {
            Some(h) => break h,
            None => {
                warn!("phloem: Failed to listen on port {}, retrying...", port);
                stem::time::sleep_ms(1000);
            }
        }
    };
    info!("phloem: Listening on port {} (handle={})", port, listen_handle);

    loop {
        if let Some(accept) = net.tcp_accept(listen_handle) {
            info!("phloem: Connection from {}.{}.{}.{}:{}",
                accept.remote_ip[0], accept.remote_ip[1],
                accept.remote_ip[2], accept.remote_ip[3],
                accept.remote_port
            );

            // Handle session (blocking for MVP)
            handle_session(&net, accept.conn_handle);
            info!("phloem: Session ended");
        }
        stem::time::sleep_ms(10);
    }
}

fn handle_session(net: &NetClient, conn_handle: u32) {
    let mut exec = executor::GraphExecutor::new();
    let mut input_buf = Vec::new();

    // Send Banner
    let banner = b"Thing-OS Graph Shell (OpenGQL subset)\r\nType HELP for commands.\r\ngql> ";
    net.tcp_send(conn_handle, banner);

    let start_time = stem::time::now();
    let mut last_activity = start_time;

    loop {
        // Check timeout
        let now = stem::time::now();
        if now.saturating_sub(last_activity).as_millis() > SESSION_TIMEOUT_MS {
            net.tcp_send(conn_handle, b"Session timed out.\r\n");
            net.tcp_close(conn_handle);
            return;
        }

        // Read byte by byte or chunks
        let data = match net.tcp_recv(conn_handle, 1024) {
            Some(d) => {
                last_activity = now;
                d
            },
            None => {
                // No data received.
                // We should check if we've been idle too long loop?
                // Also, if `net_client` had a way to report "Closed", we'd use it.
                // For now, simple poll loop.
                stem::time::sleep_ms(10);
                continue;
            }
        };

        // Process data
        let mut i = 0;
        while i < data.len() {
            let b = data[i];
            i += 1;

            if b == 0xFF {
                // Telnet IAC: 0xFF <CMD> <OPT>
                // Skip next 2 bytes if available
                // If not available in this chunk, we technically should buffer state.
                // But for MVP, assume they come in same packet or ignore.
                // Let's just try to skip if in buffer.
                if i < data.len() { i += 1; } // Skip CMD
                if i < data.len() { i += 1; } // Skip OPT
                continue;
            }

            if b == b'\n' || b == b'\r' {
                if !input_buf.is_empty() {
                    // Process line
                    let line = String::from_utf8_lossy(&input_buf).into_owned();
                    input_buf.clear();

                    if line.trim().is_empty() {
                         net.tcp_send(conn_handle, b"gql> ");
                         continue;
                    }

                    // Echo newline
                    net.tcp_send(conn_handle, b"\r\n");

                    if line.trim().eq_ignore_ascii_case("QUIT") || line.trim().eq_ignore_ascii_case("EXIT") {
                        net.tcp_send(conn_handle, b"Bye.\r\n");
                        net.tcp_close(conn_handle);
                        return;
                    }

                    // Parse and Execute
                    let result = match gql::parse(&line) {
                        Ok(cmd) => exec.execute(cmd),
                        Err(e) => format!("error: {}\n", e),
                    };

                    net.tcp_send(conn_handle, result.as_bytes());
                    net.tcp_send(conn_handle, b"\r\ngql> ");
                }
            } else if b == 0x08 || b == 0x7F {
                // Backspace
                if !input_buf.is_empty() {
                    input_buf.pop();
                    // Echo backspace sequence
                    net.tcp_send(conn_handle, b"\x08 \x08");
                }
            } else {
                // Append
                input_buf.push(b);
                // Echo back
                net.tcp_send(conn_handle, &[b]);
            }
        }
    }
}
