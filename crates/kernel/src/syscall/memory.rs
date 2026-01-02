//! Memory Syscalls

use abi::wire::SyscallResult;
use abi::syscall::err;
use crate::memory::bytespace::Bytespace;
use crate::memory::map::MapPerms;

pub fn sys_bytespace_create(size: u64, _flags: u64) -> SyscallResult {
    // 1. Alloc (Leak Memory)
    if let Ok(bs) = Bytespace::new_ram(size as usize) {
        // Return ThingId
        let id = bs.id;
        core::mem::forget(bs); // Leak handle to keep memory alive
        SyscallResult::new(0, id.high(), id.low())
    } else {
        SyscallResult::new(err::ENOMEM, 0, 0)
    }
}

pub fn sys_space_map(bs_id_low: u64, vaddr: u64, _len: u64, _perms: u64) -> SyscallResult {
    use graph::store;
    use graph::symbols::sym;
    
    // 1. Resolve Bytespace from Graph
    let bs_id = abi::ids::ThingId(bs_id_low as u128); 
    
    // 2. Read Properties (Phys Base, Size)
    let read_prop = |pred: abi::ids::SymbolId| -> Option<u64> {
        let rels = store::relationships_from(bs_id);
        for r_id in rels {
            if let Some(r) = store::get_relationship(r_id) {
                if r.kind == pred {
                     if let Some(payload) = store::get_payload(r.to) {
                         if payload.len() >= 8 {
                             return Some(u64::from_le_bytes(payload[0..8].try_into().unwrap()));
                         }
                     }
                }
            }
        }
        None
    };

    let phys = if let Some(p) = read_prop(sym::PRED_BASE_PHYS) { p } else { return SyscallResult::new(err::EINVAL, 0, 0) };
    let size = if let Some(s) = read_prop(sym::PRED_SIZE) { s } else { return SyscallResult::new(err::EINVAL, 0, 0) };
    
    // 3. Create wrapper Bytespace
    let bs = Bytespace::new_device(phys, size as usize);
    
    // 4. Map it
    crate::sched::with_current_task(|task| {
         // Round up size? Logic in Bytespace.map?
         // map_bytespace_shared handles it?
         if let Err(_) = task.address_space.map_bytespace_shared(vaddr, &bs, 0, size as usize, MapPerms::READ | MapPerms::WRITE | MapPerms::USER) {
             return SyscallResult::new(err::EFAULT, 0, 0); 
         }
         SyscallResult::new(0, vaddr, 0)
    }).unwrap_or(SyscallResult::new(err::EFAULT, 0, 0))
}

pub fn sys_space_unmap(_vaddr: u64, _len: u64) -> SyscallResult {
     SyscallResult::new(err::ENOSYS, 0, 0)
}

// Keeping Heap Grow for compatibility/smoke tests
pub fn sys_heap_grow(increment: u64) -> SyscallResult {
    crate::sched::with_current_task(|task| {
        let old_brk = task.heap_brk;
        if increment == 0 {
             return SyscallResult::new(0, old_brk, 0);
        }
        
        let page_size = 4096;
        let alloc_size = (increment + page_size - 1) & !(page_size - 1);
        
        match Bytespace::new_ram(alloc_size as usize) {
            Ok(bs) => {
                let map_addr = old_brk;
                if let Err(_) = task.address_space.map_bytespace_shared(map_addr, &bs, 0, alloc_size as usize, MapPerms::READ | MapPerms::WRITE | MapPerms::USER) {
                     return SyscallResult::new(err::ENOMEM, 0, 0);
                }
                task.heap_brk = map_addr + alloc_size;
                SyscallResult::new(0, map_addr, 0)
            },
            Err(_) => SyscallResult::new(err::ENOMEM, 0, 0)
        }
    }).unwrap_or(SyscallResult::new(err::EFAULT, 0, 0))
}
