#![cfg(target_os = "none")]

use abi::wire::common::UserSlice;
use abi::{USER_HEAP_END, USER_HEAP_START};
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use core::cell::UnsafeCell;
use linked_list_allocator::Heap;

// First bad allocation recorded as (align << 32) | (size_low32)
static FIRST_BAD_LAYOUT: AtomicU64 = AtomicU64::new(0);

#[repr(C)]
struct SimpleLockedHeap {
    lock: AtomicBool,
    inner: UnsafeCell<Heap>,
}

unsafe impl Sync for SimpleLockedHeap {}

impl SimpleLockedHeap {
    pub const fn new() -> Self {
        Self {
            lock: AtomicBool::new(false),
            inner: UnsafeCell::new(Heap::empty()),
        }
    }

    pub fn lock(&self) -> HeapGuard {
        loop {
            if self.lock.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                break;
            }
            core::hint::spin_loop();
        }
        HeapGuard { lock: &self.lock, inner: unsafe { &mut *self.inner.get() } }
    }
}

pub struct HeapGuard<'a> {
    lock: &'a AtomicBool,
    inner: &'a mut Heap,
}

impl core::ops::Deref for HeapGuard<'_> {
    type Target = Heap;
    fn deref(&self) -> &Heap { self.inner }
}

impl core::ops::DerefMut for HeapGuard<'_> {
    fn deref_mut(&mut self) -> &mut Heap { self.inner }
}

impl Drop for HeapGuard<'_> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}

unsafe impl GlobalAlloc for SimpleLockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut heap = self.lock();
        heap.allocate_first_fit(layout)
            .ok()
            .map(|ptr| ptr.as_ptr())
            .unwrap_or(core::ptr::null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut heap = self.lock();
        if let Some(p) = core::ptr::NonNull::new(ptr) {
            heap.deallocate(p, layout);
        }
    }
}

struct CheckedHeap(SimpleLockedHeap);

unsafe impl GlobalAlloc for CheckedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let a = layout.align();
        let s = layout.size();

        // If alignment is ever not a power-of-two, that is a *hard invariant break*.
        if a == 0 || (a & (a - 1)) != 0 {
            let packed = ((a as u64) << 32) | ((s as u64) & 0xFFFF_FFFF);
            FIRST_BAD_LAYOUT.compare_exchange(0, packed, Ordering::AcqRel, Ordering::Relaxed).ok();

            // Log without allocating
            log_raw("ALLOC ERROR: layout.align is not power-of-two (halting)");

            // Halt: we want the earliest failure point, not cascading corruption.
            loop {}
        }

        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CheckedHeap = CheckedHeap(SimpleLockedHeap::new());

fn log_raw(s: &str) {
    let ptr = s.as_ptr() as u64;
    let len = s.len() as u64;
    unsafe {
        crate::sys::raw_syscall(abi::syscalls::SYSCALL_LOG, ptr, len, 0, 0, 0, 0);
    }
}

fn log_hex(label: &str, val: usize) {
    let mut buf = [0u8; 128];
    let mut idx = 0;

    // Append label
    for &b in label.as_bytes() {
        if idx < buf.len() {
            buf[idx] = b;
            idx += 1;
        }
    }

    // Append "0x"
    if idx + 2 <= buf.len() {
        buf[idx] = b'0';
        buf[idx + 1] = b'x';
        idx += 2;
    }

    // Convert to hex (16 hex digits for 64-bit usize)
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        let c = if digit < 10 {
            b'0' + digit as u8
        } else {
            b'a' + (digit - 10) as u8
        };
        if idx < buf.len() {
            buf[idx] = c;
            idx += 1;
        }
    }

    // No newline appended here because kernel::log adds one
    let s = unsafe { core::str::from_utf8_unchecked(&buf[..idx]) };
    log_raw(s);
}

pub fn init_user_heap() {
    log_raw("heap::init_heap: starting");
    log_hex("heap::init_heap: start=", USER_HEAP_START);

    unsafe {
        let heap_start = USER_HEAP_START as *mut u8;
        let heap_size = USER_HEAP_END.saturating_sub(USER_HEAP_START);

        GLOBAL_ALLOCATOR.0.lock().init(heap_start, heap_size);

        log_raw("heap::init_heap: initialized");
    }
}

#[cfg(target_os = "none")]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    log_raw("ALLOCATION FAILED");

    // Exit thread gracefully (code 1 for error)
    unsafe {
        crate::sys::raw_syscall(abi::syscalls::SYSCALL_EXIT_THREAD, 1, 0, 0, 0, 0, 0);
    }
    loop {}
}
