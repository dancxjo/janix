use core::alloc::Layout;
use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

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
        if i == 0 { break; }
        i -= 1;
    }
    crate::serial::write(&buf[i + 1..]);
}
