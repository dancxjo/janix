//! Kernel IRQ Subsystem
//!
//! Manages interrupt routing and userspace IRQ subscriptions.

use alloc::vec::Vec;
use spin::Mutex;

pub mod msi;

pub const EXTERNAL_VECTOR_START: u8 = 0x40;
pub const EXTERNAL_VECTOR_END: u8 = 0xEF;

/// Maximum supported vectors for IRQ dispatch
pub const MAX_VECTORS: usize = 256;

/// IRQ subscriber entry
#[derive(Clone)]
struct IrqSubscription {
    task_id: usize,
    pending_count: u32,
}

/// Global IRQ registry
pub struct IrqRegistry {
    /// Subscribers per vector (vector -> list of subscribers)
    subscribers: [Vec<IrqSubscription>; MAX_VECTORS],
}

impl IrqRegistry {
    pub const fn new() -> Self {
        Self {
            subscribers: [const { Vec::new() }; MAX_VECTORS],
        }
    }
    
    /// Subscribe a task to receive interrupts for a vector
    pub fn subscribe(&mut self, vector: u8, task_id: usize) -> Result<(), ()> {
        let subs = &mut self.subscribers[vector as usize];
        
        // Check if already subscribed
        for sub in subs.iter() {
            if sub.task_id == task_id {
                return Err(()); // Already subscribed
            }
        }
        
        subs.push(IrqSubscription {
            task_id,
            pending_count: 0,
        });
        
        Ok(())
    }
    
    /// Unsubscribe a task from a vector
    #[allow(dead_code)]
    pub fn unsubscribe(&mut self, vector: u8, task_id: usize) {
        let subs = &mut self.subscribers[vector as usize];
        subs.retain(|s| s.task_id != task_id);
    }
    
    /// Dispatch an interrupt - increment pending count and wake waiters
    pub fn dispatch(&mut self, vector: u8) {
        let subs = &mut self.subscribers[vector as usize];
        for sub in subs.iter_mut() {
            sub.pending_count = sub.pending_count.saturating_add(1);
            // Wake the task using type-erased hook
            unsafe {
                crate::task::scheduler::wake_task_erased(sub.task_id);
            }
        }
    }
    
    /// Wait for interrupt - returns pending count and resets it
    /// Returns 0 if caller should block
    pub fn try_wait(&mut self, vector: u8, task_id: usize) -> u32 {
        let subs = &mut self.subscribers[vector as usize];
        for sub in subs.iter_mut() {
            if sub.task_id == task_id {
                let count = sub.pending_count;
                sub.pending_count = 0;
                return count;
            }
        }
        0
    }
}

/// Global IRQ registry instance
pub static IRQ_REGISTRY: Mutex<IrqRegistry> = Mutex::new(IrqRegistry::new());

pub struct VectorAllocator {
    used: [bool; MAX_VECTORS],
    owner_graph: [u64; MAX_VECTORS],
    owner_irq: [u8; MAX_VECTORS],
}

impl VectorAllocator {
    pub const fn new() -> Self {
        Self {
            used: [false; MAX_VECTORS],
            owner_graph: [0; MAX_VECTORS],
            owner_irq: [0; MAX_VECTORS],
        }
    }

    pub fn alloc(&mut self, graph_id: u64, irq_index: u8) -> Option<u8> {
        for v in EXTERNAL_VECTOR_START..=EXTERNAL_VECTOR_END {
            let idx = v as usize;
            if !self.used[idx] {
                self.used[idx] = true;
                self.owner_graph[idx] = graph_id;
                self.owner_irq[idx] = irq_index;
                return Some(v);
            }
        }
        None
    }

    pub fn free(&mut self, vector: u8) {
        let idx = vector as usize;
        if idx < MAX_VECTORS {
            self.used[idx] = false;
            self.owner_graph[idx] = 0;
            self.owner_irq[idx] = 0;
        }
    }

    pub fn owner(&self, vector: u8) -> Option<(u64, u8)> {
        let idx = vector as usize;
        if idx < MAX_VECTORS && self.used[idx] && self.owner_graph[idx] != 0 {
            return Some((self.owner_graph[idx], self.owner_irq[idx]));
        }
        None
    }
}

pub static VECTOR_ALLOC: Mutex<VectorAllocator> = Mutex::new(VectorAllocator::new());

/// Called from interrupt handlers to dispatch IRQ
pub fn dispatch_irq(vector: u8) {
    IRQ_REGISTRY.lock().dispatch(vector);
}

pub fn alloc_vector(graph_id: u64, irq_index: u8) -> Option<u8> {
    VECTOR_ALLOC.lock().alloc(graph_id, irq_index)
}

pub fn free_vector(vector: u8) {
    VECTOR_ALLOC.lock().free(vector);
}

pub fn vector_owner(vector: u8) -> Option<(u64, u8)> {
    VECTOR_ALLOC.lock().owner(vector)
}

/// Subscribe current task to a vector
pub fn subscribe(vector: u8) -> Result<(), ()> {
    let task_id = unsafe { crate::task::scheduler::current_tid_current() } as usize;
    IRQ_REGISTRY.lock().subscribe(vector, task_id)
}

/// Wait for IRQ - blocks until interrupt fires
/// Returns number of pending interrupts
pub fn wait(vector: u8) -> u32 {
    let task_id = unsafe { crate::task::scheduler::current_tid_current() } as usize;
    
    loop {
        {
            let mut reg = IRQ_REGISTRY.lock();
            let count = reg.try_wait(vector, task_id);
            if count > 0 {
                return count;
            }
        }
        // Block until woken by interrupt
        unsafe {
            crate::task::scheduler::block_current_erased();
        }
    }
}
