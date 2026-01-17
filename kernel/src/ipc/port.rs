//! Port: Fixed-size ring buffer for IPC (SPSC for v0)
//!
//! Ports are kernel-managed byte pipes with capability-gated access.
//! Each port has a single writer and single reader handle.

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Unique identifier for a port in the global registry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortId(pub u32);

/// Fixed-size ring buffer port (SPSC for v0)
pub struct Port {
    buf: Box<[u8]>,
    capacity: usize,
    head: AtomicUsize, // Write position (producer advances)
    tail: AtomicUsize, // Read position (consumer advances)
    waiters: Mutex<VecDeque<u64>>,
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
    pub fn send(&self, data: &[u8]) -> usize {
        let available = self.available();
        let to_write = data.len().min(available);
        
        if to_write == 0 {
            return 0;
        }

        let head = self.head.load(Ordering::Relaxed);
        let mask = self.capacity - 1; // Works because capacity is power of 2

        for (i, &byte) in data[..to_write].iter().enumerate() {
            let idx = (head + i) & mask;
            // SAFETY: We hold exclusive write access (SPSC), idx is within bounds
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
    pub fn recv(&self, buf: &mut [u8]) -> usize {
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
}

// SAFETY: Port is safe to share between threads (atomic indices, SPSC access pattern)
unsafe impl Send for Port {}
unsafe impl Sync for Port {}
