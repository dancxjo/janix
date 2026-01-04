//! Syscall Dispatch Router

use crate::syscall::cap::CapOp;
use crate::syscall::{cap, graph, log, memory, surface, time, wait, watch};
use abi::syscall::nr;
use abi::wire::SyscallResult;

/// Main syscall dispatch function
#[no_mangle]
pub extern "C" fn dispatch(
    nr: u32,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> SyscallResult {
    if nr == abi::syscall::nr::SYS_MACHINE {
        crate::log::klog(crate::log::Level::Info, "SYSCALL", "SYS_MACHINE called");
    }

    let result = match nr {
        // === Logging ===
        nr::SYS_LOG => {
            if let Err(e) = cap::check(CapOp::Log, None) {
                return e;
            }
            log::sys_log_emit(a0, a1, a2)
        }

        // === Scheduling ===
        nr::SYS_SCHED_YIELD => {
            crate::sched::yield_current();
            SyscallResult::new(0, 0, 0)
        }

        // === Graph Mutation ===
        nr::SYS_THING_CREATE => {
            let _kind_id = a0;
            let parent_id = abi::ids::ThingId(a1 as u128);
            if let Err(e) = cap::check(CapOp::GraphCreate, Some(parent_id)) {
                return e;
            }
            graph::sys_thing_create(a0, a1)
        }
        nr::SYS_REL_CREATE => {
            let from_id = abi::ids::ThingId(a1 as u128);
            if let Err(e) = cap::check(CapOp::GraphLink, Some(from_id)) {
                return e;
            }
            graph::sys_relationship_create(a0, a1, a2)
        }
        nr::SYS_REL_DELETE => {
            let rel_id = abi::ids::ThingId(a0 as u128);
            if let Some(rel) = ::graph::store::get_relationship(rel_id) {
                if let Err(e) = cap::check(CapOp::GraphUnlink, Some(rel.from)) {
                    return e;
                }
                graph::sys_relationship_delete(a0)
            } else {
                SyscallResult::new(abi::syscall::err::EINVAL, 0, 0)
            }
        }

        // === Graph Observation ===
        nr::SYS_THING_GET => {
            let thing_id = abi::ids::ThingId(a0 as u128);
            if let Err(e) = cap::check(CapOp::GraphRead, Some(thing_id)) {
                return e;
            }
            graph::sys_thing_get(a0, a1, a2)
        }
        nr::SYS_REL_GET_FROM => {
            let thing_id = abi::ids::ThingId(a0 as u128);
            if let Err(e) = cap::check(CapOp::GraphRead, Some(thing_id)) {
                return e;
            }
            graph::sys_relationships_from(a0, a1, a2, a3)
        }
        nr::SYS_SYMBOL_RESOLVE => graph::sys_symbol_resolve(a0, a1, a2),
        nr::SYS_SYMBOL_INTERN => graph::sys_symbol_intern(a0, a1),
        nr::SYS_THING_FIND => {
            if let Err(e) = cap::check(CapOp::GraphRead, None) {
                return e;
            }
            graph::sys_thing_find(a0, a1)
        }
        nr::SYS_THING_REGISTER_NAME => {
            let thing_id = abi::ids::ThingId(a0 as u128);
            if let Err(e) = cap::check(CapOp::GraphWrite, Some(thing_id)) {
                return e;
            }
            graph::sys_thing_register_name(a0, a1, a2)
        }

        // === Memory ===
        nr::SYS_BYTESPACE_CREATE => {
            if let Err(e) = cap::check(CapOp::MemManage, None) {
                return e;
            }
            memory::sys_bytespace_create(a0, a1)
        }
        nr::SYS_SPACE_MAP => {
            if let Err(e) = cap::check(CapOp::MemManage, None) {
                return e;
            }
            memory::sys_space_map(a0, a1, a2, a3)
        }
        nr::SYS_SPACE_UNMAP => {
            if let Err(e) = cap::check(CapOp::MemManage, None) {
                return e;
            }
            memory::sys_space_unmap(a0, a1)
        }
        nr::SYS_HEAP_GROW => {
            if let Err(e) = cap::check(CapOp::MemManage, None) {
                return e;
            }
            memory::sys_heap_grow(a0)
        }

        // === Watch ===
        nr::SYS_WATCH_CREATE => {
            let target = abi::ids::ThingId(a1 as u128);
            if let Err(e) = cap::check(CapOp::GraphWatch, Some(target)) {
                return e;
            }
            watch::sys_watch_create(a0, a1)
        }
        nr::SYS_WATCH_POLL => {
            let watcher = abi::ids::ThingId(a0 as u128);
            if let Err(e) = cap::check(CapOp::GraphRead, Some(watcher)) {
                return e;
            }
            watch::sys_watch_poll(a0, a1, a2)
        }

        // === Wait ===
        nr::SYS_WAIT => {
            if let Err(e) = cap::check(CapOp::GraphWatch, None) {
                return e;
            }
            wait::sys_wait(a0, a1, a2, a3)
        }

        // === Input ===
        nr::SYS_INPUT_READ => {
            let buf = a0 as *mut u8;
            let len = a1 as usize;
            match crate::syscall::input::sys_input_read(buf, len) {
                Ok(n) => SyscallResult::new(0, n as u64, 0),
                Err(_) => SyscallResult::new(abi::syscall::err::EFAULT, 0, 0),
            }
        }

        // === Display (Deprecated) ===
        nr::SYS_DISPLAY_PRIMARY => {
            // Deprecated in favor of Graph Discovery
            SyscallResult::new(abi::syscall::err::ENOSYS, 0, 0)
        }

        // === Graphics ===
        nr::SYS_SURFACE_CREATE => {
            if let Err(e) = cap::check(cap::CapOp::GraphCreate, None) {
                return e;
            }
            surface::sys_surface_create(a0, a1, a2)
        }
        nr::SYS_SURFACE_DRAW => {
            let surface_id = abi::ids::ThingId(((a1 as u128) << 64) | (a0 as u128));
            if let Err(e) = cap::check(cap::CapOp::GraphWrite, Some(surface_id)) {
                return e;
            }
            surface::sys_surface_draw(a0, a1, a2, a3, a4, a5)
        }

        // === Process ===
        nr::SYS_PROC_SPAWN => {
            if let Err(e) = cap::check(CapOp::MemManage, None) {
                return e;
            }
            crate::syscall::dispatch::mod_legacy_spawn(a0, a1)
        }
        nr::SYS_PROC_EXIT => {
            crate::sched::exit_current_task(a0 as i32);
        }

        // === Time ===
        nr::SYS_TIME_MONOTONIC_NOW => time::sys_time_monotonic_now(),
        nr::SYS_TIME_SYSTEM_NOW => time::sys_time_system_now(),
        nr::SYS_TIME_SET_SYSTEM => time::sys_time_set_system(a0 as i64),
        nr::SYS_SLEEP_UNTIL => time::sys_sleep_until(a0),

        // === Machine (Restricted) ===
        nr::SYS_MACHINE => {
            if let Err(e) = cap::check(CapOp::Hardware, None) {
                return e;
            }
            crate::syscall::dispatch::sys_machine(a0, a1, a2, a3)
        }

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
        core::str::from_utf8(core::slice::from_raw_parts(
            name_ptr as *const u8,
            name_len as usize,
        ))
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
    use crate::machine::{self, MmioFlags, MmioRange};
    use abi::syscall::err;

    // Define machine_op if not exists in abi
    const CONSOLE_WRITE: u64 = 0;
    const MMIO_MAP: u64 = 1;

    match op {
        CONSOLE_WRITE => {
            if a1 == 0 {
                return SyscallResult::new(err::EFAULT, 0, 0);
            }
            let bytes = unsafe { core::slice::from_raw_parts(a1 as *const u8, a2 as usize) };
            let written = machine::machine().console_write(bytes) as u64;
            SyscallResult::new(0, written, 0)
        }
        MMIO_MAP => {
            if a2 == 0 {
                return SyscallResult::new(err::EINVAL, 0, 0);
            }
            let flags = match MmioFlags::from_bits(a3 as u32) {
                Some(f) => f,
                None => return SyscallResult::new(err::EINVAL, 0, 0),
            };
            let range = MmioRange {
                phys: a1,
                len: a2 as usize,
            };
            if let Some(mapping) = machine::machine().mmio_map(range, flags) {
                SyscallResult::new(0, mapping.virt, mapping.len as u64)
            } else {
                SyscallResult::new(err::EFAULT, 0, 0)
            }
        }
        _ => SyscallResult::new(err::EINVAL, 0, 0),
    }
}
