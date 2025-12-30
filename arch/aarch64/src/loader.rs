use alloc::alloc::{alloc, Layout};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::ThingId;

use crate::KERNEL;
use bridge_aarch64::Bridge;
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

pub struct ScanArgs {
    pub bb_info: Option<u64>, // placeholder
    pub hhdm: u64,
    pub fb_phys: u64,
    pub fb_size: usize,
    pub modules: Vec<boot::ModuleInfo>,
}

// --- BOOT LOGIC ---

static APP_LOAD_ADDR: AtomicU64 = AtomicU64::new(0x40_0000_0000);

fn write_user_bytes(raw_addr: u64, data: &[u8], root_table: u64, hhdm_offset: u64) {
    let start = raw_addr;
    let end = raw_addr + data.len() as u64;
    const PAGE_SIZE: u64 = 4096;

    let start_page = start & !(PAGE_SIZE - 1);
    let end_page = (end + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);

    let map_flags = PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_NORMAL;

    let mut current = start_page;
    while current < end_page {
        unsafe {
            let mut phys = paging::get_phys(root_table, current);
            if phys.is_none() {
                let (p, _v) = paging::allocate_frame().expect("OOM user frame");
                paging::map_page_at_root(root_table, p, current, map_flags);
                phys = Some(p);
            }
            let frame_phys = phys.unwrap();
            let frame_virt = hhdm_offset + frame_phys;

            let overlap_start = core::cmp::max(current, start);
            let overlap_end = core::cmp::min(current + PAGE_SIZE, end);
            if overlap_end > overlap_start {
                let copy_len = overlap_end - overlap_start;
                let src_offset = overlap_start - start;
                let page_offset = overlap_start - current;

                let src_ptr = data.as_ptr().add(src_offset as usize);
                let dest_ptr = (frame_virt as *mut u8).add(page_offset as usize);
                core::ptr::copy_nonoverlapping(src_ptr, dest_ptr, copy_len as usize);
            }
        }
        current += PAGE_SIZE;
    }
}

fn apply_relative_relocations(
    elf_data: &[u8],
    load_base: u64,
    root_table: u64,
    hhdm_offset: u64,
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
        let mut symtab_addr = 0u64;
        let mut strtab_addr = 0u64;
        let mut syment = 0u64;

        for chunk in dyn_entries.chunks(16) {
            if chunk.len() < 16 {
                break;
            }
            let tag = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
            let val = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
            match tag {
                7 => rela_addr = val, // DT_RELA
                8 => rela_sz = val,
                9 => rela_ent = val,
                6 => symtab_addr = val, // DT_SYMTAB
                5 => strtab_addr = val, // DT_STRTAB
                11 => syment = val,     // DT_SYMENT
                0 => break,
                _ => {}
            }
        }

        if rela_addr == 0 || rela_sz == 0 {
            return 0;
        }

        // Find file offsets for tables
        let mut rela_offset = None;
        let mut symtab_offset = None;

        // We assume VAddr == Offset mostly for PIE, but let's verify via PHDRs
        for ph in elf.program_iter() {
            if ph.get_type().unwrap_or(Type::Null) == Type::Load {
                let vaddr = ph.virtual_addr();
                let mem_sz = ph.mem_size();
                let file_off = ph.offset();

                // Check RELA
                if rela_addr >= vaddr && rela_addr < vaddr + mem_sz {
                    rela_offset = Some(file_off + (rela_addr - vaddr));
                }
                // Check SYMTAB
                if symtab_addr != 0 && symtab_addr >= vaddr && symtab_addr < vaddr + mem_sz {
                    symtab_offset = Some(file_off + (symtab_addr - vaddr));
                }
                // Check STRTAB (Optional)
                // if strtab_addr ...
            }
        }

        let file_off = if let Some(off) = rela_offset {
            off
        } else {
            return 0;
        };
        let rela_data = &elf_data[file_off as usize..(file_off + rela_sz) as usize];
        let ent_size = if rela_ent > 0 { rela_ent } else { 24 };
        let mut applied = 0;
        let mut unhandled_count = 0;
        let mut first_unhandled = 0;

        let sym_ent_size = if syment > 0 { syment } else { 24 }; // Elf64_Sym is 24 bytes

        for chunk in rela_data.chunks(ent_size as usize) {
            if chunk.len() < 24 {
                break;
            }
            let r_offset = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
            let r_info = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
            let r_addend = i64::from_le_bytes(chunk[16..24].try_into().unwrap());

            let r_type = r_info & 0xFFFF_FFFF;
            let r_sym = r_info >> 32;

            // R_AARCH64_RELATIVE = 1027
            // R_AARCH64_GLOB_DAT = 1025
            // R_AARCH64_JUMP_SLOT = 1026
            // R_AARCH64_ABS64     = 257

            let mut val_to_write = None;

            if r_type == 1027 {
                val_to_write = Some(load_base.wrapping_add(r_addend as u64));
            } else if r_type == 1025 || r_type == 1026 || r_type == 257 {
                if let Some(sym_off) = symtab_offset {
                    // Read symbol
                    let sym_idx = r_sym;
                    let sym_loc = sym_off + (sym_idx * sym_ent_size);
                    if (sym_loc as usize + 24) <= elf_data.len() {
                        let sym_bytes = &elf_data[sym_loc as usize..sym_loc as usize + 24];
                        // st_value is at offset 8 (u64)
                        let st_value = u64::from_le_bytes(sym_bytes[8..16].try_into().unwrap());
                        // st_shndx is at offset 6 (u16). If UNDEF (0), usually 0 value?
                        // For static PIE, symbols should be defined.
                        // Val = Base + SymVal + Addend
                        val_to_write = Some(
                            load_base
                                .wrapping_add(st_value)
                                .wrapping_add(r_addend as u64),
                        );
                    }
                }
            }

            if let Some(value) = val_to_write {
                let target_addr = load_base + r_offset;
                write_user_bytes(target_addr, &value.to_le_bytes(), root_table, hhdm_offset);
                applied += 1;
            } else {
                if unhandled_count == 0 {
                    first_unhandled = r_type;
                }
                unhandled_count += 1;
            }
        }

        if unhandled_count > 0 {
            unsafe {
                bridge_aarch64::Bridge.log(
                    alloc::format!(
                        "loader: {} unhandled relocs. First type: {}\n",
                        unhandled_count,
                        first_unhandled
                    )
                    .as_str(),
                );
            }
        }
        return applied;
    }
    0
}

pub extern "C" fn scan_boot_fs_task(arg: u64) {
    let args_ptr = arg as *mut ScanArgs;
    let args = unsafe { Box::from_raw(args_ptr) };

    let hhdm = args.hhdm;
    let fb_phys = args.fb_phys;
    let fb_size = args.fb_size;
    let modules = &args.modules;

    unsafe {
        Bridge.log("loader: Scanning modules...\n");
    }

    for module in modules {
        // Shared Ingestion Logic (Refactored)
        let _id = unsafe {
            let data_slice = core::slice::from_raw_parts(module.start as *const u8, module.size as usize);
            // Assuming Limine modules are HHDM mapped already, or identity. 
            // In Limine, modules are usually in HHDM.
            
            let mut guard = KERNEL.lock();
            if let Some(k) = guard.as_mut() {
                 kernel::boot_fs::ingest_module(k, &module.path, data_slice, None, hhdm)
            } else {
                abi::ThingId(0)
            }
        };

        if module.path.ends_with("loaded.elf") {
            unsafe {
                Bridge.log("loader: Found loaded.elf in modules!\n");
            }
            let phys_start = module.start;
            let size = module.size;
            let virt_start = phys_start; // Already virtual (HHDM)

            let slice =
                unsafe { core::slice::from_raw_parts(virt_start as *const u8, size as usize) };

            // Spawn it
            let mut guard = KERNEL.lock();
            if let Some(k) = guard.as_mut() {
                let name = "loaded";
                // Only spawn here. Graph ingestion happened above.
                process_file(k, None, name, slice, 0, Some(true), hhdm, fb_phys, fb_size);
            }
        }
    }

    loop {
        core::hint::spin_loop();
    }
}

pub fn process_file(
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
            let heap_size = 16 * 1024 * 1024;
            let heap_end = heap_start + heap_size;

            {
                let map_flags =
                    PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_NORMAL;
                let mut current = heap_start;
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
            let _process_id = kernel::boot_fs::register_boot_process(k, name, current_app_base + img.entry_point);
        }
    }
    
    unsafe {
        Bridge.log("loader: Setup complete. Sleeping...\n");
    }

    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}

unsafe fn sync_icache
(start: u64, len: usize) {
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
