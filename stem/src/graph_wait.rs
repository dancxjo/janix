use crate::errors::Errno;
use crate::root_watch::{watch_drain, DrainStats};
use crate::syscall;
use crate::thing::sys;
use crate::thing::ThingId;
use abi::root::RootWatchFilter;
use abi::symbols::SymbolId;
use abi::types::{Edge, WatchMode, WatchSpec as RootWatchSpec};
use abi::wait::{interest, WaitKind, WaitSpec};
use alloc::vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphWatch {
    id: usize,
}

impl GraphWatch {
    pub fn open_all() -> Result<Self, Errno> {
        Self::open_filtered(RootWatchFilter::all())
    }

    pub fn open_subject(subject: ThingId) -> Result<Self, Errno> {
        Self::open_filtered(RootWatchFilter::subject(subject.to_u64_lossy()))
    }

    pub fn open_kind_id(kind: SymbolId) -> Result<Self, Errno> {
        Self::open_filtered(RootWatchFilter::kind(kind))
    }

    pub fn open_kind(kind: &str) -> Result<Self, Errno> {
        Self::open_kind_id(sys::intern(kind)?)
    }

    pub fn open_predicate_id(predicate: SymbolId) -> Result<Self, Errno> {
        Self::open_filtered(RootWatchFilter::predicate(predicate))
    }

    pub fn open_predicate(predicate: &str) -> Result<Self, Errno> {
        Self::open_predicate_id(sys::intern(predicate)?)
    }

    pub fn open_filtered(filter: RootWatchFilter) -> Result<Self, Errno> {
        let spec = RootWatchSpec {
            mode: WatchMode::QueryThenStream as u32,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        Ok(Self {
            id: syscall::root_watch_open(&spec)?,
        })
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn wait_spec(&self, token: u64) -> WaitSpec {
        WaitSpec {
            kind: WaitKind::RootWatch as u32,
            flags: interest::READABLE,
            object: self.id as u64,
            token,
        }
    }

    pub fn next(&self, seq_out: &mut u64, out: &mut [u8]) -> Result<usize, Errno> {
        syscall::root_watch_next(self.id, seq_out, out)
    }

    pub fn drain<F>(&self, buf: &mut [u8], handler: F) -> Result<DrainStats, Errno>
    where
        F: FnMut(u64, &[u8]),
    {
        watch_drain(self.id, buf, handler)
    }

    pub fn close(self) -> Result<(), Errno> {
        syscall::root_watch_close(self.id)
    }
}

#[derive(Debug)]
pub struct GraphOpHandle {
    id: u64,
    owned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphOpStatus {
    Pending,
    Done,
    Error,
}

impl GraphOpHandle {
    pub fn prop_set<S: crate::thing::symbol::IntoSymbolRef>(
        node: ThingId,
        key: S,
        value: u64,
    ) -> Result<Self, Errno> {
        Ok(Self {
            id: sys::prop_set_async(node, key, value)?,
            owned: true,
        })
    }

    pub fn wait_spec(&self, token: u64) -> WaitSpec {
        WaitSpec {
            kind: WaitKind::GraphOp as u32,
            flags: 0,
            object: self.id,
            token,
        }
    }

    pub fn raw(&self) -> u64 {
        self.id
    }

    pub fn status(&self) -> Result<GraphOpStatus, Errno> {
        match sys::async_status(self.id)? {
            0 => Ok(GraphOpStatus::Pending),
            1 => Ok(GraphOpStatus::Done),
            2 => Ok(GraphOpStatus::Error),
            _ => Err(Errno::EINVAL),
        }
    }

    pub fn take_result(mut self) -> Result<u64, Errno> {
        self.owned = false;
        sys::async_wait(self.id)
    }

    pub fn cancel(mut self) {
        if self.owned {
            sys::async_drop(self.id);
            self.owned = false;
        }
    }
}

impl Drop for GraphOpHandle {
    fn drop(&mut self) {
        if self.owned {
            sys::async_drop(self.id);
            self.owned = false;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphConditionKind {
    PropEq {
        node: ThingId,
        key: SymbolId,
        value: u64,
    },
    EdgeExists {
        src: ThingId,
        rel: SymbolId,
        dst: ThingId,
    },
}

#[derive(Debug)]
pub struct GraphCondition {
    watch: GraphWatch,
    kind: GraphConditionKind,
    fired: bool,
}

impl GraphCondition {
    pub fn prop_eq(node: ThingId, key: &str, value: u64) -> Result<Self, Errno> {
        Ok(Self {
            watch: GraphWatch::open_subject(node)?,
            kind: GraphConditionKind::PropEq {
                node,
                key: sys::intern(key)?,
                value,
            },
            fired: false,
        })
    }

    pub fn edge_exists(src: ThingId, rel: &str, dst: ThingId) -> Result<Self, Errno> {
        Ok(Self {
            watch: GraphWatch::open_subject(src)?,
            kind: GraphConditionKind::EdgeExists {
                src,
                rel: sys::intern(rel)?,
                dst,
            },
            fired: false,
        })
    }

    pub fn arm(&mut self, token: u64) -> Result<Option<WaitSpec>, Errno> {
        if self.fired || self.evaluate()? {
            self.fired = true;
            Ok(None)
        } else {
            Ok(Some(self.watch.wait_spec(token)))
        }
    }

    pub fn on_ready(&mut self, buf: &mut [u8]) -> Result<bool, Errno> {
        if self.fired {
            return Ok(true);
        }

        let mut seq = 0u64;
        loop {
            match self.watch.next(&mut seq, buf) {
                Ok(_) => {
                    if self.evaluate()? {
                        self.fired = true;
                        return Ok(true);
                    }
                }
                Err(Errno::EAGAIN) => return self.evaluate_and_cache(),
                Err(Errno::EOVERFLOW) => {
                    if self.evaluate()? {
                        self.fired = true;
                        return Ok(true);
                    }
                }
                Err(err) => return Err(err),
            }
        }
    }

    pub fn close(self) -> Result<(), Errno> {
        self.watch.close()
    }

    fn evaluate_and_cache(&mut self) -> Result<bool, Errno> {
        if self.evaluate()? {
            self.fired = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn evaluate(&self) -> Result<bool, Errno> {
        match self.kind {
            GraphConditionKind::PropEq { node, key, value } => {
                Ok(sys::prop_get(node, key)? == value)
            }
            GraphConditionKind::EdgeExists { src, rel, dst } => edge_exists(src, rel, dst),
        }
    }
}

fn edge_exists(src: ThingId, rel: SymbolId, dst: ThingId) -> Result<bool, Errno> {
    let mut cap = 16usize;
    loop {
        let mut edges = vec![Edge::default(); cap];
        let len = sys::get_edges(src, &mut edges)?;
        if edges[..len]
            .iter()
            .any(|edge| edge.predicate.to_u64_lossy() == rel as u64 && edge.to == dst)
        {
            return Ok(true);
        }
        if len < cap {
            return Ok(false);
        }
        cap = core::cmp::min(cap.saturating_mul(2), 1024);
        if cap == 1024 && len >= cap {
            return Ok(false);
        }
    }
}
