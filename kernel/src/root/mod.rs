use spin::Mutex;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicI32, AtomicU64};

pub mod service;
pub mod journal;
pub mod graph;
pub mod watch;
pub mod resources;
pub mod abi;
pub mod schema;

pub use service::root_main;

#[derive(Debug)]
pub enum SymbolShell {
    Id(u32),
    Str(alloc::string::String),
}

#[derive(Debug)]
pub enum RootOp {
    Intern { name: alloc::string::String },
    GetKind { id: u64 },
    CreateNode { kind: SymbolShell },
    BytespaceCreate { len: u64, flags: u64, format: u64 },
    
    WatchSubscribe { target_id: u64, mask: u64 },
    StreamPoll { stream_id: u64, max: usize, out_ptr: u64 },
    PropSet { id: u64, key: SymbolShell, value: u64 },
    PropGet { id: u64, key: SymbolShell },
    Query { plan: alloc::vec::Vec<crate::root::query::PreparedStep>, out_buffer: u64, out_len: u64 },
    Find { kind: SymbolShell, buffer: u64, len: u64 },
    DescribeThing { id: u64, buffer: u64, len: u64 },
    DescribeEdge { src: u64, rel: SymbolShell, dst: u64, buffer: u64, len: u64 },
    DumpEdges { id: u64, buffer: u64, len: u64 },
    DumpGraph { limit: u64 },
    BytespaceWrite { id: u64, offset: u64, ptr: u64, len: u64 },
    BytespaceRead { id: u64, offset: u64, ptr: u64, len: u64 },
    BytespaceCreateFromPtr { ptr: u64, len: u64 },
    Link { src: u64, rel: SymbolShell, dst: u64 },
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
}

static ROOT_INBOX: Mutex<Option<VecDeque<RootMsg>>> = Mutex::new(None);

pub fn init_root_service<R: crate::BootRuntime>() {
    *ROOT_INBOX.lock() = Some(VecDeque::new());
    crate::kinfo!("Spawning Root service...");
    crate::task::spawn::<R>(service::root_main::<R>, 0);
}

pub fn enqueue(op: RootOp) -> Arc<ReplyCell> {
    let reply = Arc::new(ReplyCell::new());
    let msg = RootMsg { op, reply: reply.clone() };
    
    if let Some(q) = ROOT_INBOX.lock().as_mut() {
        q.push_back(msg);
    } else {
        panic!("Root inbox not initialized");
    }
    reply
}

pub fn pop_msg() -> Option<RootMsg> {
    ROOT_INBOX.lock().as_mut()?.pop_front()
}

pub mod debug {
    use core::fmt;
    use crate::root::SymbolShell;
    
    pub struct ThingDebug(pub u64);
    
    impl fmt::Display for ThingDebug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut buf = [0u8; 256];
            let reply = super::enqueue(super::RootOp::DescribeThing { 
                id: self.0, 
                buffer: buf.as_mut_ptr() as u64, 
                len: buf.len() as u64 
            });
            
            crate::kinfo!("ThingDebug: Waiting for reply...");
            let mut timeout = 0;
            loop {
                // simple wait loop
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
                unsafe { crate::task::scheduler::yield_now_current(); }
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
                len: buf.len() as u64 
            });
             loop {
                // simple wait loop
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
                unsafe { crate::task::scheduler::yield_now_current(); }
            }
        }
    }
}
pub mod debug_fmt;
pub mod boot_register;
pub mod debug_dump;
pub mod symbols;
pub mod query;
