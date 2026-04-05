mod console;

use self::console::{ConsoleEvent, LineConsole};
use crate::root::{self, RootOp};
use crate::sched::StdioSpec;
use abi::errors::Errno;
use abi::petals_shell::{ShellAction, ShellSession};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use spin::Mutex;

const DIR_ENTRY_WIRE_SIZE: usize = 264;

static PETALS: Mutex<Option<PetalsSession>> = Mutex::new(None);

#[repr(C)]
#[derive(Clone, Copy)]
struct DirEntryWire {
    thing_id: u64,
    kind_id: u32,
    name_len: u16,
    _padding: u16,
    name: [u8; 248],
}

struct PetalsSession {
    console: LineConsole,
    shell: ShellSession,
    foreground_tid: Option<u64>,
}

pub fn init() {
    let mut lock = PETALS.lock();
    if lock.is_some() {
        return;
    }

    let shell = ShellSession::new();
    let mut session = PetalsSession {
        console: LineConsole::new(),
        shell,
        foreground_tid: None,
    };
    for line in session.shell.banner_lines() {
        session.write_line(line);
    }
    session.write_line("");
    session.show_prompt();
    *lock = Some(session);
}

pub fn poll() {
    let mut lock = PETALS.lock();
    let Some(session) = lock.as_mut() else {
        return;
    };

    session.reap_foreground();

    while let Some(byte) = crate::runtime_base().getchar() {
        session.handle_input(byte);
    }
}

pub fn write_output(_fd: usize, data: &[u8]) -> usize {
    let _guard = PETALS.lock();
    write_bytes(data);
    data.len()
}

impl PetalsSession {
    fn handle_input(&mut self, byte: u8) {
        match byte {
            b'\r' | b'\n' => write_str("\r\n"),
            0x08 | 0x7f => {
                if matches!(self.console.handle_byte(byte), None) {
                    write_str("\x08 \x08");
                }
                return;
            }
            0x20..=0x7e => {
                let _ = self.console.handle_byte(byte);
                crate::runtime_base().putchar(byte);
                return;
            }
            _ => {}
        }

        match self.console.handle_byte(byte) {
            Some(ConsoleEvent::Submit(line)) => {
                if let Some(tid) = self.foreground_tid {
                    if queue_console_input(tid, line.as_bytes()).is_err()
                        || queue_console_input(tid, b"\n").is_err()
                    {
                        self.write_error("failed to deliver stdin to foreground task");
                    }
                } else {
                    self.run_shell_line(&line);
                }
            }
            Some(ConsoleEvent::Interrupt) => {
                write_str("^C\r\n");
                if let Some(tid) = self.foreground_tid.take() {
                    let _ = unsafe { crate::sched::kill_by_tid_current(tid) };
                    self.write_line("");
                }
                self.show_prompt();
            }
            Some(ConsoleEvent::Cancel) | None => {}
        }
    }

    fn run_shell_line(&mut self, line: &str) {
        let actions = self.shell.handle_line(line);
        let mut launched = false;

        for action in actions {
            if self.apply_action(action) {
                launched = true;
            }
        }

        if !launched {
            self.show_prompt();
        }
    }

    fn apply_action(&mut self, action: ShellAction) -> bool {
        match action {
            ShellAction::PrintLine(line) => {
                self.write_line(&line);
                false
            }
            ShellAction::PrintError(line) => {
                self.write_error(&line);
                false
            }
            ShellAction::ClearScreen => {
                write_str("\x1b[2J\x1b[H");
                false
            }
            ShellAction::ListDir(path) => {
                match list_dir(&path) {
                    Ok(entries) if entries.is_empty() => self.write_line("(empty)"),
                    Ok(entries) => {
                        for entry in entries {
                            self.write_line(&entry);
                        }
                    }
                    Err(err) => self.write_error(&format!("ls: {}: {}", path, err)),
                }
                false
            }
            ShellAction::ShowTasks => {
                crate::sched::dump_stats_current();
                false
            }
            ShellAction::ShowMem => {
                for line in memory_lines() {
                    self.write_line(&line);
                }
                false
            }
            ShellAction::DumpGraph => {
                crate::root::debug_dump::dump_all_to_console();
                false
            }
            ShellAction::RunProgram(path) => match spawn_foreground(&path) {
                Ok(tid) => {
                    self.foreground_tid = Some(tid);
                    true
                }
                Err(err) => {
                    self.write_error(&format!("run: {}: {:?}", path, err));
                    false
                }
            },
        }
    }

    fn reap_foreground(&mut self) {
        let Some(tid) = self.foreground_tid else {
            return;
        };

        if let Some((state, code)) = unsafe { crate::sched::task_status_current(tid) } {
            if state == crate::task::TaskState::Dead {
                self.foreground_tid = None;
                self.write_line("");
                self.write_line(&format!(
                    "[process {} exited with {}]",
                    tid,
                    code.unwrap_or(0)
                ));
                self.show_prompt();
            }
        } else {
            self.foreground_tid = None;
            self.show_prompt();
        }
    }

    fn show_prompt(&mut self) {
        write_str(self.shell.prompt());
    }

    fn write_line(&mut self, line: &str) {
        write_str(line);
        write_str("\r\n");
    }

    fn write_error(&mut self, line: &str) {
        self.write_line(&format!("error: {}", line));
    }
}

fn memory_lines() -> Vec<String> {
    let (largest_alloc, large_allocs) = crate::memory::global_alloc::alloc_stats();
    let free_frames = crate::memory::FRAME_ALLOCATOR.with_lock(|alloc| alloc.free_count());
    let heap_stats = crate::memory::kheap::kernel_heap().lock().stats();

    vec![
        format!("frames.free={}", free_frames),
        format!(
            "heap.pinned={} heap.evictable={}",
            heap_stats.total_pinned_bytes, heap_stats.total_evictable_bytes
        ),
        format!(
            "heap.evictions={} heap.evicted_bytes={}",
            heap_stats.eviction_count, heap_stats.bytes_freed_by_eviction
        ),
        format!(
            "alloc.largest={} alloc.large_count={}",
            largest_alloc, large_allocs
        ),
    ]
}

fn spawn_foreground(path: &str) -> Result<u64, Errno> {
    let result = unsafe {
        crate::sched::spawn_process_ex_current(
            path,
            Vec::new(),
            alloc::collections::BTreeMap::new(),
            StdioSpec::Inherit,
            StdioSpec::Inherit,
            StdioSpec::Inherit,
        )
    }?;
    Ok(result.child_tid)
}

fn list_dir(path: &str) -> Result<Vec<String>, &'static str> {
    let dir_id = resolve_path(path).map_err(|_| "not found")?;
    let mut buf = vec![0u8; DIR_ENTRY_WIRE_SIZE * 32];
    let reply = root::enqueue(RootOp::DirList {
        id: dir_id,
        out_ptr: buf.as_mut_ptr() as u64,
        out_len: buf.len() as u64,
    });
    wait_reply(&reply)?;

    let count = reply.value.load(Ordering::Relaxed) as usize;
    let mut out = Vec::new();
    for idx in 0..count {
        let ptr = unsafe { buf.as_ptr().add(idx * DIR_ENTRY_WIRE_SIZE) as *const DirEntryWire };
        let entry = unsafe { *ptr };
        let name_len = usize::from(entry.name_len).min(entry.name.len());
        let name = core::str::from_utf8(&entry.name[..name_len]).unwrap_or("?");
        out.push(name.to_string());
    }
    Ok(out)
}

fn resolve_path(path: &str) -> Result<u64, &'static str> {
    let reply = root::enqueue(RootOp::ResolvePath { path: path.into() });
    wait_reply(&reply)?;
    Ok(reply.value.load(Ordering::Relaxed))
}

fn wait_reply(reply: &alloc::sync::Arc<root::ReplyCell>) -> Result<(), &'static str> {
    loop {
        if reply.done.load(Ordering::Acquire) != 0 {
            return if reply.status.load(Ordering::Relaxed) == 0 {
                Ok(())
            } else {
                Err("operation failed")
            };
        }
        unsafe {
            crate::sched::yield_now_current();
        }
    }
}

fn queue_console_input(tid: u64, bytes: &[u8]) -> Result<(), ()> {
    let Some(process_info) = crate::sched::process_info_for_tid_current(tid) else {
        return Err(());
    };
    process_info
        .lock()
        .console_stdin
        .extend(bytes.iter().copied());
    Ok(())
}

fn write_str(s: &str) {
    write_bytes(s.as_bytes());
}

fn write_bytes(bytes: &[u8]) {
    for &byte in bytes {
        if byte == b'\n' {
            crate::runtime_base().putchar(b'\r');
        }
        crate::runtime_base().putchar(byte);
    }
}
