use alloc::alloc::{alloc, Layout};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::ThingId;

use crate::bridge::Bridge;
use crate::KERNEL;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::boot_fs;
use kernel::bridge::HardwareBridge;
use kernel::Kernel;
use models::value::ThingBody;
use x86_64::structures::paging::mapper::TranslateError;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags, PhysFrame,
    Size2MiB, Size4KiB, Translate,
};
use x86_64::VirtAddr;
use xmas_elf::{program::Type, ElfFile};

// --- SHARED STRUCTS ---

// ScanArgs removed

// --- BOOT LOGIC ---

static APP_LOAD_ADDR: AtomicU64 = AtomicU64::new(0x40_0000_0000);

struct HeapFrameAllocator {
    hhdm_offset: VirtAddr,
}

unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        use alloc::alloc::{alloc_zeroed, Layout};
        let layout = Layout::from_size_align(4096, 4096).ok()?;
        // Bridge.log("loader: HFA Alloc start\n");
        let ptr = unsafe { alloc_zeroed(layout) };
        if ptr.is_null() {
            return None;
        }

        use x86_64::registers::control::Cr3;
        let (l4_frame, _) = Cr3::read();
        let phys_l4 = l4_frame.start_address();
        // Safe HHDM Add
        let raw_virt_l4 = self.hhdm_offset.as_u64().wrapping_add(phys_l4.as_u64());
        let virt_l4 = match VirtAddr::try_new(raw_virt_l4) {
            Ok(a) => a,
            Err(_) => {
                Bridge.log("loader: HFA VirtAddr Add Fail!\n");
                return None;
            }
        };
        let page_table_ptr = virt_l4.as_mut_ptr();
        let mapper = unsafe { OffsetPageTable::new(&mut *page_table_ptr, self.hhdm_offset) };

        let virt_addr = VirtAddr::try_new(ptr as u64).ok()?;
        let phys_frame = mapper
            .translate_addr(virt_addr)
            .map(|phys| PhysFrame::containing_address(phys));

        if let Some(f) = phys_frame {
            // HFA Logging reduced
        } else {
            Bridge.log("loader: HFA Translate Fail!\n");
        }

        phys_frame
    }
}

fn write_user_bytes(
    raw_addr: u64,
    data: &[u8],
    mapper: &mut OffsetPageTable<'static>,
    frame_allocator: &mut HeapFrameAllocator,
    hhdm_offset: VirtAddr,
) {
    use x86_64::structures::paging::{
        mapper::TranslateError, mapper::TranslateResult, Mapper, Page, PageTableFlags, Size2MiB,
        Size4KiB,
    };

    let start = VirtAddr::new(raw_addr);
    let end = VirtAddr::new(raw_addr + data.len() as u64);
    let start_page = Page::<Size4KiB>::containing_address(start);
    let end_page = Page::<Size4KiB>::containing_address(end - 1u64);

    for page in Page::range_inclusive(start_page, end_page) {
        let page_start_virt = page.start_address();
        let mut needs_alloc = true;

        match mapper.translate_page(page) {
            Ok(_) => {
                let new_flags = PageTableFlags::PRESENT
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::USER_ACCESSIBLE;
                unsafe {
                    if let Ok(flush) = mapper.update_flags(page, new_flags) {
                        flush.flush();
                    }
                }
                needs_alloc = false;
            }
            Err(TranslateError::ParentEntryHugePage) => {
                let huge_page = Page::<Size2MiB>::containing_address(page_start_virt);
                unsafe {
                    if let Ok((_phys, flush)) = mapper.unmap(huge_page) {
                        flush.flush();
                        let s = alloc::format!(
                            "loader: Unmapped conflicting Huge Page: {:?} (phys frame: {:?})\n",
                            huge_page.start_address(),
                            _phys.start_address()
                        );
                        Bridge.log(&s);
                    }
                }
            }
            Err(TranslateError::PageNotMapped) => {}
            Err(_) => {
                Bridge.log("loader: Translate Error!\n");
                return;
            }
        }

        if needs_alloc {
            if let Some(frame) = frame_allocator.allocate_frame() {
                let flags = PageTableFlags::PRESENT
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::USER_ACCESSIBLE;
                unsafe {
                    if let Ok(map_to) = mapper.map_to(page, frame, flags, frame_allocator) {
                        map_to.flush();
                    }
                }
            } else {
                Bridge.log("loader: Frame alloc failed during write\n");
                return;
            }
        }

        let overlap_start = core::cmp::max(page_start_virt, start);
        let overlap_end = core::cmp::min(page_start_virt + 4096u64, end);
        if overlap_end <= overlap_start {
            continue;
        }

        let copy_len = overlap_end - overlap_start;
        let seg_offset = overlap_start - start;
        let page_offset = overlap_start - page_start_virt;

        if let TranslateResult::Mapped { frame, offset, .. } = mapper.translate(page_start_virt) {
            let phys = frame.start_address() + offset;
            let frame_virt = hhdm_offset + phys.as_u64();

            let src_ptr = unsafe { data.as_ptr().add(seg_offset as usize) };
            let dest_ptr = unsafe { (frame_virt.as_mut_ptr::<u8>()).add(page_offset as usize) };
            unsafe {
                core::ptr::copy_nonoverlapping(src_ptr, dest_ptr, copy_len as usize);
            }
        }
    }
}

fn apply_relative_relocations(
    elf_data: &[u8],
    load_base: u64,
    mapper: &mut OffsetPageTable<'static>,
    frame_allocator: &mut HeapFrameAllocator,
    hhdm_offset: VirtAddr,
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

                // Debug: Print every relocation type to catch non-RELATIVE ones
                // match r_type {
                //    8 => { ... } // R_X86_64_RELATIVE
                //    6 => { ... } // R_X86_64_GLOB_DAT
                //    ...
                // }

                if r_type == 8 {
                    let value = load_base.wrapping_add(r_addend as u64);
                    write_user_bytes(
                        load_base + r_offset,
                        &value.to_le_bytes(),
                        mapper,
                        frame_allocator,
                        hhdm_offset,
                    );

                    applied += 1;
                } else {
                    Bridge.log(
                        alloc::format!("loader: Unknown Relocation Type: {}\n", r_type).as_str(),
                    );
                    panic!("loader: Unknown Relocation Type: {}", r_type);
                }
            }

            return applied;
        }
    }

    0
}

// scan_boot_fs_task and file_loader_task removed (Hardware Loader Killed)

pub fn spawn_elf(
    k: &mut Kernel<Bridge>,
    parent_dir_id: Option<ThingId>,
    name: &str,
    data: &[u8],
    _idx: usize,
    spawn_override: Option<bool>,
    hhdm_u64: u64,
) {
    Bridge.log("loader: spawn_elf ");
    Bridge.log(name);
    Bridge.log("\n");
    // Removal of Arch-side policy: We only spawn if explicitly requested by the caller (Kernel).
    let should_spawn = spawn_override.unwrap_or(false);

    if should_spawn {
        use kernel::sched::elf::load_elf;
        use x86_64::registers::control::Cr3;

        // Atomic Increment
        let current_app_base = APP_LOAD_ADDR.fetch_add(0x1000_0000, Ordering::Relaxed);

        let hhdm_offset = match VirtAddr::try_new(hhdm_u64) {
            Ok(a) => a,
            Err(_) => {
                unsafe {
                    Bridge.log("loader: Invalid HHDM Offset!\n");
                }
                return;
            }
        };

        unsafe {
            let s = alloc::format!("loader: HHDM Offset: {:#x}\n", hhdm_offset.as_u64());
            Bridge.log(&s);
        }

        // Mapper
        let mut frame_allocator = HeapFrameAllocator { hhdm_offset };
        let mut mapper = unsafe {
            let (level_4_table_frame, _) = Cr3::read();
            let phys = level_4_table_frame.start_address();
            let raw_virt = hhdm_offset.as_u64().wrapping_add(phys.as_u64());
            let virt = match VirtAddr::try_new(raw_virt) {
                Ok(a) => a,
                Err(_) => {
                    unsafe {
                        Bridge.log("loader: PageTable VirtAddr Add Fail!\n");
                    }
                    // Panic here manually or return dummy to fail later?
                    // We can't return from unsafe block easily.
                    // But we can panic with message
                    panic!("loader: PageTable VirtAddr Add Fail: {:#x}", raw_virt);
                }
            };
            let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
            let table = &mut *page_table_ptr;

            // Fix: Limine might have mapped P4[0] (Identity) with NX.
            // User Space (0x2000_0000) is in P4[0] -> P3[0].
            // We must clear NX to allow executing user code.
            if !table[0].is_unused() {
                // Clear NX
                let flags = table[0].flags();
                unsafe {
                    let s = alloc::format!("loader: PML4[0] Flags: {:?}\n", flags);
                    Bridge.log(&s);
                }

                if flags.contains(PageTableFlags::NO_EXECUTE) {
                    Bridge.log("loader: Clearing NX from PML4[0]\n");
                    table[0].set_flags(flags & !PageTableFlags::NO_EXECUTE);
                }

                // Check P3[0]
                let p3_phys = table[0].addr();
                let p3_virt = hhdm_offset.as_u64().wrapping_add(p3_phys.as_u64());
                if let Ok(p3_virt_addr) = VirtAddr::try_new(p3_virt) {
                    let p3_ptr: *mut PageTable = p3_virt_addr.as_mut_ptr();
                    let p3 = &mut *p3_ptr;
                    if !p3[0].is_unused() {
                        let f3 = p3[0].flags();
                        unsafe {
                            let s = alloc::format!("loader: PDP[0] Flags: {:?}\n", f3);
                            Bridge.log(&s);
                        }

                        if f3.contains(PageTableFlags::NO_EXECUTE) {
                            Bridge.log("loader: Clearing NX from PDP[0]\n");
                            p3[0].set_flags(f3 & !PageTableFlags::NO_EXECUTE);
                        }
                    }
                }

                x86_64::instructions::tlb::flush_all();
            }

            OffsetPageTable::new(&mut *page_table_ptr, hhdm_offset)
        };

        // Map Framebuffer (User Space 0x1_0000_0000)
        // Matches kernel/src/drivers/limine_fb.rs
        unsafe {
            if let Some((fb_phys, fb_size)) = crate::FRAMEBUFFER_INFO {
                unsafe {
                    let s = alloc::format!(
                        "loader: Mapping FB Phys={:#x} Size={:#x}\n",
                        fb_phys,
                        fb_size
                    );
                    Bridge.log(&s);
                }

                use x86_64::structures::paging::{
                    Mapper, Page, PageTableFlags, PhysFrame, Size4KiB,
                };
                use x86_64::PhysAddr;

                let start_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(fb_phys));
                let end_frame =
                    PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(fb_phys + fb_size - 1));

                let user_virt_base = VirtAddr::new(0x80_0000_0000);
                let mut virt_iter = user_virt_base;

                for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
                    let page = Page::<Size4KiB>::containing_address(virt_iter);
                    let flags = PageTableFlags::PRESENT
                        | PageTableFlags::WRITABLE
                        | PageTableFlags::USER_ACCESSIBLE
                        | PageTableFlags::NO_CACHE;
                    if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                        map_to.flush();
                    }
                    virt_iter += 4096u64;
                }

                Bridge.log("loader: Mapped User Framebuffer at 0x80_0000_0000\n");
            }
        }

        Bridge.log("loader: calling load_elf\n");
        let loaded = load_elf(data, current_app_base, |vaddr, segment| {
            let raw_addr = current_app_base + vaddr;
            write_user_bytes(
                raw_addr,
                segment,
                &mut mapper,
                &mut frame_allocator,
                hhdm_offset,
            );
        });

        Bridge.log("loader: load_elf success check\n");

        if let Some(img) = loaded {
            let applied = apply_relative_relocations(
                data,
                current_app_base,
                &mut mapper,
                &mut frame_allocator,
                hhdm_offset,
            );
            unsafe {
                let s = alloc::format!("loader: reloc applied count={}\n", applied);
                Bridge.log(&s);
                // Spot check a few early GOT slots used in _start
                use x86_64::structures::paging::Translate;
                let check_slots = [0x6eed8u64, 0x6eee0u64, 0x6eef0u64];
                for slot in check_slots {
                    if let x86_64::structures::paging::mapper::TranslateResult::Mapped {
                        frame,
                        offset,
                        ..
                    } = mapper.translate(VirtAddr::new(current_app_base + slot))
                    {
                        let phys = frame.start_address() + offset;
                        let virt = hhdm_offset + phys.as_u64();
                        let val = core::ptr::read(virt.as_ptr::<u64>());
                        let s = alloc::format!("loader: GOT[{:#x}]={:#x}\n", slot, val);
                        Bridge.log(&s);
                    }
                }
            }

            unsafe {
                use x86_64::structures::paging::mapper::TranslateResult;
                use x86_64::structures::paging::Translate;
                let entry_virt = VirtAddr::new(current_app_base + img.entry_point);
                if let TranslateResult::Mapped { flags, .. } = mapper.translate(entry_virt) {
                    let s = alloc::format!(
                        "loader: Entry Point {:#x} Flags: {:?}\n",
                        entry_virt.as_u64(),
                        flags
                    );
                    Bridge.log(&s);
                } else {
                    Bridge.log("loader: Entry Point NOT MAPPED!\n");
                }
            }
            // Debug: peek a GOT slot to verify relocations
            unsafe {
                use x86_64::structures::paging::Translate;
                if let x86_64::structures::paging::mapper::TranslateResult::Mapped {
                    frame,
                    offset,
                    ..
                } = mapper.translate(VirtAddr::new(current_app_base + 0x61ab8))
                {
                    let phys = frame.start_address() + offset;
                    let virt = hhdm_offset + phys.as_u64();
                    let val = core::ptr::read(virt.as_ptr::<u64>());
                    let s = alloc::format!("loader: GOT[0x61ab8]={:#x}\n", val);
                    Bridge.log(&s);
                }
            }
            Bridge.log("loader: setting up stack\n");
            // Stack and Heap (Simplified alloc)
            let raw_stack_bottom = current_app_base + 0x0800_0000;
            let stack_bottom_virt = match VirtAddr::try_new(raw_stack_bottom) {
                Ok(a) => a,
                Err(_) => {
                    Bridge.log("loader: Stack Bottom VirtAddr Invalid!\n");
                    return;
                }
            };
            let stack_size = 131072;
            let raw_stack_top = raw_stack_bottom + stack_size;
            let stack_top_virt = match VirtAddr::try_new(raw_stack_top) {
                Ok(a) => a,
                Err(_) => {
                    Bridge.log("loader: Stack Top VirtAddr Invalid!\n");
                    return;
                }
            };

            let start_page = Page::<Size4KiB>::containing_address(stack_bottom_virt);

            let raw_end_addr = raw_stack_top - 1;
            let end_addr_virt = match VirtAddr::try_new(raw_end_addr) {
                Ok(a) => a,
                Err(_) => {
                    Bridge.log("loader: Stack End VirtAddr Invalid!\n");
                    return;
                }
            };
            let end_page = Page::<Size4KiB>::containing_address(end_addr_virt);
            for page in Page::range_inclusive(start_page, end_page) {
                let frame = match frame_allocator.allocate_frame() {
                    Some(f) => f,
                    None => {
                        Bridge.log("loader: Stack Alloc OOM\n");
                        return;
                    }
                };
                let flags = PageTableFlags::PRESENT
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::USER_ACCESSIBLE;
                unsafe {
                    if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                        map_to.flush();
                    }
                }
            }

            let heap_virt_start = current_app_base + 0x0100_0000;
            // Default 128MB heap (Virtual only - no physical cost yet).
            let heap_size = 128 * 1024 * 1024;
            let heap_virt_end = heap_virt_start + heap_size;

            // Map Framebuffer
            Bridge.log("loader: checking fb_info\n");
            let fb_info = unsafe { crate::FRAMEBUFFER_INFO };
            if let Some((phys_base_raw, size)) = fb_info {
                let s = alloc::format!(
                    "loader: mapping framebuffer. Base={:#x} Size={:#x}\n",
                    phys_base_raw,
                    size
                );
                Bridge.log(&s);
                // Fix: Limine returns a Virtual Address (HHDM mapped). Convert to physical.
                let phys_base = if phys_base_raw >= hhdm_offset.as_u64() {
                    phys_base_raw - hhdm_offset.as_u64()
                } else {
                    phys_base_raw
                };
                // Fix: Subtract HHDM offset to get physical IF needed
                // If it came from Limine FB, Limine provides HHDM-mapped address? No, Limine provides physical.
                // But let's check HHDM just in case someone stored virtual.
                // Assuming FRAMEBUFFER_INFO stores PHYSICAL address.

                // 4GB base
                let virt_base = 0x1_0000_0000;

                let raw_heap_start = virt_base;
                let raw_heap_end = virt_base + size - 1;

                let start_page = if let Ok(addr) = VirtAddr::try_new(raw_heap_start) {
                    Page::<Size4KiB>::containing_address(addr)
                } else {
                    Bridge.log("loader: Heap Start VirtAddr Invalid!\n");
                    return;
                };

                let end_page = if let Ok(addr) = VirtAddr::try_new(raw_heap_end) {
                    Page::<Size4KiB>::containing_address(addr)
                } else {
                    Bridge.log("loader: Heap End VirtAddr Invalid!\n");
                    return;
                };

                for page in Page::range_inclusive(start_page, end_page) {
                    let offset = page.start_address().as_u64() - virt_base;
                    let phys =
                        PhysFrame::containing_address(x86_64::PhysAddr::new(phys_base + offset));
                    // User | RW | NoCache (generic safe default)
                    let flags = PageTableFlags::PRESENT
                        | PageTableFlags::WRITABLE
                        | PageTableFlags::USER_ACCESSIBLE
                        | PageTableFlags::NO_CACHE;

                    unsafe {
                        if let Ok(map_to) = mapper.map_to(page, phys, flags, &mut frame_allocator) {
                            map_to.flush();
                        }
                    }
                }
            }

            // Verify driver symbol (heuristic: if symbol exists, use it)
            // Verify driver symbol (heuristic: if symbol exists, use it)
            // Logic is now generic.
            let mut final_entry_point = current_app_base + img.entry_point;
            if let Some(offset) = kernel::sched::elf::find_symbol(data, "thingos_driver_init") {
                unsafe {
                    Bridge.log("loader: Found thingos_driver_init override!\n");
                }
                final_entry_point = current_app_base + offset;
            }

            // Spawn
            k.scheduler.spawn(
                &k.bridge,
                name,
                final_entry_point,
                stack_top_virt.as_u64() - 8, // Adjust for Canary
                heap_virt_start,             // Arg passed to main (heap_start)
                heap_virt_start,             // Process heap start
                heap_virt_end,               // Process heap end
            );

            // Stack Canary: Write return address to top of stack to debug RIP=0
            unsafe {
                let stack_ptr = stack_top_virt.as_mut_ptr::<u64>();
                // Write at -1 (top of stack, since stack grows down and RSP points here?)
                // Actually, if we set RSP = stack_top_virt, then a RET will pop [RSP].
                // So we write to stack_ptr.
                // Wait, make sure we mapped it! stack_top_virt is raw_stack_top which is stack_bottom + size.
                // containing_address(stack_top_virt) might be the *next* page if aligned?
                // stack_top_virt is exclusive end?
                // raw_stack_top = raw_stack_bottom + size.
                // If size is 4096, bottom=0, top=4096.
                // Byte at 4095 is last byte.
                // SP usually points to *used* or *empty*? x86 SP points to Last Pushed (Valid).
                // So if we start with empty stack, SP should be Top. PUSH decrements then writes.
                // RET reads then increments.
                // So if we RET immediately, we read [SP]. So SP must point to a valid value.
                // We need to write to `stack_top_virt`? No, that's just outside.
                // We should assume SP = stack_top_virt.
                // But for RET to work, SP must point to data. So we need to predecrement?
                // Or does `spawn` set RSP to `stack_top - 8`?
                // In `spawn`: `thread.regs.rsp = stack`.
                // If we want `ret` to consume 0xDEADBEEF, we must place it at address `stack`.
                // And `stack` MUST be mapped.
                // `stack_top_virt` is the boundary. The byte at `stack_top_virt` is NOT mapped likely (next page).
                // The stack grows down from there.
                // So we want RSP to be `stack_top_virt - 8`.
                // And we write 0xDEADBEEF at `stack_top_virt - 8`.
                // Then `spawn` should take `stack_top_virt - 8`.
                // Currently `spawn` takes `stack_top_virt.as_u64()`.
                // Let's write canary at `stack_top_virt - 8` and pass `stack_top_virt - 8` to spawn.

                let canary_addr = stack_top_virt - 8u64;
                let canary_ptr = canary_addr.as_mut_ptr::<u64>();
                *canary_ptr = 0xDEAD_BEEF_DEAD_BEEF;
                Bridge.log("loader: Wrote Stack Canary 0xDEAD_BEEF_DEAD_BEEF\n");
            }

            // Create Process Thing (via Kernel Helper)
            let _process_id =
                kernel::boot_fs::register_boot_process(k, name, current_app_base + img.entry_point);
        }
    }
}
