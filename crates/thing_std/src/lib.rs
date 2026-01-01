#![no_std]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;

struct BumpAllocator {
    heap: UnsafeCell<[u8; 64 * 1024]>,
    pos: UnsafeCell<usize>,
}

unsafe impl Sync for BumpAllocator {}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator {
    heap: UnsafeCell::new([0; 64 * 1024]),
    pos: UnsafeCell::new(0),
};

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pos = self.pos.get();
        let heap = self.heap.get() as *mut u8;

        let align = layout.align();
        let size = layout.size();

        let current = *pos;
        let aligned_pos = (current + align - 1) & !(align - 1);
        if aligned_pos + size > 64 * 1024 {
            return core::ptr::null_mut();
        }
        let ptr = heap.add(aligned_pos);
        *pos = aligned_pos + size;
        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
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
        syscall(1, 2, msg.as_ptr() as u64, msg.len() as u64, 0);
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

