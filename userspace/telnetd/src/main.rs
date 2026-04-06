#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod net_client;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use net_client::{NetClient, TcpRecvResult};
use stem::{info, warn};

const TELNET_PORT: u16 = 2323;
const IAC: u8 = 255;
const DONT: u8 = 254;
const DO: u8 = 253;
const WONT: u8 = 252;
const WILL: u8 = 251;

enum InputEvent {
    Submit(String),
    Interrupt,
}

enum TelnetState {
    Data,
    Iac,
    IacOption(u8),
}

struct TelnetConsole {
    line: Vec<u8>,
    state: TelnetState,
    swallow_lf: bool,
}

impl TelnetConsole {
    fn new() -> Self {
        Self {
            line: Vec::new(),
            state: TelnetState::Data,
            swallow_lf: false,
        }
    }

    fn ingest(&mut self, byte: u8) -> (Option<InputEvent>, Option<Vec<u8>>) {
        match self.state {
            TelnetState::Iac => {
                if matches!(byte, DO | DONT | WILL | WONT) {
                    self.state = TelnetState::IacOption(byte);
                    return (None, None);
                }
                self.state = TelnetState::Data;
                return (None, None);
            }
            TelnetState::IacOption(cmd) => {
                self.state = TelnetState::Data;
                let response = match cmd {
                    DO | DONT => vec![IAC, WONT, byte],
                    WILL | WONT => vec![IAC, DONT, byte],
                    _ => Vec::new(),
                };
                return (
                    None,
                    if response.is_empty() {
                        None
                    } else {
                        Some(response)
                    },
                );
            }
            TelnetState::Data => {}
        }

        match byte {
            IAC => {
                self.state = TelnetState::Iac;
                (None, None)
            }
            b'\n' if self.swallow_lf => {
                self.swallow_lf = false;
                (None, None)
            }
            0 if self.swallow_lf => {
                self.swallow_lf = false;
                (None, None)
            }
            b'\r' | b'\n' => {
                self.swallow_lf = byte == b'\r';
                let line = String::from_utf8(core::mem::take(&mut self.line)).unwrap_or_default();
                (Some(InputEvent::Submit(line)), None)
            }
            0x03 => {
                self.line.clear();
                (Some(InputEvent::Interrupt), None)
            }
            0x08 | 0x7f => {
                if !self.line.is_empty() {
                    self.line.pop();
                    (None, Some(b"\x08 \x08".to_vec()))
                } else {
                    (None, None)
                }
            }
            0x20..=0x7e => {
                self.line.push(byte);
                (None, Some(vec![byte]))
            }
            _ => (None, None),
        }
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("telnetd: starting on port {}", TELNET_PORT);

    let net = loop {
        match NetClient::connect() {
            Some(net) => break net,
            None => {
                info!("telnetd: waiting for network stack");
                stem::time::sleep_ms(500);
            }
        }
    };

    let listen_handle = loop {
        match net.tcp_listen(TELNET_PORT) {
            Some(handle) => break handle,
            None => {
                warn!("telnetd: listen failed, retrying");
                stem::time::sleep_ms(1000);
            }
        }
    };

    info!(
        "telnetd: listening on guest port {} (handle={})",
        TELNET_PORT, listen_handle
    );

    loop {
        if let Some(accept) = net.tcp_accept(listen_handle) {
            handle_connection(&net, accept.conn_handle);
            net.tcp_close(accept.conn_handle);
        }
        stem::time::sleep_ms(10);
    }
}

fn handle_connection(net: &NetClient, conn_handle: u32) {
    let mut console = TelnetConsole::new();
    let mut executor = phloem::GraphExecutor::new();

    write_raw(net, conn_handle, b"\xff\xfb\x01\xff\xfb\x03"); // WILL ECHO, WILL SUPPRESS GO AHEAD
    write_line(net, conn_handle, "ThingOS GQL Server");
    write_line(net, conn_handle, "Type your query, or 'quit' to exit.");
    write_line(net, conn_handle, "");
    write_raw(net, conn_handle, b"gql> ");

    loop {
        let data = match net.tcp_recv(conn_handle, NetClient::MAX_RECV_LEN) {
            TcpRecvResult::Data(data) => data,
            TcpRecvResult::Empty => {
                stem::time::sleep_ms(10);
                continue;
            }
            TcpRecvResult::Closed => break,
        };

        for byte in data {
            let (event, response) = console.ingest(byte);
            if let Some(bytes) = response {
                write_raw(net, conn_handle, &bytes);
            }

            match event {
                Some(InputEvent::Submit(line)) => {
                    write_raw(net, conn_handle, b"\r\n");
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        write_raw(net, conn_handle, b"gql> ");
                        continue;
                    }
                    if trimmed == "quit" || trimmed == "exit" {
                        return;
                    }
                    let parsed_query = trimmed.trim_end_matches(';');

                    match phloem::parse(parsed_query) {
                        Ok(cmd) => {
                            let result = executor.execute(cmd);
                            
                            if !result.message.is_empty() {
                                write_line(net, conn_handle, &result.message);
                            }
                            
                            if !result.columns.is_empty() {
                                // Simple tabular printing
                                let header = result.columns.join(" | ");
                                write_line(net, conn_handle, &header);
                                let sep = "-".repeat(header.len());
                                write_line(net, conn_handle, &sep);
                                
                                for row in result.rows {
                                    let mut row_str = String::new();
                                    for (i, val) in row.into_iter().enumerate() {
                                        if i > 0 { row_str.push_str(" | "); }
                                        match val {
                                            phloem::ResultValue::Node(id) => {
                                                row_str.push_str(&format!("node({})", id));
                                            }
                                            phloem::ResultValue::String(s) => {
                                                row_str.push_str(&s);
                                            }
                                            phloem::ResultValue::Number(n) => {
                                                row_str.push_str(&format!("{}", n));
                                            }
                                        }
                                    }
                                    write_line(net, conn_handle, &row_str);
                                }
                            }
                        }
                        Err(e) => {
                            write_line(net, conn_handle, &format!("error: GQL parse failed: {}", e));
                        }
                    }

                    write_raw(net, conn_handle, b"gql> ");
                }
                Some(InputEvent::Interrupt) => {
                    write_line(net, conn_handle, "^C");
                    write_raw(net, conn_handle, b"gql> ");
                }
                None => {}
            }
        }
    }
}

fn write_line(net: &NetClient, conn_handle: u32, line: &str) -> bool {
    write_raw(net, conn_handle, line.as_bytes()) && write_raw(net, conn_handle, b"\r\n")
}

fn write_raw(net: &NetClient, conn_handle: u32, bytes: &[u8]) -> bool {
    let mut sent = 0usize;
    let mut stalled = 0u16;
    while sent < bytes.len() {
        let n = net.tcp_send(conn_handle, &bytes[sent..]);
        if n == 0 {
            stalled = stalled.saturating_add(1);
            if stalled >= 1000 {
                return false;
            }
            stem::time::sleep_ms(5);
            continue;
        }
        stalled = 0;
        sent += n;
    }
    true
}
