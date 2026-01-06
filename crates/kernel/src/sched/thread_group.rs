//! Thread Group - represents a shared address space and resources.
//!
//! All threads in a group share the same AddressSpace, heap, and capabilities.

use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::memory::space::AddressSpace;
use abi::cap::Cap;
use abi::ids::ThingId;

/// Stack slot allocator for thread user stacks.
/// Each slot represents a 1MB region starting from STACK_REGION_TOP.
/// Uses a bitmap to track allocated slots, supporting up to 64 threads per group.
pub struct StackSlotAllocator {
    /// Bitmap of allocated slots (bit N = slot N is in use)
    slots: u64,
    /// Number of active allocations
    count: usize,
}

impl StackSlotAllocator {
    /// Thread stacks start at 0x8F00_0000 and grow DOWN with 1MB stride per slot.
    /// This places them between the heap (0x9000_0000+) and assets (0x8000_0000-0x87FF_FFFF).
    pub const STACK_REGION_TOP: u64 = 0x8F00_0000;
    pub const STACK_STRIDE: u64 = 0x10_0000; // 1MB per thread slot
    pub const STACK_SIZE: u64 = 0x1_0000;    // 64KB actual stack
    pub const MAX_SLOTS: usize = 64;         // Up to 64 threads per process

    pub const fn new() -> Self {
        Self { slots: 0, count: 0 }
    }

    /// Allocate a stack slot, returning (stack_bottom, stack_top) or None.
    pub fn alloc(&mut self) -> Option<(u64, u64)> {
        // Find first free slot
        for i in 0..Self::MAX_SLOTS {
            if (self.slots & (1 << i)) == 0 {
                self.slots |= 1 << i;
                self.count += 1;
                let top = Self::STACK_REGION_TOP - (i as u64 * Self::STACK_STRIDE);
                let bottom = top - Self::STACK_SIZE;
                return Some((bottom, top));
            }
        }
        None
    }

    /// Free a stack slot given its top address.
    pub fn free(&mut self, stack_top: u64) -> bool {
        if stack_top > Self::STACK_REGION_TOP {
            return false;
        }
        let slot = (Self::STACK_REGION_TOP - stack_top) / Self::STACK_STRIDE;
        if slot < Self::MAX_SLOTS as u64 && (self.slots & (1 << slot)) != 0 {
            self.slots &= !(1 << slot);
            if self.count > 0 {
                self.count -= 1;
            }
            true
        } else {
            false
        }
    }

    pub fn active_count(&self) -> usize {
        self.count
    }
}

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
    /// Per-group stack slot allocator for spawned threads
    pub stack_allocator: StackSlotAllocator,
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
            stack_allocator: StackSlotAllocator::new(),
        }
    }
}
