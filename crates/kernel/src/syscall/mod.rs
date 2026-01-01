//! Syscall dispatch
//!
//! Provides the kernel's system call interface. Syscalls are dispatched
//! by number and return (status, val0, val1).

use core::slice;
use alloc::format;

use crate::log::{self, Level};
use crate::machine::{self, MmioFlags, MmioRange};
use graph::store;
use graph::symbols;

use abi::ids::ThingId;

/// Syscall numbers
pub mod nr {
    pub const SYS_VERSION_GET: u32 = 0;
    pub const SYS_LOG_EMIT: u32 = 1;
    pub const SYS_SYMBOL_INTERN: u32 = 2;
    pub const SYS_MACHINE: u32 = 3;
    pub const SYS_THING_CREATE: u32 = 10;
    pub const SYS_THING_SET_PAYLOAD: u32 = 11;
    pub const SYS_LINK_CREATE: u32 = 20;
    pub const SYS_PROC_SPAWN: u32 = 100;
    pub const SYS_PROC_EXIT: u32 = 101;
    pub const SYS_HEAP_GROW: u32 = 120;
    pub const SYS_SCHED_YIELD: u32 = 200;
    
    /// Get the root PlaceId
    pub const SYS_GET_ROOT_PLACE: u32 = 300;
    /// Multi-purpose ontology operation
    pub const SYS_PLACE_OP: u32 = 301;

    /// Reactivity
    pub const SYS_WATCH: u32 = 500;
    pub const SYS_WAIT_EVENT: u32 = 501;
}

/// Machine syscall operations
pub mod machine_op {
    pub const CONSOLE_WRITE: u64 = 0;
    pub const MMIO_MAP: u64 = 1;
}

/// Error codes
pub mod err {
    /// Operation not implemented
    pub const ENOSYS: i32 = -38;
    /// Invalid argument
    pub const EINVAL: i32 = -22;
    /// Bad address
    pub const EFAULT: i32 = -14;
    /// Out of memory
    pub const ENOMEM: i32 = -12;
}

/// Syscall result type
pub use abi::wire::SyscallResult;

/// Syscall dispatch function type
pub type SyscallDispatch = extern "C" fn(u32, u64, u64, u64, u64, u64, u64) -> SyscallResult;

/// Initialize syscall dispatch
pub fn init() {
    // Nothing to initialize yet - dispatch table is static
}

/// Main syscall dispatch function
///
/// Called by the architecture layer when a syscall trap occurs.
/// Returns (status, value0, value1).
#[unsafe(no_mangle)]
pub extern "C" fn dispatch(nr: u32, a0: u64, a1: u64, a2: u64, a3: u64, _a4: u64, _a5: u64) -> SyscallResult {
    // Debug for Sprout bringup
    if nr == abi::syscall::SYSCALL_LOG as u32 || nr == 1 {
        log::klog(Level::Info, "SYSCALL", "log_emit called");
    }

    match nr as u64 {
        abi::syscall::SYSCALL_LOG => sys_log_emit(a0, a1, a2),
        abi::syscall::SYSCALL_HEAP_GROW => sys_heap_grow(a0),
        abi::syscall::SYSCALL_BYTESPACE_CREATE => SyscallResult::new(err::ENOSYS, 0, 0), // TODO
        abi::syscall::SYSCALL_SPACE_MAP => SyscallResult::new(err::ENOSYS, 0, 0), // TODO
        
        // Legacy/Internal mappings (keep for internal components if any)
        0 => sys_version_get(),
        1 => sys_log_emit(a0, a1, a2), // Alias 1 to LOG for compatibility if needed
        2 => sys_symbol_intern(a0, a1),
        3 => sys_machine(a0, a1, a2, a3),
        10 => sys_place_op(10, a0, a1, a2), // THING_CREATE
        100 => sys_proc_spawn(a0, a1),
        101 => sys_proc_exit(a0),
        120 => sys_heap_grow(a0),
        200 => sys_sched_yield(),
        300 => sys_get_root_place(),
        500 => sys_watch(a0, a1),
        501 => sys_wait_event(a0),
        
        // Broad catch-all for legacy place_op mapping?
        // For now, explicit mapping is safer.
        
        _ => SyscallResult::new(err::ENOSYS, 0, 0),
    }
}

// ... logic ...

/// SYS_HEAP_GROW: Grow the heap
/// a0: increment (bytes)
fn sys_heap_grow(increment: u64) -> SyscallResult {
    use crate::memory::bytespace::Bytespace;
    use crate::memory::map::MapPerms;

    crate::sched::with_current_task(|task| {
        let old_brk = task.heap_brk;
        crate::log::klog(crate::log::Level::Info, "SYSCALL", &alloc::format!("sys_heap_grow: increment={} old_brk={:x}", increment, old_brk));

        if increment == 0 {
             return SyscallResult::new(0, old_brk, 0);
        }
        
        let _new_brk = old_brk + increment;
        
        // If we exceed hard limit? (e.g. 1GB?)
        // For now, no hard limit check other than address space arithmetic.
        
        // We need to map new memory backing this growth.
        // We assume contiguous virtual growth.
        // We check if (old_brk .. new_brk) is mapped.
        // Currently we map lazily or eagerly?
        // Task 06 says: "kernel creates bytespace... maps it".
        
        // Calculate required bytespace size
        // We allocate EXACTLY what's requested + padding?
        // Or Page Aligned?
        // User (thing_std) requests aligned chunk.
        
        // We need page alignment for mapping.
        // old_brk must be page aligned? NO. Heap structure doesn't require page align.
        // BUT mapping require page align.
        // If old_brk is 0x...120. We can't map at 0x...120.
        // We must map at next Page Boundary.
        
        // Strategy:
        // Check if `new_brk` is covered by existing mappings.
        // Mapped Top = task.heap_brk (if we consider brk = mapped top).
        // If `increment` implies growing execution-accessible memory.
        
        // wait. `heap_brk` usually means "end of valid heap".
        // Backing memory usually grows in Pages.
        // Does `heap_brk` track mapped limit or used limit?
        // Usually `brk` is "end of data segment".
        // `thing_std` allocator calls `heap_grow` when it needs more space.
        // It expects valid memory up to returned address + increment.
        
        // Simple strategy:
        // Always allocate a NEW Bytespace for the increment.
        // Map it at `old_brk`?
        // MMU requires Page Aligned Virtual Address.
        // If `old_brk` is NOT page aligned, we have a problem.
        // We can't map at `...120`.
        
        // Solution:
        // Round `old_brk` UP to next page?
        // But then we have a hole.
        // `thing_std` allocator expects contiguous.
        
        // Correct Solution:
        // `heap_brk` should track the VIRTUAL END.
        // But mappings are GRANULAR (4KB).
        // We should track `mapped_top`.
        // If `new_brk > mapped_top`, we allocate (new_brk - mapped_top) rounded up to page.
        // Map it at `mapped_top`.
        // Update `mapped_top`.
        
        // `Task` struct needs `heap_mapped_limit`.
        // Currently it only has `heap_brk`?
        // Let's assume `heap_brk` IS the mapped limit (aligned).
        // If `thing_std` calls `heap_grow(100)`, we move `heap_brk` by 4096 (min).
        // And return old_brk.
        // But `thing_std` manages sub-page allocation.
        // So Kernel `sys_heap_grow` is essentially `sbrk` but granular.
        
        // Let's round up increment to 4KB.
        let page_size = 4096;
        let alloc_size = (increment + page_size - 1) & !(page_size - 1);
        
        let bs = Bytespace::new_ram(alloc_size as usize);
        let map_addr = old_brk; // Assume old_brk is page aligned (enforced by boot.rs and previous grows)
        
        // Map it
        // task.address_space is Arc<AddressSpace>. &Arc coerces to &AddressSpace.
        // MapPerms::USER is CRITICAL
        task.address_space.map_bytespace_shared(map_addr, &bs, 0, alloc_size as usize, MapPerms::READ | MapPerms::WRITE | MapPerms::USER).unwrap();
        
        task.heap_brk = map_addr + alloc_size;
        
        SyscallResult::new(0, map_addr, 0)
    }).unwrap_or(SyscallResult::new(err::EFAULT, 0, 0))
}

/// SYS_VERSION_GET: Get kernel version
///
/// Returns: (0, major, minor)
fn sys_version_get() -> SyscallResult {
    const MAJOR: u64 = 0;
    const MINOR: u64 = 3;
    SyscallResult::new(0, MAJOR, MINOR)
}

/// SYS_LOG_EMIT: Emit a log message
///
/// a0: level (0=trace, 1=debug, 2=info, 3=warn, 4=error)
/// a1: pointer to message
/// a2: message length
///
/// Returns: (0, thing_id_high, thing_id_low) or (error, 0, 0)
fn sys_log_emit(level_raw: u64, msg_ptr: u64, msg_len: u64) -> SyscallResult {
    // Validate level
    let level = match level_raw {
        0 => Level::Trace,
        1 => Level::Debug,
        2 => Level::Info,
        3 => Level::Warn,
        4 => Level::Error,
        _ => return SyscallResult::new(err::EINVAL, 0, 0),
    };

    // Safety: In Ring 0 demo, we assume msg_ptr/len are valid kernel memory.
    // In a real implementation with userland, we MUST validate and/or copy this.
    let msg = unsafe {
        core::slice::from_raw_parts(msg_ptr as *const u8, msg_len as usize)
    };

    let subsystem = symbols::intern(b"userland");

    if let Some(id) = log::log_emit(level, subsystem, msg) {
        SyscallResult::new(0, id.high(), id.low())
    } else {
        SyscallResult::new(err::EFAULT, 0, 0)
    }
}

/// SYS_SYMBOL_INTERN: Intern a symbol string
///
/// a0: pointer to string
/// a1: string length
///
/// Returns: (0, symbol_id, 0) or (error, 0, 0)
fn sys_symbol_intern(str_ptr: u64, str_len: u64) -> SyscallResult {
    // Safety: In Ring 0 demo, we assume str_ptr/len are valid kernel memory.
    let name = unsafe {
        core::slice::from_raw_parts(str_ptr as *const u8, str_len as usize)
    };
    let id = symbols::intern(name);
    SyscallResult::new(0, id.0, 0)
}

/// SYS_MACHINE: Machine operations
///
/// a0: op (0=console_write, 1=mmio_map)
/// a1..a3: op-specific arguments
fn sys_machine(op: u64, a1: u64, a2: u64, a3: u64) -> SyscallResult {
    match op {
        machine_op::CONSOLE_WRITE => sys_machine_console_write(a1, a2),
        machine_op::MMIO_MAP => sys_machine_mmio_map(a1, a2, a3),
        _ => SyscallResult::new(err::EINVAL, 0, 0),
    }
}

/// SYS_MACHINE[console_write]: write bytes to the machine console
fn sys_machine_console_write(ptr: u64, len: u64) -> SyscallResult {
    if ptr == 0 {
        return SyscallResult::new(err::EFAULT, 0, 0);
    }

    // For now, assume kernel/user share address space for early logging.
    let bytes = unsafe { slice::from_raw_parts(ptr as *const u8, len as usize) };
    let written = machine::machine().console_write(bytes) as u64;
    SyscallResult::new(0, written, 0)
}

/// SYS_MACHINE[mmio_map]: map a physical MMIO range
fn sys_machine_mmio_map(phys: u64, len: u64, flags_raw: u64) -> SyscallResult {
    if len == 0 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    let flags = match MmioFlags::from_bits(flags_raw as u32) {
        Some(f) => f,
        None => return SyscallResult::new(err::EINVAL, 0, 0),
    };

    let range = MmioRange {
        phys,
        len: len as usize,
    };

    if let Some(mapping) = machine::machine().mmio_map(range, flags) {
        SyscallResult::new(0, mapping.virt, mapping.len as u64)
    } else {
        SyscallResult::new(err::EFAULT, 0, 0)
    }
}


/// SYS_GET_ROOT_PLACE: Get the ID of the root place
fn sys_get_root_place() -> SyscallResult {
    // For now, place.root is hardcoded to ID 2 (first thing created after symbols)
    let root_id = ThingId(2);
    SyscallResult::new(0, root_id.high(), root_id.low())
}

/// SYS_PLACE_OP: Perform an ontology operation
fn sys_place_op(op: u64, a1: u64, a2: u64, a3: u64) -> SyscallResult {
    match op {
        // OP_THING_CREATE: a1=kind_low
        10 => {
            // Ignored schema(a2) and version(a3) for now
            let id = store::thing_create(abi::ids::SymbolId(a1));
            SyscallResult::new(0, id.high(), id.low())
        }

        // OP_THING_CREATE_NAMED: a1=name_low, a2=kind_low
        11 => {
             // Ignored schema(a3) for now
            let id = store::thing_create(abi::ids::SymbolId(a2));
            store::thing_register_name(id, abi::ids::SymbolId(a1));
            SyscallResult::new(0, id.high(), id.low())
        }

        // OP_THING_SET_PAYLOAD: a1=id_low, a2=ptr, a3=len
        12 => {
            // Safety: assume valid kernel/user shared memory for now
            let payload = unsafe { core::slice::from_raw_parts(a2 as *const u8, a3 as usize) };
            if store::thing_set_inline_payload(ThingId(a1 as u128), payload) {
                SyscallResult::new(0, 0, 0)
            } else {
                SyscallResult::new(err::EINVAL, 0, 0)
            }
        }

        // OP_THING_GET_PAYLOAD: a1=id_low, a2=ptr, a3=len
        13 => {
            if let Some(payload) = store::get_payload(ThingId(a1 as u128)) {
                let len = core::cmp::min(payload.len(), a3 as usize);
                let target = unsafe { core::slice::from_raw_parts_mut(a2 as *mut u8, len) };
                target.copy_from_slice(&payload[..len]);
                SyscallResult::new(0, payload.len() as u64, 0)
            } else {
                SyscallResult::new(err::EINVAL, 0, 0)
            }
        }
        
        // OP_REL_CREATE: a1=from_low, a2=to_low, a3=pred_low
        20 => {
            let pred_provides = symbols::intern(b"predicate.provides");
            let sym_desktop = symbols::intern(b"place.desktop");
            let desktop_id = store::find_thing_by_name(sym_desktop).unwrap_or(ThingId(0));

            // Note: thing_std currently only passes low 64 bits of ThingId
            // Args: kind (predicate), from, to
            let id = store::relationship_create(abi::ids::SymbolId(a3), ThingId(a1 as u128), ThingId(a2 as u128));
            
            if abi::ids::SymbolId(a3) == pred_provides && ThingId(a2 as u128) == desktop_id {
                log::klog(Level::Info, "KERNEL", &format!("desktop provider: {:?}", ThingId(a1 as u128)));
            }

            SyscallResult::new(0, id.high(), id.low())
        }
        
        // OP_CONTAINED_IN: a1=place_low
        30 => {
            let contained = crate::place::contained_in(ThingId(a1 as u128));
            SyscallResult::new(0, contained.len() as u64, 0)
        }

        // OP_THING_FIND_BY_NAME: a1=name_low
        40 => {
            if let Some(id) = store::find_thing_by_name(abi::ids::SymbolId(a1)) {
                SyscallResult::new(0, id.high(), id.low())
            } else {
                SyscallResult::new(err::EINVAL, 0, 0)
            }
        }

        // OP_REL_GET_OUTGOING: a1=id_low, a2=buf_ptr, a3=buf_len
        50 => {
            let id = ThingId(a1 as u128);
            let rels = store::relationships_from(id);
            
            let buf_len = a3 as usize;
            if buf_len == 0 {
                 return SyscallResult::new(0, rels.len() as u64, 0);
            }

            // Safety: user buffer
            let buffer = unsafe { core::slice::from_raw_parts_mut(a2 as *mut u8, buf_len) };
            
            let mut count = 0;
            for (i, rel_id) in rels.iter().enumerate() {
                let start = i * 16;
                if start + 16 > buffer.len() {
                    break;
                }
                buffer[start..start+16].copy_from_slice(&rel_id.0.to_le_bytes());
                count += 1;
            }
            
            SyscallResult::new(0, count as u64, 0)
        }

        _ => SyscallResult::new(err::ENOSYS, 0, 0),
    }
}

/// SYS_PROC_SPAWN: Spawn a userland process by name
///
/// a0: name pointer
/// a1: name length
fn sys_proc_spawn(name_ptr: u64, name_len: u64) -> SyscallResult {
    if name_ptr == 0 || name_len == 0 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    // Safety: assume valid kernel/user shared memory for now
    let name = unsafe {
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(name_ptr as *const u8, name_len as usize))
    };

    let ctx = crate::boot::get_boot_ctx();
    crate::boot::spawn_module_by_name(ctx, name);

    // If spawn_module returns, it means it didn't jump (e.g. error or multitasking supported)
    // But currently it jumps and never returns. 
    // In a multitasking system, this would return the new process ID.
    SyscallResult::new(0, 0, 0)
}

/// SYS_PROC_EXIT: Exit the current process
/// a0: exit code
fn sys_proc_exit(code: u64) -> SyscallResult {
    crate::sched::exit_current_task(code as i32);
    // Should not return
}

/// SYS_SCHED_YIELD: Yield execution
fn sys_sched_yield() -> SyscallResult {
    crate::sched::yield_current();
    SyscallResult::new(0, 0, 0)
}

/// SYS_WATCH: Register a watcher
/// a0: watcher_low (ThingId)
/// a1: target_low (ThingId)
fn sys_watch(watcher_low: u64, target_low: u64) -> SyscallResult {
    let watcher = ThingId(watcher_low as u128);
    let target = ThingId(target_low as u128);
    
    // 1. Subscribe in PlaceStore (memory)
    store::watch(watcher, target);

    // 2. Create Relationship (Graph Truth)
    // This mutation will trigger an emit_event for the target,
    // which the watcher (just subscribed) should receive.
    let pred_watches = symbols::intern(b"predicate.watches");
    store::relationship_create(pred_watches, watcher, target);

    SyscallResult::new(0, 0, 0)
}

/// SYS_WAIT_EVENT: Wait for next event
/// a0: watcher_low (ThingId)
fn sys_wait_event(watcher_low: u64) -> SyscallResult {
    let watcher = ThingId(watcher_low as u128);
    
    // Block until event is available
    loop {
        if let Some(event_id) = store::dequeue_event(watcher) {
            return SyscallResult::new(0, event_id.high(), event_id.low());
        }
        // Yield to other tasks
        crate::sched::yield_current();
    }
}

