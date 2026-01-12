use crate::memory;
use crate::{
    BootModuleDesc, BootRuntime, BootTasking, FrameAllocatorHook, MapKind, MapPerms, UserEntry,
};
use abi::types::StackInfo;

struct LoaderAllocHook;
impl FrameAllocatorHook for LoaderAllocHook {
    fn alloc_frame(&self) -> Option<u64> {
        memory::alloc_frame()
    }
}

pub fn load_module<R: BootRuntime>(
    rt: &R,
    aspace: <R::Tasking as BootTasking>::AddressSpace,
    module: &BootModuleDesc,
) -> Option<(UserEntry, StackInfo)> {
    // crate::kinfo!("Loading module: {}", module.name);
    if module.bytes.len() >= 16 {
        // crate::kinfo!("  Header: {:02x?}", &module.bytes[0..16]);
    }

    // Hardcoded load address for simple PIE/or-not-PIE loading
    // For now we just load at a fixed address because we only run one process per address space?
    // Wait, threads share address space. Different processes have different address spaces.
    // So fixed address 0x200000 is fine for the main executable.

    let load_addr = 0x200000;
    let stack_top = 0x0080_0000;
    let reserve_bytes = 2 * 1024 * 1024;
    let guard_pages = 1usize;
    let initial_commit_bytes = 64 * 1024;
    let grow_chunk_bytes = 64 * 1024;

    let hook = LoaderAllocHook;
    let text_perms = MapPerms {
        user: true,
        read: true,
        write: false,
        exec: true,
    };
    let data_perms = MapPerms {
        user: true,
        read: true,
        write: true,
        exec: false,
    };

    // 1. Map segments
    let mut virt = load_addr as u64;
    for chunk in module.bytes.chunks(4096) {
        let phys = memory::alloc_frame().expect("OOM loading module");
        // crate::kinfo!("  Chunk Phys: {:x}", phys);
        let hhdm_virt = phys + rt.phys_to_virt_offset();
        unsafe {
            core::ptr::copy_nonoverlapping(chunk.as_ptr(), hhdm_virt as *mut u8, chunk.len());
            if chunk.len() < 4096 {
                core::ptr::write_bytes(
                    (hhdm_virt as *mut u8).add(chunk.len()),
                    0,
                    4096 - chunk.len(),
                );
            }
        }

        rt.tasking()
            .map_page(aspace, virt, phys, text_perms, MapKind::Normal, &hook)
            .unwrap();
        virt += 4096;
    }

    // 2. Map BSS (128 pages = 512KB)
    for _ in 0..128 {
        let phys = memory::alloc_frame().expect("OOM loading BSS");
        let hhdm_virt = phys + rt.phys_to_virt_offset();
        unsafe {
            core::ptr::write_bytes(hhdm_virt as *mut u8, 0, 4096);
        }

        rt.tasking()
            .map_page(aspace, virt, phys, data_perms, MapKind::Normal, &hook)
            .unwrap();
        virt += 4096;
    }

    // 3. Map Stack (guard + reserve with initial commit)
    let page_size = rt.page_size() as u64;
    let guard_bytes = (guard_pages as u64).saturating_mul(page_size);
    let reserve_bytes = align_up_u64(reserve_bytes as u64, page_size);
    let total = guard_bytes.saturating_add(reserve_bytes);
    let reserve_end = stack_top as u64;
    let base = reserve_end.saturating_sub(total);
    let guard_start = base;
    let guard_end = base.saturating_add(guard_bytes);
    let reserve_start = guard_end;
    let commit_len = align_up_u64(initial_commit_bytes as u64, page_size);
    let commit_start = reserve_end.saturating_sub(commit_len);

    let mut virt = commit_start;
    while virt < reserve_end {
        let phys = memory::alloc_frame().expect("OOM loading stack");
        let hhdm_virt = phys + rt.phys_to_virt_offset();
        unsafe {
            core::ptr::write_bytes(hhdm_virt as *mut u8, 0, page_size as usize);
        }
        rt.tasking()
            .map_page(aspace, virt, phys, data_perms, MapKind::Normal, &hook)
            .unwrap();
        virt += page_size;
    }

    // Ensure instruction cache sees freshly loaded code
    rt.icache_invalidate();

    let stack_info = StackInfo {
        guard_start: guard_start as usize,
        guard_end: guard_end as usize,
        reserve_start: reserve_start as usize,
        reserve_end: reserve_end as usize,
        committed_start: commit_start as usize,
        grow_chunk_bytes,
    };

    Some((
        UserEntry {
            entry_pc: load_addr,
            user_sp: stack_top,
            arg0: 0,
        },
        stack_info,
    ))
}

fn align_up_u64(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    (value + align - 1) & !(align - 1)
}
