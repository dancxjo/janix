//! Port: Fixed-size ring buffer for IPC (SPSC for v0)
//!
//! Ports are kernel-managed byte pipes with capability-gated access.
//! Each port has a single writer and single reader handle.

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use alloc::sync::Arc;

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
    waiters: Mutex<VecDeque<u64>>,

    #[cfg(debug_assertions)]
    sender_tid: AtomicU64,
    #[cfg(debug_assertions)]
    receiver_tid: AtomicU64,
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
            waiters: Mutex::new(VecDeque::new()),
            #[cfg(debug_assertions)]
            sender_tid: AtomicU64::new(0),
            #[cfg(debug_assertions)]
            receiver_tid: AtomicU64::new(0),
        }
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

        self.head.store(head.wrapping_add(to_write), Ordering::Release);
        
        // Wake up waiters
        let mut handlers = self.waiters.lock();
        while let Some(tid) = handlers.pop_front() {
            unsafe {
                crate::task::scheduler::wake_task_erased(tid as usize);
            }
        }
        
        to_write
    }

    /// Add a waiter to the port
    pub fn add_waiter(&self, tid: u64) {
        let mut waiters = self.waiters.lock();
        if !waiters.contains(&tid) {
            waiters.push_back(tid);
        }
    }

    /// Remove a waiter from the port
    pub fn remove_waiter(&self, tid: u64) {
        let mut waiters = self.waiters.lock();
        if let Some(pos) = waiters.iter().position(|&id| id == tid) {
            waiters.remove(pos);
        }
    }

    /// Receive bytes from the port. Returns number of bytes read.
    ///
    /// This method is gated by debug assertions to ensure only one consumer task
    /// accesses the port (SPSC).
    pub fn recv(&self, buf: &mut [u8]) -> usize {
        #[cfg(debug_assertions)]
        self.check_ownership(false);

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

        self.tail.store(tail.wrapping_add(to_read), Ordering::Release);
        to_read
    }

    #[cfg(debug_assertions)]
    fn check_ownership(&self, is_sender: bool) {
        // We use the erased hook to avoid generic param requirements
        let current = unsafe { crate::task::scheduler::current_tid_current() };
        if current == 0 {
            return; // Allow kernel/idle access
        }

        let target = if is_sender { &self.sender_tid } else { &self.receiver_tid };
        let owner = target.load(Ordering::Acquire);
        
        if owner == 0 {
            // First task to use this half becomes the permanent owner
            target.store(current, Ordering::Release);
        } else {
            assert_eq!(owner, current, 
                "IPC SPSC violation: Task {} tried to {} on a port owned by task {}",
                current, if is_sender { "send" } else { "receive" }, owner);
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
        self.inner.add_waiter(tid);
    }

    pub fn remove_waiter(&self, tid: u64) {
        self.inner.remove_waiter(tid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::sync::Arc;

    /// Test basic send/receive on a Port
    #[test]
    fn test_port_send_recv() {
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
}
