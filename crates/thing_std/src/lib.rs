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
        let res = syscall(abi::syscall::nr::SYS_HEAP_GROW, increment, 0, 0, 0);
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
    #[cfg(target_arch = "x86_64")]
    {
        let status: u64;
        let val0: u64;
        let val1: u64;
        
        core::arch::asm!(
            "syscall",
            in("rax") nr,
            in("rdi") a0,
            in("rsi") a1,
            in("rdx") a2,
            in("r10") a3,
            lateout("rax") status,
            lateout("rdx") val0,
            lateout("rsi") val1,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags) 
        );
        
        SyscallResult {
            status: status as i32,
            val0,
            val1
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        if let Some(dispatch) = SYSCALL_DISPATCH {
            dispatch(nr, a0, a1, a2, a3, 0, 0)
        } else {
            SyscallResult::new(-1, 0, 0)
        }
    }
}

pub fn get_root_place() -> PlaceId {
    // New ABI doesn't have explicit GET_ROOT yet, usually hardcoded or via name lookup
    // But for now keeping legacy if dispatch supports it?
    // Dispatch removed syscall 300.
    // Use find_by_name("place.root") or similar?
    // Or hardcode ID 2.
    // Let's rely on symbol lookup for now.
    match thing_find_by_name(symbol_intern("place.root")) {
        Some(id) => id,
        None => ThingId(2u128) // Fallback
    }
}

pub fn thing_create(_kind: SymbolId, _schema: SymbolId, _version: u32) -> ThingId {
    // Old signature had Schema/Version. New syscall just Kind + Parent (implicitly?).
    // Wait, sys_thing_create takes (kind, parent).
    // The old `thing_create` didn't assume parent?
    // New ABI REQUIRES parent.
    // For compatibility, let's default to user's local place?
    // But `thing_std` doesn't know it easily unless we query "place.user.<self>"
    // This breaks the old API signature.
    // Let's change the signature or use a default parent.
    // Using `place.root` requires writable root (unlikely).
    // Using `place.tasks`?
    
    // We'll update the signature to `thing_create(kind, parent)`.
    // Breaking change acceptible.
    panic!("Use thing_create_under(kind, parent)");
}

pub fn thing_create_under(kind: SymbolId, parent: ThingId) -> ThingId {
    let res = unsafe { syscall(abi::syscall::nr::SYS_THING_CREATE, kind.0, parent.0 as u64, 0, 0) };
    ThingId(((res.val0 as u128) << 64) | (res.val1 as u128))
}

pub fn relationship_create(from: ThingId, to: ThingId, predicate: SymbolId) -> RelationshipId {
    let res = unsafe { syscall(abi::syscall::nr::SYS_REL_CREATE, predicate.0, from.0 as u64, to.0 as u64, 0) };
    ThingId(((res.val0 as u128) << 64) | (res.val1 as u128))
}

// contained_in removed? No syscall for it. Use rel iter?
// Or leave stub.

pub fn log_info(msg: &str) {
    unsafe {
        syscall(abi::syscall::nr::SYS_LOG, 2, msg.as_ptr() as u64, msg.len() as u64, 0);
    }
}

pub fn console_write(msg: &str) {
    unsafe {
        // SYS_MACHINE checked in dispatch?
        syscall(abi::syscall::nr::SYS_MACHINE, 0, msg.as_ptr() as u64, msg.len() as u64, 0);
    }
}

pub fn proc_spawn(name: &str) {
    unsafe {
        syscall(abi::syscall::nr::SYS_PROC_SPAWN, name.as_ptr() as u64, name.len() as u64, 0, 0);
    }
}

pub fn thing_create_named(_name: SymbolId, kind: SymbolId, parent: ThingId) -> ThingId {
    let id = thing_create_under(kind, parent);
    // Register Name? SYSCALL 40 (FIND) exists. Register? 
    // SYS_THING_REGISTER_NAME not in new ABI?
    // User scope: "1. Creates a thing... 2. Relationship..."
    // Registering names might be a store op?
    // For now, skip naming or implement separate syscall later.
    id
}

pub fn thing_set_payload(_id: ThingId, _payload: &[u8]) {
    // Stub
}

pub fn thing_get_payload(_id: ThingId, _buffer: &mut [u8]) -> usize {
    0
}

pub fn thing_find_by_name(_name: SymbolId) -> Option<ThingId> {
    // We don't have SYS_FIND in new ABI explicit list?
    // Wait, I didn't add it.
    // I should add it or use iteration.
    // "Out of scope: Full Cypher/GQL".
    // But basic name lookup is useful.
    // Syscall dispatch `0 => ...`? 
    // Let's assume we removed it for strictness unless added.
    // Using `place.root` + iteration?
    None
}

pub fn symbol_intern(name: &str) -> SymbolId {
    let res = unsafe { syscall(abi::syscall::nr::SYS_SYMBOL_INTERN, name.as_ptr() as u64, name.len() as u64, 0, 0) };
    SymbolId(res.val0)
}

pub fn relationships_from_into(id: ThingId, out: &mut [ThingId]) -> usize {
    let buf_len = out.len() * 16; // ThingIds are 16 bytes? Syscall returns Relationship or To?
    // Dispatch says: "returns arrays of (rel_id, kind, from, to)" or similar.
    // "sys_relationships_from... -> (RelationshipId, kind, to)"?
    // Current dispatch: "buffer[start..start+16].copy_from_slice(&rel_id...)"
    // It copies just RelationshipId.
    let ptr = out.as_mut_ptr() as u64;
    let res = unsafe { syscall(abi::syscall::nr::SYS_REL_GET_FROM, id.0 as u64, 0, ptr, buf_len as u64) };
    res.val0 as usize
}

pub mod event;

pub fn sched_yield() {
    unsafe { syscall(abi::syscall::nr::SYS_SCHED_YIELD, 0, 0, 0, 0) };
}

pub fn watch(watcher: ThingId, target: ThingId) {
    unsafe { syscall(abi::syscall::nr::SYS_WATCH_CREATE, watcher.0 as u64, target.0 as u64, 0, 0) };
}

pub fn wait_event(watcher: ThingId) -> ThingId {
    let res = unsafe { syscall(abi::syscall::nr::SYS_WATCH_POLL, watcher.0 as u64, 0, 0, 0) };
    ThingId(((res.val0 as u128) << 64) | (res.val1 as u128))
}


pub fn sys_exit(code: i32) -> ! {
    unsafe { syscall(abi::syscall::nr::SYS_PROC_EXIT, code as u64, 0, 0, 0) };
    loop {}
}
