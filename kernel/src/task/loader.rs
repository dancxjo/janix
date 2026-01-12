use crate::{BootRuntime, BootTasking, BootModuleDesc, UserEntry, MapPerms, MapKind, FrameAllocatorHook};
use crate::memory;

struct LoaderAllocHook;
impl FrameAllocatorHook for LoaderAllocHook {
    fn alloc_frame(&self) -> Option<u64> {
        memory::alloc_frame()
    }
}

pub fn load_module<R: BootRuntime>(
    rt: &R, 
    aspace: <R::Tasking as BootTasking>::AddressSpace, 
    module: &BootModuleDesc
) -> Option<UserEntry> {
    // crate::kinfo!("Loading module: {}", module.name);
    if module.bytes.len() >= 16 {
        // crate::kinfo!("  Header: {:02x?}", &module.bytes[0..16]);
    }

    // Hardcoded load address for simple PIE/or-not-PIE loading
    // For now we just load at a fixed address because we only run one process per address space?
    // Wait, threads share address space. Different processes have different address spaces.
    // So fixed address 0x200000 is fine for the main executable.
    
    let load_addr = 0x200000;
    let stack_top = 0x400000;
    let stack_size = 65536; // 64KB
    
    let hook = LoaderAllocHook;
    let text_perms = MapPerms { user: true, read: true, write: false, exec: true };
    let data_perms = MapPerms { user: true, read: true, write: true, exec: false };
    
    // 1. Map segments
    let mut virt = load_addr as u64;
    for chunk in module.bytes.chunks(4096) {
         let phys = memory::alloc_frame().expect("OOM loading module");
         // crate::kinfo!("  Chunk Phys: {:x}", phys);
         let hhdm_virt = phys + rt.phys_to_virt_offset();
         unsafe {
             core::ptr::copy_nonoverlapping(chunk.as_ptr(), hhdm_virt as *mut u8, chunk.len());
             if chunk.len() < 4096 {
                 core::ptr::write_bytes((hhdm_virt as *mut u8).add(chunk.len()), 0, 4096 - chunk.len());
             }
         }
         
         rt.tasking().map_page(aspace, virt, phys, text_perms, MapKind::Normal, &hook).unwrap();
         virt += 4096;
    }
    
    // 2. Map BSS (128 pages = 512KB)
    for _ in 0..128 {
         let phys = memory::alloc_frame().expect("OOM loading BSS");
         let hhdm_virt = phys + rt.phys_to_virt_offset();
         unsafe { core::ptr::write_bytes(hhdm_virt as *mut u8, 0, 4096); }
         
         rt.tasking().map_page(aspace, virt, phys, data_perms, MapKind::Normal, &hook).unwrap();
         virt += 4096;
    }
    
    // 3. Map Stack
    let stack_base = (stack_top - stack_size) as u64;
    let mut virt = stack_base;
    let stack_limit = stack_top as u64;
    
    while virt < stack_limit {
         let phys = memory::alloc_frame().expect("OOM loading stack");
         rt.tasking().map_page(aspace, virt, phys, data_perms, MapKind::Normal, &hook).unwrap();
         virt += 4096;
    }

    // Ensure instruction cache sees freshly loaded code
    rt.icache_invalidate();
    
    Some(UserEntry {
        entry_pc: load_addr,
        user_sp: stack_top,
        arg0: 0,
    })
}
