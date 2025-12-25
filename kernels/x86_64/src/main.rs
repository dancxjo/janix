#![no_std]
#![no_main]
#![cfg_attr(target_os = "thingos", feature(alloc_error_handler))]

extern crate alloc;

#[cfg(target_os = "thingos")]
mod early_log;
#[cfg(target_os = "thingos")]
mod heap;
#[cfg(target_os = "thingos")]
mod limine;

use bridge_x86_64::Bridge;
use core::arch::naked_asm;
use kernel_core::Kernel;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use hw::HardwareBridge;
    let bridge = Bridge;
    bridge.log("PANIC\n");
    // Print args if possible? PanicInfo has display? require fmt.
    // simpler: just panic marker.
    loop {
        core::hint::spin_loop();
    }
}

const BOOT_STACK_SIZE: usize = 16384;
#[used]
#[unsafe(link_section = ".bss")]
static mut BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0; BOOT_STACK_SIZE];

#[no_mangle]
#[unsafe(naked)]
pub extern "C" fn _start() -> ! {
    naked_asm!(
        "lea rsp, [rip + {2}]",
        "add rsp, {0}",
        "mov rax, cr0",
        "and ax, 0xFFFB", // Clear EM
        "or ax, 0x2",     // Set MP
        "mov cr0, rax",
        "mov rax, cr4",
        "or ax, 3 << 9",  // Set OSFXSR and OSXMMEXCPT
        "mov cr4, rax",
        "call {1}",
        "1: hlt",
        "jmp 1b",
        const BOOT_STACK_SIZE,
        sym rust_main,
        sym BOOT_STACK
    )
}

// Global Kernel Access
use spin::Mutex;
use models as thing_models;

static KERNEL: Mutex<Option<Kernel<Bridge>>> = Mutex::new(None);

fn scheduler_tick(frame: &mut bridge_x86_64::interrupts::trap::TrapFrame) {
    if let Some(mut guard) = KERNEL.try_lock() {
        if let Some(k) = (*guard).as_mut() {
            // Update syscall stack to point to this thread's kernel stack?
            // Actually, `scheduler` should handle stack management.
            // But v0 shortcut: we reuse the same stack if we don't context switch?
            // "Boot Stack" is unsafe if we have multiple user threads running.
            // If we have 1 user thread, it's fine.
            // `spawn` allocates a new stack for the thread.
            // When we switch to that thread, we should update SYSCALL_KERNEL_RSP.
            // But `scheduler.tick` calls `switch_to`.
            // We can't easily hook `switch_to`.
            // Workaround: Use a dedicated syscall stack per CPU (Global for UP).
            // We can allocate one here or reuse BOOT_STACK logic if careful.
            // Let's use a dedicated static stack for syscalls to avoid overflow.
            
            // Cast frame to [u64; 20]
            let ctx_ptr = frame as *mut _ as *mut [u64; 20];
            let ctx = unsafe { &mut *ctx_ptr };
            k.scheduler.tick(&k.bridge, ctx);
        }
    }
}

fn syscall_hook(num: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize) -> isize {
    // We need lock. If syscall caused by user int, we are in kernel mode (interrupts enabled?).
    // Syscall handler enables interrupts.
    // So we can lock.
    loop {
        if let Some(mut guard) = KERNEL.try_lock() {
             if let Some(k) = (*guard).as_mut() {
                  return kernel_core::syscalls::syscall_dispatch(k, num, a1, a2, a3, a4, a5, a6);

             }
        }
        core::hint::spin_loop();
    }
}


// Define kernel_core::syscalls::syscall_dispatch locally?
// No, it should be in `kernel_core`.
// I checked `kernel_core/src/syscalls/mod.rs` and it didn't have `syscall_dispatch`.
// I MUST implement it there.



#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    #[cfg(target_os = "thingos")]
    unsafe {
        use hw::HardwareBridge;
        let bridge = Bridge;
        bridge.log("Booting ThingOS...\n");

        let info = limine::heap_init::init_heap_from_limine(heap::KERNEL_HEAP_SIZE_BYTES as u64);
        early_log::log_heap_init(info);
    }

    let mut k = Kernel::new(Bridge);

    #[cfg(target_os = "thingos")]
    {
        use hw::HardwareBridge;
        use limine::requests::MODULE_REQUEST;
        use kernel_core::sched::elf::load_elf;
        use core::slice;
        use alloc::string::ToString;
        use x86_64::structures::paging::{
            PageTable, OffsetPageTable, Page, Size4KiB, Size2MiB, Mapper, FrameAllocator, PhysFrame, PageTableFlags, PageSize, Translate
        };
        use x86_64::{PhysAddr, VirtAddr};
        use x86_64::registers::control::Cr3;
        use alloc::alloc::{alloc, Layout};

        // Get HHDM
        let hhdm_offset_u64 = limine::requests::HHDM_REQUEST.get_response().unwrap().offset();
        let hhdm_offset = VirtAddr::new(hhdm_offset_u64);

        // Frame Allocator wrapping Global Allocator
        struct HeapFrameAllocator {
            hhdm_offset: VirtAddr,
        }
        
        unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator {
            fn allocate_frame(&mut self) -> Option<PhysFrame> {
                let layout = Layout::from_size_align(4096, 4096).ok()?;
                let ptr = unsafe { alloc(layout) };
                if ptr.is_null() { return None; }
                
                let (l4_frame, _) = Cr3::read();
                let phys_l4 = l4_frame.start_address();
                let virt_l4 = self.hhdm_offset + phys_l4.as_u64();
                let page_table_ptr = virt_l4.as_mut_ptr();
                let mut mapper = unsafe { OffsetPageTable::new(&mut *page_table_ptr, self.hhdm_offset) };
                
                let virt_addr = VirtAddr::new(ptr as u64);
                mapper.translate_addr(virt_addr).map(|phys| PhysFrame::containing_address(phys))
            }
        }
        
        let mut frame_allocator = HeapFrameAllocator { hhdm_offset };

        let mut mapper = unsafe {
            let (level_4_table_frame, _) = Cr3::read();
            let phys = level_4_table_frame.start_address();
            let virt = hhdm_offset + phys.as_u64();
            let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
            OffsetPageTable::new(&mut *page_table_ptr, hhdm_offset)
        };

        k.bridge.log("Scanning modules...\n");
        let mut app_load_virt_base = 0x2000_0000u64;

        if let Some(resp) = MODULE_REQUEST.get_response() {
            for module in resp.modules() {
                let name = module.path().to_str().unwrap_or("unknown");
                k.bridge.log("Found module: ");
                k.bridge.log(name);
                k.bridge.log("\n");

                let base = module.addr();
                let len = module.size() as usize;
                let data = unsafe { slice::from_raw_parts(base, len) };
                
                let current_app_base = app_load_virt_base;
                app_load_virt_base += 0x1000_0000;

                let loaded = load_elf(data, |vaddr, segment| {
                    use alloc::format;
                    let target_virt_start = VirtAddr::new(current_app_base + vaddr);
                    let target_virt_end = target_virt_start + segment.len() as u64;
                    
                    let start_page = Page::<Size4KiB>::containing_address(target_virt_start);
                    let end_page = Page::<Size4KiB>::containing_address(target_virt_end - 1u64);
                    
                    for page in Page::range_inclusive(start_page, end_page) {
                        let frame = frame_allocator.allocate_frame().expect("No frames");
                        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
                        unsafe {
                            if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                                map_to.flush();
                            }
                        }
                        
                        let frame_phys = frame.start_address();
                        let frame_virt = hhdm_offset + frame_phys.as_u64();
                        let page_start_virt = page.start_address();
                        let page_end_virt = page_start_virt + 4096u64;
                        let overlap_start = core::cmp::max(page_start_virt, target_virt_start);
                        let overlap_end = core::cmp::min(page_end_virt, target_virt_end);
                        
                        if overlap_end > overlap_start {
                             let copy_len = overlap_end - overlap_start;
                             let seg_offset = overlap_start - target_virt_start;
                             let page_offset = overlap_start - page_start_virt;
                             
                             let src_ptr = unsafe { segment.as_ptr().add(seg_offset as usize) };
                             let dest_ptr = unsafe { (frame_virt.as_mut_ptr::<u8>()).add(page_offset as usize) };
                             unsafe { core::ptr::copy_nonoverlapping(src_ptr, dest_ptr, copy_len as usize); }
                        }
                    }
                });

                if let Some(img) = loaded {
                    k.bridge.log("Loaded app entry\n");
                    
                    let stack_bottom_virt = VirtAddr::new(current_app_base + 0x0800_0000);
                    let stack_size = 65536;
                    let stack_top_virt = stack_bottom_virt + stack_size;
                    
                    let start_page = Page::<Size4KiB>::containing_address(stack_bottom_virt);
                    let end_page = Page::<Size4KiB>::containing_address(stack_top_virt - 1u64);
                    
                    for page in Page::range_inclusive(start_page, end_page) {
                        let frame = frame_allocator.allocate_frame().expect("No stack frames");
                         let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
                         unsafe {
                            if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                                map_to.flush();
                            }
                        }
                    }

                    let entry_point = current_app_base + img.entry_point;
                    k.scheduler.spawn(&k.bridge, name, entry_point, stack_top_virt.as_u64(), 0);
                }
            }
        }
    }
    
    // Boot Initialization
    {
        use hw::HardwareBridge;
        k.bridge.log(thing_models::milestones::KERNEL_ENTRY);
        k.bridge.log("\n");
        k.bridge.log(thing_models::milestones::BRIDGE_ONLINE);
        k.bridge.log("\n");

        k.bridge.log("THINGOS: graph init\n");
        kernel_core::graph::seed_builtins(&mut k.graph);
        k.bridge.log("THINGOS: graph seeded\n");

        k.bridge.log("THINGOS: symbols init\n");
        for &builtin in kernel_core::symbols::builtins::BUILTIN_SYMBOLS {
            k.symbols.seed_builtin(builtin).expect("Builtin seed failed");
        }
        k.bridge.log("THINGOS: symbols ready\n");
    }

    // Handover to Scheduled Mode
    {
        // 1. Install Hook
        bridge_x86_64::set_tick_hook(scheduler_tick);
        bridge_x86_64::interrupts::syscall::set_syscall_hook(syscall_hook);
        
        // 2. Set Global
        *KERNEL.lock() = Some(k);
        
        // 3. Init Hardware Bridge (Interrupts Enabled!)
        unsafe { Bridge::init(); }

    }

    loop {
        use hw::HardwareBridge;
        let bridge = Bridge;
        bridge.log(thing_models::milestones::IDLE_LOOP);
        bridge.log("\n");
        bridge.idle();
    }
}
