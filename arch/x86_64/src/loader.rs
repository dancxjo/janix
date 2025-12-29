use alloc::alloc::{alloc, Layout};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::{ThingId, SymbolId};

use crate::KERNEL;
use bridge_x86_64::Bridge;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::bridge::HardwareBridge;
use kernel::fs::iso9660::{BlockReader, Iso9660Reader};
use kernel::Kernel;
use thing_models::payload::*;
use thing_models::link::LinkBody;
use thing_models::Thing;
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

static APP_LOAD_ADDR: AtomicU64 = AtomicU64::new(0x40_0000_0000);

struct HeapFrameAllocator {
    hhdm_offset: VirtAddr,
}

unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        use alloc::alloc::{alloc_zeroed, Layout};
        let layout = Layout::from_size_align(4096, 4096).ok()?;
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

        if phys_frame.is_none() {
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
    use x86_64::structures::paging::{mapper::TranslateError, mapper::TranslateResult, Mapper, Page, PageTableFlags, Size2MiB};

    let start = VirtAddr::new(raw_addr);
    let end = VirtAddr::new(raw_addr + data.len() as u64);
    let start_page = Page::<Size4KiB>::containing_address(start);
    let end_page = Page::<Size4KiB>::containing_address(end - 1u64);

    for page in Page::range_inclusive(start_page, end_page) {
        let page_start_virt = page.start_address();
        let mut needs_alloc = true;

        match mapper.translate_page(page) {
            Ok(_) => {
                let new_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
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
                    }
                }
            }
            Err(TranslateError::PageNotMapped) => {}
            Err(_) => { return; }
        }

        if needs_alloc {
            if let Some(frame) = frame_allocator.allocate_frame() {
                let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
                unsafe {
                    if let Ok(map_to) = mapper.map_to(page, frame, flags, frame_allocator) {
                        map_to.flush();
                    }
                }
            } else {
                return;
            }
        }

        let overlap_start = core::cmp::max(page_start_virt, start);
        let overlap_end = core::cmp::min(page_start_virt + 4096u64, end);
        if overlap_end <= overlap_start { continue; }

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

    let dyn_ph = elf.program_iter().find(|ph| ph.get_type().map(|t| t == Type::Dynamic).unwrap_or(false));
    if let Some(dyn_ph) = dyn_ph {
        let dyn_offset = dyn_ph.offset();
        let dyn_size = dyn_ph.file_size();
        let dyn_entries = &elf_data[dyn_offset as usize..(dyn_offset + dyn_size) as usize];
        let mut rela_addr = 0u64;
        let mut rela_sz = 0u64;
        let mut rela_ent = 0u64;

        for chunk in dyn_entries.chunks(16) {
             if chunk.len() < 16 { break; }
            let tag = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
            let val = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
            match tag {
                7 => rela_addr = val,
                8 => rela_sz = val,
                9 => rela_ent = val,
                0 => break,
                _ => {}
            }
        }

        if rela_addr == 0 || rela_sz == 0 { return 0; }

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
                 if chunk.len() < 24 { break; }
                let r_offset = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
                let r_info = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
                let r_addend = i64::from_le_bytes(chunk[16..24].try_into().unwrap());
                let r_type = r_info & 0xFFFF_FFFF;

                if r_type == 8 {
                    let value = load_base.wrapping_add(r_addend as u64);
                    write_user_bytes(load_base + r_offset, &value.to_le_bytes(), mapper, frame_allocator, hhdm_offset);
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
    let args = unsafe { Box::from_raw(args_ptr) };
    let base = args.base;
    let port = args.port;
    let hhdm = args.hhdm;

    let reader = move |lba, buf: &mut [u8]| unsafe {
        use alloc::alloc::{alloc, dealloc, Layout};
        let layout = Layout::from_size_align(2048, 2048).unwrap();
        let ptr = alloc(layout);
        if ptr.is_null() { return false; }
        let bounce = core::slice::from_raw_parts_mut(ptr, 2048);
        let res = kernel::drivers::ahci::read_sector_yielding(
            &bridge_x86_64::Bridge, base, port, lba, bounce, hhdm, || x86_64::instructions::hlt(),
        );
        if res {
            let len = core::cmp::min(buf.len(), 2048);
            core::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), len);
        }
        dealloc(ptr, layout);
        res
    };

    let boxed_reader: Box<dyn BlockReader + Send + Sync> = Box::new(reader);
    let iso = match Iso9660Reader::new(boxed_reader) {
        Some(i) => Arc::new(i),
        None => { Bridge.log("loader: Failed to init ISO reader\n"); return; }
    };

    Bridge.log("loader: ISO Reader Ready. Scanning...\n");

    let (apps_dir_id, _drivers_dir_id, fonts_dir_id, cursors_dir_id, icons_dir_id) = {
        let mut guard = KERNEL.lock();
        if let Some(k) = guard.as_mut() {
            // New Payload Usage
            use models::builtins::ids::*;

            // 1. Mount "/boot"
            let m_payload = Mount { path: String::from("/boot"), readonly: true };
            let m_bytes = postcard::to_allocvec(&m_payload).unwrap();
            let m_id = k.graph.create_thing(Mount::KIND, m_bytes);

            // Link BootRoot -> Mount
            let l_root = LinkBody { from: THING_BOOT_ROOT, to: m_id, predicate: HAS_MOUNT };
            let l_root_bytes = postcard::to_allocvec(&l_root).unwrap();
            k.graph.create_thing(LinkBody::KIND, l_root_bytes);

            // 2. Root Dir
            let d_payload = Dir { name: String::from("/boot") };
            let d_bytes = postcard::to_allocvec(&d_payload).unwrap();
            let root_id = k.graph.create_thing(Dir::KIND, d_bytes);

            // Link Mount -> Root Dir
            let l_mnt = LinkBody { from: m_id, to: root_id, predicate: MOUNTS };
            let l_mnt_bytes = postcard::to_allocvec(&l_mnt).unwrap();
            k.graph.create_thing(LinkBody::KIND, l_mnt_bytes);

            let mut make_dir = |name: &str, parent: ThingId| {
                let payload = Dir { name: String::from(name) };
                let bytes = postcard::to_allocvec(&payload).unwrap();
                let did = k.graph.create_thing(Dir::KIND, bytes);

                let l = LinkBody { from: parent, to: did, predicate: HAS_ENTRY };
                let l_bytes = postcard::to_allocvec(&l).unwrap();
                k.graph.create_thing(LinkBody::KIND, l_bytes);
                did
            };

            let apps = make_dir("apps", root_id);
            let drivers = make_dir("drivers", root_id);
            let fonts = make_dir("fonts", root_id);
            let cursors = make_dir("cursors", root_id);
            let icons = make_dir("icons", root_id);

            Bridge.log("loader: Mounts created\n");
            (apps, drivers, fonts, cursors, icons)
        } else { return; }
    };

    let apps_entries = iso.read_dir("/boot/apps").unwrap_or_default();
    Bridge.log("loader: Spawning app loaders...\n");

    for entry in apps_entries {
        if entry.is_dir { continue; }
        let path = alloc::format!("/boot/apps/{}", entry.name);
        let f_args = FileArgs { iso: iso.clone(), path, dir_id: Some(apps_dir_id), should_spawn: entry.name.ends_with(".elf"), hhdm };
        spawn_file_loader(f_args);
    }

    let drv_entries = iso.read_dir("/boot/drivers").unwrap_or_default();
    Bridge.log("loader: Spawning driver loaders...\n");
    for entry in drv_entries {
        if entry.is_dir { continue; }
         let path = alloc::format!("/boot/drivers/{}", entry.name);
         let f_args = FileArgs { iso: iso.clone(), path, dir_id: Some(_drivers_dir_id), should_spawn: false, hhdm };
         spawn_file_loader(f_args);
    }

    let asset_dirs = [("/boot/cursors", cursors_dir_id), ("/boot/icons", icons_dir_id)];
    for (path, dir_id) in asset_dirs {
        let entries = iso.read_dir(path).unwrap_or_default();
        for entry in entries {
            if entry.is_dir { continue; }
            let full_path = alloc::format!("{}/{}", path, entry.name);
            let f_args = FileArgs { iso: iso.clone(), path: full_path, dir_id: Some(dir_id), should_spawn: false, hhdm };
            spawn_file_loader(f_args);
        }
    }

    Bridge.log("loader: All scan tasks spawned.\n");
    loop { x86_64::instructions::hlt(); }
}

fn spawn_file_loader(args: FileArgs) {
    let args_box = Box::new(args);
    let args_ptr = Box::into_raw(args_box) as u64;
    let mut guard = KERNEL.lock();
    if let Some(k) = guard.as_mut() {
        let layout = Layout::from_size_align(64 * 1024, 16).unwrap();
        let stack_ptr = unsafe { alloc(layout) };
        let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 } - 8;
        k.scheduler.spawn(&k.bridge, "file_loader", file_loader_task as *const () as usize as u64, stack_top, args_ptr, 0, 0);
    }
}

pub extern "C" fn file_loader_task(arg: u64) {
    let args = unsafe { Box::from_raw(arg as *mut FileArgs) };
    if let Some(handle) = args.iso.open(&args.path) {
        Bridge.log("loader: Opened "); Bridge.log(&args.path); Bridge.log("\n");
        let mut data = alloc::vec![0u8; handle.size as usize];
        args.iso.read(&handle, 0, handle.size as usize, &mut data);
        Bridge.log("loader: Read complete\n");

        let mut guard = KERNEL.lock();
        if let Some(k) = guard.as_mut() {
            let name = args.path.rsplit('/').next().unwrap_or(&args.path);
            process_file(k, args.dir_id, name, &data, 0, Some(args.should_spawn), args.hhdm);
        }
    }
    loop { x86_64::instructions::hlt(); }
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
    let mtype = classify_bytes(data);
    let role_enum = get_module_role(name, &mtype);

    let file_payload = File { name: String::from(name), size: data.len() as u64 };
    let file_bytes = postcard::to_allocvec(&file_payload).unwrap();
    let file_id = k.graph.create_thing(File::KIND, file_bytes);

    if let Some(parent) = parent_dir_id {
        let link = LinkBody { from: parent, to: file_id, predicate: HAS_ENTRY };
        let l_bytes = postcard::to_allocvec(&link).unwrap();
        k.graph.create_thing(LinkBody::KIND, l_bytes);
    }

    // Use create_from_slice to copy bytes into store (solves ownership & read)
    let backing_id = k.bytespaces.create_from_slice(data, 1).unwrap();

    let bs_payload = ByteSpace { len: data.len() as u64, flags: 1, backing: abi::symbols::sym("module") };
    let bs_bytes = postcard::to_allocvec(&bs_payload).unwrap();
    let bs_thing_id = k.graph.create_thing(ByteSpace::KIND, bs_bytes);
    k.bytespaces.bind_thing(bs_thing_id, backing_id);

    unsafe {
        let s = alloc::format!("BYTESPACE: created id={:?} len={} backing=module\n", bs_thing_id, data.len());
        Bridge.log(&s);
    }

    let meta_id = match mtype {
        ModuleType::Elf | ModuleType::Unknown | ModuleType::Other(_) => {
            let m_payload = Module {
                name: abi::symbols::sym(name),
                image_len: data.len() as u64,
                image_kind: abi::symbols::sym("elf"),
                entry: 0
            };
            let m_bytes = postcard::to_allocvec(&m_payload).unwrap();
            k.graph.create_thing(Module::KIND, m_bytes)
        },
        ModuleType::Bmp | ModuleType::Png => {
             let b_payload = Bitmap { width: 0, height: 0, format: abi::symbols::sym("unknown") };
             let b_bytes = postcard::to_allocvec(&b_payload).unwrap();
             k.graph.create_thing(Bitmap::KIND, b_bytes)
        },
        ModuleType::Ttf | ModuleType::Otf | ModuleType::Woff | ModuleType::Woff2 | ModuleType::Psf1 | ModuleType::Psf2 => {
             let f_payload = Font { family: abi::symbols::sym("unknown"), style: abi::symbols::sym("regular"), weight: 400 };
             let f_bytes = postcard::to_allocvec(&f_payload).unwrap();
             k.graph.create_thing(Font::KIND, f_bytes)
        }
    };

    let predicate = match mtype {
         ModuleType::Elf | ModuleType::Unknown | ModuleType::Other(_) => HAS_BYTES,
         ModuleType::Bmp | ModuleType::Png => HAS_PIXELS,
         _ => HAS_FONT_DATA,
    };
    let l = LinkBody { from: meta_id, to: bs_thing_id, predicate };
    let l_bytes = postcard::to_allocvec(&l).unwrap();
    k.graph.create_thing(LinkBody::KIND, l_bytes);

    let l_fm = LinkBody { from: file_id, to: meta_id, predicate: HAS_MODULE };
    let l_fm_bytes = postcard::to_allocvec(&l_fm).unwrap();
    k.graph.create_thing(LinkBody::KIND, l_fm_bytes);

    let should_spawn = if let Some(s) = spawn_override { s } else {
         match role_enum {
             ModuleRole::App | ModuleRole::Driver => true,
             _ => false,
         }
    };

    if should_spawn && matches!(mtype, ModuleType::Elf) {
        use kernel::sched::elf::load_elf;
        use x86_64::registers::control::Cr3;

        let current_app_base = APP_LOAD_ADDR.fetch_add(0x1000_0000, Ordering::Relaxed);
        let hhdm_offset = VirtAddr::new(hhdm_u64);
        let mut frame_allocator = HeapFrameAllocator { hhdm_offset };

        let mut mapper = unsafe {
             let (l4_frame, _) = Cr3::read();
             let phys = l4_frame.start_address();
             let virt = hhdm_offset + phys.as_u64();
             let page_table_ptr = virt.as_mut_ptr();
             OffsetPageTable::new(&mut *page_table_ptr, hhdm_offset)
        };

        Bridge.log("loader: calling load_elf\n");
        let loaded = load_elf(data, current_app_base, |vaddr, segment| {
            let raw_addr = current_app_base + vaddr;
            write_user_bytes(raw_addr, segment, &mut mapper, &mut frame_allocator, hhdm_offset);
        });

        if let Some(img) = loaded {
             apply_relative_relocations(data, current_app_base, &mut mapper, &mut frame_allocator, hhdm_offset);

             let stack_bottom = current_app_base + 0x0800_0000;
             let stack_size = 131072;
             let stack_top = stack_bottom + stack_size;
             for i in 0..(stack_size/4096) {
                 let page = Page::<Size4KiB>::containing_address(VirtAddr::new(stack_bottom + i*4096));
                 if let Some(frame) = frame_allocator.allocate_frame() {
                      unsafe { mapper.map_to(page, frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE, &mut frame_allocator).unwrap().flush(); }
                 }
             }

             let heap_start = current_app_base + 0x0100_0000;

             k.scheduler.spawn(
                &k.bridge,
                name,
                current_app_base + img.entry_point,
                stack_top - 8,
                heap_start,
                heap_start,
                heap_start + 128 * 1024 * 1024,
            );
            
            let p_payload = Process { pid: k.scheduler.processes.len() as u64, name: abi::symbols::sym(name), state: 0 };
            let p_bytes = postcard::to_allocvec(&p_payload).unwrap();
            let pid = k.graph.create_thing(Process::KIND, p_bytes);

            let bp_payload = BootProgram { name: String::from(name), entry: current_app_base + img.entry_point };
            let bp_bytes = postcard::to_allocvec(&bp_payload).unwrap();
            let bp_id = k.graph.create_thing(BootProgram::KIND, bp_bytes);

            let l = LinkBody { from: pid, to: bp_id, predicate: RUNS };
            let l_bytes = postcard::to_allocvec(&l).unwrap();
            k.graph.create_thing(LinkBody::KIND, l_bytes);

            let l_spawn = LinkBody { from: THING_BOOT_ROOT, to: pid, predicate: SPAWNED };
            let l_s_bytes = postcard::to_allocvec(&l_spawn).unwrap();
            k.graph.create_thing(LinkBody::KIND, l_s_bytes);
        }
    }
}
