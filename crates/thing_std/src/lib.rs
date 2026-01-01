#![no_std]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};


struct BumpAllocator;

unsafe impl Sync for BumpAllocator {}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // 1. Get current break to calculate alignment
        let current_brk = heap_grow(0);
        if current_brk == 0 { return core::ptr::null_mut(); } // Syscall failed or early boot?
        
        let align = layout.align() as u64;
        let size = layout.size() as u64;
        
        // Calculate padding needed for alignment
        let new_start = (current_brk + align - 1) & !(align - 1);
        let padding = new_start - current_brk;
        
        // 2. Allocate needed space (padding + size)
        let ptr = heap_grow(padding + size);
        if ptr == 0 { return core::ptr::null_mut(); }
        
        // ptr should == current_brk. The valid memory starts at ptr + padding.
        (ptr + padding) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op: we never free
    }
}

pub fn heap_grow(increment: u64) -> u64 {
    unsafe {
        let res = syscall(abi::syscall::SYSCALL_HEAP_GROW as u32, increment, 0, 0, 0);
        if res.status != 0 {
            0
        } else {
            res.val0
        }
    }
}





use abi::ids::{PlaceId, RelationshipId, SymbolId, ThingId};
use abi::wire::SyscallResult;


static mut SYSCALL_DISPATCH: Option<abi::wire::SyscallDispatch> = None;

pub fn init(ptr: u64) {
    unsafe {
        SYSCALL_DISPATCH = Some(core::mem::transmute(ptr));
    }
}

#[unsafe(no_mangle)]
pub unsafe fn syscall(nr: u32, a0: u64, a1: u64, a2: u64, a3: u64) -> SyscallResult {
    if let Some(dispatch) = SYSCALL_DISPATCH {
        dispatch(nr, a0, a1, a2, a3, 0, 0)
    } else {
        SyscallResult::new(-1, 0, 0)
    }
}

pub fn get_root_place() -> PlaceId {
    let res = unsafe { syscall(300, 0, 0, 0, 0) };
    ThingId(((res.val0 as u128) << 64) | (res.val1 as u128))
}

pub fn thing_create(kind: SymbolId, schema: SymbolId, version: u32) -> ThingId {
    let res = unsafe { syscall(301, 10, kind.0, schema.0, version as u64) };
    ThingId(((res.val0 as u128) << 64) | (res.val1 as u128))
}

pub fn relationship_create(from: ThingId, to: ThingId, predicate: SymbolId) -> RelationshipId {
    let res = unsafe { syscall(301, 20, from.0 as u64, to.0 as u64, predicate.0) };
    ThingId(((res.val0 as u128) << 64) | (res.val1 as u128))
}

pub fn contained_in(place: PlaceId) -> usize {
    let res = unsafe { syscall(301, 30, place.0 as u64, 0, 0) };
    res.val0 as usize
}

pub fn log_info(msg: &str) {
    unsafe {
        syscall(abi::syscall::SYSCALL_LOG as u32, 2, msg.as_ptr() as u64, msg.len() as u64, 0);
    }
}

pub fn console_write(msg: &str) {
    unsafe {
        syscall(3, 0, msg.as_ptr() as u64, msg.len() as u64, 0);
    }
}

pub fn proc_spawn(name: &str) {
    unsafe {
        syscall(100, name.as_ptr() as u64, name.len() as u64, 0, 0);
    }
}

pub fn thing_create_named(name: SymbolId, kind: SymbolId, schema: SymbolId) -> ThingId {
    let res = unsafe { syscall(301, 11, name.0, kind.0, schema.0) };
    ThingId(((res.val0 as u128) << 64) | (res.val1 as u128))
}

pub fn thing_set_payload(id: ThingId, payload: &[u8]) {
    unsafe {
        syscall(301, 12, id.0 as u64, payload.as_ptr() as u64, payload.len() as u64);
    }
}

pub fn thing_get_payload(id: ThingId, buffer: &mut [u8]) -> usize {
    let res = unsafe { syscall(301, 13, id.0 as u64, buffer.as_mut_ptr() as u64, buffer.len() as u64) };
    res.val0 as usize
}

pub fn thing_find_by_name(name: SymbolId) -> Option<ThingId> {
    let res = unsafe { syscall(301, 40, name.0, 0, 0) };
    if res.status == 0 {
        Some(ThingId(((res.val0 as u128) << 64) | (res.val1 as u128)))
    } else {
        None
    }
}
pub fn symbol_intern(name: &str) -> SymbolId {
    let res = unsafe { syscall(2, name.as_ptr() as u64, name.len() as u64, 0, 0) };
    SymbolId(res.val0)
}

pub fn relationships_from_into(id: ThingId, out: &mut [ThingId]) -> usize {
    let buf_len = out.len() * 16;
    let ptr = out.as_mut_ptr() as u64;
    
    // Syscall returns NUMBER OF RELATIONSHIPS copied
    let res = unsafe { syscall(301, 50, id.0 as u64, ptr, buf_len as u64) };
    res.val0 as usize
}

pub mod event;

pub fn sched_yield() {
    unsafe { syscall(200, 0, 0, 0, 0) };
}

pub fn watch(watcher: ThingId, target: ThingId) {
    unsafe { syscall(500, watcher.0 as u64, target.0 as u64, 0, 0) };
}

pub fn wait_event(watcher: ThingId) -> ThingId {
    let res = unsafe { syscall(501, watcher.0 as u64, 0, 0, 0) };
    ThingId(((res.val0 as u128) << 64) | (res.val1 as u128))
}


pub fn sys_exit(code: i32) -> ! {
    unsafe { syscall(101, code as u64, 0, 0, 0) };
    loop {}
}
