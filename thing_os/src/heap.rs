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
        while self.lock.swap(true, Ordering::Acquire) {
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
    log_raw(label);
    log_raw("0x");
    // Simple hex print logic if needed, or just rely on the fact that we fixed the lock
    // For now, let's keep it simple or remove it if not critical.
    // Actually, let's just use log_raw("...") for simplicity to verify the fix works first.
}

pub fn init_user_heap() {
     unsafe {
        let heap_start = USER_HEAP_START as *mut u8;
        let heap_size = USER_HEAP_END.saturating_sub(USER_HEAP_START);

        GLOBAL_ALLOCATOR.0.lock().init(heap_start, heap_size);
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
