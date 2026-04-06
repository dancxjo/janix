#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod net_client;

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use net_client::{NetClient, TcpRecvResult};
use spin::Mutex;
use stem::stack::{Stack, StackSpec};
use stem::{info, warn};

const TELNET_PORT: u16 = 2323;
const QUERY_PAGE_SIZE: usize = 50;
const QUERY_WORKER_COUNT: usize = 2;
const IAC: u8 = 255;
const DONT: u8 = 254;
const DO: u8 = 253;
const WONT: u8 = 252;
const WILL: u8 = 251;

#[derive(Clone)]
struct QueryPage {
    success: bool,
    message: String,
    columns: Vec<String>,
    rows: Vec<Vec<phloem::ResultValue>>,
    has_more: bool,
}

struct QueryCursor {
    command: phloem::Command,
    total_limit: Option<usize>,
    page_size: usize,
    next_offset: usize,
    pending: bool,
    done: bool,
    cancelled: bool,
    page: Option<QueryPage>,
}

type CursorHandle = Arc<Mutex<QueryCursor>>;

static CURSORS: Mutex<BTreeMap<u64, CursorHandle>> = Mutex::new(BTreeMap::new());
static QUERY_JOBS: Mutex<VecDeque<u64>> = Mutex::new(VecDeque::new());
static NEXT_CURSOR_ID: AtomicU64 = AtomicU64::new(1);
static QUERY_WORKERS_STARTED: AtomicBool = AtomicBool::new(false);

enum InputEvent {
    Submit(String),
    Interrupt,
}

enum TelnetState {
    Data,
    Iac,
    IacOption(u8),
}

enum SessionCommand {
    Next(Option<u64>),
    Close(Option<u64>),
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
            info!(
                "telnetd: accepted connection conn_handle={}",
                accept.conn_handle
            );
            handle_connection(&net, accept.conn_handle);
            info!(
                "telnetd: closing connection conn_handle={}",
                accept.conn_handle
            );
            net.tcp_close(accept.conn_handle);
        }
        stem::time::sleep_ms(10);
    }
}

fn handle_connection(net: &NetClient, conn_handle: u32) {
    let mut console = TelnetConsole::new();
    let mut executor = phloem::GraphExecutor::new();
    let mut active_cursor = None;

    let ok = write_raw(net, conn_handle, b"\xff\xfb\x01\xff\xfb\x03")
        && write_line(net, conn_handle, "ThingOS GQL Server")
        && write_line(net, conn_handle, "Type your query, or 'quit' to exit.")
        && write_line(
            net,
            conn_handle,
            "Cursor commands: NEXT, FETCH <id>, CLOSE.",
        )
        && write_line(net, conn_handle, "")
        && write_raw(net, conn_handle, b"gql> ");
    info!(
        "telnetd: banner write conn_handle={} ok={}",
        conn_handle, ok
    );
    if !ok {
        return;
    }

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

                    if trimmed.eq_ignore_ascii_case("quit") || trimmed.eq_ignore_ascii_case("exit")
                    {
                        if let Some(cursor_id) = active_cursor.take() {
                            close_cursor(cursor_id);
                        }
                        return;
                    }

                    if let Some(command) = parse_session_command(trimmed) {
                        handle_session_command(net, conn_handle, command, &mut active_cursor);
                        write_raw(net, conn_handle, b"gql> ");
                        continue;
                    }

                    if let Some(cursor_id) = active_cursor.take() {
                        close_cursor(cursor_id);
                    }

                    let parsed_query = trimmed.trim_end_matches(';');
                    match phloem::parse_with_metadata(parsed_query) {
                        Ok(parsed) => {
                            if is_cursor_eligible(&parsed.command) {
                                let cursor_id =
                                    create_cursor(parsed.command, parsed.explicit_limit);
                                submit_cursor(cursor_id);
                                if let Some(page) = wait_for_cursor_page(cursor_id) {
                                    write_query_page(net, conn_handle, cursor_id, &page);
                                    if page.success && page.has_more {
                                        active_cursor = Some(cursor_id);
                                    } else {
                                        close_cursor(cursor_id);
                                    }
                                } else {
                                    write_line(
                                        net,
                                        conn_handle,
                                        "error: query cursor did not return a page",
                                    );
                                    close_cursor(cursor_id);
                                }
                            } else {
                                let result = executor.execute(parsed.command);
                                write_execution_result(net, conn_handle, result);
                            }
                        }
                        Err(e) => {
                            write_line(
                                net,
                                conn_handle,
                                &format!("error: GQL parse failed: {}", e),
                            );
                        }
                    }

                    write_raw(net, conn_handle, b"gql> ");
                }
                Some(InputEvent::Interrupt) => {
                    if let Some(cursor_id) = active_cursor.take() {
                        close_cursor(cursor_id);
                    }
                    write_line(net, conn_handle, "^C");
                    write_raw(net, conn_handle, b"gql> ");
                }
                None => {}
            }
        }
    }

    if let Some(cursor_id) = active_cursor {
        close_cursor(cursor_id);
    }
}

fn ensure_query_workers() {
    if QUERY_WORKERS_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    for _ in 0..QUERY_WORKER_COUNT {
        let stack = match Stack::alloc_growing_stack(StackSpec {
            reserve_bytes: 256 * 1024,
            initial_commit_bytes: 64 * 1024,
            ..StackSpec::default()
        }) {
            Ok(stack) => stack,
            Err(e) => {
                warn!("telnetd: failed to allocate query worker stack: {:?}", e);
                continue;
            }
        };

        match stem::thread::spawn_on(stack, query_worker_entry) {
            Ok(tid) => {
                let _ = stem::thread::set_priority(tid, 1);
            }
            Err(e) => warn!("telnetd: failed to spawn query worker: {:?}", e),
        }
    }
}

extern "C" fn query_worker_entry() -> ! {
    loop {
        let cursor_id = {
            let mut jobs = QUERY_JOBS.lock();
            jobs.pop_front()
        };

        if let Some(cursor_id) = cursor_id {
            process_cursor_job(cursor_id);
        } else {
            stem::time::sleep_ms(5);
        }
    }
}

fn process_cursor_job(cursor_id: u64) {
    let handle = {
        let cursors = CURSORS.lock();
        cursors.get(&cursor_id).cloned()
    };
    let Some(handle) = handle else {
        return;
    };

    let (command, skip, visible_limit, total_limit) = {
        let mut cursor = handle.lock();
        if cursor.cancelled || cursor.done || !cursor.pending {
            return;
        }

        let remaining = cursor
            .total_limit
            .map(|total| total.saturating_sub(cursor.next_offset));
        let visible_limit = remaining
            .map(|remaining| core::cmp::min(cursor.page_size, remaining))
            .unwrap_or(cursor.page_size);
        let probe_limit = if remaining
            .map(|remaining| remaining <= visible_limit)
            .unwrap_or(false)
        {
            visible_limit
        } else {
            visible_limit.saturating_add(1)
        };

        let command = command_with_page(&cursor.command, cursor.next_offset, probe_limit);
        cursor.pending = false;
        (
            command,
            cursor.next_offset,
            visible_limit,
            cursor.total_limit,
        )
    };

    let mut executor = phloem::GraphExecutor::new();
    let mut result = executor.execute(command);
    let mut has_more = false;

    if result.success && result.rows.len() > visible_limit {
        result.rows.truncate(visible_limit);
        has_more = true;
    }

    if let Some(total_limit) = total_limit {
        if skip + result.rows.len() >= total_limit {
            has_more = false;
        }
    }

    if result.success {
        result.message = format!("ok: {} rows", result.rows.len());
    }

    let page = QueryPage {
        success: result.success,
        message: result.message,
        columns: result.columns,
        rows: result.rows,
        has_more,
    };

    {
        let mut cursor = handle.lock();
        if cursor.cancelled {
            return;
        }
        cursor.next_offset = skip + page.rows.len();
        cursor.done = !page.success || !page.has_more;
        cursor.page = Some(page);
    }
}

fn handle_session_command(
    net: &NetClient,
    conn_handle: u32,
    command: SessionCommand,
    active_cursor: &mut Option<u64>,
) {
    match command {
        SessionCommand::Next(requested_id) => {
            let Some(cursor_id) = active_cursor_target(*active_cursor, requested_id) else {
                write_line(net, conn_handle, "error: no active cursor");
                return;
            };

            if !submit_cursor(cursor_id) {
                write_line(net, conn_handle, "error: cursor is not active");
                *active_cursor = None;
                close_cursor(cursor_id);
                return;
            }

            if let Some(page) = wait_for_cursor_page(cursor_id) {
                write_query_page(net, conn_handle, cursor_id, &page);
                if !page.success || !page.has_more {
                    *active_cursor = None;
                    close_cursor(cursor_id);
                }
            } else {
                write_line(net, conn_handle, "error: cursor did not return a page");
                *active_cursor = None;
                close_cursor(cursor_id);
            }
        }
        SessionCommand::Close(requested_id) => {
            let Some(cursor_id) = active_cursor_target(*active_cursor, requested_id) else {
                write_line(net, conn_handle, "error: no active cursor");
                return;
            };
            close_cursor(cursor_id);
            *active_cursor = None;
            write_line(
                net,
                conn_handle,
                &format!("ok: cursor {} closed", cursor_id),
            );
        }
    }
}

fn active_cursor_target(active_cursor: Option<u64>, requested_id: Option<u64>) -> Option<u64> {
    match (active_cursor, requested_id) {
        (Some(active), None) => Some(active),
        (Some(active), Some(requested)) if active == requested => Some(active),
        _ => None,
    }
}

fn parse_session_command(input: &str) -> Option<SessionCommand> {
    let mut parts = input.split_whitespace();
    let command = parts.next()?;

    if command.eq_ignore_ascii_case("next") || command.eq_ignore_ascii_case("more") {
        return Some(SessionCommand::Next(None));
    }

    if command.eq_ignore_ascii_case("fetch") {
        let cursor_id = parts.next()?.parse().ok()?;
        return Some(SessionCommand::Next(Some(cursor_id)));
    }

    if command.eq_ignore_ascii_case("close") || command.eq_ignore_ascii_case("cancel") {
        let cursor_id = parts.next().and_then(|raw| raw.parse().ok());
        return Some(SessionCommand::Close(cursor_id));
    }

    None
}

fn is_cursor_eligible(command: &phloem::Command) -> bool {
    matches!(
        command,
        phloem::Command::Match {
            returns,
            order_by: None,
            ..
        } if returns
            .iter()
            .all(|expr| matches!(expr, phloem::ReturnExpression::Variable(_)))
    )
}

fn create_cursor(command: phloem::Command, explicit_limit: Option<usize>) -> u64 {
    ensure_query_workers();

    let cursor_id = NEXT_CURSOR_ID.fetch_add(1, Ordering::Relaxed);
    let page_size = explicit_limit
        .map(|limit| core::cmp::min(limit, QUERY_PAGE_SIZE))
        .unwrap_or(QUERY_PAGE_SIZE);

    let cursor = QueryCursor {
        command,
        total_limit: explicit_limit,
        page_size,
        next_offset: 0,
        pending: false,
        done: false,
        cancelled: false,
        page: None,
    };

    let mut cursors = CURSORS.lock();
    cursors.insert(cursor_id, Arc::new(Mutex::new(cursor)));
    cursor_id
}

fn submit_cursor(cursor_id: u64) -> bool {
    let handle = {
        let cursors = CURSORS.lock();
        cursors.get(&cursor_id).cloned()
    };
    let Some(handle) = handle else {
        return false;
    };

    {
        let mut cursor = handle.lock();
        if cursor.cancelled || cursor.done || cursor.pending {
            return false;
        }
        cursor.pending = true;
    }

    let mut jobs = QUERY_JOBS.lock();
    jobs.push_back(cursor_id);
    true
}

fn wait_for_cursor_page(cursor_id: u64) -> Option<QueryPage> {
    loop {
        let handle = {
            let cursors = CURSORS.lock();
            cursors.get(&cursor_id).cloned()
        };
        let Some(handle) = handle else {
            return None;
        };

        {
            let mut cursor = handle.lock();
            if let Some(page) = cursor.page.take() {
                return Some(page);
            }
            if cursor.done || cursor.cancelled {
                return None;
            }
        }

        stem::time::sleep_ms(10);
    }
}

fn close_cursor(cursor_id: u64) {
    let handle = {
        let mut cursors = CURSORS.lock();
        cursors.remove(&cursor_id)
    };

    if let Some(handle) = handle {
        let mut cursor = handle.lock();
        cursor.cancelled = true;
        cursor.pending = false;
        cursor.done = true;
        cursor.page = None;
    }
}

fn command_with_page(command: &phloem::Command, skip: usize, limit: usize) -> phloem::Command {
    match command {
        phloem::Command::Match {
            pattern,
            where_clause,
            returns,
            order_by,
            ..
        } => phloem::Command::Match {
            pattern: pattern.clone(),
            where_clause: where_clause.clone(),
            returns: returns.clone(),
            order_by: order_by.clone(),
            limit,
            skip,
        },
        _ => command.clone(),
    }
}

fn write_query_page(net: &NetClient, conn_handle: u32, cursor_id: u64, page: &QueryPage) {
    if !page.message.is_empty() {
        if page.success && page.has_more {
            write_line(
                net,
                conn_handle,
                &format!(
                    "{} (cursor {}, next: NEXT or FETCH {})",
                    page.message, cursor_id, cursor_id
                ),
            );
        } else {
            write_line(net, conn_handle, &page.message);
        }
    }

    write_rows(net, conn_handle, &page.columns, &page.rows);
}

fn write_execution_result(net: &NetClient, conn_handle: u32, result: phloem::ExecutionResult) {
    if !result.message.is_empty() {
        write_line(net, conn_handle, &result.message);
    }

    write_rows(net, conn_handle, &result.columns, &result.rows);
}

fn write_rows(
    net: &NetClient,
    conn_handle: u32,
    columns: &[String],
    rows: &[Vec<phloem::ResultValue>],
) {
    if columns.is_empty() {
        return;
    }

    let header = columns.join(" | ");
    write_line(net, conn_handle, &header);
    write_line(net, conn_handle, &"-".repeat(header.len()));

    for row in rows {
        let mut row_str = String::new();
        for (index, value) in row.iter().enumerate() {
            if index > 0 {
                row_str.push_str(" | ");
            }
            match value {
                phloem::ResultValue::Node(id) => row_str.push_str(&format!("node({})", id)),
                phloem::ResultValue::String(s) => row_str.push_str(s),
                phloem::ResultValue::Number(n) => row_str.push_str(&format!("{}", n)),
            }
        }
        write_line(net, conn_handle, &row_str);
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
