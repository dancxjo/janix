#![no_std]
#![feature(alloc_error_handler)]
#![allow(internal_features)]
#![feature(lang_items)]

extern crate abi;
extern crate alloc;

pub use abi::ids::{SymbolId, ThingId};
use abi::syscall::nr;
use abi::wire::SyscallResult;

use core::alloc::{GlobalAlloc, Layout};
use core::fmt::{self, Write};
use core::panic::PanicInfo;

pub use debug::log as log_info;
pub use process::exit as sys_exit;
pub use process::sched_yield;

pub mod codec;
pub mod cap;
pub mod event;
pub mod graph;
pub mod graphics;
pub mod input;
pub mod memory;
pub mod process;
pub mod time;

// Re-exports
pub use codec::*;
pub use graph::*;
pub use graphics::*;
pub use input::*;
pub use memory::*;
pub use process::*;
pub use time::*;

// ============================================================================
// Entry Point & runtime
// ============================================================================

#[cfg(not(any(test, target_os = "linux")))]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    extern "C" {
        fn main();
    }
    init_heap(1024 * 1024);
    main();
    process::exit(0);
}

#[inline(always)]
pub unsafe fn syscall(
    nr: u32,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> SyscallResult {
    let status: u64;
    let val0: u64;
    let val1: u64;

    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
        "syscall",
        in("rax") nr as u64,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        in("r8") a4,
        in("r9") a5,
        lateout("rax") status,
        lateout("rdx") val0,
        lateout("r8") val1,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );

    #[cfg(target_arch = "aarch64")]
    core::arch::asm!(
        "svc #0",
        in("x8") nr as u64,
        in("x0") a0,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
        in("x4") a4,
        in("x5") a5,
        lateout("x0") status,
        lateout("x1") val0,
        lateout("x2") val1,
        options(nostack, preserves_flags)
    );

    #[cfg(target_arch = "riscv64")]
    core::arch::asm!(
        "ecall",
        in("a7") nr as u64,
        in("a0") a0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        in("a5") a5,
        lateout("a0") status,
        lateout("a1") val0,
        lateout("a2") val1,
        options(nostack, preserves_flags)
    );

    #[cfg(target_arch = "loongarch64")]
    core::arch::asm!(
        "syscall 0",
        in("$a7") nr as u64,
        in("$a0") a0,
        in("$a1") a1,
        in("$a2") a2,
        in("$a3") a3,
        in("$a4") a4,
        in("$a5") a5,
        lateout("$a0") status,
        lateout("$a1") val0,
        lateout("$a2") val1,
        options(nostack, preserves_flags)
    );

    SyscallResult { status, val0, val1 }
}

pub fn init(_ptr: u64) {}

// ============================================================================
// Allocator
// ============================================================================

struct BumpAllocator;

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

static mut HEAP_START: usize = 0;
static mut HEAP_CURRENT: usize = 0;
static mut HEAP_LIMIT: usize = 0;

pub unsafe fn init_heap(size: usize) {
    // Check if NR exists. We use SYS_HEAP_GROW (33).
    loop {
        let res = syscall(nr::SYS_HEAP_GROW, size as u64, 0, 0, 0, 0, 0);
        if res.status == 0 {
            // Kernel returns the OLD break, which is our new start
            let start = res.val0 as usize;
            HEAP_START = start;
            HEAP_CURRENT = start;
            HEAP_LIMIT = start + size;
            break;
        }
        process::sched_yield();
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();

        let mut current = HEAP_CURRENT;
        let padding = (align - (current % align)) % align;
        current += padding;

        if current + size > HEAP_LIMIT {
            let grow_size = (size + padding).max(64 * 1024);
            if syscall(nr::SYS_HEAP_GROW, grow_size as u64, 0, 0, 0, 0, 0).status == 0 {
                HEAP_LIMIT += grow_size;
            } else {
                return core::ptr::null_mut();
            }
        }

        let ptr = current as *mut u8;
        HEAP_CURRENT = current + size;
        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log_info("panic: aborting process");
    log_panic_details(info);
    process::exit(-1);
}

fn log_panic_details(info: &PanicInfo) {
    log_fmt("panic message: ", format_args!("{}", info.message()));

    if let Some(loc) = info.location() {
        let mut buf = LogBuf::new();
        let _ = buf.write_str("panic location: ");
        let _ = buf.write_fmt(format_args!(
            "{}:{}:{}",
            loc.file(),
            loc.line(),
            loc.column()
        ));
        buf.flush();
    } else {
        log_info("panic location: <unknown>");
    }
}

fn log_fmt(prefix: &str, args: fmt::Arguments) {
    let mut buf = LogBuf::new();
    let _ = buf.write_str(prefix);
    let _ = buf.write_fmt(args);
    buf.flush();
}

struct LogBuf {
    buf: [u8; 192],
    len: usize,
}

impl LogBuf {
    const fn new() -> Self {
        Self {
            buf: [0; 192],
            len: 0,
        }
    }

    fn flush(&mut self) {
        if self.len == 0 {
            return;
        }

        if let Ok(s) = core::str::from_utf8(&self.buf[..self.len]) {
            debug::log(s);
        }
        self.len = 0;
    }
}

impl Write for LogBuf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let available = self.buf.len().saturating_sub(self.len);
        let to_copy = bytes.len().min(available);

        if to_copy > 0 {
            self.buf[self.len..self.len + to_copy].copy_from_slice(&bytes[..to_copy]);
            self.len += to_copy;
        }

        Ok(())
    }
}

pub mod debug {
    use super::*;
    pub fn log(msg: &str) {
        unsafe {
            syscall(
                nr::SYS_LOG,
                2,
                msg.as_ptr() as u64,
                msg.len() as u64,
                0,
                0,
                0,
            );
        }
    }
}

// lang items
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
