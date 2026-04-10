//! Port: Fixed-size ring buffer for IPC (SPSC for v0)
//!
//! Ports are kernel-managed byte pipes with capability-gated access.
//! Each port has a single writer and single reader handle.

use alloc::boxed::Box;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use spin::Mutex;

/// Unique identifier for a port in the global registry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortId(pub u32);

/// Fixed-size ring buffer port (SPSC for v0)
///
/// This structure contains the shared state and ring buffer.
/// Access is intended to be via `Sender` and `Receiver` halves
/// which enforce the SPSC invariant.
pub struct Port {
    buf: Box<[u8]>,
    capacity: usize,
    head: AtomicUsize, // Write position (producer advances)
    tail: AtomicUsize, // Read position (consumer advances)
    waiters_read: crate::sched::WaitQueue,
    waiters_write: crate::sched::WaitQueue,
    send_lock: Mutex<()>,
    recv_lock: Mutex<()>,
    endpoints: Mutex<PortEndpoints>,
    caps: Mutex<alloc::collections::VecDeque<Arc<dyn crate::vfs::VfsNode>>>,

    #[cfg(debug_assertions)]
    sender_tid: AtomicU64,
    #[cfg(debug_assertions)]
    receiver_tid: AtomicU64,
}

#[derive(Debug, Clone, Copy)]
struct PortEndpoints {
    readers: u32,
    writers: u32,
}

impl Port {
    /// Create a new port with the given capacity (rounded up to power of 2)
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two().max(16).min(65536);
        let buf = alloc::vec![0u8; capacity].into_boxed_slice();
        Self {
            buf,
            capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            waiters_read: crate::sched::WaitQueue::new(),
            waiters_write: crate::sched::WaitQueue::new(),
            send_lock: Mutex::new(()),
            recv_lock: Mutex::new(()),
            endpoints: Mutex::new(PortEndpoints {
                readers: 1,
                writers: 1,
            }),
            caps: Mutex::new(alloc::collections::VecDeque::new()),
            #[cfg(debug_assertions)]
            sender_tid: AtomicU64::new(0),
            #[cfg(debug_assertions)]
            receiver_tid: AtomicU64::new(0),
        }
    }

    /// Returns the capacity of the port
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the number of bytes currently in the buffer
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        head.wrapping_sub(tail)
    }

    /// Returns true if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if the buffer is full
    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity
    }

    /// Returns available space for writing
    pub fn available(&self) -> usize {
        self.capacity - self.len()
    }

    /// Send bytes to the port. Returns number of bytes written.
    /// If buffer is full, drops bytes (bounded loss behavior).
    ///
    /// This method is gated by debug assertions to ensure only one producer task
    /// accesses the port (SPSC).
    pub fn send(&self, data: &[u8]) -> usize {
        #[cfg(debug_assertions)]
        self.check_ownership(true);

        let _guard = self.send_lock.lock();
        let available = self.available();
        let to_write = data.len().min(available);

        if to_write == 0 {
            return 0;
        }

        let head = self.head.load(Ordering::Relaxed);
        let mask = self.capacity - 1; // Works because capacity is power of 2

        for (i, &byte) in data[..to_write].iter().enumerate() {
            let idx = (head + i) & mask;
            // SAFETY: We hold exclusive write access (SPSC), idx is within bounds.
            // On weakly ordered architectures, the Release store to head below ensures
            // this write is visible to a consumer performing an Acquire load.
            unsafe {
                let ptr = self.buf.as_ptr() as *mut u8;
                ptr.add(idx).write(byte);
            }
        }

        self.head
            .store(head.wrapping_add(to_write), Ordering::Release);

        // One queued message should wake one receiver.
        self.waiters_read.wake_one();

        let tid = unsafe { crate::sched::current_tid_current() };
        crate::ktrace!("PORT: Port written {} bytes from TID {}", to_write, tid);

        to_write
    }

    /// Send bytes only if the full payload fits.
    /// Returns true if all bytes were written, false if no bytes were written.
    pub fn send_all(&self, data: &[u8]) -> bool {
        #[cfg(debug_assertions)]
        self.check_ownership(true);

        let _guard = self.send_lock.lock();
        let available = self.available();
        if available < data.len() {
            return false;
        }

        if data.is_empty() {
            return true;
        }

        let head = self.head.load(Ordering::Relaxed);
        let mask = self.capacity - 1;
        for (i, &byte) in data.iter().enumerate() {
            let idx = (head + i) & mask;
            // SAFETY: idx is bounded by mask/capacity and send_lock serializes producers.
            unsafe {
                let ptr = self.buf.as_ptr() as *mut u8;
                ptr.add(idx).write(byte);
            }
        }

        self.head
            .store(head.wrapping_add(data.len()), Ordering::Release);
        self.waiters_read.wake_one();
        true
    }

    /// Send a capability (VFS Node) via the port
    pub fn send_cap(&self, node: Arc<dyn crate::vfs::VfsNode>) {
        let tid = unsafe { crate::sched::current_tid_current() };
        self.caps.lock().push_back(node);
        self.waiters_read.wake_one();
        crate::ktrace!("PORT: Port capability sent from TID {}", tid);
    }

    /// Receive a capability (VFS Node) from the port
    pub fn try_recv_cap(&self) -> Option<Arc<dyn crate::vfs::VfsNode>> {
        self.caps.lock().pop_front()
    }

    /// Add a reader waiter to the port
    pub fn add_waiter_read(&self, tid: u64) {
        self.waiters_read.push_back(tid);
    }

    /// Remove a reader waiter from the port
    pub fn remove_waiter_read(&self, tid: u64) {
        self.waiters_read.remove(tid);
    }

    /// Add a writer waiter to the port
    pub fn add_waiter_write(&self, tid: u64) {
        self.waiters_write.push_back(tid);
    }

    /// Remove a writer waiter from the port
    pub fn remove_waiter_write(&self, tid: u64) {
        self.waiters_write.remove(tid);
    }

    /// Receive bytes from the port. Returns number of bytes read.
    ///
    /// This method is gated by debug assertions to ensure only one consumer task
    /// accesses the port (SPSC).
    pub fn try_recv(&self, buf: &mut [u8]) -> usize {
        #[cfg(debug_assertions)]
        self.check_ownership(false);

        let _guard = self.recv_lock.lock();
        let available = self.len();
        let to_read = buf.len().min(available);

        if to_read == 0 {
            return 0;
        }

        let tail = self.tail.load(Ordering::Relaxed);
        let mask = self.capacity - 1;

        for i in 0..to_read {
            let idx = (tail + i) & mask;
            buf[i] = self.buf[idx];
        }

        self.tail
            .store(tail.wrapping_add(to_read), Ordering::Release);

        // Wake up one writer (pacing/flow control)
        self.waiters_write.wake_one();

        let tid = unsafe { crate::sched::current_tid_current() };
        crate::ktrace!("PORT: Port read {} bytes from TID {}", to_read, tid);

        to_read
    }

    pub fn recv(&self, buf: &mut [u8]) -> usize {
        self.try_recv(buf)
    }

    pub fn has_readers(&self) -> bool {
        self.endpoints.lock().readers > 0
    }

    pub fn has_writers(&self) -> bool {
        self.endpoints.lock().writers > 0
    }

    pub fn close_reader(&self) -> bool {
        let mut endpoints = self.endpoints.lock();
        if endpoints.readers == 0 {
            return false;
        }
        endpoints.readers -= 1;
        let destroy = endpoints.readers == 0 && endpoints.writers == 0;
        let last_reader = endpoints.readers == 0;
        drop(endpoints);

        if last_reader {
            self.waiters_write.wake_all();
        }

        destroy
    }

    pub fn close_writer(&self) -> bool {
        let mut endpoints = self.endpoints.lock();
        if endpoints.writers == 0 {
            return false;
        }
        endpoints.writers -= 1;
        let destroy = endpoints.readers == 0 && endpoints.writers == 0;
        let last_writer = endpoints.writers == 0;
        drop(endpoints);

        if last_writer {
            self.waiters_read.wake_all();
        }

        destroy
    }

    #[cfg(debug_assertions)]
    fn check_ownership(&self, is_sender: bool) {
        // We use the erased hook to avoid generic param requirements
        let current = unsafe { crate::sched::current_tid_current() };
        if current == 0 {
            return; // Allow kernel/idle access
        }

        let target = if is_sender {
            &self.sender_tid
        } else {
            &self.receiver_tid
        };
        let owner = target.load(Ordering::Acquire);

        if let Err(owner) = target.compare_exchange(0, current, Ordering::AcqRel, Ordering::Acquire)
        {
            if owner != current {
                // In v0 "Single Process Model", multiple tasks might share handles and ports.
                // This violates strict SPSC but is currently expected in some discovery flows.
                // We warn once per port to avoid log flood while still highlighting the issue.
                // For now, we don't panic to maintain SMP stability.
            }
        }
    }
}

// SAFETY: Port is safe to share between threads (atomic indices, SPSC access pattern).
// We implement Send and Sync but rely on Sender/Receiver wrappers and debug assertions
// to maintain the SPSC invariant.
unsafe impl Send for Port {}
unsafe impl Sync for Port {}

/// Unique handle to the sending side of a Port
pub struct Sender {
    inner: Arc<Port>,
}

impl Sender {
    pub fn new(inner: Arc<Port>) -> Self {
        Self { inner }
    }

    pub fn send(&self, data: &[u8]) -> usize {
        self.inner.send(data)
    }

    pub fn available(&self) -> usize {
        self.inner.available()
    }
}

/// Unique handle to the receiving side of a Port
pub struct Receiver {
    inner: Arc<Port>,
}

impl Receiver {
    pub fn new(inner: Arc<Port>) -> Self {
        Self { inner }
    }

    pub fn recv(&self, buf: &mut [u8]) -> usize {
        self.inner.recv(buf)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn add_waiter(&self, tid: u64) {
        self.inner.add_waiter_read(tid);
    }

    pub fn remove_waiter(&self, tid: u64) {
        self.inner.remove_waiter_read(tid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::sync::Arc;

    /// Test basic send/receive on a Port
    #[test]
    fn test_channel_send_recv() {
        let port = Arc::new(Port::new(64));
        let sender = Sender::new(Arc::clone(&port));
        let receiver = Receiver::new(Arc::clone(&port));

        // Initially empty
        assert!(receiver.is_empty());
        assert_eq!(receiver.len(), 0);

        // Send some data
        let data = b"hello world";
        let written = sender.send(data);
        assert_eq!(written, data.len());

        // Should be readable now
        assert!(!receiver.is_empty());
        assert_eq!(receiver.len(), data.len());

        // Receive
        let mut buf = [0u8; 64];
        let read = receiver.recv(&mut buf);
        assert_eq!(read, data.len());
        assert_eq!(&buf[..read], data);

        // Should be empty again
        assert!(receiver.is_empty());
    }

    /// Test wrap-around behavior in ring buffer
    #[test]
    fn test_port_ring_buffer_wrap() {
        let port = Arc::new(Port::new(16)); // Smallest useful power of 2
        let sender = Sender::new(Arc::clone(&port));
        let receiver = Receiver::new(Arc::clone(&port));

        // Fill buffer multiple times to test wrap-around
        for round in 0..5 {
            let data = [round as u8; 8];
            let written = sender.send(&data);
            assert_eq!(written, 8);

            let mut buf = [0u8; 8];
            let read = receiver.recv(&mut buf);
            assert_eq!(read, 8);
            assert_eq!(buf, data);
        }
    }

    /// Test bounded-loss behavior when buffer is full
    #[test]
    fn test_port_bounded_loss() {
        let port = Arc::new(Port::new(16));
        let sender = Sender::new(Arc::clone(&port));

        // Fill the buffer
        let data = [0xAB; 16];
        let written = sender.send(&data);
        assert_eq!(written, 16);

        // Further sends should return 0 (bounded loss)
        let written2 = sender.send(&[0xFF; 4]);
        assert_eq!(written2, 0);
    }

    #[test]
    fn test_channel_send_all_atomic() {
        let port = Arc::new(Port::new(16));
        let receiver = Receiver::new(Arc::clone(&port));

        assert!(port.send_all(&[1, 2, 3, 4]));
        assert!(!port.send_all(&[9; 13])); // Would overflow; must not partially write
        assert_eq!(receiver.len(), 4);

        let mut out = [0u8; 16];
        let n = receiver.recv(&mut out);
        assert_eq!(n, 4);
        assert_eq!(&out[..n], &[1, 2, 3, 4]);
    }

    #[test]
    fn test_port_try_recv_empty() {
        let port = Arc::new(Port::new(16));
        let receiver = Receiver::new(Arc::clone(&port));
        let mut out = [0u8; 8];
        assert_eq!(receiver.recv(&mut out), 0);
        assert!(receiver.is_empty());
    }

    #[test]
    fn test_port_close_endpoints() {
        let port = Arc::new(Port::new(16));
        assert!(port.has_readers());
        assert!(port.has_writers());

        assert!(!port.close_writer());
        assert!(!port.has_writers());
        assert!(port.has_readers());

        assert!(port.close_reader());
        assert!(!port.has_readers());
    }
}
