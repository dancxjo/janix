use alloc::alloc::{alloc, dealloc, Layout};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::boot::{LoadedBootArgs, UserBootBlob};
use kernel::boot::{BootAddrKind, BootBlob, blob_as_slice};
use kernel::userimg::{load_elf_user_image, LoadedImage};
use kernel::bridge::HardwareBridge;
use kernel::fs::iso9660::{BlockReader, Iso9660Reader};
use kernel::Kernel;
use crate::KERNEL;
use bridge_x86_64::Bridge;
use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size4KiB, Translate,
};
use x86_64::{PhysAddr, VirtAddr};

pub struct ScanArgs {
    pub base: u64,
    pub port: usize,
    pub hhdm: u64,
}

static APP_LOAD_ADDR: AtomicU64 = AtomicU64::new(0x40_0000_0000);

struct HeapFrameAllocator {
    hhdm_offset: VirtAddr,
}

unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let layout = Layout::from_size_align(4096, 4096).ok()?;
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return None;
        }

        use x86_64::registers::control::Cr3;
        let (l4_frame, _) = Cr3::read();
        let phys_l4 = l4_frame.start_address();
        let raw_virt_l4 = self.hhdm_offset.as_u64().wrapping_add(phys_l4.as_u64());
        let virt_l4 = VirtAddr::new(raw_virt_l4);
        let page_table_ptr = virt_l4.as_mut_ptr();
        let mapper = unsafe { OffsetPageTable::new(&mut *page_table_ptr, self.hhdm_offset) };

        let virt_addr = VirtAddr::new(ptr as u64);
        mapper.translate_addr(virt_addr).map(|phys| PhysFrame::containing_address(phys))
    }
}

pub extern "C" fn scan_boot_fs_task(arg: u64) {
    let args_ptr = arg as *mut ScanArgs;
    let args = unsafe { Box::from_raw(args_ptr) };
    let base = args.base;
    let port = args.port;
    let hhdm = args.hhdm;

    // ISO Reader Setup
    let reader = move |lba, buf: &mut [u8]| unsafe {
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

    let mut blobs = Vec::new();
    let dirs = ["/boot/apps", "/boot/drivers", "/boot/fonts", "/boot/cursors", "/boot/icons"];

    for dir in dirs {
        if let Some(entries) = iso.read_dir(dir) {
            for entry in entries {
                if !entry.is_dir {
                    let path = alloc::format!("{}/{}", dir, entry.name);
                    if let Some(handle) = iso.open(&path) {
                        let size = handle.size as usize;
                        let layout = Layout::from_size_align(size, 4096).unwrap_or(Layout::from_size_align(4096, 4096).unwrap());
                        let ptr = unsafe { alloc(layout) };
                        if !ptr.is_null() {
                            let buf = unsafe { core::slice::from_raw_parts_mut(ptr, size) };
                            iso.read(&handle, 0, size, buf);

                            blobs.push(BootBlob {
                                path,
                                start: ptr as u64,
                                size: size as u64,
                                addr_kind: BootAddrKind::Virt,
                            });
                        }
                    }
                }
            }
        }
    }

    Bridge.log(alloc::format!("loader: Scanned {} blobs\n", blobs.len()).as_str());

    let loaded_blob = blobs.iter().find(|b| b.path.ends_with("/loaded.elf") || b.path == "loaded.elf");

    if let Some(lb) = loaded_blob {
        spawn_loaded(lb, &blobs, hhdm);
    } else {
        Bridge.log("loader: loaded.elf not found!\n");
    }

    loop { x86_64::instructions::hlt(); }
}

fn spawn_loaded(elf_blob: &BootBlob, all_blobs: &[BootBlob], hhdm: u64) {
    use x86_64::registers::control::Cr3;
    let elf_data = unsafe { blob_as_slice(elf_blob, hhdm) };
    let current_app_base = APP_LOAD_ADDR.fetch_add(0x1000_0000, Ordering::Relaxed);
    let hhdm_offset = VirtAddr::new(hhdm);
    let mut frame_allocator = HeapFrameAllocator { hhdm_offset };

    let mut mapper = unsafe {
         let (l4_frame, _) = Cr3::read();
         let phys = l4_frame.start_address();
         let virt = hhdm_offset + phys.as_u64();
         let page_table_ptr = virt.as_mut_ptr();
         OffsetPageTable::new(&mut *page_table_ptr, hhdm_offset)
    };

    let loaded_image = load_elf_user_image(elf_data, current_app_base, |vaddr, segment| {
        let raw_addr = current_app_base + vaddr;
        let start = VirtAddr::new(raw_addr);
        let end = VirtAddr::new(raw_addr + segment.len() as u64);
        let start_page = Page::<Size4KiB>::containing_address(start);
        let end_page = Page::<Size4KiB>::containing_address(end - 1u64);

        for page in Page::range_inclusive(start_page, end_page) {
            if mapper.translate_page(page).is_err() {
                if let Some(frame) = frame_allocator.allocate_frame() {
                    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
                    unsafe { mapper.map_to(page, frame, flags, &mut frame_allocator).unwrap().flush(); }
                }
            }

            if let Some(phys) = mapper.translate_page(page) {
                let frame_virt = hhdm_offset + phys.start_address().as_u64();
                let page_start_virt = page.start_address();

                let overlap_start = core::cmp::max(page_start_virt, start);
                let overlap_end = core::cmp::min(page_start_virt + 4096u64, end);
                if overlap_end > overlap_start {
                    let copy_len = overlap_end - overlap_start;
                    let seg_offset = overlap_start - start;
                    let page_offset = overlap_start - page_start_virt;

                    let src_ptr = unsafe { segment.as_ptr().add(seg_offset as usize) };
                    let dest_ptr = unsafe { (frame_virt.as_mut_ptr::<u8>()).add(page_offset as usize) };
                    unsafe { core::ptr::copy_nonoverlapping(src_ptr, dest_ptr, copy_len as usize); }
                }
            }
        }
    });

    if let Some(img) = loaded_image {
        let mut user_blob_ptr = (img.max_mapped + 0x100000 + 4095) & !4095;
        user_blob_ptr = (user_blob_ptr + 0xFFFFF) & !0xFFFFF;

        let mut user_blobs = Vec::new();

        for blob in all_blobs {
            let blob_data = unsafe { blob_as_slice(blob, hhdm) };
            let blob_len = blob_data.len() as u64;
            let blob_start = user_blob_ptr;

            let pages = (blob_len + 4095) / 4096;
            for i in 0..pages {
                let page = Page::<Size4KiB>::containing_address(VirtAddr::new(blob_start + i * 4096));
                if let Some(frame) = frame_allocator.allocate_frame() {
                     let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
                     unsafe { mapper.map_to(page, frame, flags, &mut frame_allocator).unwrap().flush(); }

                     let phys = frame.start_address();
                     let frame_virt = hhdm_offset + phys.as_u64();
                     let offset = i * 4096;
                     let len = core::cmp::min(4096, blob_len - offset);
                     let src = unsafe { blob_data.as_ptr().add(offset as usize) };
                     let dst = frame_virt.as_mut_ptr::<u8>();
                     unsafe { core::ptr::copy_nonoverlapping(src, dst, len as usize); }
                }
            }
            user_blob_ptr += (blob_len + 4095) & !4095;

            let path_bytes = blob.path.as_bytes();
            let path_len = path_bytes.len() as u64;
            let path_start = user_blob_ptr;
            let path_pages = (path_len + 4095) / 4096;
             for i in 0..path_pages {
                let page = Page::<Size4KiB>::containing_address(VirtAddr::new(path_start + i * 4096));
                if let Some(frame) = frame_allocator.allocate_frame() {
                     let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
                     unsafe { mapper.map_to(page, frame, flags, &mut frame_allocator).unwrap().flush(); }

                     let phys = frame.start_address();
                     let frame_virt = hhdm_offset + phys.as_u64();
                     let offset = i * 4096;
                     let len = core::cmp::min(4096, path_len - offset);
                     let src = unsafe { path_bytes.as_ptr().add(offset as usize) };
                     let dst = frame_virt.as_mut_ptr::<u8>();
                     unsafe { core::ptr::copy_nonoverlapping(src, dst, len as usize); }
                }
            }
            user_blob_ptr += (path_len + 4095) & !4095;

            user_blobs.push(UserBootBlob {
                start: blob_start,
                size: blob_len,
                path_ptr: path_start,
                path_len,
            });
        }

        let array_len = (user_blobs.len() * core::mem::size_of::<UserBootBlob>()) as u64;
        let array_start = user_blob_ptr;
        let array_pages = (array_len + 4095) / 4096;
        for i in 0..array_pages {
             let page = Page::<Size4KiB>::containing_address(VirtAddr::new(array_start + i * 4096));
                if let Some(frame) = frame_allocator.allocate_frame() {
                     let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
                     unsafe { mapper.map_to(page, frame, flags, &mut frame_allocator).unwrap().flush(); }

                     let phys = frame.start_address();
                     let frame_virt = hhdm_offset + phys.as_u64();
                     let offset = i * 4096;
                     let len = core::cmp::min(4096, array_len - offset);
                     let src_slice = unsafe {
                         core::slice::from_raw_parts(user_blobs.as_ptr() as *const u8, array_len as usize)
                     };
                     let src = unsafe { src_slice.as_ptr().add(offset as usize) };
                     let dst = frame_virt.as_mut_ptr::<u8>();
                     unsafe { core::ptr::copy_nonoverlapping(src, dst, len as usize); }
                }
        }
        user_blob_ptr += (array_len + 4095) & !4095;

        let stack_bottom = user_blob_ptr + 0x10000;
        let stack_size = 256 * 1024;
        let stack_top = stack_bottom + stack_size;
        for i in 0..(stack_size/4096) {
             let page = Page::<Size4KiB>::containing_address(VirtAddr::new(stack_bottom + i*4096));
             if let Some(frame) = frame_allocator.allocate_frame() {
                  unsafe { mapper.map_to(page, frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE, &mut frame_allocator).unwrap().flush(); }
             }
        }

        let heap_start = stack_top + 0x10000;

        let fb_info = unsafe { crate::FRAMEBUFFER_INFO };
        let args = LoadedBootArgs {
            hhdm,
            framebuffer: fb_info,
            blobs_ptr: array_start,
            blobs_len: user_blobs.len() as u64,
            heap_start,
            heap_size: 128 * 1024 * 1024,
        };

        let args_size = core::mem::size_of::<LoadedBootArgs>() as u64;
        let args_addr = stack_top - args_size;
        let page = Page::<Size4KiB>::containing_address(VirtAddr::new(args_addr));
        if let Some(phys) = mapper.translate_page(page) {
             let frame_virt = hhdm_offset + phys.start_address().as_u64();
             let page_offset = args_addr % 4096;
             let dst = unsafe { frame_virt.as_mut_ptr::<u8>().add(page_offset as usize) };
             let src = &args as *const _ as *const u8;
             unsafe { core::ptr::copy_nonoverlapping(src, dst, args_size as usize); }
        }

        let mut guard = KERNEL.lock();
        if let Some(k) = guard.as_mut() {
            k.scheduler.spawn(
                &k.bridge,
                "loaded",
                current_app_base + img.entry,
                args_addr,
                heap_start,
                heap_start,
                heap_start + 128 * 1024 * 1024,
            );
        }
    } else {
        Bridge.log("loader: Failed to load loaded.elf user image\n");
    }
}
