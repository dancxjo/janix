//! Memory Syscalls

use abi::wire::SyscallResult;
use abi::syscall::err;
use crate::memory::bytespace::Bytespace;
use crate::memory::map::MapPerms;

pub fn sys_bytespace_create(size: u64, _flags: u64) -> SyscallResult {
    // 1. Alloc
    let _bs = Bytespace::new_ram(size as usize);
    // 2. Return Handle (ThingId)
    // Wait, Bytespace is a Rust Arc<Hz>, not a Graph Thing yet directly?
    // Task 07: "sys_bytespace_create... -> ThingId(bytespace)"
    // We need to wrap it in a Thing!
    
    // For now, in v0.3 migration, we might just return Handle ID?
    // But "Goal: Syscalls as Graph Mutations".
    
    // Implementation:
    // Create Thing of kind `kind.bytespace`
    // Associate internal kernel object (Bytespace) with it?
    // We don't have an "Object Store" mapping ThingId -> KernelObj yet.
    // We have Inline Payload.
    
    // Workaround:
    // Use `sys_heap_grow` style for now (implicit mapping)
    // Or return ENOSYS until we have proper resource handles.
    
    SyscallResult::new(err::ENOSYS, 0, 0)
}

pub fn sys_space_map(_bs_id: u64, _vaddr: u64, _len: u64, _perms: u64) -> SyscallResult {
     SyscallResult::new(err::ENOSYS, 0, 0)
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
        
        let bs = Bytespace::new_ram(alloc_size as usize);
        let map_addr = old_brk;
        
        // Map
        if let Err(_) = task.address_space.map_bytespace_shared(map_addr, &bs, 0, alloc_size as usize, MapPerms::READ | MapPerms::WRITE | MapPerms::USER) {
             return SyscallResult::new(err::ENOMEM, 0, 0);
        }
        
        task.heap_brk = map_addr + alloc_size;
        SyscallResult::new(0, map_addr, 0)
    }).unwrap_or(SyscallResult::new(err::EFAULT, 0, 0))
}
