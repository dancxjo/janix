use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use spin::Mutex;

pub mod abi;
pub mod graph;
pub mod handlers;
pub mod journal;
pub mod pci;
pub mod resources;
pub mod schema;
pub mod service;

pub use service::root_main;

#[derive(Debug, Clone)]
pub enum SymbolShell {
    Id(u32),
    Str(alloc::string::String),
    Static(&'static str),
}

#[derive(Debug, Clone)]
pub struct LogProvenance {
    pub tid: u64,
    pub cpu: u32,
    pub module: &'static str,
    pub file: &'static str,
    pub line: u32,
}

#[derive(Debug, Clone)]
pub enum RootOp {
    Intern {
        name: alloc::string::String,
    },
    GetKind {
        id: u64,
    },
    CreateNode {
        kind: SymbolShell,
    },
    BytespaceCreate {
        len: u64,
        flags: u64,
        format: u64,
    },
    BytespaceInfo {
        id: u64,
    },
    BytespaceMap {
        id: u64,
        tid: u64,
    },
    BytespaceUnmap {
        id: u64,
        user_va: u64,
        tid: u64,
    },
    BytespacePhys {
        id: u64,
    },
    WatchSubscribe {
        target_id: u64,
        mask: u64,
    },
    StreamPoll {
        stream_id: u64,
        max: usize,
        out_ptr: u64,
    },
    PropSet {
        id: u64,
        key: SymbolShell,
        value: u64,
    },
    PropGet {
        id: u64,
        key: SymbolShell,
    },
    Query {
        plan: alloc::vec::Vec<crate::root::query::PreparedStep>,
        out_buffer: u64,
        out_len: u64,
    },
    Find {
        kind: SymbolShell,
        buffer: u64,
        len: u64,
    },
    DescribeThing {
        id: u64,
        buffer: u64,
        len: u64,
    },
    DescribeEdge {
        src: u64,
        rel: SymbolShell,
        dst: u64,
        buffer: u64,
        len: u64,
    },
    DumpEdges {
        id: u64,
        buffer: u64,
        len: u64,
    },
    GetEdges {
        id: u64,
        buffer: u64,
        len: u64,
    },
    DumpGraph {
        limit: u64,
    },
    BytespaceWrite {
        id: u64,
        offset: u64,
        ptr: u64,
        len: u64,
    },
    BytespaceRead {
        id: u64,
        offset: u64,
        ptr: u64,
        len: u64,
    },
    BytespaceCreateFromPtr {
        ptr: u64,
        len: u64,
    },
    Link {
        src: u64,
        rel: SymbolShell,
        dst: u64,
    },
    // Structured Logging
    LogEvent {
        level: u8,
        event: SymbolShell,
        message: alloc::string::String,
        timestamp: u64,
        provenance: LogProvenance,
        fields: alloc::vec::Vec<(SymbolShell, u64)>, // Scalar fields
        about: alloc::vec::Vec<u64>,                 // Linked Thing IDs
    },
    WatchOpen {
        mode: u32,
        start_seq: u64,
        query: alloc::vec::Vec<crate::root::query::PreparedStep>,
        filter: crate::root::graph::WatchFilter,
    },
    WatchNext {
        id: u64,
        out_seq_ptr: u64,
        out_ptr: u64,
        out_len: u64,
    },
    WatchClose {
        id: u64,
    },
    ApplyBatch {
        batch: alloc::vec::Vec<u8>,
    },
}

pub struct ReplyCell {
    pub status: AtomicI32,
    pub value: AtomicU64,
    pub p0: AtomicU64,
    pub p1: AtomicU64,
    pub p2: AtomicU64,
    pub done: AtomicU64,
}

impl ReplyCell {
    pub fn new() -> Self {
        Self {
            status: AtomicI32::new(0),
            value: AtomicU64::new(0),
            p0: AtomicU64::new(0),
            p1: AtomicU64::new(0),
            p2: AtomicU64::new(0),
            done: AtomicU64::new(0),
        }
    }
}

pub struct RootMsg {
    pub op: RootOp,
    pub reply: Arc<ReplyCell>,
    pub tid: u64,
}

static ROOT_INBOX: Mutex<Option<VecDeque<RootMsg>>> = Mutex::new(None);
static ROOT_TID: AtomicU64 = AtomicU64::new(0);

pub fn init_root_service<R: crate::BootRuntime>() {
    *ROOT_INBOX.lock() = Some(VecDeque::new());
    crate::kinfo!("Spawning Root service...");
    let tid = crate::task::spawn_with_priority::<R>(
        service::root_main::<R>,
        0,
        crate::task::TaskPriority::High,
    );
    ROOT_TID.store(tid, Ordering::SeqCst);
}

pub fn enqueue(op: RootOp) -> Arc<ReplyCell> {
    let reply = Arc::new(ReplyCell::new());
    let tid = unsafe { crate::task::scheduler::current_tid_current() };
    let msg = RootMsg {
        op,
        reply: reply.clone(),
        tid,
    };

    if let Some(q) = ROOT_INBOX.lock().as_mut() {
        q.push_back(msg);
        let tid = ROOT_TID.load(Ordering::Relaxed);
        if tid != 0 {
            unsafe {
                crate::task::scheduler::wake_task_erased(tid as usize);
            }
        }
    } else {
        panic!("Root inbox not initialized");
    }
    reply
}

pub fn pop_msg() -> Option<RootMsg> {
    ROOT_INBOX.lock().as_mut()?.pop_front()
}

pub fn is_inbox_ready() -> bool {
    ROOT_INBOX.lock().is_some()
}

pub fn queue_len() -> usize {
    ROOT_INBOX.lock().as_ref().map(|q| q.len()).unwrap_or(0)
}

pub mod debug {
    use crate::root::SymbolShell;
    use core::fmt;

    pub struct ThingDebug(pub u64);

    impl fmt::Display for ThingDebug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut buf = [0u8; 256];
            let reply = super::enqueue(super::RootOp::DescribeThing {
                id: self.0,
                buffer: buf.as_mut_ptr() as u64,
                len: buf.len() as u64,
            });

            crate::kinfo!("ThingDebug: Waiting for reply...");
            let mut timeout = 0;
            loop {
                let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
                if done != 0 {
                    crate::kinfo!("ThingDebug: Got reply!");
                    let status = reply.status.load(core::sync::atomic::Ordering::Relaxed);
                    let written = reply.value.load(core::sync::atomic::Ordering::Relaxed) as usize;
                    if status == 0 {
                        if let Ok(s) = core::str::from_utf8(&buf[..written]) {
                            f.write_str(s)?;
                        } else {
                            f.write_str("<invalid utf8>")?;
                        }
                        return Ok(());
                    } else {
                        return f.write_str("<error>");
                    }
                }
                timeout += 1;
                if timeout > 10_000_000 {
                    crate::kinfo!("ThingDebug: TIMEOUT waiting for reply (done={})", done);
                    return f.write_str("<timeout>");
                }
                unsafe {
                    crate::task::scheduler::yield_now_current();
                }
            }
        }
    }

    pub struct EdgeDebug(pub u64, pub u64, pub u64);

    impl fmt::Display for EdgeDebug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut buf = [0u8; 512];
            let reply = super::enqueue(super::RootOp::DescribeEdge {
                src: self.0,
                rel: SymbolShell::Id(self.1 as u32),
                dst: self.2,
                buffer: buf.as_mut_ptr() as u64,
                len: buf.len() as u64,
            });
            loop {
                let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
                if done != 0 {
                    let status = reply.status.load(core::sync::atomic::Ordering::Relaxed);
                    let written = reply.value.load(core::sync::atomic::Ordering::Relaxed) as usize;
                    if status == 0 {
                        if let Ok(s) = core::str::from_utf8(&buf[..written]) {
                            f.write_str(s)?;
                        } else {
                            f.write_str("<invalid utf8>")?;
                        }
                        return Ok(());
                    } else {
                        return f.write_str("<error>");
                    }
                }
                unsafe {
                    crate::task::scheduler::yield_now_current();
                }
            }
        }
    }
}
pub mod boot_register;
pub mod debug_dump;
pub mod debug_fmt;
pub mod query;
pub mod symbols;
