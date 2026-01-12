//! Kernel IRQ Subsystem
//!
//! Manages interrupt routing and userspace IRQ subscriptions.

use alloc::vec::Vec;
use spin::Mutex;

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

/// Called from interrupt handlers to dispatch IRQ
pub fn dispatch_irq(vector: u8) {
    IRQ_REGISTRY.lock().dispatch(vector);
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
