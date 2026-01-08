//! Memory Syscalls

use crate::memory::bytespace::Bytespace;
use crate::memory::journal;
use crate::memory::map::MapPerms;
use crate::syscall::user_mem;
use abi::cap::CapOp;
use abi::syscall::err;
use abi::wire::SyscallResult;

fn align_up(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    (value + align - 1) & !(align - 1)
}

pub fn sys_bytespace_create(size: u64, _flags: u64) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::MemManage, None) {
        return SyscallResult::new(code, 0, 0);
    }

    if let Ok(bs) = Bytespace::new_ram(size as usize) {
        let id = bs.id;
        core::mem::forget(bs);
        SyscallResult::new(0, id.high(), id.low())
    } else {
        SyscallResult::new(err::ENOMEM, 0, 0)
    }
}

/// Create a DMA-safe bytespace with physically contiguous memory.
/// Returns (0, thing_id_low, phys_base) on success.
/// Note: We pack the ID low bits and phys into the result since we only have 3 values.
/// Userspace can use thing_id_low (val0) to reference the bytespace.
pub fn sys_dma_bytespace_create(size: u64, _flags: u64) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::MemManage, None) {
        return SyscallResult::new(code, 0, 0);
    }

    if size == 0 || size % 4096 != 0 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    match Bytespace::new_dma(size as usize) {
        Ok((bs, phys_base)) => {
            let id = bs.id;
            core::mem::forget(bs);
            // Return: status=0, val0=id_low (for mapping), val1=phys_base (for DMA)
            SyscallResult::new(0, id.low(), phys_base)
        }
        Err(_) => SyscallResult::new(err::ENOMEM, 0, 0),
    }
}

pub fn sys_space_map(
    bs_id_lo: u64,
    bs_id_hi: u64,
    vaddr: u64,
    offset: u64,
    len: u64,
) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::MemManage, None) {
        return SyscallResult::new(code, 0, 0);
    }

    let bs_id = abi::ids::ThingId::from_parts(bs_id_hi, bs_id_lo);

    let info = if let Some(info) = Bytespace::lookup(bs_id) {
        info
    } else {
        crate::log::klog(
            crate::log::Level::Error,
            "SYSCALL",
            "sys_space_map: bytespace not found",
        );
        return SyscallResult::new(err::EINVAL, 0, 0);
    };

    let phys = if let Some(phys) = info.phys_base {
        phys
    } else {
        crate::log::klog(
            crate::log::Level::Error,
            "SYSCALL",
            "sys_space_map: bytespace missing phys_base",
        );
        return SyscallResult::new(err::EINVAL, 0, 0);
    };

    let size = if info.size == 0 { len } else { info.size as u64 };

    let bs = Bytespace::new_device(phys, size as usize);

    crate::sched::with_current_task(|task| {
        if let Err(e) = task.address_space.map_bytespace_shared(
            vaddr,
            &bs,
            offset,
            len as usize,
            MapPerms::READ | MapPerms::WRITE | MapPerms::USER,
        ) {
            crate::log::klog(
                crate::log::Level::Error,
                "SYSCALL",
                &alloc::format!("sys_space_map: map failed: {:?}", e),
            );
            return SyscallResult::new(err::EFAULT, 0, 0);
        }
        SyscallResult::new(0, vaddr, 0)
    })
    .unwrap_or(SyscallResult::new(err::EFAULT, 0, 0))
}


pub fn sys_space_unmap(vaddr: u64, len: u64, _flags: u64) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::MemManage, None) {
        return SyscallResult::new(code, 0, 0);
    }

    let len = match usize::try_from(len) {
        Ok(len) if len > 0 => len,
        _ => return SyscallResult::new(err::EINVAL, 0, 0),
    };

    crate::sched::with_current_task(|task| {
        let user_end = task.address_space.user_range_end();
        let end = match vaddr.checked_add(len as u64) {
            Some(end) => end,
            None => return SyscallResult::new(err::EINVAL, 0, 0),
        };

        if vaddr == 0 || vaddr >= user_end || end > user_end {
            return SyscallResult::new(err::EFAULT, 0, 0);
        }

        if let Err(e) = task.address_space.unmap(vaddr, len) {
            crate::log::klog(
                crate::log::Level::Error,
                "SYSCALL",
                &alloc::format!("sys_space_unmap: unmap failed: {:?}", e),
            );
            return SyscallResult::new(err::EFAULT, 0, 0);
        }

        SyscallResult::new(0, vaddr, 0)
    })
    .unwrap_or(SyscallResult::new(err::EFAULT, 0, 0))
}

/// Heap growth syscall - journal-only path.
///
/// This function intentionally avoids graph writes and uses the memory journal
/// to record changes. Heap growth must remain safe even when the graph is busy.
/// The heap break is process-scoped; all threads in the same address space share
/// this state.
pub fn sys_heap_grow(increment: u64) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::MemManage, None) {
        return SyscallResult::new(code, 0, 0);
    }

    crate::sched::with_current_task(|task| {
        let address_space = task.address_space.clone();
        let task_id = task.id.0;
        let page_size = 4096u64;

        address_space.with_heap_state(|heap| {
            let old_brk = heap.brk;
            if increment == 0 {
                return SyscallResult::new(0, old_brk, 0);
            }

            let Some(new_brk) = old_brk.checked_add(increment) else {
                return SyscallResult::new(err::ENOMEM, 0, 0);
            };

            if new_brk < heap.base {
                return SyscallResult::new(err::EINVAL, 0, 0);
            }

            if let Some(limit) = heap.limit {
                if new_brk > limit {
                    return SyscallResult::new(err::ENOMEM, 0, 0);
                }
            }

            let map_from = align_up(old_brk, page_size);
            let map_to = align_up(new_brk, page_size);
            let map_len = map_to.saturating_sub(map_from);

            if map_len == 0 {
                heap.brk = new_brk;
                return SyscallResult::new(0, old_brk, 0);
            }

            let map_len_usize = match usize::try_from(map_len) {
                Ok(len) if len > 0 => len,
                _ => return SyscallResult::new(err::ENOMEM, 0, 0),
            };

            let layout = match core::alloc::Layout::from_size_align(map_len_usize, page_size as usize) {
                Ok(l) => l,
                Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
            };

            let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
            if ptr.is_null() {
                return SyscallResult::new(err::ENOMEM, 0, 0);
            }

            let phys = crate::machine::machine().virt_to_phys(ptr as u64);
            if let Err(_) = address_space.map(
                map_from,
                phys,
                map_len_usize,
                MapPerms::READ | MapPerms::WRITE | MapPerms::USER,
            ) {
                unsafe { alloc::alloc::dealloc(ptr, layout) };
                return SyscallResult::new(err::ENOMEM, 0, 0);
            }

            heap.brk = new_brk;
            journal::emit_heap_grow(task_id, old_brk, new_brk, map_len);
            SyscallResult::new(0, old_brk, 0)
        })
    })
    .unwrap_or(SyscallResult::new(err::EFAULT, 0, 0))
}
