use linked_list_allocator::LockedHeap;
use core::alloc::Layout;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    crate::serial::write(b"PANIC: allocation failed. Size: ");
    safe_print_u64(layout.size() as u64);
    crate::serial::write(b"\n");
    panic!("allocation failed")
}

fn safe_print_u64(v: u64) {

    let mut i = 0;
    if v == 0 {
        crate::serial::write(b"0x0");
        return;
    }
    // write hex
    crate::serial::write(b"0x");
    // count leading zeros or just print? 
    // simple hex dump
    for shift in (0..16).rev() {
        let digit = (v >> (shift * 4)) & 0xF;
        if digit > 0 || i > 0 || shift == 0 {
             let c = if digit < 10 { b'0' + digit as u8 } else { b'a' + (digit - 10) as u8 };
             crate::serial::write(&[c]);
             i += 1;
        }
    }
}
