use alloc::alloc::{alloc, Layout};
use alloc::boxed::Box;
use alloc::vec::Vec;

use abi::boot::{LoadedBootArgs, UserBootBlob};
use kernel::boot::{BootAddrKind, BootBlob, blob_as_slice};
use kernel::userimg::{load_elf_user_image, LoadedImage};
use kernel::bridge::HardwareBridge;
use kernel::Kernel;
use crate::KERNEL;
use bridge_aarch64::Bridge;
use core::sync::atomic::{AtomicU64, Ordering};
use crate::paging::{self, PTE_AF, PTE_AP_RW_EL0, PTE_ATTR_DEVICE, PTE_ATTR_NORMAL, PTE_PAGE, PTE_SH_INNER, PTE_UXN, PTE_VALID};

// --- SHARED STRUCTS ---

pub struct ScanArgs {
    pub bb_info: Option<u64>,
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

pub extern "C" fn scan_boot_fs_task(arg: u64) {
    let args_ptr = arg as *mut ScanArgs;
    let args = unsafe { Box::from_raw(args_ptr) };
    
    let hhdm = args.hhdm;
    let fb_phys = args.fb_phys;
    let fb_size = args.fb_size;
    let modules = &args.modules;

    unsafe { Bridge.log("loader: Scanning modules...\n"); }

    // 1. Convert to BootBlobs
    let mut blobs = Vec::new();
    for m in modules {
        blobs.push(BootBlob {
            path: m.path.clone(),
            start: m.start,
            size: m.size,
            addr_kind: BootAddrKind::Phys,
        });
    }

    // 2. Find loaded.elf
    let loaded_blob = blobs.iter().find(|b| b.path.ends_with("loaded.elf"));

    if let Some(lb) = loaded_blob {
        spawn_loaded(lb, &blobs, hhdm, fb_phys, fb_size);
    } else {
        unsafe { Bridge.log("loader: loaded.elf not found!\n"); }
    }
    
    loop {
        core::hint::spin_loop();
    }
}

fn spawn_loaded(elf_blob: &BootBlob, all_blobs: &[BootBlob], hhdm: u64, fb_phys: u64, fb_size: usize) {
    let elf_data = unsafe { blob_as_slice(elf_blob, hhdm) };
    let current_app_base = APP_LOAD_ADDR.fetch_add(0x1000_0000, Ordering::Relaxed);

    let root_table = unsafe { paging::create_user_root().expect("OOM Root") };

    // Shared Loader
    let loaded_image = load_elf_user_image(elf_data, current_app_base, |vaddr, segment| {
        let raw_addr = current_app_base + vaddr;
        write_user_bytes(raw_addr, segment, root_table, hhdm);
    });

    if let Some(img) = loaded_image {
        // Map Stack
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
        }
        
        // Map Heap
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
        
        // Map Framebuffer
        if fb_size > 0 {
              let user_fb = 0x80_0000_0000;
              let map_flags = PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_DEVICE | PTE_UXN;
              let mut current = 0;
              while current < fb_size {
                  let phys = fb_phys + current as u64;
                  unsafe { paging::map_page_at_root(root_table, phys, user_fb + current as u64, map_flags) };
                  current += 4096;
              }
        }
        
        // Map Blobs
        let mut user_blob_ptr = (img.max_mapped + 0x100000 + 4095) & !4095;
        // Align to 1MB
        user_blob_ptr = (user_blob_ptr + 0xFFFFF) & !0xFFFFF;
        let mut user_blobs = Vec::new();
        
        for blob in all_blobs {
            let blob_data = unsafe { blob_as_slice(blob, hhdm) };
            let blob_len = blob_data.len() as u64;
            let blob_start = user_blob_ptr;
            write_user_bytes(blob_start, blob_data, root_table, hhdm);
            user_blob_ptr += (blob_len + 4095) & !4095;
            
            let path_bytes = blob.path.as_bytes();
            let path_len = path_bytes.len() as u64;
            let path_start = user_blob_ptr;
            write_user_bytes(path_start, path_bytes, root_table, hhdm);
            user_blob_ptr += (path_len + 4095) & !4095;

            user_blobs.push(UserBootBlob {
                start: blob_start,
                size: blob_len,
                path_ptr: path_start,
                path_len,
            });
        }

        // Map UserBlobs array
        let array_len = (user_blobs.len() * core::mem::size_of::<UserBootBlob>()) as u64;
        let array_start = user_blob_ptr;
        let array_bytes = unsafe { core::slice::from_raw_parts(user_blobs.as_ptr() as *const u8, array_len as usize) };
        write_user_bytes(array_start, array_bytes, root_table, hhdm);
        user_blob_ptr += (array_len + 4095) & !4095;

        // Construct Args
        let args = LoadedBootArgs {
            hhdm,
            framebuffer: if fb_size > 0 { Some((fb_phys, fb_size)) } else { None },
            blobs_ptr: array_start,
            blobs_len: user_blobs.len() as u64,
            heap_start,
            heap_size: 16 * 1024 * 1024,
        };

        // Push args to stack
        let args_size = core::mem::size_of::<LoadedBootArgs>() as u64;
        let args_addr = stack_top - args_size;
        let args_bytes = unsafe { core::slice::from_raw_parts(&args as *const _ as *const u8, args_size as usize) };
        write_user_bytes(args_addr, args_bytes, root_table, hhdm);

        // Spawn
        struct TrampolineArgs {
             entry: u64,
             stack: u64,
             root: u64,
             arg0: u64,
        }
        let t_args = Box::new(TrampolineArgs {
             entry: current_app_base + img.entry,
             stack: stack_top, // Original stack top (args are at the very top)
             root: root_table,
             arg0: args_addr,
        });
        let t_ptr = Box::into_raw(t_args) as u64;

        let mut guard = KERNEL.lock();
        if let Some(k) = guard.as_mut() {
             k.scheduler.spawn(
                 &k.bridge,
                 "loaded",
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
    arg0: u64,
}

pub extern "C" fn trampoline(arg: u64) {
    let args = unsafe { Box::from_raw(arg as *mut TrampolineArgs) };
    unsafe {
       core::arch::asm!("msr ttbr0_el1, {}", in(reg) args.root);
       core::arch::asm!("isb"); 
       
       bridge_aarch64::cpu::enter_user_mode(args.entry, args.stack, args.arg0);
    }
}
