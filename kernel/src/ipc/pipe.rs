//! Anonymous pipe IPC — kernel-resident bounded byte channel.
//!
//! Each pipe has a ring buffer, reader/writer ref counts, and wait queues.
//! Blocking uses the scheduler's `block_current_erased()` / `wake_task_erased()`.

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use crate::sched::wait_queue::WaitQueue;

// ---------------------------------------------------------------------------
// Ring buffer
// ---------------------------------------------------------------------------

struct RingBuf {
    data: Vec<u8>,
    head: usize, // next read position
    tail: usize, // next write position
    len: usize,  // bytes currently in buffer
    cap: usize,
}

impl RingBuf {
    fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            data: {
                let mut v = Vec::with_capacity(cap);
                v.resize(cap, 0);
                v
            },
            head: 0,
            tail: 0,
            len: 0,
            cap,
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.cap
    }

    fn free_space(&self) -> usize {
        self.cap - self.len
    }

    fn available(&self) -> usize {
        self.len
    }

    /// Dequeue up to `dst.len()` bytes. Returns number of bytes read.
    fn dequeue(&mut self, dst: &mut [u8]) -> usize {
        let n = dst.len().min(self.len);
        for i in 0..n {
            dst[i] = self.data[self.head];
            self.head = (self.head + 1) % self.cap;
        }
        self.len -= n;
        n
    }

    /// Enqueue up to `src.len()` bytes. Returns number of bytes written.
    fn enqueue(&mut self, src: &[u8]) -> usize {
        let n = src.len().min(self.free_space());
        for i in 0..n {
            self.data[self.tail] = src[i];
            self.tail = (self.tail + 1) % self.cap;
        }
        self.len += n;
        n
    }
}

// ---------------------------------------------------------------------------
// Pipe inner state
// ---------------------------------------------------------------------------

pub struct PipeInner {
    buf: RingBuf,
    readers: u32,
    writers: u32,
    nonblock: bool,
    read_waitq: WaitQueue,
    write_waitq: WaitQueue,
}

// ---------------------------------------------------------------------------
// Global registry
// ---------------------------------------------------------------------------

static NEXT_PIPE_ID: AtomicU64 = AtomicU64::new(1);
static PIPES: Mutex<BTreeMap<u64, Arc<Mutex<PipeInner>>>> = Mutex::new(BTreeMap::new());

/// Default pipe capacity in bytes.
const DEFAULT_PIPE_CAPACITY: usize = 4096;

// ---------------------------------------------------------------------------
// Public API for syscall handlers
// ---------------------------------------------------------------------------

/// Create a new anonymous pipe. Returns the internal pipe ID used to back
/// read/write VFS endpoints.
pub fn create(capacity: u32, flags: u32) -> u64 {
    let cap = if capacity == 0 {
        DEFAULT_PIPE_CAPACITY
    } else {
        capacity as usize
    };
    let nonblock = (flags & abi::syscall::pipe_flags::NONBLOCK) != 0;

    let inner = Arc::new(Mutex::new(PipeInner {
        buf: RingBuf::new(cap),
        readers: 1,
        writers: 1,
        nonblock,
        read_waitq: WaitQueue::new(),
        write_waitq: WaitQueue::new(),
    }));

    let id = NEXT_PIPE_ID.fetch_add(1, Ordering::Relaxed);
    PIPES.lock().insert(id, inner);
    id
}

/// Read from a pipe. Blocks (or returns EAGAIN) when empty and writers exist.
/// Returns Ok(0) on EOF (all writers closed).
pub fn read(pipe_id: u64, dst: &mut [u8]) -> Result<usize, abi::errors::Errno> {
    let pipe = get_pipe(pipe_id)?;

    loop {
        // Get current TID for wait queue registration
        let tid = unsafe { crate::sched::current_tid_current() };

        {
            let mut inner = pipe.lock();

            // Data available — dequeue and wake writers
            if !inner.buf.is_empty() {
                let n = inner.buf.dequeue(dst);
                inner.write_waitq.wake_one();
                return Ok(n);
            }

            // No data, no writers => EOF
            if inner.writers == 0 {
                return Ok(0);
            }

            // Empty, writers exist — block or EAGAIN
            if inner.nonblock {
                return Err(abi::errors::Errno::EAGAIN);
            }

            // Register in read wait queue before dropping lock
            inner.read_waitq.push_back(tid as u64);
        }
        // Lock dropped — now park
        unsafe {
            crate::task::block_current_erased();
        }
        // Woken up — retry loop
    }
}

/// Write to a pipe. Blocks (or returns EAGAIN) when full and readers exist.
/// Returns EPIPE if no readers remain.
pub fn write(pipe_id: u64, src: &[u8]) -> Result<usize, abi::errors::Errno> {
    if src.is_empty() {
        return Ok(0);
    }

    let pipe = get_pipe(pipe_id)?;

    loop {
        let tid = unsafe { crate::sched::current_tid_current() };

        {
            let mut inner = pipe.lock();

            // No readers => broken pipe
            if inner.readers == 0 {
                return Err(abi::errors::Errno::EPIPE);
            }

            // Space available — enqueue and wake readers
            if !inner.buf.is_full() {
                let n = inner.buf.enqueue(src);
                inner.read_waitq.wake_one();
                return Ok(n);
            }

            // Full, readers exist — block or EAGAIN
            if inner.nonblock {
                return Err(abi::errors::Errno::EAGAIN);
            }

            inner.write_waitq.push_back(tid as u64);
        }
        unsafe {
            crate::task::block_current_erased();
        }
    }
}

/// Close the read end of a pipe.
pub fn close_read(pipe_id: u64) -> Result<(), abi::errors::Errno> {
    let pipe = get_pipe(pipe_id)?;
    let should_remove;
    {
        let mut inner = pipe.lock();
        if inner.readers == 0 {
            return Err(abi::errors::Errno::EBADF);
        }
        inner.readers -= 1;
        if inner.readers == 0 {
            // Wake all blocked writers so they can discover BrokenPipe
            inner.write_waitq.wake_all();
        }
        should_remove = inner.readers == 0 && inner.writers == 0;
    }
    if should_remove {
        PIPES.lock().remove(&pipe_id);
    }
    Ok(())
}

/// Close the write end of a pipe.
pub fn close_write(pipe_id: u64) -> Result<(), abi::errors::Errno> {
    let pipe = get_pipe(pipe_id)?;
    let should_remove;
    {
        let mut inner = pipe.lock();
        if inner.writers == 0 {
            return Err(abi::errors::Errno::EBADF);
        }
        inner.writers -= 1;
        if inner.writers == 0 {
            // Wake all blocked readers so they can observe EOF
            inner.read_waitq.wake_all();
        }
        should_remove = inner.readers == 0 && inner.writers == 0;
    }
    if should_remove {
        PIPES.lock().remove(&pipe_id);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn get_pipe(id: u64) -> Result<Arc<Mutex<PipeInner>>, abi::errors::Errno> {
    PIPES
        .lock()
        .get(&id)
        .cloned()
        .ok_or(abi::errors::Errno::EBADF)
}

// ---------------------------------------------------------------------------
// VfsNode wrappers for pipe file descriptors
// ---------------------------------------------------------------------------

/// The read end of an anonymous pipe, exposed as a [`crate::vfs::VfsNode`].
///
/// Created by [`create_fd_pair`] and inserted into the process fd table as
/// stdin (fd 0) or as the read end of a pipe passed to `pipe()`.
pub struct PipeReadNode {
    inner: Arc<Mutex<PipeInner>>,
    // Pipe ID kept for the global registry lookup (allows `close` to work).
    pipe_id: u64,
}

/// The write end of an anonymous pipe, exposed as a [`crate::vfs::VfsNode`].
pub struct PipeWriteNode {
    inner: Arc<Mutex<PipeInner>>,
    pipe_id: u64,
}

impl crate::vfs::VfsNode for PipeReadNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> abi::errors::SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            let tid = unsafe { crate::sched::current_tid_current() };
            {
                let mut inner = self.inner.lock();
                if !inner.buf.is_empty() {
                    let n = inner.buf.dequeue(buf);
                    inner.write_waitq.wake_one();
                    return Ok(n);
                }
                if inner.writers == 0 {
                    return Ok(0); // EOF
                }
                if inner.nonblock {
                    return Err(abi::errors::Errno::EAGAIN);
                }
                inner.read_waitq.push_back(tid as u64);
            }
            unsafe { crate::task::block_current_erased() };
        }
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> abi::errors::SysResult<usize> {
        Err(abi::errors::Errno::EBADF)
    }

    fn stat(&self) -> abi::errors::SysResult<crate::vfs::VfsStat> {
        Ok(crate::vfs::VfsStat {
            mode: crate::vfs::VfsStat::S_IFIFO | 0o400,
            size: 0,
            ino: 0,
        })
    }

    fn close(&self) {
        let should_remove;
        {
            let mut inner = self.inner.lock();
            if inner.readers > 0 {
                inner.readers -= 1;
            }
            if inner.readers == 0 {
                inner.write_waitq.wake_all();
            }
            should_remove = inner.readers == 0 && inner.writers == 0;
        }
        if should_remove {
            PIPES.lock().remove(&self.pipe_id);
        }
    }
}

impl crate::vfs::VfsNode for PipeWriteNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> abi::errors::SysResult<usize> {
        Err(abi::errors::Errno::EBADF)
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> abi::errors::SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            let tid = unsafe { crate::sched::current_tid_current() };
            {
                let mut inner = self.inner.lock();
                if inner.readers == 0 {
                    return Err(abi::errors::Errno::EPIPE);
                }
                if !inner.buf.is_full() {
                    let n = inner.buf.enqueue(buf);
                    inner.read_waitq.wake_one();
                    return Ok(n);
                }
                if inner.nonblock {
                    return Err(abi::errors::Errno::EAGAIN);
                }
                inner.write_waitq.push_back(tid as u64);
            }
            unsafe { crate::task::block_current_erased() };
        }
    }

    fn stat(&self) -> abi::errors::SysResult<crate::vfs::VfsStat> {
        Ok(crate::vfs::VfsStat {
            mode: crate::vfs::VfsStat::S_IFIFO | 0o200,
            size: 0,
            ino: 0,
        })
    }

    fn close(&self) {
        let should_remove;
        {
            let mut inner = self.inner.lock();
            if inner.writers > 0 {
                inner.writers -= 1;
            }
            if inner.writers == 0 {
                inner.read_waitq.wake_all();
            }
            should_remove = inner.readers == 0 && inner.writers == 0;
        }
        if should_remove {
            PIPES.lock().remove(&self.pipe_id);
        }
    }
}

/// Wrap an existing pipe (by `pipe_id`) as a read-end `VfsNode`.
///
/// The pipe's existing `readers` count (set to 1 by [`create`]) represents
/// this node's reference.  When the node is closed via `VfsNode::close`,
/// `readers` is decremented, matching POSIX behaviour.
///
/// Returns `None` if `pipe_id` is not found.
pub fn read_node_for_id(pipe_id: u64) -> Option<alloc::sync::Arc<dyn crate::vfs::VfsNode>> {
    let inner = get_pipe(pipe_id).ok()?;
    Some(Arc::new(PipeReadNode { inner, pipe_id }))
}

/// Wrap an existing pipe (by `pipe_id`) as a write-end `VfsNode`.
///
/// See [`read_node_for_id`] for reference-count semantics.
///
/// Returns `None` if `pipe_id` is not found.
pub fn write_node_for_id(pipe_id: u64) -> Option<alloc::sync::Arc<dyn crate::vfs::VfsNode>> {
    let inner = get_pipe(pipe_id).ok()?;
    Some(Arc::new(PipeWriteNode { inner, pipe_id }))
}

/// Create an anonymous pipe and return a `(pipe_id, read_node, write_node)` triple.
///
/// The pipe ID remains an internal kernel identifier. The read/write nodes can
/// be inserted into a child process fd table via [`crate::vfs::fd_table::FdTable::insert_at`].
pub fn create_fd_pair_with_id(
    capacity: u32,
    nonblock: bool,
) -> (
    u64,
    alloc::sync::Arc<dyn crate::vfs::VfsNode>,
    alloc::sync::Arc<dyn crate::vfs::VfsNode>,
) {
    let cap = if capacity == 0 {
        DEFAULT_PIPE_CAPACITY
    } else {
        capacity as usize
    };
    let inner = Arc::new(Mutex::new(PipeInner {
        buf: RingBuf::new(cap),
        readers: 1,
        writers: 1,
        nonblock,
        read_waitq: WaitQueue::new(),
        write_waitq: WaitQueue::new(),
    }));
    let id = NEXT_PIPE_ID.fetch_add(1, Ordering::Relaxed);
    PIPES.lock().insert(id, inner.clone());
    let read_node: alloc::sync::Arc<dyn crate::vfs::VfsNode> = Arc::new(PipeReadNode {
        inner: inner.clone(),
        pipe_id: id,
    });
    let write_node: alloc::sync::Arc<dyn crate::vfs::VfsNode> =
        Arc::new(PipeWriteNode { inner, pipe_id: id });
    (id, read_node, write_node)
}

/// Create an anonymous pipe and return a `(read_node, write_node)` pair.
///
/// Both nodes are `Arc<dyn VfsNode>` and can be inserted directly into a
/// process fd table via `FdTable::insert_at`.
pub fn create_fd_pair(
    capacity: u32,
    nonblock: bool,
) -> (
    alloc::sync::Arc<dyn crate::vfs::VfsNode>,
    alloc::sync::Arc<dyn crate::vfs::VfsNode>,
) {
    let (_id, r, w) = create_fd_pair_with_id(capacity, nonblock);
    (r, w)
}
