#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod net_client;

use abi::petals_shell::{ShellAction, ShellSession};
use abi::types::stdio_mode;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use net_client::NetClient;
use stem::syscall::{pipe, spawn_process_ex};
use stem::{info, warn};

const TELNET_PORT: u16 = 2323;
const IAC: u8 = 255;
const DONT: u8 = 254;
const DO: u8 = 253;
const WONT: u8 = 252;
const WILL: u8 = 251;

#[repr(C)]
#[derive(Clone, Copy)]
struct DirEntryWire {
    thing_id: u64,
    kind_id: u32,
    name_len: u16,
    _padding: u16,
    name: [u8; 248],
}

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
    let mut shell = ShellSession::new();
    let mut console = TelnetConsole::new();

    write_raw(net, conn_handle, b"\xff\xfb\x01\xff\xfb\x03");
    for line in shell.banner_lines() {
        write_line(net, conn_handle, line);
    }
    write_line(net, conn_handle, "");
    write_raw(net, conn_handle, shell.prompt().as_bytes());

    loop {
        let Some(data) = net.tcp_recv(conn_handle, NetClient::MAX_RECV_LEN) else {
            stem::time::sleep_ms(10);
            continue;
        };

        if data.is_empty() {
            break;
        }

        for byte in data {
            let (event, response) = console.ingest(byte);
            if let Some(bytes) = response {
                write_raw(net, conn_handle, &bytes);
            }

            match event {
                Some(InputEvent::Submit(line)) => {
                    write_raw(net, conn_handle, b"\r\n");
                    if !handle_shell_line(net, conn_handle, &mut shell, line.as_str()) {
                        write_raw(net, conn_handle, shell.prompt().as_bytes());
                    }
                }
                Some(InputEvent::Interrupt) => {
                    write_line(net, conn_handle, "^C");
                    write_raw(net, conn_handle, shell.prompt().as_bytes());
                }
                None => {}
            }
        }
    }
}

fn handle_shell_line(
    net: &NetClient,
    conn_handle: u32,
    shell: &mut ShellSession,
    line: &str,
) -> bool {
    let actions = shell.handle_line(line);
    let mut launched = false;
    for action in actions {
        if handle_action(net, conn_handle, action) {
            launched = true;
        }
    }
    launched
}

fn handle_action(net: &NetClient, conn_handle: u32, action: ShellAction) -> bool {
    match action {
        ShellAction::PrintLine(line) => write_line(net, conn_handle, &line),
        ShellAction::PrintError(line) => write_line(net, conn_handle, &format!("error: {}", line)),
        ShellAction::ClearScreen => write_raw(net, conn_handle, b"\x1b[2J\x1b[H"),
        ShellAction::ListDir(path) => match list_dir(&path) {
            Ok(entries) if entries.is_empty() => write_line(net, conn_handle, "(empty)"),
            Ok(entries) => {
                for entry in entries {
                    write_line(net, conn_handle, &entry);
                }
            }
            Err(err) => write_line(net, conn_handle, &format!("error: ls: {}: {}", path, err)),
        },
        ShellAction::ShowTasks => {
            for line in task_lines() {
                write_line(net, conn_handle, &line);
            }
        }
        ShellAction::ShowMem => {
            for line in mem_lines() {
                write_line(net, conn_handle, &line);
            }
        }
        ShellAction::DumpGraph => {
            write_line(net, conn_handle, "error: graph dump is serial-only for now");
        }
        ShellAction::RunProgram(path) => match run_program(net, conn_handle, &path) {
            Ok(()) => return true,
            Err(err) => write_line(net, conn_handle, &format!("error: run: {}: {}", path, err)),
        },
    }
    false
}

fn run_program(net: &NetClient, conn_handle: u32, path: &str) -> Result<(), &'static str> {
    let env = BTreeMap::new();
    let resp = spawn_process_ex(
        path,
        &[],
        &env,
        stdio_mode::PIPE,
        stdio_mode::PIPE,
        stdio_mode::PIPE,
    )
    .map_err(|_| "spawn failed")?;

    let mut console = TelnetConsole::new();
    loop {
        drain_pipe_to_socket(net, conn_handle, resp.stdout_pipe);
        drain_pipe_to_socket(net, conn_handle, resp.stderr_pipe);

        if let Some(data) = net.tcp_recv(conn_handle, NetClient::MAX_RECV_LEN) {
            for byte in data {
                let (event, response) = console.ingest(byte);
                if let Some(bytes) = response {
                    write_raw(net, conn_handle, &bytes);
                }
                match event {
                    Some(InputEvent::Submit(line)) => {
                        write_raw(net, conn_handle, b"\r\n");
                        let _ = pipe::pipe_write(resp.stdin_pipe, line.as_bytes());
                        let _ = pipe::pipe_write(resp.stdin_pipe, b"\n");
                    }
                    Some(InputEvent::Interrupt) => {
                        let _ = stem::syscall::task_kill(resp.child_tid);
                    }
                    None => {}
                }
            }
        }

        match stem::syscall::task_poll(resp.child_tid) {
            Ok((abi::types::TaskStatus::Dead, code)) => {
                drain_pipe_to_socket(net, conn_handle, resp.stdout_pipe);
                drain_pipe_to_socket(net, conn_handle, resp.stderr_pipe);
                write_line(
                    net,
                    conn_handle,
                    &format!("[process {} exited with {}]", resp.child_tid, code),
                );
                break;
            }
            Ok(_) => stem::time::sleep_ms(10),
            Err(_) => break,
        }
    }

    let _ = pipe::pipe_close(resp.stdin_pipe, 1);
    let _ = pipe::pipe_close(resp.stdout_pipe, 0);
    let _ = pipe::pipe_close(resp.stderr_pipe, 0);
    Ok(())
}

fn drain_pipe_to_socket(net: &NetClient, conn_handle: u32, pipe_id: u64) {
    let mut buf = [0u8; 512];
    loop {
        match pipe::pipe_read(pipe_id, &mut buf) {
            Ok(0) => break,
            Ok(n) => write_raw(net, conn_handle, &buf[..n]),
            Err(abi::errors::Errno::EAGAIN) => break,
            Err(_) => break,
        }
    }
}

fn list_dir(path: &str) -> Result<Vec<String>, &'static str> {
    let dir_id = stem::syscall::root_resolve_path(path).map_err(|_| "not found")?;
    let mut buf = vec![0u8; core::mem::size_of::<DirEntryWire>() * 32];
    let count = stem::syscall::root_dir_list(dir_id, &mut buf).map_err(|_| "list failed")?;
    let mut out = Vec::new();
    for idx in 0..count {
        let ptr = unsafe {
            buf.as_ptr().add(idx * core::mem::size_of::<DirEntryWire>()) as *const DirEntryWire
        };
        let entry = unsafe { *ptr };
        let name_len = usize::from(entry.name_len).min(entry.name.len());
        let name = core::str::from_utf8(&entry.name[..name_len]).unwrap_or("?");
        out.push(name.to_string());
    }
    Ok(out)
}

fn task_lines() -> Vec<String> {
    let mut ids = [stem::thing::ThingId::default(); 64];
    let count = stem::thing::sys::find("proc.Thread", &mut ids).unwrap_or(0);
    let mut out = Vec::new();
    for id in ids.into_iter().take(count) {
        let mut buf = [0u8; 256];
        let len = stem::thing::sys::describe_thing(id, &mut buf).unwrap_or(0);
        let line = core::str::from_utf8(&buf[..len]).unwrap_or("<invalid utf8>");
        out.push(line.to_string());
    }
    if out.is_empty() {
        out.push("no proc.Thread nodes found".to_string());
    }
    out
}

fn mem_lines() -> Vec<String> {
    let mut ranges = [stem::thing::ThingId::default(); 128];
    let mut bytespaces = [stem::thing::ThingId::default(); 128];
    let range_count = stem::thing::sys::find("mem.Range", &mut ranges).unwrap_or(0);
    let bs_count = stem::thing::sys::find("thing.bytespace", &mut bytespaces).unwrap_or(0);
    vec![
        format!("mem.range_nodes={}", range_count),
        format!("bytespace_nodes={}", bs_count),
        "kernel allocator stats are not exposed to userspace yet".to_string(),
    ]
}

fn write_line(net: &NetClient, conn_handle: u32, line: &str) {
    write_raw(net, conn_handle, line.as_bytes());
    write_raw(net, conn_handle, b"\r\n");
}

fn write_raw(net: &NetClient, conn_handle: u32, bytes: &[u8]) {
    let mut sent = 0usize;
    while sent < bytes.len() {
        let n = net.tcp_send(conn_handle, &bytes[sent..]);
        if n == 0 {
            stem::time::sleep_ms(5);
            continue;
        }
        sent += n;
    }
}
