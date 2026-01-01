//! Syscall Dispatch Router

use crate::syscall::{cap, graph, log, memory, watch};
use crate::syscall::cap::CapOp;
use abi::syscall::nr;
use abi::wire::SyscallResult;
// use abi::syscall::err;

/// Main syscall dispatch function
#[no_mangle]
pub extern "C" fn dispatch(nr: u32, a0: u64, a1: u64, a2: u64, a3: u64, _a4: u64, _a5: u64) -> SyscallResult {
    // Debug logging for specific syscalls can be verbose, maybe limit it?
    // crate::log::klog(crate::log::Level::Trace, "SYSCALL", &alloc::format!("nr={} a0={:x}", nr, a0));

    // Resolve arguments based on calling convention (handled by caller, passed here)

    let result = match nr {
        // === Logging ===
        nr::SYS_LOG => {
            // Permission Check: Log
            // Target: place.logs (implicit)
            if let Err(e) = cap::check(CapOp::Log, None) {
                 return e;
            }
            log::sys_log_emit(a0, a1, a2)
        },

        // === Scheduling ===
        nr::SYS_SCHED_YIELD => {
            crate::sched::yield_current();
            SyscallResult::new(0, 0, 0)
        },

        // === Graph Mutation ===
        nr::SYS_THING_CREATE => {
            // Args: kind_low, parent_low
            // Op: Create
            // Target: parent
            let _kind_id = a0; // Partial symbol? Or full ID handling needed?
            // Note: Current ABI passed partials. We should move to full IDs if possible or adapt.
            // For now, assuming lower 64 bits of SymbolId for Kind, lower 64 for Parent ThingId.
            // But ThingId is 128 bit. 
            // In demo, we used u64.
            // We need to clarify if we are using Handles or just truncated IDs.
            // Task 06 used `abi::ids::ThingId(a1 as u128)`.
            // We will stick to that for now.
            
            let parent_id = abi::ids::ThingId(a1 as u128);
            if let Err(e) = cap::check(CapOp::GraphCreate, Some(parent_id)) {
                return e;
            }
            graph::sys_thing_create(a0, a1)
        },
        nr::SYS_REL_CREATE => {
            // Args: kind_low, from_low, to_low
            let from_id = abi::ids::ThingId(a1 as u128);
            let to_id = abi::ids::ThingId(a2 as u128);
            if let Err(e) = cap::check(CapOp::GraphLink, Some(from_id)) {
                return e;
            }
            // Optional: check 'to'?
            // User requested: "require permission on from only... rely on containment boundaries"
            
            graph::sys_relationship_create(a0, a1, a2)
        },
        nr::SYS_REL_DELETE => {
            // Args: rel_id_low (or from/to/kind tuple?)
            // We need a stable RelationshipId.
            // Let's assume a0 is rel_id (low).
            let rel_id = abi::ids::ThingId(a0 as u128);
            
            // To check permissions, we need to know the 'from' of this relationship.
            // That requires a graph lookup *inside* cap::check or before it.
            // We must retrieve the relationship first.
            if let Some(rel) = ::graph::store::get_relationship(rel_id) {
                if let Err(e) = cap::check(CapOp::GraphUnlink, Some(rel.from)) {
                    return e;
                }
                graph::sys_relationship_delete(a0)
            } else {
                 SyscallResult::new(abi::syscall::err::EINVAL, 0, 0)
            }
        },

        // === Graph Observation ===
        nr::SYS_THING_GET => {
            // Target: thing
            let thing_id = abi::ids::ThingId(a0 as u128);
            if let Err(e) = cap::check(CapOp::GraphRead, Some(thing_id)) {
                 return e;
            }
            graph::sys_thing_get(a0, a1, a2)
        },
        nr::SYS_REL_GET_FROM => {
            let thing_id = abi::ids::ThingId(a0 as u128);
             if let Err(e) = cap::check(CapOp::GraphRead, Some(thing_id)) {
                 return e;
            }
            graph::sys_relationships_from(a0, a1, a2)
        },
        
        // === Memory ===
        nr::SYS_BYTESPACE_CREATE => {
            // Op: MemUpdate (on self)
            // Target: self address space (implicit)
            if let Err(e) = cap::check(CapOp::MemManage, None) {
                return e;
            }
            memory::sys_bytespace_create(a0, a1)
        },
        nr::SYS_SPACE_MAP => {
            if let Err(e) = cap::check(CapOp::MemManage, None) {
                return e;
            }
            memory::sys_space_map(a0, a1, a2, a3)
        },
        nr::SYS_SPACE_UNMAP => {
             if let Err(e) = cap::check(CapOp::MemManage, None) {
                return e;
            }
            memory::sys_space_unmap(a0, a1)
        },
        nr::SYS_HEAP_GROW => {
             if let Err(e) = cap::check(CapOp::MemManage, None) {
                return e;
            }
            memory::sys_heap_grow(a0)
        },

        // === Watch ===
        nr::SYS_WATCH_CREATE => {
             // Target: thing to watch
             let target = abi::ids::ThingId(a1 as u128);
             if let Err(e) = cap::check(CapOp::GraphWatch, Some(target)) {
                 return e;
             }
             watch::sys_watch_create(a0, a1)
        },
        nr::SYS_WATCH_POLL => {
             // Target: watch handle (which is a Thing).
             // Cap: Read on watch handle?
             let watcher = abi::ids::ThingId(a0 as u128);
             if let Err(e) = cap::check(CapOp::GraphRead, Some(watcher)) {
                 return e;
             }
             watch::sys_watch_poll(a0, a1, a2)
        },

        // === Process ===
        // TODO: Cap checks for Spawn?
        nr::SYS_PROC_SPAWN => {
             // Should require Cap? Yes.
             // But let's leave it open for 'sprout' capability or similar?
             // Kernel panic if random app tries it?
             // For now: Only allow if task has 'cap.proc.spawn'?
             // Implementing minimal check:
             // Let's require MemManage for now (implicit superuser-ish)
             if let Err(e) = cap::check(CapOp::MemManage, None) {
                  return e;
             }
              crate::syscall::dispatch::mod_legacy_spawn(a0, a1)
        },
        nr::SYS_PROC_EXIT => {
             crate::sched::exit_current_task(a0 as i32);
        },
        
        // === Machine (Restricted) ===
        nr::SYS_MACHINE => {
            // Kernel tasks only? Or special cap?
             if let Err(e) = cap::check(CapOp::Hardware, None) {
                  return e;
             }
              crate::log::klog(crate::log::Level::Info, "SYS", "sys_machine called");
              crate::syscall::dispatch::sys_machine(a0, a1, a2, a3)
        },

        // === Legacy / Catch-all ===
        // nr::SYS_VERSION_GET => ...
        0 => SyscallResult::new(0, 0, 3), // Version

        _ => SyscallResult::new(abi::syscall::err::ENOSYS, 0, 0),
    };

    result
}

// Temporary helpers for legacy functions not yet moved
pub fn mod_legacy_spawn(name_ptr: u64, name_len: u64) -> SyscallResult {
    if name_ptr == 0 || name_len == 0 {
        return SyscallResult::new(abi::syscall::err::EINVAL, 0, 0);
    }
    let name = unsafe {
        core::str::from_utf8(core::slice::from_raw_parts(name_ptr as *const u8, name_len as usize))
    };
    if name.is_err() {
         return SyscallResult::new(abi::syscall::err::EINVAL, 0, 0);
    }
    let name = name.unwrap();
    let ctx = crate::boot::get_boot_ctx();
    crate::boot::spawn_module_by_name(ctx, name);
    SyscallResult::new(0, 0, 0)
}

pub fn sys_machine(op: u64, a1: u64, a2: u64, a3: u64) -> SyscallResult {
    use abi::syscall::err;
    use crate::machine::{self, MmioFlags, MmioRange};
    
    // Define machine_op if not exists in abi
    // assuming constants:
    const CONSOLE_WRITE: u64 = 0;
    const MMIO_MAP: u64 = 1;

    match op {
        CONSOLE_WRITE => {
            if a1 == 0 { return SyscallResult::new(err::EFAULT, 0, 0); }
            let bytes = unsafe { core::slice::from_raw_parts(a1 as *const u8, a2 as usize) };
            let written = machine::machine().console_write(bytes) as u64;
            SyscallResult::new(0, written, 0)
        },
        MMIO_MAP => {
             if a2 == 0 { return SyscallResult::new(err::EINVAL, 0, 0); }
             let flags = match MmioFlags::from_bits(a3 as u32) {
                Some(f) => f,
                None => return SyscallResult::new(err::EINVAL, 0, 0),
             };
             let range = MmioRange { phys: a1, len: a2 as usize };
             if let Some(mapping) = machine::machine().mmio_map(range, flags) {
                 SyscallResult::new(0, mapping.virt, mapping.len as u64)
             } else {
                 SyscallResult::new(err::EFAULT, 0, 0)
             }
        },
        _ => SyscallResult::new(err::EINVAL, 0, 0),
    }
}
