#![cfg(target_os = "none")]

use abi::wire::common::UserSlice;
use abi::{USER_HEAP_END, USER_HEAP_START};
use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
// use linked_list_allocator::Heap;

// First bad allocation recorded as (align << 32) | (size_low32)
static FIRST_BAD_LAYOUT: AtomicU64 = AtomicU64::new(0);

// Bump allocator for debug stability
pub struct BumpAllocator {
    start: usize,
    end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    const fn empty() -> Self {
        BumpAllocator {
            start: 0,
            end: 0,
            next: 0,
            allocations: 0,
        }
    }

    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.next = start;
    }

    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let alloc_start = align_up(self.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return core::ptr::null_mut(),
        };

        if alloc_end > self.end {
            return core::ptr::null_mut();
        }

        self.next = alloc_end;
        self.allocations += 1;
        alloc_start as *mut u8
    }

    fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        self.allocations = self.allocations.saturating_sub(1);
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[repr(C)]
struct SimpleLockedHeap {
    lock: AtomicBool,
    inner: UnsafeCell<BumpAllocator>,
}

unsafe impl Sync for SimpleLockedHeap {}

impl SimpleLockedHeap {
    pub const fn new() -> Self {
        Self {
            lock: AtomicBool::new(false),
            inner: UnsafeCell::new(BumpAllocator::empty()),
        }
    }

    pub fn lock(&self) -> HeapGuard {
        while self.lock.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        HeapGuard {
            lock: &self.lock,
            inner: unsafe { &mut *self.inner.get() },
        }
    }

    pub unsafe fn init(&self, start: *mut u8, size: usize) {
        self.lock.store(false, Ordering::Relaxed);
        let heap = &mut *self.inner.get();
        heap.init(start as usize, size);
    }
}

pub struct HeapGuard<'a> {
    lock: &'a AtomicBool,
    inner: &'a mut BumpAllocator,
}

impl<'a> core::ops::Deref for HeapGuard<'a> {
    type Target = BumpAllocator;
    fn deref(&self) -> &BumpAllocator {
        self.inner
    }
}

impl core::ops::DerefMut for HeapGuard<'_> {
    fn deref_mut(&mut self) -> &mut BumpAllocator {
        self.inner
    }
}

impl Drop for HeapGuard<'_> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}

unsafe impl GlobalAlloc for SimpleLockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut heap = self.lock();
        heap.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut heap = self.lock();
        heap.dealloc(ptr, layout)
    }
}

struct CheckedHeap(SimpleLockedHeap);

unsafe impl GlobalAlloc for CheckedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let a = layout.align();
        // Check power-of-two alignment
        if a == 0 || (a & (a - 1)) != 0 {
            // ... error handling ...
            log_raw("ALLOC ERROR: layout.align is not power-of-two (halting)");
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
    log_hex_val(val as u64);
}

fn log_hex_val(val: u64) {
    let mut buf = [0u8; 16];
    for i in 0..16 {
        let shift = (15 - i) * 4;
        let nibble = ((val >> shift) & 0xF) as u8;
        buf[i] = if nibble < 10 {
            b'0' + nibble
        } else {
            b'A' + (nibble - 10)
        };
    }
    let s = unsafe { core::str::from_utf8_unchecked(&buf) };
    log_raw(s);
}

pub fn init_user_heap() {
    unsafe {
        let heap_start = USER_HEAP_START as *mut u8;
        let heap_size = USER_HEAP_END.saturating_sub(USER_HEAP_START);

        GLOBAL_ALLOCATOR.0.init(heap_start, heap_size);
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
