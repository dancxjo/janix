#![cfg(target_os = "none")]

use abi::wire::common::UserSlice;
use abi::{USER_HEAP_END, USER_HEAP_START};
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use linked_list_allocator::LockedHeap;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

// First bad allocation recorded as (align << 32) | (size_low32)
static FIRST_BAD_LAYOUT: AtomicU64 = AtomicU64::new(0);

struct CheckedHeap(LockedHeap);

unsafe impl GlobalAlloc for CheckedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let a = layout.align();
        let s = layout.size();

        // If alignment is ever not a power-of-two, that is a *hard invariant break*.
        if a == 0 || (a & (a - 1)) != 0 {
            let packed = ((a as u64) << 32) | ((s as u64) & 0xFFFF_FFFF);
            FIRST_BAD_LAYOUT.compare_exchange(0, packed, Ordering::AcqRel, Ordering::Relaxed).ok();

            // Log without allocating
            let req = abi::KernelRequest::Log { message: UserSlice::from_slice("ALLOC ERROR: layout.align is not power-of-two (halting)".as_bytes()) };
            crate::syscalls::syscall(req);

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
static GLOBAL_ALLOCATOR: CheckedHeap = CheckedHeap(LockedHeap::empty());

pub fn init_user_heap() {
    // Manually log to avoid allocation/formatting issues during heap init
    let req = abi::KernelRequest::Log { message: UserSlice::from_slice("init_user_heap: entered".as_bytes()) };
    unsafe { crate::syscalls::syscall(req); }

    if INITIALIZED.swap(true, Ordering::AcqRel) {
        let req = abi::KernelRequest::Log { message: UserSlice::from_slice("init_user_heap: already initialized".as_bytes()) };
        unsafe { crate::syscalls::syscall(req); }
        return;
    }
    unsafe {
        let heap_start = USER_HEAP_START as *mut u8;
        let heap_size = USER_HEAP_END.saturating_sub(USER_HEAP_START);
        
        // Check and log address
        let ga_addr = &GLOBAL_ALLOCATOR as *const _ as usize;
        
        fn log_addr(label: &str, val: usize) {
            // Minimal hex buffer
            let mut buf = [0u8; 64];
            let mut i = 0;
            for &b in label.as_bytes() {
                if i < buf.len() { buf[i] = b; i += 1; }
            }
            if i < buf.len() { buf[i] = b'='; i += 1; }
            if val == 0 {
                if i < buf.len() { buf[i] = b'0'; i += 1; }
            } else {
                // write hex (reverse)
                let mut v = val;
                let start = i;
                while v > 0 && i < buf.len() {
                    let d = v % 16;
                    buf[i] = if d < 10 { b'0' + d as u8 } else { b'a' + (d - 10) as u8 };
                    v /= 16;
                    i += 1;
                }
                // reverse
                let end = i;
                let len = end - start;
                for j in 0..len/2 {
                    let tmp = buf[start+j];
                    buf[start+j] = buf[end-1-j];
                    buf[end-1-j] = tmp;
                }
            }
            let req = abi::KernelRequest::Log { message: UserSlice::from_slice(&buf[..i]) };
            unsafe { crate::syscalls::syscall(req); }
        }

        log_addr("GLOBAL_ALLOCATOR", ga_addr);

        if ga_addr == 0 { loop {} }

        let req = abi::KernelRequest::Log { message: UserSlice::from_slice("init_user_heap: locking...".as_bytes()) };
        crate::syscalls::syscall(req);

        let lock_ref = &GLOBAL_ALLOCATOR.0;
        log_addr("LockRef", lock_ref as *const _ as usize);

        let mut guard = lock_ref.lock();
        log_addr("GuardAcquired", 1); // just a marker

        let heap_ref = &mut *guard;
        log_addr("HeapRef", heap_ref as *const _ as usize);

        if (heap_ref as *const _ as usize) == 0 {
             log_addr("HeapRefIsNull", 0);
             loop {}
        }

        heap_ref.init(heap_start, heap_size);
        
        let req = abi::KernelRequest::Log { message: UserSlice::from_slice("init_user_heap: initialized".as_bytes()) };
        crate::syscalls::syscall(req);
        
        let req = abi::KernelRequest::Log { message: UserSlice::from_slice("init_user_heap: allocator initialized".as_bytes()) };
        crate::syscalls::syscall(req);

        let packed = FIRST_BAD_LAYOUT.load(Ordering::Acquire);
        if packed != 0 {
            // Don’t format! Just log a fixed string.
            let req = abi::KernelRequest::Log { message: UserSlice::from_slice("init_user_heap: WARNING: FIRST_BAD_LAYOUT was set".as_bytes()) };
            crate::syscalls::syscall(req);
        }
    }
}

#[cfg(target_os = "none")]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Allocation failed");
}
