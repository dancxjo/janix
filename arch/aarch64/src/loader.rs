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

use crate::paging::{self, PTE_AF, PTE_AP_RW_EL0, PTE_ATTR_DEVICE, PTE_ATTR_NORMAL, PTE_PAGE, PTE_SH_INNER, PTE_UXN, PTE_VALID};

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

fn write_user_bytes(
    raw_addr: u64,
    data: &[u8],
    root_table: u64,
    hhdm_offset: u64,
) {
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
                 7 => rela_addr = val, // DT_RELA
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
             let mut applied = 0;
             
             for chunk in rela_data.chunks(ent_size as usize) {
                 if chunk.len() < 24 { break; }
                 let r_offset = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
                 let r_info = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
                 let r_addend = i64::from_le_bytes(chunk[16..24].try_into().unwrap());
                 let r_type = r_info & 0xFFFF_FFFF;
                 
                 // R_AARCH64_RELATIVE = 1027
                 if r_type == 1027 {
                     let value = load_base.wrapping_add(r_addend as u64);
                     let target_addr = load_base + r_offset;
                     
                     write_user_bytes(target_addr, &value.to_le_bytes(), root_table, hhdm_offset);
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
    
    let hhdm = args.hhdm;
    let fb_phys = args.fb_phys;
    let fb_size = args.fb_size;
    let modules = &args.modules;

    unsafe { Bridge.log("loader: Scanning modules...\n"); }

    for module in modules {
        if module.path.ends_with("loaded.elf") {
             unsafe { Bridge.log("loader: Found loaded.elf in modules!\n"); }
             let phys_start = module.start;
             let size = module.size;
             let virt_start = phys_start; // Already virtual (HHDM)
             
             let slice = unsafe { core::slice::from_raw_parts(virt_start as *const u8, size as usize) };
             
             // Spawn it
             let mut guard = KERNEL.lock();
             if let Some(k) = guard.as_mut() {
                 let name = "loaded";
                 process_file(
                     k,
                     None,
                     name,
                     slice,
                     0,
                     Some(true),
                     hhdm,
                     fb_phys,
                     fb_size
                 );
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
    if spawn_override == Some(true) {
        use kernel::sched::elf::load_elf;
        
        let current_app_base = APP_LOAD_ADDR.fetch_add(0x1000_0000, Ordering::Relaxed);
        
        let root_table = unsafe { paging::create_user_root().expect("OOM Root") };
        unsafe {
             k.bridge.log(alloc::format!("loader: Created User Root at {:#x}\n", root_table).as_str());
        }
        
        let loaded = load_elf(data, current_app_base, |vaddr, segment| {
             let raw_addr = current_app_base + vaddr;
             write_user_bytes(raw_addr, segment, root_table, hhdm_u64);
        });
        
        if let Some(img) = loaded {
             let applied = apply_relative_relocations(data, current_app_base, root_table, hhdm_u64);
             if applied == 0 {
                 unsafe { k.bridge.log("loader: WARN: Applied 0 relocations!\n"); }
             }
             
             let stack_size = 128 * 1024;
             let stack_bottom = current_app_base + 0x0800_0000;
             let stack_top = stack_bottom + stack_size;
             
             {
                 let map_flags = PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_NORMAL;
                 let mut current = stack_bottom;
                 while current < stack_top {
                     let (p, _v) = unsafe { paging::allocate_frame().expect("OOM Stack") };
                     unsafe { paging::map_page_at_root(root_table, p, current, map_flags) };
                     current += 4096;
                 }
                 unsafe { k.bridge.log("loader: Mapped Stack\n"); }
             }
             
             let heap_start = current_app_base + 0x0100_0000;
             let heap_size = 16 * 1024 * 1024;
             let heap_end = heap_start + heap_size;
            
             {
                  let map_flags = PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_NORMAL;
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
                      let map_flags = PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_DEVICE | PTE_UXN;
                      let mut current = 0;
                      while current < fb_size {
                          let phys = fb_phys + current as u64;
                          paging::map_page_at_root(root_table, phys, user_fb + current as u64, map_flags);
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
             
             struct TrampolineArgs {
                 entry: u64,
                 stack: u64,
                 root: u64,
             }
             let t_args = Box::new(TrampolineArgs {
                 entry: current_app_base + img.entry_point,
                 stack: stack_top,
                 root: root_table,
             });
             let t_ptr = Box::into_raw(t_args) as u64;
             
             unsafe {
                 k.bridge.log("loader: computed entry: ");
                 k.bridge.log(alloc::format!("{:#x}", current_app_base + img.entry_point).as_str());
                 k.bridge.log("\n");
             }
             
             k.scheduler.spawn(
                 &k.bridge,
                 name,
                 trampoline as *const () as usize as u64,
                 0,
                 t_ptr,
                 0, 0
             );
        }
    }
}

struct TrampolineArgs {
    entry: u64,
    stack: u64,
    root: u64,
}

pub extern "C" fn trampoline(arg: u64) {
    let args = unsafe { Box::from_raw(arg as *mut TrampolineArgs) };
    unsafe {
       Bridge.log("trampoline: jumping to: ");
       Bridge.log(alloc::format!("{:#x}", args.entry).as_str());
       Bridge.log("\n");

       core::arch::asm!("msr ttbr0_el1, {}", in(reg) args.root);
       core::arch::asm!("isb"); 
       
       bridge_aarch64::cpu::enter_user_mode(args.entry, args.stack, 0);
    }
}
