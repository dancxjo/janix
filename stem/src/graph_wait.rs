use crate::errors::Errno;
use crate::root_watch::{watch_drain, DrainStats};
use crate::syscall;
use crate::thing::sys;
use crate::thing::symbol::IntoSymbolRef;
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

    /// Non-blocking read of the next watch event.
    ///
    /// Returns `Err(Errno::EAGAIN)` when no event is pending.  Use this inside
    /// drain loops (after [`WaitSet::wait`](crate::wait_set::WaitSet) wakes the
    /// task) so the loop exits cleanly once all queued events are consumed.
    ///
    /// Use [`blocking_next`](Self::blocking_next) when you want to park the task
    /// until the next event arrives without managing a `WaitSet` manually.
    pub fn next(&self, seq_out: &mut u64, out: &mut [u8]) -> Result<usize, Errno> {
        syscall::root_watch_try_next(self.id, seq_out, out)
    }

    /// Blocking read of the next watch event.
    ///
    /// Parks the calling task until a matching graph commit arrives.  Never
    /// returns `Err(Errno::EAGAIN)` — use [`next`](Self::next) in drain loops
    /// where `EAGAIN` is the expected termination signal.
    pub fn blocking_next(&self, seq_out: &mut u64, out: &mut [u8]) -> Result<usize, Errno> {
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

/// A lightweight, first-class handle representing an in-flight graph mutation.
///
/// `GraphOpHandle` integrates with [`WaitSet`](abi::wait) so callers can
/// submit mutations and await completion alongside other event sources (ports,
/// IRQs, timers) without blocking the calling task for the full duration.
///
/// # Usage patterns
///
/// ```ignore
/// // Option A: submit and block until done
/// let op = GraphOpHandle::prop_set(node, "status", 1)?;
/// let result = op.await_completion()?;
///
/// // Option B: integrate with wait_many / WaitSet
/// let op = GraphOpHandle::link(src, "child", dst)?;
/// let specs = [op.wait_spec(42)];
/// let mut results = [WaitResult::default()];
/// stem::syscall::wait_many(&specs, &mut results, None)?;
/// // op is still owned; call await_completion() to consume the result
///
/// // Option C: fire-and-forget (let the mutation complete in the background)
/// let op = GraphOpHandle::create_node("thing:sensor")?;
/// op.detach();
/// ```
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
    pub fn prop_set<S: IntoSymbolRef>(
        node: ThingId,
        key: S,
        value: u64,
    ) -> Result<Self, Errno> {
        Ok(Self {
            id: sys::prop_set_async(node, key, value)?,
            owned: true,
        })
    }

    /// Submit an asynchronous graph link mutation.
    ///
    /// Returns a handle that can be awaited, polled, or detached.
    pub fn link<S: IntoSymbolRef>(src: ThingId, rel: S, dst: ThingId) -> Result<Self, Errno> {
        Ok(Self {
            id: sys::link_async(src, rel, dst)?,
            owned: true,
        })
    }

    /// Submit an asynchronous graph node creation.
    ///
    /// Returns a handle that can be awaited, polled, or detached.
    /// On completion, [`await_completion`](Self::await_completion) returns the new node's ID.
    pub fn create_node<S: IntoSymbolRef>(kind: S) -> Result<Self, Errno> {
        Ok(Self {
            id: sys::create_node_async(kind)?,
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

    /// Block until the operation completes and return its result value.
    ///
    /// For [`create_node`](Self::create_node) handles, the return value is the
    /// new node's raw ID. Consumes the handle.
    pub fn await_completion(mut self) -> Result<u64, Errno> {
        self.owned = false;
        sys::async_wait(self.id)
    }

    /// Block until the operation completes and return its result value.
    ///
    /// Alias for [`await_completion`](Self::await_completion).
    /// Retained for backward compatibility; prefer `await_completion()` in new code.
    pub fn take_result(self) -> Result<u64, Errno> {
        self.await_completion()
    }

    /// Release this handle without cancelling the underlying operation.
    ///
    /// The mutation continues to execute in the background. Use this when
    /// submitting fire-and-forget work that does not require a completion
    /// notification. Errors from the operation are silently discarded.
    pub fn detach(mut self) {
        if self.owned {
            sys::async_drop(self.id);
            self.owned = false;
        }
    }

    /// Release this handle.
    ///
    /// **Note**: in the current kernel implementation, dropping the handle via
    /// `cancel()` does **not** abort an already-queued operation — the root
    /// service will still execute it. This releases the caller's interest in
    /// the result. Prefer [`detach`](Self::detach) for explicit fire-and-forget
    /// semantics; `cancel()` is retained for backward compatibility.
    #[deprecated(note = "Use detach() for explicit fire-and-forget semantics")]
    pub fn cancel(self) {
        self.detach();
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

/// A typed handle representing a completed or in-progress graph query.
///
/// `QueryHandle<T>` provides the same composable interface as
/// [`GraphOpHandle`] for query results. Queries in the current kernel
/// implementation execute synchronously and are immediately available;
/// the handle API is designed to be forward-compatible with an async
/// query backend.
///
/// # Usage patterns
///
/// ```ignore
/// // Submit a query and take its result immediately
/// let handle = QueryHandle::submit(|| find_node_by_kind("thing:sensor"));
/// let node_id = handle.take_result()?;
///
/// // Integrate with wait_many (handle is already complete; fires immediately)
/// let handle = QueryHandle::submit(|| find_node_by_kind("thing:display"));
/// // Since queries are currently synchronous, wait_spec is a no-op placeholder.
/// // Future async backends will make this non-trivial.
/// ```
pub struct QueryHandle<T> {
    result: Result<T, Errno>,
}

impl<T> QueryHandle<T> {
    /// Execute `f` immediately and wrap the result in a handle.
    ///
    /// In the current implementation queries run synchronously. The handle
    /// API allows callers to compose query results with the same patterns
    /// used for [`GraphOpHandle`] mutations.
    pub fn submit<F: FnOnce() -> Result<T, Errno>>(f: F) -> Self {
        Self { result: f() }
    }

    /// Returns `true`. Queries currently execute synchronously.
    pub fn is_complete(&self) -> bool {
        true
    }

    /// Consume the handle and return the query result.
    pub fn take_result(self) -> Result<T, Errno> {
        self.result
    }

    /// Discard the handle and its result.
    ///
    /// This is a no-op for queries (which execute synchronously) but exists to
    /// provide API symmetry with [`GraphOpHandle::detach`] so callers can use
    /// the same fire-and-forget pattern regardless of whether the work item is
    /// a mutation or a query.
    pub fn detach(self) {
        // Result is discarded.
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for QueryHandle<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("QueryHandle")
            .field("result", &self.result)
            .finish()
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
