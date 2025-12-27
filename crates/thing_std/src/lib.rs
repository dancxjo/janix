#![no_std]
#![feature(alloc_error_handler)]

pub mod syscalls;
pub mod debug;
pub mod client;
pub mod time;
pub mod console;
pub mod typed;
pub mod font;

pub use client::GraphClient;
pub use console::{Console, StdoutConsole};
pub use typed::ThingType;
pub use syscalls::rtc_read;

extern crate alloc;
use core::alloc::{GlobalAlloc, Layout};

// Minimal allocator (placeholder or pass-through)
// For v0 user setup, we might rely on a global allocator provided by custom link script or just panic.
// But `apps/keylog` and `drivers` use `alloc`.
// We need an allocator.
// We can use the linked-list-allocator or equivalent if we had heap.
// For now, let's implement a dummy or simple bump allocator if "abi::memory" provides heap.
// OR assume the binary provided it (not true for libraries).
// Actually, `thing_std` should provide it.

pub fn init() {
    // No-op for now?
}

pub mod allocator {
    use core::alloc::{GlobalAlloc, Layout};
    pub struct Dummy;
    unsafe impl GlobalAlloc for Dummy {
        unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { core::ptr::null_mut() }
        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
    }
}
// We will rely on a "real" allocator later. For now, if we use `alloc`, we crash without one.
// The `user/drivers/Cargo.toml` did NOT include an allocator crate.
// v0.2 userland assumes `thing_std` provides it.
// I will Stub it for now but if they use `alloc`, it will fail at runtime.
// The user prompt says "crates/models... serde/postcard friendly". Postcard needs `alloc` feature for convenient use?
// The user prompt says `DriverPublish` uses `Vec<u8>`. `Vec` requires alloc.
// Implementing a real allocator is non-trivial without brk/sbrk.
// For v0, let's Assume `user` apps are linked with a static heap or we use `linked_list_allocator`.
// But I can't add dependencies to `thing_std` easily right now without checking root workspace deps?
// Root workspace has no `linked_list_allocator`.
// It has `heapless`?
// Okay, for v0, maybe I shouldn't rely on `alloc`?
// But `models` uses `Vec`? `Observation { thing_bytes: Vec<u8> }`.
// Yes.
// So I MUST provide an allocator.
// I can implement a simple bump ptr allocator using a static array.
// 64KB heap.

const HEAP_SIZE: usize = 64 * 1024;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut HEAP_TOP: usize = 0;

struct SimpleAllocator;

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        let mut top = HEAP_TOP;
        let start = (HEAP.as_ptr() as usize + top);
        let modulo = start % align;
        let offset = if modulo == 0 { 0 } else { align - modulo };
        
        if top + offset + size > HEAP_SIZE {
            return core::ptr::null_mut();
        }
        
        HEAP_TOP += offset + size;
        (HEAP.as_ptr() as usize + top + offset) as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // leakage is fine for v0 input demo
    }
}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator;

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("Alloc error: {:?}", layout);
}

#[cfg(all(not(test), not(feature = "std")))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    let _ = debug::PortWrites.write_str("USER PANIC: ");
    if let Some(loc) = info.location() {
        let _ = debug::PortWrites.write_fmt(format_args!("at {}:{}: ", loc.file(), loc.line()));
    }
    let msg = info.message();
    let _ = debug::PortWrites.write_fmt(format_args!("{}\n", msg));
    loop {}
}
