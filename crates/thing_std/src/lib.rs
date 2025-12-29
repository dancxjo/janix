#![no_std]
#![feature(alloc_error_handler)]
#![allow(unused)]
#![allow(static_mut_refs)]
#![allow(unexpected_cfgs)]

pub mod client;
pub mod console;
pub mod debug;
pub mod font;
pub mod syscalls;
pub mod time;
pub mod typed;
pub mod bytespace;
pub mod view;

pub use client::GraphClient;
pub use console::{Console, StdoutConsole};
pub use syscalls::rtc_read;
pub use typed::ThingType;
pub use abi::ThingId;
pub use abi::SymbolId;

extern crate alloc;

pub fn init() {
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = $crate::debug::PortWrites.write_fmt(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[cfg(all(target_os = "none", not(test)))]
mod runtime {
    use core::alloc::{GlobalAlloc, Layout};
    use super::debug;

    const HEAP_SIZE: usize = 128 * 1024;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    static mut HEAP_TOP: usize = 0;

    struct SimpleAllocator;

    unsafe impl GlobalAlloc for SimpleAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let size = layout.size();
            let align = layout.align();

            let top = HEAP_TOP;
            let start = HEAP.as_ptr() as usize + top;
            let modulo = start % align;
            let offset = if modulo == 0 { 0 } else { align - modulo };

            if top + offset + size > HEAP_SIZE {
                return core::ptr::null_mut();
            }

            HEAP_TOP += offset + size;
            (HEAP.as_ptr() as usize + top + offset) as *mut u8
        }
        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        }
    }

    #[global_allocator]
    static ALLOCATOR: SimpleAllocator = SimpleAllocator;

    #[alloc_error_handler]
    fn alloc_error(layout: Layout) -> ! {
        panic!("Alloc error: {:?}", layout);
    }

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
}
