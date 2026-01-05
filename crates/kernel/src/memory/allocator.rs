use core::alloc::Layout;
use linked_list_allocator::LockedHeap;
// use x86_64::instructions::interrupts; // REMOVED


pub struct SafeLockedHeap(LockedHeap);

impl SafeLockedHeap {
    pub const fn empty() -> Self {
        Self(LockedHeap::empty())
    }
}

// Delegate initialization and lock()
impl core::ops::Deref for SafeLockedHeap {
    type Target = LockedHeap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl core::alloc::GlobalAlloc for SafeLockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let token = crate::machine::irq_disable();
        let res = self.0.alloc(layout);
        crate::machine::irq_restore(token);
        res
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let token = crate::machine::irq_disable();
        self.0.dealloc(ptr, layout);
        crate::machine::irq_restore(token);
    }
}

#[global_allocator]
pub static ALLOCATOR: SafeLockedHeap = SafeLockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    use crate::serial::{write, write_hex};

    write(b"\n\n========== KERNEL ALLOCATION FAILED ==========\n");
    write(b"Requested size: ");
    write_hex(layout.size() as u64);
    write(b" (");
    print_decimal(layout.size() as u64);
    write(b" bytes)\n");
    write(b"Requested align: ");
    write_hex(layout.align() as u64);
    write(b"\n");

    // Try to get heap stats
    // Note: This lock might deadlock if we are in an interrupt and the lock is held.
    // However, if we are in alloc_error_handler, we are likely panicking anyway.
    let stats = ALLOCATOR.lock();
    write(b"\nHeap state:\n");
    write(b"  Free bytes: ");
    write_hex(stats.free() as u64);
    write(b"\n  Used bytes: ");
    write_hex(stats.used() as u64);
    write(b"\n");
    drop(stats);

    write(b"\nPossible causes:\n");
    write(b"  - Heap exhausted (increase heap size or fix leak)\n");
    write(b"  - Fragmentation (no contiguous block available)\n");
    write(b"  - Heap not initialized yet (too early allocation)\n");
    write(b"================================================\n");

    panic!("allocation failed")
}

fn print_decimal(mut v: u64) {
    if v == 0 {
        crate::serial::write(b"0");
        return;
    }
    let mut buf = [0u8; 20];
    let mut i = 19;
    while v > 0 {
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if i == 0 {
            break;
        }
        i -= 1;
    }
    crate::serial::write(&buf[i + 1..]);
}
