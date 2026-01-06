//! Memory Syscalls

use crate::memory::bytespace::Bytespace;
use crate::memory::map::MapPerms;
use abi::syscall::err;
use abi::wire::SyscallResult;

pub fn sys_bytespace_create(size: u64, _flags: u64) -> SyscallResult {
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
    if size == 0 || size % 4096 != 0 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    match Bytespace::new_dma(size as usize) {
        Ok((bs, phys_base)) => {
            let id = bs.id;
            // Store phys_base in bytespace's own payload for later retrieval
            graph::store::with_store(|s| {
                let _ = s.set_payload(id, &phys_base.to_le_bytes());
            });
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
    use graph::store;
    use graph::symbols::sym;

    let bs_id = abi::ids::ThingId::from_parts(bs_id_hi, bs_id_lo);

    let extract_phys = |payload: &[u8]| -> Option<u64> {
        if payload.len() < 8 {
            return None;
        }

        // Check for ThingEnvelopeV1
        if payload.len() >= core::mem::size_of::<abi::bodies::ThingEnvelopeV1>() {
             let magic = u32::from_le_bytes(payload[0..4].try_into().unwrap());
             if magic == abi::bodies::ThingEnvelopeV1::MAGIC {
                 let header_size = core::mem::size_of::<abi::bodies::ThingEnvelopeV1>();
                 if payload.len() >= header_size + 8 {
                      return Some(u64::from_le_bytes(payload[header_size..header_size+8].try_into().unwrap()));
                 }
                 return None;
             }
        }

        // Fallback: raw payload
        Some(u64::from_le_bytes(payload[0..8].try_into().unwrap()))
    };

    let read_prop = |pred: abi::ids::SymbolId| -> Option<u64> {
        let rels = store::relationships_from(bs_id);
        for r_id in rels {
            if let Some(r) = store::get_relationship(r_id) {
                if r.kind == pred {
                    if let Some(payload) = store::get_payload(r.to) {
                        if let Some(val) = extract_phys(&payload) {
                            return Some(val);
                        }
                    }
                }
            }
        }
        None
    };

    let phys = if let Some(p) = read_prop(sym::PRED_BASE_PHYS) {
        p
    } else {
        // Try reading from payload (for DMA bytespaces)
        if let Some(payload) = store::get_payload(bs_id) {
            if let Some(val) = extract_phys(&payload) {
                val
            } else {
                return SyscallResult::new(err::EINVAL, 0, 0);
            }
        } else {
            return SyscallResult::new(err::EINVAL, 0, 0);
        }
    };

    let size = if let Some(s) = read_prop(sym::PRED_SIZE) {
        s
    } else {
        // Default to len if size not found
        len
    };

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


pub fn sys_space_unmap(_vaddr: u64, _len: u64, _flags: u64) -> SyscallResult {
    SyscallResult::new(err::ENOSYS, 0, 0)
}

/// Heap growth syscall - GRAPH-FREE to avoid deadlock.
/// 
/// This function intentionally bypasses Bytespace::new_ram and the graph store
/// to avoid a deadlock when heap allocation is triggered inside a with_store closure.
/// The deadlock chain was:
///   with_store -> allocation -> heap grow -> Bytespace::new_ram -> with_store (deadlock!)
pub fn sys_heap_grow(increment: u64) -> SyscallResult {
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
        SyscallResult::new(0, map_addr, 0)
    })
    .unwrap_or(SyscallResult::new(err::EFAULT, 0, 0))
}
