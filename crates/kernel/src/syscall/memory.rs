//! Memory Syscalls

use crate::memory::bytespace::Bytespace;
use crate::memory::journal;
use crate::memory::map::MapPerms;
use crate::syscall::user_mem;
use abi::cap::CapOp;
use abi::syscall::err;
use abi::wire::SyscallResult;

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
pub fn sys_heap_grow(increment: u64) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::MemManage, None) {
        return SyscallResult::new(code, 0, 0);
    }

    crate::sched::with_current_task(|task| {
        let old_brk = task.heap_brk;
        if increment == 0 {
            return SyscallResult::new(0, old_brk, 0);
        }

        let page_size = 4096u64;
        let alloc_size = (increment + page_size - 1) & !(page_size - 1);

        // Direct allocation - bypass Bytespace to avoid graph store deadlock
        let layout = match core::alloc::Layout::from_size_align(alloc_size as usize, 4096) {
            Ok(l) => l,
            Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
        };

        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return SyscallResult::new(err::ENOMEM, 0, 0);
        }

        // Get physical address for mapping
        let phys = crate::machine::machine().virt_to_phys(ptr as u64);

        // Direct map without graph registration
        let map_addr = old_brk;
        if let Err(_) = task.address_space.map(
            map_addr,
            phys,
            alloc_size as usize,
            MapPerms::READ | MapPerms::WRITE | MapPerms::USER,
        ) {
            // Free the allocation on failure
            unsafe { alloc::alloc::dealloc(ptr, layout) };
            return SyscallResult::new(err::ENOMEM, 0, 0);
        }

        task.heap_brk = map_addr + alloc_size;
        journal::emit_heap_grow(task.id.0, old_brk, task.heap_brk, alloc_size);
        SyscallResult::new(0, map_addr, 0)
    })
    .unwrap_or(SyscallResult::new(err::EFAULT, 0, 0))
}
