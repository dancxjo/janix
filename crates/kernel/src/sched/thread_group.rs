//! Thread Group - represents a shared address space and resources.
//!
//! All threads in a group share the same AddressSpace, heap, and capabilities.

use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::memory::space::AddressSpace;
use abi::cap::Cap;
use abi::ids::ThingId;

/// A thread group owns the shared resources for a set of threads.
pub struct ThreadGroup {
    /// Unique group identifier
    pub id: u64,
    /// Graph representation of this group
    pub thing: ThingId,
    /// Shared address space for all threads in the group
    pub address_space: Arc<AddressSpace>,
    /// Heap base address
    pub heap_base: u64,
    /// Current heap size
    pub heap_size: u64,
    /// Current heap break (brk)
    pub heap_brk: u64,
    /// Capabilities granted to this group
    pub caps: Vec<Cap>,
    /// Number of active threads in this group
    pub thread_count: usize,
}

impl ThreadGroup {
    /// Create a new thread group with the given address space.
    pub fn new(id: u64, thing: ThingId, address_space: Arc<AddressSpace>) -> Self {
        Self {
            id,
            thing,
            address_space,
            heap_base: 0x9000_0000,
            heap_size: 0,
            heap_brk: 0x9000_0000,
            caps: Vec::new(),
            thread_count: 1,
        }
    }
}
