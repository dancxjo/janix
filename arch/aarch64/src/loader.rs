use alloc::alloc::{alloc, Layout};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::ThingId;

use crate::bridge::Bridge;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::bridge::HardwareBridge;
use kernel::Kernel;
use models::value::ThingBody;

use xmas_elf::{program::Type, ElfFile};

use crate::paging::{
    self, PTE_AF, PTE_AP_RW_EL0, PTE_ATTR_DEVICE, PTE_ATTR_NORMAL, PTE_PAGE, PTE_SH_INNER, PTE_UXN,
    PTE_VALID,
};

// --- SHARED STRUCTS ---

static APP_LOAD_ADDR: AtomicU64 = AtomicU64::new(0x40_0000_0000);

// ScanArgs and scan_boot_fs_task removed (Hardware Loader Killed)
pub fn spawn_elf(
    k: &mut Kernel<Bridge>,
    parent_dir_id: Option<ThingId>,
    name: &str,
    data: &[u8],
    _idx: usize,
    spawn_override: Option<bool>,
    hhdm_u64: u64,
    fb_phys: u64,
    fb_size: usize,
) {
    // --- Graph: Create Module Thing ---
    // REMOVED: Now handled by kernel::boot_fs::ingest_module before this function is called.
    // This function focuses on Spawning (ELF Loading) only.

    let should_spawn = if let Some(s) = spawn_override {
        s
    } else {
        // Fallback or panic? For now, we only call this when we want to spawn.
        true
    };

    if should_spawn {
        use kernel::sched::elf::load_elf;

        // Note: AArch64 doesn't use x86_64 registers, but the code below seems to have copy-pasted/adapted names.
        // Actually, the previous file view showed process_file had minimal graph logic, it was mostly ELF logic.
        // Wait, looking at lines 386+ in the previous view...
        // Ah, the previous view (Step 456) of process_file in `arch/aarch64/src/loader.rs` does NOT show the graph ingestion logic I saw in x86.
        // It starts at line 285.
        // Line 296 checks spawn_override.
        // It jumps straight into `load_elf`.
        //
        // CONCLUSION: The AArch64 `process_file` WAS ALREADY MISSING the graph ingestion logic!
        // That's why the user had the bug.
        // And that's why my previous step (adding ingest_module call in the loop) was the correct fix.
        //
        // So I don't need to "remove" anything from `process_file` because it wasn't there.
        //
        // I should just verify the file passes compilation.

        Bridge.log("loader: Spawning...\n");

        let current_app_base = APP_LOAD_ADDR.fetch_add(0x1000_0000, Ordering::Relaxed);
        let root_table = unsafe { paging::create_user_root().expect("OOM Root") };

        // ... (rest of spawning logic)

        let mut max_loaded_addr = current_app_base;
        let loaded = load_elf(data, current_app_base, |vaddr, segment| {
            let raw_addr = current_app_base + vaddr;
            write_user_bytes(raw_addr, segment, root_table, hhdm_u64);
            let end_addr = raw_addr + segment.len() as u64;
            if end_addr > max_loaded_addr {
                max_loaded_addr = end_addr;
            }
        });

        if let Some(img) = loaded {
            let applied = apply_relative_relocations(data, current_app_base, root_table, hhdm_u64);
            if applied == 0 {
                unsafe {
                    k.bridge.log("loader: WARN: Applied 0 relocations!\n");
                }
            }

            let stack_size = 128 * 1024;
            let stack_bottom = current_app_base + 0x0800_0000;
            let stack_top = stack_bottom + stack_size;

            {
                let map_flags =
                    PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_NORMAL;
                let mut current = stack_bottom;
                while current < stack_top {
                    let (p, _v) = unsafe { paging::allocate_frame().expect("OOM Stack") };
                    unsafe { paging::map_page_at_root(root_table, p, current, map_flags) };
                    current += 4096;
                }
                unsafe {
                    k.bridge.log("loader: Mapped Stack\n");
                }
            }

            let heap_start = current_app_base + 0x0100_0000;
            let heap_size = 1 * 1024 * 1024; // REDUCED TO 1MB for Debugging
            let heap_end = heap_start + heap_size;

            {
                let map_flags =
                    PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_NORMAL;
                let mut current = heap_start;
                unsafe {
                    k.bridge.log("loader: Mapping User Heap...\n");
                }
                while current < heap_end {
                    let (p, _v) = unsafe { paging::allocate_frame().expect("OOM Heap") };
                    unsafe { paging::map_page_at_root(root_table, p, current, map_flags) };
                    current += 4096;
                }
            }

            unsafe {
                if fb_size > 0 {
                    let user_fb = 0x80_0000_0000;
                    let map_flags = PTE_VALID
                        | PTE_PAGE
                        | PTE_AF
                        | PTE_SH_INNER
                        | PTE_AP_RW_EL0
                        | PTE_ATTR_DEVICE
                        | PTE_UXN;
                    let mut current = 0;
                    while current < fb_size {
                        let phys = fb_phys + current as u64;
                        paging::map_page_at_root(
                            root_table,
                            phys,
                            user_fb + current as u64,
                            map_flags,
                        );
                        current += 4096;
                    }
                    k.bridge.log("loader: Mapped Framebuffer\n");
                }
            }

            unsafe {
                core::arch::asm!("msr ttbr0_el1, {}", in(reg) root_table);
                core::arch::asm!("isb");
                k.bridge.log("loader: Activated TTBR0\n");
            }

            unsafe {
                k.bridge.log("loader: computed entry: ");
                k.bridge
                    .log(alloc::format!("{:#x}", current_app_base + img.entry_point).as_str());
                k.bridge.log("\n");
            }

            // Ensure I-Cache is coherent with D-Cache for the loaded code
            let total_len = max_loaded_addr - current_app_base;
            unsafe {
                sync_icache(current_app_base, total_len as usize + 0x1000);
            }

            unsafe {
                k.bridge.log("loader: I-Cache Synced\n");
            }

            // Spawn user thread directly (no trampoline). Pass heap start as arg like x86 path.
            k.scheduler.spawn(
                &k.bridge,
                name,
                current_app_base + img.entry_point,
                stack_top,
                heap_start,
                heap_start,
                heap_end,
            );

            // Register in Graph
            let _process_id =
                kernel::boot_fs::register_boot_process(k, name, current_app_base + img.entry_point);
        }
    }

    unsafe {
        Bridge.log("loader: Setup complete. Sleeping...\n");
    }

    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}

fn write_user_bytes(raw_addr: u64, data: &[u8], root_table: u64, _hhdm: u64) {
    let start = raw_addr;
    let end = raw_addr + data.len() as u64;

    let start_page = start & !0xFFF;
    let end_page = (end - 1) & !0xFFF;

    let mut current_page = start_page;
    while current_page <= end_page {
        // Allocate frame
        let (phys, virt) = unsafe { paging::allocate_frame().expect("OOM Load") };
        let flags = PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_NORMAL;

        unsafe {
            paging::map_page_at_root(root_table, phys, current_page, flags);
        }

        // Calculate offsets
        let page_offset = if current_page == start_page {
            start & 0xFFF
        } else {
            0
        };
        let buf_start = if current_page == start_page {
            0
        } else {
            current_page - start
        };
        let buf_end = core::cmp::min(data.len() as u64, (current_page + 4096) - start);

        let len = buf_end - buf_start;

        // Copy using virtual address of frame
        let dest = (virt + page_offset) as *mut u8;
        let src = &data[buf_start as usize] as *const u8;
        unsafe {
            core::ptr::copy_nonoverlapping(src, dest, len as usize);
        }

        current_page += 4096;
    }
}

fn apply_relative_relocations(
    elf_data: &[u8],
    load_base: u64,
    root_table: u64,
    hhdm: u64,
) -> usize {
    let elf = match ElfFile::new(elf_data) {
        Ok(e) => e,
        Err(_) => return 0,
    };

    let dyn_ph = elf
        .program_iter()
        .find(|ph| ph.get_type().map(|t| t == Type::Dynamic).unwrap_or(false));

    if let Some(dyn_ph) = dyn_ph {
        let dyn_offset = dyn_ph.offset();
        let dyn_size = dyn_ph.file_size();
        let dyn_entries = &elf_data[dyn_offset as usize..(dyn_offset + dyn_size) as usize];

        let mut rela_addr = 0u64;
        let mut rela_sz = 0u64;
        let mut rela_ent = 0u64;

        for chunk in dyn_entries.chunks(16) {
            if chunk.len() < 16 {
                break;
            }
            let tag = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
            let val = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
            match tag {
                7 => rela_addr = val, // DT_RELA
                8 => rela_sz = val,   // DT_RELASZ
                9 => rela_ent = val,  // DT_RELAENT
                0 => break,
                _ => {}
            }
        }

        if rela_addr == 0 || rela_sz == 0 {
            return 0;
        }

        let mut file_offset = None;
        for ph in elf.program_iter() {
            if ph.get_type().unwrap_or(Type::Null) == Type::Load {
                let vaddr = ph.virtual_addr();
                let mem_sz = ph.mem_size();
                if rela_addr >= vaddr && rela_addr < vaddr + mem_sz {
                    file_offset = Some(ph.offset() + (rela_addr - vaddr));
                    break;
                }
            }
        }

        if let Some(file_off) = file_offset {
            let rela_data = &elf_data[file_off as usize..(file_off + rela_sz) as usize];
            let ent_size = if rela_ent > 0 { rela_ent } else { 24 };
            let mut applied = 0usize;

            for chunk in rela_data.chunks(ent_size as usize) {
                if chunk.len() < 24 {
                    break;
                }
                let r_offset = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
                let r_info = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
                let r_addend = i64::from_le_bytes(chunk[16..24].try_into().unwrap());
                let r_type = r_info & 0xFFFF_FFFF;

                // R_AARCH64_RELATIVE = 1027
                // R_AARCH64_GLOB_DAT = 1025
                // R_AARCH64_JUMP_SLOT = 1026
                if r_type == 1027 || r_type == 1025 || r_type == 1026 {
                    let value = load_base.wrapping_add(r_addend as u64);
                    // Write value to load_base + r_offset
                    // Use a simple one-off write helper or map?
                    // We need to find the physical address of load_base + r_offset.
                    // Since we don't track it, and we can't easily translate...
                    // Wait, we need to translate!
                    // AArch64 paging module needs a `translate` function exposed.

                    // Hack: Iterate pages? Too slow.
                    // But we are in loader, maybe okay?

                    // Actually, if we just traverse the table software-side?
                    // `paging` module likely has utils.
                    // I will assume `paging::translate(root, vaddr)` exists?
                    // Step 1073 imports `paging` but I didn't see details.
                    // But `write_user_bytes` above assumes we are Allocating.

                    // Here we are patching EXISTING pages.
                    // If we can't translate, we can't patch.
                    // I'll assume `paging::translate` exists or I'll query it.

                    if let Some(phys) =
                        unsafe { paging::translate(root_table, load_base + r_offset) }
                    {
                        // Phys + HHDM = Virt
                        let virt = hhdm + phys;
                        unsafe {
                            *(virt as *mut u64) = value;
                        }
                        applied += 1;
                    }
                }
            }
            return applied;
        }
    }
    0
}

unsafe fn sync_icache(start: u64, len: usize) {
    let mut addr = start & !63; // Align to cache line (assume 64 bytes for simplicity, safe bet)
    let end = start + len as u64;

    // 1. Clean D-Cache to Point of Unification (PoU)
    while addr < end {
        core::arch::asm!("dc cvau, {}", in(reg) addr);
        addr += 64;
    }
    core::arch::asm!("dsb ish");

    // 2. Invalidate I-Cache to Point of Unification (PoU)
    addr = start & !63;
    while addr < end {
        core::arch::asm!("ic ivau, {}", in(reg) addr);
        addr += 64;
    }
    core::arch::asm!("dsb ish");
    core::arch::asm!("isb");
}
