use alloc::alloc::{alloc, Layout};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::ThingId;

use crate::KERNEL;
use bridge_x86_64::Bridge;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::bridge::HardwareBridge;
use kernel::fs::iso9660::{BlockReader, Iso9660Reader};
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

pub struct ScanArgs {
    pub base: u64,
    pub port: usize,
    pub hhdm: u64,
}

pub struct FileArgs {
    pub iso: Arc<Iso9660Reader<Box<dyn BlockReader + Send + Sync>>>,
    pub path: String,
    pub dir_id: Option<ThingId>,
    pub should_spawn: bool,
    pub hhdm: u64,
}

// --- HELPER ENUMS ---

enum ModuleType {
    Elf,
    Psf1,
    Psf2,
    Bmp,
    Png,
    Ttf,
    Otf,
    Woff,
    Woff2,
    Unknown,
    Other(String),
}

fn classify_bytes(data: &[u8]) -> ModuleType {
    if data.len() >= 4 && data[0] == 0x7F && data[1] == b'E' && data[2] == b'L' && data[3] == b'F' {
        return ModuleType::Elf;
    }
    if data.len() >= 4 && data[0] == 0x00 && data[1] == 0x01 && data[2] == 0x00 && data[3] == 0x00 {
        return ModuleType::Ttf;
    }
    if data.len() >= 4 && data[0] == b'O' && data[1] == b'T' && data[2] == b'T' && data[3] == b'O' {
        return ModuleType::Otf;
    }
    if data.len() >= 4 && data[0] == b'w' && data[1] == b'O' && data[2] == b'F' && data[3] == b'F' {
        return ModuleType::Woff;
    }
    if data.len() >= 4 && data[0] == b'w' && data[1] == b'O' && data[2] == b'F' && data[3] == b'2' {
        return ModuleType::Woff2;
    }
    if data.len() >= 2 && data[0] == 0x36 && data[1] == 0x04 {
        return ModuleType::Psf1;
    }
    if data.len() >= 4 && data[0] == 0x72 && data[1] == 0xB5 && data[2] == 0x4A && data[3] == 0x86 {
        return ModuleType::Psf2;
    }
    if data.len() >= 2 && data[0] == b'B' && data[1] == b'M' {
        return ModuleType::Bmp;
    }
    if data.len() >= 4 && data[0] == 0x89 && data[1] == b'P' && data[2] == b'N' && data[3] == b'G' {
        return ModuleType::Png;
    }

    if let Some(kind) = infer::get(data) {
        match kind.mime_type() {
            "application/x-executable" | "application/x-elf" | "application/x-sharedlib" => {
                ModuleType::Elf
            }
            "image/bmp" => ModuleType::Bmp,
            "image/png" => ModuleType::Png,
            other => ModuleType::Other(String::from(other)),
        }
    } else {
        ModuleType::Unknown
    }
}

enum ModuleRole {
    App,
    Driver,
    #[allow(dead_code)]
    Debug,
    Asset,
    #[allow(dead_code)]
    Ignore,
}

fn get_module_role(name: &str, mtype: &ModuleType) -> ModuleRole {
    if name.contains("/drivers/") || name.contains("ps2_") {
        return ModuleRole::Driver;
    }
    match mtype {
        ModuleType::Elf => ModuleRole::App,
        ModuleType::Psf1
        | ModuleType::Psf2
        | ModuleType::Bmp
        | ModuleType::Png
        | ModuleType::Ttf
        | ModuleType::Otf
        | ModuleType::Woff
        | ModuleType::Woff2 => ModuleRole::Asset,
        ModuleType::Other(s) if s.starts_with("image/") || s.starts_with("font/") => {
            ModuleRole::Asset
        }
        _ => {
            if name.contains("font") {
                ModuleRole::Asset
            } else {
                ModuleRole::Ignore
            }
        }
    }
}

// --- BOOT LOGIC ---

static APP_LOAD_ADDR: AtomicU64 = AtomicU64::new(0x2000_0000);

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
        // Log allocated virt addr
        let s = alloc::format!("loader: HFA Alloc Virt: {:#x}\n", ptr as u64);
        Bridge.log(&s);

        let phys_frame = mapper
            .translate_addr(virt_addr)
            .map(|phys| PhysFrame::containing_address(phys));

        if let Some(f) = phys_frame {
            let s = alloc::format!(
                "loader: HFA Alloc Phys: {:#x}\n",
                f.start_address().as_u64()
            );
            Bridge.log(&s);
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
                        Bridge.log("loader: Unmapped conflicting Huge Page\n");
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
                }
            }

            return applied;
        }
    }

    0
}

pub extern "C" fn scan_boot_fs_task(arg: u64) {
    let args_ptr = arg as *mut ScanArgs;
    let args = unsafe { Box::from_raw(args_ptr) }; // Take ownership

    // We need to access AHCI via yield_aware reader.
    // Create shared IsoReader.
    // We need a BlockReader implementation that captures args (base, port).

    let base = args.base;
    let port = args.port;
    let hhdm = args.hhdm;

    let reader = move |lba, buf: &mut [u8]| unsafe {
        use alloc::alloc::{alloc, dealloc, Layout};
        // Allocate 2048-byte aligned buffer to ensure physical contiguity and handle partial reads
        let layout = Layout::from_size_align(2048, 2048).unwrap();
        let ptr = alloc(layout);
        if ptr.is_null() {
            return false;
        }
        let bounce = core::slice::from_raw_parts_mut(ptr, 2048);

        let res = kernel::drivers::ahci::read_sector_yielding(
            &bridge_x86_64::Bridge,
            base,
            port,
            lba,
            bounce,
            hhdm,
            || x86_64::instructions::hlt(),
        );

        if res {
            let len = core::cmp::min(buf.len(), 2048);
            core::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), len);
        }
        dealloc(ptr, layout);
        res
    };

    // Create IsoReader
    let boxed_reader: Box<dyn BlockReader + Send + Sync> = Box::new(reader);
    let iso = match Iso9660Reader::new(boxed_reader) {
        Some(i) => Arc::new(i),
        None => {
            Bridge.log("loader: Failed to init ISO reader\n");
            return;
        }
    };

    Bridge.log("loader: ISO Reader Ready. Scanning...\n");

    // 1. Mount /boot and Dirs (Needs Kernel Lock)
    let (apps_dir_id, _drivers_dir_id, fonts_dir_id, cursors_dir_id, icons_dir_id) = {
        let mut guard = KERNEL.lock();
        if let Some(k) = guard.as_mut() {
            use abi::wire::typed::{CodecId, TypeId, TypedBytes};
            use models::builtins::ids::*;
            use models::core::fs::{DirBody, MountBody};

            // 1. Mount "/boot"
            let m_body = MountBody {
                path: String::from("/boot"),
                readonly: true,
            };
            let m_tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_MOUNT_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: postcard::to_allocvec(&m_body).unwrap(),
            })
            .unwrap();
            let m_id = k.graph.create_thing(THING_MOUNT_KIND, m_tb);

            // Link BootRoot -> Mount
            let l_root = models::link::LinkBody {
                from: THING_BOOT_ROOT,
                to: m_id,
                predicate: THING_HAS_MOUNT_KIND,
            };
            let l_root_tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_LINK_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: postcard::to_allocvec(&l_root).unwrap(),
            })
            .unwrap();
            k.graph.create_thing(THING_LINK_KIND, l_root_tb);

            // 2. Root Dir
            let d_body = DirBody {
                name: String::from("/boot"),
                lba: 0,
                size: 0,
                expanded: true,
            };
            let d_tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_DIR_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: postcard::to_allocvec(&d_body).unwrap(),
            })
            .unwrap();
            let root_id = k.graph.create_thing(THING_DIR_KIND, d_tb);

            // Link Mount -> Root Dir
            let l_mnt = models::link::LinkBody {
                from: m_id,
                to: root_id,
                predicate: THING_MOUNTS_KIND,
            };
            let l_mnt_tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_LINK_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: postcard::to_allocvec(&l_mnt).unwrap(),
            })
            .unwrap();
            k.graph.create_thing(THING_LINK_KIND, l_mnt_tb);

            let mut make_dir = |name: &str, parent: ThingId| {
                let body = DirBody {
                    name: String::from(name),
                    lba: 0,
                    size: 0,
                    expanded: false,
                };
                let tb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_DIR_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: postcard::to_allocvec(&body).unwrap(),
                })
                .unwrap();
                let did = k.graph.create_thing(THING_DIR_KIND, tb);
                let l = models::link::LinkBody {
                    from: parent,
                    to: did,
                    predicate: THING_HAS_ENTRY_KIND,
                };
                let l_tb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_LINK_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: postcard::to_allocvec(&l).unwrap(),
                })
                .unwrap();
                k.graph.create_thing(THING_LINK_KIND, l_tb);
                did
            };

            let apps = make_dir("apps", root_id);
            let drivers = make_dir("drivers", root_id);
            let fonts = make_dir("fonts", root_id);
            let cursors = make_dir("cursors", root_id);
            let icons = make_dir("icons", root_id);

            Bridge.log("loader: Mounts created\n");
            (apps, drivers, fonts, cursors, icons)
        } else {
            return;
        }
    };

    // 2. Read Policy (Parallel? Nah, small)
    let mut whitelist = None;
    if let Some(h) = iso.open("/boot/init.txt") {
        let mut data = alloc::vec![0u8; h.size as usize];
        iso.read(&h, 0, h.size as usize, &mut data);
        if let Ok(s) = core::str::from_utf8(&data) {
            let list: Vec<String> = s
                .lines()
                .map(|l| l.trim().to_ascii_lowercase())
                .filter(|l| !l.is_empty())
                .map(String::from)
                .collect();
            whitelist = Some(list);
            Bridge.log("loader: init.txt loaded\n");
        }
    }

    // 3. Scan /boot/apps
    // We can read directory entries serially (fast), then spawn loaders.
    let apps_entries = iso.read_dir("/boot/apps").unwrap_or_default();
    Bridge.log("loader: Spawning app loaders...\n");

    for entry in apps_entries {
        if entry.is_dir {
            continue;
        }

        let should_run = if !entry.name.ends_with(".elf") {
            false
        } else {
            if let Some(wl) = &whitelist {
                wl.contains(&entry.name) // name is already lowercase? Iso9660Reader returns logic.
            } else {
                false // Loaded now orchestrates startup; only whitelist auto-runs.
            }
        };

        // Spawn Task
        let path = alloc::format!("/boot/apps/{}", entry.name);

        // We only spawn if we want to process it?
        // User asked to speed up launches. Processing everything is good.

        let f_args = FileArgs {
            iso: iso.clone(),
            path,
            dir_id: Some(apps_dir_id),
            should_spawn: should_run, // For now, just pass bool. Main logic also checks.
            hhdm,
        };

        let args_box = Box::new(f_args);
        let args_ptr = Box::into_raw(args_box) as u64;

        let mut guard = KERNEL.lock();
        if let Some(k) = guard.as_mut() {
            // Allocate Aligned Stack (64KB)
            let layout = Layout::from_size_align(256 * 1024, 16).unwrap();
            let stack_ptr = unsafe { alloc(layout) };
            // Subtract 8 to satisfy System V ABI
            let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 } - 8;

            k.scheduler.spawn(
                &k.bridge,
                "app_loader",
                file_loader_task as *const () as usize as u64,
                stack_top,
                args_ptr,
                0, // Heap Start (Kernel task, no user heap)
                0, // Heap End
            );
        } // drop lock
          // file_loader_task(args_ptr);
    }

    // 4. Drivers
    // Drivers are NOT loaded as modules anymore (only loaded.elf is).
    // So we MUST scan /boot/drivers here.
    let drv_entries = iso.read_dir("/boot/drivers").unwrap_or_default();
    Bridge.log("loader: Spawning driver loaders...\n");
    for entry in drv_entries {
        if entry.is_dir {
            continue;
        }

        // Default FALSE unless whitelist
        let should_run = if let Some(wl) = &whitelist {
            wl.contains(&entry.name)
        } else {
            false // Drivers should not be auto-spawned by loader?
                  // Wait, if loaded app spawns them, loader shouldn't spawn them as processes.
                  // But loader MUST create the Things in the graph.
                  // file_loader_task does both: creates thing AND spawns if should_spawn is true.
                  // loaded app will spawn them via syscall.
                  // So here should_run should be FALSE.
        };

        let path = alloc::format!("/boot/drivers/{}", entry.name);
        let f_args = FileArgs {
            iso: iso.clone(),
            path,
            dir_id: Some(_drivers_dir_id),
            should_spawn: should_run,
            hhdm,
        };
        let args_box = Box::new(f_args);
        let args_ptr = Box::into_raw(args_box) as u64;

        let mut guard = KERNEL.lock();
        if let Some(k) = guard.as_mut() {
            // Allocate Aligned Stack (64KB)
            let layout = Layout::from_size_align(64 * 1024, 16).unwrap();
            let stack_ptr = unsafe { alloc(layout) };
            // Subtract 8 to satisfy System V ABI
            let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 } - 8;

            k.scheduler.spawn(
                &k.bridge,
                "driver_loader",
                file_loader_task as *const () as usize as u64,
                stack_top,
                args_ptr,
                0,
                0,
            );
        } // drop lock
          // file_loader_task(args_ptr);
    }

    // 4.5 Assets (Cursors, Icons)
    let asset_dirs = [
        ("/boot/cursors", cursors_dir_id),
        ("/boot/icons", icons_dir_id),
    ];
    for (path, dir_id) in asset_dirs {
        let entries = iso.read_dir(path).unwrap_or_default();
        for entry in entries {
            if entry.is_dir {
                continue;
            }
            let full_path = alloc::format!("{}/{}", path, entry.name);
            let f_args = FileArgs {
                iso: iso.clone(),
                path: full_path,
                dir_id: Some(dir_id),
                should_spawn: false,
                hhdm,
            };
            let args_box = Box::new(f_args);
            let args_ptr = Box::into_raw(args_box) as u64;

            let mut guard = KERNEL.lock();
            if let Some(k) = guard.as_mut() {
                let layout = Layout::from_size_align(64 * 1024, 16).unwrap();
                let stack_ptr = unsafe { alloc(layout) };
                let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 } - 8;
                k.scheduler.spawn(
                    &k.bridge,
                    "asset_loader",
                    file_loader_task as *const () as usize as u64,
                    stack_top,
                    args_ptr,
                    0,
                    0,
                );
            }
        }
    }

    // 5. Fonts (Metadata Cache)
    if let Some(h) = iso.open("/boot/.fontcache") {
        Bridge.log("loader: Loading .fontcache...\n");
        let mut data = alloc::vec![0u8; h.size as usize];
        iso.read(&h, 0, h.size as usize, &mut data);

        use models::font_cache::FontCache;
        if let Ok(cache) = postcard::from_bytes::<FontCache>(&data) {
            Bridge.log("loader: Parsed FontCache. Registering...\n");

            // Create Catalog Thing
            {
                let mut guard = KERNEL.lock();
                if let Some(k) = guard.as_mut() {
                    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
                    use models::builtins::ids::*;

                    let tb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(THING_FONT_CATALOG_KIND.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&cache).unwrap(),
                    })
                    .unwrap();

                    let cat_id = k.graph.create_thing(THING_FONT_CATALOG_KIND, tb);

                    // Link FontsDir -> Catalog
                    let link = models::link::LinkBody {
                        from: fonts_dir_id,
                        to: cat_id,
                        predicate: THING_HAS_ENTRY_KIND,
                    };
                    let l_tb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(THING_LINK_KIND.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&link).unwrap(),
                    })
                    .unwrap();
                    k.graph.create_thing(THING_LINK_KIND, l_tb);
                }
            }

            // Spawn loaders for minimal fonts
            for entry in &cache.entries {
                // Tactical: Load "Hack" and "Noto Sans" (Regular)
                let lower_fam = entry.family.to_lowercase();
                // Note: Noto Sans usually has weight 400 for Regular.
                let is_hack = lower_fam.contains("hack") && entry.weight == 400 && !entry.italic;
                let is_noto =
                    lower_fam.contains("noto sans") && entry.weight == 400 && !entry.italic;

                if is_hack || is_noto {
                    Bridge.log("loader: Spawning preload for ");
                    Bridge.log(&entry.family);
                    Bridge.log("\n");
                    let path = alloc::format!("{}", entry.path);
                    let f_args = FileArgs {
                        iso: iso.clone(),
                        path,
                        dir_id: Some(fonts_dir_id),
                        should_spawn: false,
                        hhdm,
                    };
                    let args_box = Box::new(f_args);
                    let args_ptr = Box::into_raw(args_box) as u64;

                    let mut guard = KERNEL.lock();
                    if let Some(k) = guard.as_mut() {
                        // Allocate Aligned Stack (64KB)
                        let layout = Layout::from_size_align(64 * 1024, 16).unwrap();
                        let stack_ptr = unsafe { alloc(layout) };
                        let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 } - 8;

                        k.scheduler.spawn(
                            &k.bridge,
                            "font_preload",
                            file_loader_task as *const () as usize as u64,
                            stack_top,
                            args_ptr,
                            0, // Heap Start
                            0, // Heap End
                        );
                    }
                }
            }
        } else {
            Bridge.log("loader: Failed to parse .fontcache\n");
        }
    } else {
        Bridge.log("loader: No .fontcache found.\n");
    }

    Bridge.log("loader: All scan tasks spawned.\n");
    loop {
        x86_64::instructions::hlt();
    }
}

pub extern "C" fn file_loader_task(arg: u64) {
    let args = unsafe { Box::from_raw(arg as *mut FileArgs) };

    // Open and Read (Concurrent IO)
    // NOTE: arg.iso is Arc, so access is efficient.
    // open() calls read_sector_yielding internally.
    if let Some(handle) = args.iso.open(&args.path) {
        Bridge.log("loader: Opened ");
        Bridge.log(&args.path);
        Bridge.log("\n");
        let mut data = alloc::vec![0u8; handle.size as usize];
        args.iso.read(&handle, 0, handle.size as usize, &mut data);
        Bridge.log("loader: Read complete\n");

        // Process (Serialized by Kernel Lock)
        let mut guard = KERNEL.lock();
        if let Some(k) = guard.as_mut() {
            // We reuse process_file.
            // But we need to define it or import.
            // Implementing here.

            let name = args.path.rsplit('/').next().unwrap_or(&args.path);
            process_file(
                k,
                args.dir_id,
                name,
                &data,
                0,
                Some(args.should_spawn),
                args.hhdm,
            );
        }
    } else {
        Bridge.log("loader: Failed to open ");
        Bridge.log(&args.path);
        Bridge.log("\n");
    }
    loop {
        x86_64::instructions::hlt();
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
) {
    Bridge.log("loader: processing file ");
    Bridge.log(name);
    Bridge.log("\n");
    let mtype = classify_bytes(data);
    let role_enum = get_module_role(name, &mtype);

    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
    use models::builtins::ids::*;
    use models::core::fs::FileBody;

    use models::builtins::core_kinds::BootProgramBody;
    use models::core::process::{ProcessBody, ProcessState};

    let file_id = {
        let file = FileBody {
            name: String::from(name),
            size: data.len() as u64,
            lba: 0,
            flags: 1, // Ram-backed by module data
        };
        let f_bytes = postcard::to_allocvec(&file).unwrap();
        let f_tb = ThingBody::from(&TypedBytes {
            type_id: TypeId(THING_FILE_KIND.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: f_bytes,
        })
        .unwrap();
        k.graph.create_thing(THING_FILE_KIND, f_tb)
    };

    if let Some(parent) = parent_dir_id {
        let link = models::link::LinkBody {
            from: parent,
            to: file_id,
            predicate: THING_HAS_ENTRY_KIND,
        };
        let lb = ThingBody::from(&TypedBytes {
            type_id: TypeId(THING_LINK_KIND.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: postcard::to_allocvec(&link).unwrap(),
        })
        .unwrap();
        k.graph.create_thing(THING_LINK_KIND, lb);
    }

    // --- Graph: Create Module Thing ---
    let role_str = match role_enum {
        ModuleRole::App => "app",
        ModuleRole::Driver => "driver",
        ModuleRole::Debug => "debug",
        ModuleRole::Asset => "asset",
        ModuleRole::Ignore => "ignore",
    };

    use models::builtins::core_kinds::ModuleBody;
    use models::builtins::ids::THING_MODULE_KIND;

    // Track physical base if memory is identity or HHDM mapped.
    let base_ptr = data.as_ptr() as u64;
    let base_phys = if base_ptr >= hhdm_u64 {
        base_ptr - hhdm_u64
    } else {
        0
    };

    let mod_body = ModuleBody {
        path: String::from(name),
        size_bytes: data.len() as u64,
        base_phys,
        index: 0,
        role: String::from(role_str),
        mime: String::from("application/octet-stream"),
        kind: String::from(role_str),
        sniff: 0,
        valid: true,
        data: data.to_vec(), // Potential huge allocation
    };

    k.bridge.log(alloc::format!("loader: created ModuleBody for {}, data len={}\n", name, data.len()).as_str());

    k.bridge.log("loader: serializing mod_body...\n");
    let m_bytes = postcard::to_allocvec(&mod_body).unwrap();
    k.bridge.log(alloc::format!("loader: serialized mod_body, size={}\n", m_bytes.len()).as_str());

    k.bridge.log("loader: creating ThingBody...\n");
    let m_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_MODULE_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: m_bytes,
    })
    .unwrap();

    let mod_id = k.graph.create_thing(THING_MODULE_KIND, m_tb);

    use models::builtins::ids::{THING_BACKED_BY_KIND, THING_BOOT_ROOT, THING_HAS_MODULE_KIND};
    let link = models::link::LinkBody {
        from: file_id,
        to: mod_id,
        predicate: THING_BACKED_BY_KIND,
    };
    let lb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&link).unwrap(),
    })
    .unwrap();
    k.graph.create_thing(THING_LINK_KIND, lb);

    // Root -> Module link for discovery
    let root_link = models::link::LinkBody {
        from: THING_BOOT_ROOT,
        to: mod_id,
        predicate: THING_HAS_MODULE_KIND,
    };
    let rl_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&root_link).unwrap(),
    })
    .unwrap();
    k.graph.create_thing(THING_LINK_KIND, rl_tb);

    // Skipped ModuleBody and Font body logic to match main.rs needs?
    // I should probably copy the full logic if I want exact behavior.
    // However, for this task, I'll include the "Spawn App" logic which is critical.

    let should_spawn = if let Some(s) = spawn_override {
        s
    } else {
        match role_enum {
            ModuleRole::App => true,
            ModuleRole::Driver => true,
            ModuleRole::Debug => false, // No smoke check here for now
            _ => false,
        }
    };

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

                let user_virt_base = VirtAddr::new(0x1_0000_0000);
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

                Bridge.log("loader: Mapped User Framebuffer at 0x1_0000_0000\n");
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

            // Spawn
            k.scheduler.spawn(
                &k.bridge,
                name,
                current_app_base + img.entry_point,
                stack_top_virt.as_u64(),
                heap_virt_start,
                heap_virt_start,
                heap_virt_end,
            );

            // Create Process Thing (Simplified)
            let p_body = ProcessBody {
                pid: (k.scheduler.processes.len()) as u64, // Estimate
                name: k
                    .symbols
                    .intern(name)
                    .unwrap_or(models::builtins::symbols::SYM_PROCESS),
                state: ProcessState::Running,
            };
            let tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_PROCESS_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: postcard::to_allocvec(&p_body).unwrap(),
            })
            .unwrap();
            let process_id = k.graph.create_thing(THING_PROCESS_KIND, tb);

            let link = models::link::LinkBody {
                from: THING_BOOT_ROOT,
                to: process_id,
                predicate: models::builtins::ids::THING_SPAWNED_KIND,
            };
            let l_tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_LINK_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: postcard::to_allocvec(&link).unwrap(),
            })
            .unwrap();
            k.graph.create_thing(THING_LINK_KIND, l_tb);

            // Create BootProgram and links (To match main.rs)
            let bp = BootProgramBody {
                name: alloc::string::String::from(name),
                binary: alloc::string::String::from(name),
                priority: 0,
                entry_point: current_app_base + img.entry_point,
            };
            let tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(models::builtins::ids::THING_BOOT_PROGRAM_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: postcard::to_allocvec(&bp).unwrap(),
            })
            .unwrap();
            let prog_id = k
                .graph
                .create_thing(models::builtins::ids::THING_BOOT_PROGRAM_KIND, tb);

            let link = models::link::LinkBody {
                from: process_id,
                to: prog_id,
                predicate: models::builtins::ids::THING_RUNS_KIND,
            };
            let lb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_LINK_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: postcard::to_allocvec(&link).unwrap(),
            })
            .unwrap();
            k.graph.create_thing(THING_LINK_KIND, lb);
        }
    }
}
