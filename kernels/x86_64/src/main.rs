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
mod memory_intrinsics;

use bridge_x86_64::Bridge;
use core::arch::naked_asm;
use kernel_core::Kernel;
use core::sync::atomic::{AtomicBool, Ordering};

static PANICKING: AtomicBool = AtomicBool::new(false);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use hw::HardwareBridge;

    // Recursion guard
    if PANICKING.swap(true, Ordering::Relaxed) {
        loop { core::hint::spin_loop(); }
    }

    let bridge = Bridge;

    // Capture to ring
    // We try to extract string if possible, or just "PANIC"
    kernel_core::diag::record_panic("PANIC");

    bridge.log("PANIC\n");
    if let Some(loc) = info.location() {
        bridge.log("File: ");
        bridge.log(loc.file());
        bridge.log("\n");
        // Line?
    }

    // Dump Ring to Serial
    bridge.log("\n--- RING DUMP ---\n");
    let ring = kernel_core::diag::LogRing::global();
    ring.drain(|entry| {
        // Simple formatter
        let level_char = match entry.level {
            0 => 'T', 1 => 'D', 2 => 'I', 3 => 'W', 4 => 'E', 5 => 'F', _ => '?'
        };
        // Print level
        bridge.log(core::str::from_utf8(&[level_char as u8]).unwrap());
        bridge.log(": ");

        let len = entry.msg_len as usize;
        if len > 0 && len <= 256 { // MSG_MAX
             if let Ok(s) = core::str::from_utf8(&entry.msg_bytes[..len]) {
                 bridge.log(s);
             } else {
                 bridge.log("<utf8 error>");
             }
        }
        bridge.log("\n");

        // Print payload if Fault
        if entry.kind == 2 {
            bridge.log("  RIP: ");
            print_hex(&bridge, entry.payload_a);
            bridge.log(" ERR: ");
            print_hex(&bridge, entry.payload_b);
            bridge.log(" CR2: ");
            print_hex(&bridge, entry.payload_c);
            bridge.log("\n");
        }
    });
    bridge.log("--- END DUMP ---\n");

    loop {
        core::hint::spin_loop();
    }
}

fn print_hex(bridge: &Bridge, val: u64) {
    use hw::HardwareBridge;
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        let c = if digit < 10 { digit as u8 + b'0' } else { digit as u8 - 10 + b'a' };
        bridge.log(core::str::from_utf8(&[c]).unwrap());
    }
}

const BOOT_STACK_SIZE: usize = 65536;
#[repr(align(16))]
struct AlignedStack([u8; BOOT_STACK_SIZE]);

#[used]
#[unsafe(link_section = ".bss")]
static mut BOOT_STACK: AlignedStack = AlignedStack([0; BOOT_STACK_SIZE]);

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
use core::sync::atomic::AtomicU64;
static LAST_TICKS: AtomicU64 = AtomicU64::new(0);

fn scheduler_tick(frame: &mut bridge_x86_64::interrupts::trap::TrapFrame) {
    if let Some(mut guard) = KERNEL.try_lock() {
        if let Some(k) = (*guard).as_mut() {
            // Fix: Do NOT cast TrapFrame directly to ThreadContext.
            // TrapFrame is 160 bytes (20 u64s). ThreadContext is 272 bytes (34 u64s).
            // Casting implies we can write 272 bytes to the stack, which overflows 
            // the interrupt frame and corrupts the stack (e.g. return address).
            
            // 1. Create local ThreadContext (on stack, 272 bytes)
            use kernel_core::sched::scheduler::ThreadContext;
            let mut ctx = ThreadContext::default();
            
            // 2. Copy TrapFrame (20 u64s) into local Context
            // We assume ThreadContext layout starts with the same fields as TrapFrame.
            // [r15...rax, rip, cs, rflags, rsp, ss] -> 20 words
            unsafe {
                let frame_ptr = frame as *const _ as *const u64;
                let ctx_ptr = ctx.0.as_mut_ptr();
                core::ptr::copy_nonoverlapping(frame_ptr, ctx_ptr, 20);
            }

            // Time Service Update & Sleep Management
            {
                use hw::HardwareBridge; 
                let now_raw = k.bridge.ticks();
                let last = LAST_TICKS.swap(now_raw, Ordering::Relaxed);
                
                // If first run (last=0) or valid delta
                // Note: on first run if now_raw is huge, delta is huge. 
                // We assume ticks start near 0.
                let delta = if now_raw >= last { now_raw - last } else { 0 };
                
                // 1. Advance kernel time (and update Graph)
                kernel_core::time::tick(&mut k.graph, delta);
                
                // 2. Wake sleepers
                let monotonic = kernel_core::time::monotonic_ns();
                k.scheduler.wake_sleepers(monotonic);
            }

            // 3. Tick Scheduler (updates ctx if switch occurs)
            k.scheduler.tick(&k.bridge, &mut ctx);
            
            // 4. Copy Back (only 20 u64s) to TrapFrame
            unsafe {
                let ctx_ptr = ctx.0.as_ptr();
                let frame_ptr = frame as *mut _ as *mut u64;
                core::ptr::copy_nonoverlapping(ctx_ptr, frame_ptr, 20);
            }
        }
    }
}

fn syscall_hook(num: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize) -> isize {
    // We need lock.
    use hw::HardwareBridge; 
    // Simple loop-spin is fine for lock, but we don't need 'hlt' loop logic for wait flag.
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

    // Init Bridge (GDT/IDT/PIC) EARLY so selectors are ready for spawn
    unsafe { Bridge::init(); }

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

    // Boot Initialization
    {
        use hw::HardwareBridge;
        use abi::wire::typed::{TypedBytes, TypeId, CodecId};
        k.bridge.log(thing_models::milestones::KERNEL_ENTRY);
        k.bridge.log("\n");
        k.bridge.log(thing_models::milestones::BRIDGE_ONLINE);
        k.bridge.log("\n");

        k.bridge.log("THINGOS: graph init\n");
        kernel_core::graph::seed_builtins(&mut k.graph);
        
        // Seed Boot Root
        // We need symbol for "kernel"
        let kernel_name_sym = k.symbols.intern("kernel").unwrap_or(thing_models::builtins::symbols::SYM_PROCESS);
        
        let boot_root_body = thing_models::core::process::ProcessBody {
            pid: 0,
            name: kernel_name_sym,
            state: thing_models::core::process::ProcessState::Running,
        };
        
        let root_bytes = postcard::to_allocvec(&boot_root_body).unwrap();
        let typed_root = TypedBytes {
             type_id: TypeId(thing_models::builtins::ids::THING_PROCESS_KIND.0 as u128),
             codec_id: CodecId::POSTCARD,
             bytes: root_bytes,
        };

        if let Ok(tr_body) = thing_models::value::ThingBody::from(&typed_root) {
            let boot_root = thing_models::Thing {
                id: thing_models::builtins::ids::THING_BOOT_ROOT,
                kind: thing_models::builtins::ids::THING_PROCESS_KIND,
                body: tr_body,
            };
            k.graph.insert_seed(boot_root);
        }

        k.bridge.log("THINGOS: graph seeded\n");

        k.bridge.log("THINGOS: symbols init\n");
        for &builtin in kernel_core::symbols::builtins::BUILTIN_SYMBOLS {
            k.symbols.seed_builtin(builtin).expect("Builtin seed failed");
        }
        k.bridge.log("THINGOS: symbols ready\n");
    }



        // --- CMDLINE PARSING (Rudimentary) ---
        let mut smoke_target = None;
        {
            use limine::requests::KERNEL_FILE_REQUEST;
            if let Some(resp) = KERNEL_FILE_REQUEST.get_response() {
                let file = resp.file();
                let cmd_bytes = file.cmdline();
                if let Ok(cmd_str) = core::str::from_utf8(cmd_bytes) {
                     // Looking for "thingos.smoke=<name>"
                     for part in cmd_str.split(' ') {
                         if let Some(rest) = part.strip_prefix("thingos.smoke=") {
                              smoke_target = Some(rest.trim());
                              k.bridge.log("SMOKE MODE: Target is ");
                              k.bridge.log(rest);
                              k.bridge.log("\n");
                         }
                     }
                }
            }
        }


        // --- MODULE LOADING ---
        k.bridge.log("Scanning modules...\n");
        let mut app_load_virt_base = 0x2000_0000u64;

        if let Some(resp) = MODULE_REQUEST.get_response() {
            enum ModuleType { Elf, Psf1, Psf2, Bmp, Png, Unknown, Other(alloc::string::String) }
            fn classify_bytes(data: &[u8]) -> ModuleType {
                if data.len() >= 2 && data[0] == 0x36 && data[1] == 0x04 { return ModuleType::Psf1; }
                if data.len() >= 4 && data[0] == 0x72 && data[1] == 0xB5 && data[2] == 0x4A && data[3] == 0x86 { return ModuleType::Psf2; }
                
                if let Some(kind) = infer::get(data) {
                    match kind.mime_type() {
                        "application/x-executable" | "application/x-elf" | "application/x-sharedlib" => ModuleType::Elf,
                        "image/bmp" => ModuleType::Bmp,
                        "image/png" => ModuleType::Png,
                        other => ModuleType::Other(alloc::string::String::from(other)),
                    }
                } else {
                    ModuleType::Unknown
                }
            }

            enum ModuleRole { App, Driver, Debug, Asset, Ignore }
            fn get_module_role(name: &str, mtype: &ModuleType) -> ModuleRole {
                if name.contains("syscall_crud_smoke") { return ModuleRole::Debug; }
                if name.contains("keylog") { return ModuleRole::Driver; } 
                if name.contains("ps2_keyboard") { return ModuleRole::Driver; }
                match mtype {
                    ModuleType::Psf1 | ModuleType::Psf2 | ModuleType::Bmp | ModuleType::Png => ModuleRole::Asset,
                    ModuleType::Elf => ModuleRole::App,
                    ModuleType::Other(s) if s.starts_with("image/") || s.starts_with("font/") => ModuleRole::Asset,
                    _ => if name.contains("font") { ModuleRole::Asset } else { ModuleRole::Ignore },
                }
            }

            for (idx, module) in resp.modules().iter().enumerate() {
                let name = module.path().to_str().unwrap_or("unknown");
                let base = module.addr();
                let len = module.size() as usize;
                let data = unsafe { slice::from_raw_parts(base, len) };
                
                let mtype = classify_bytes(data);
                let role_enum = get_module_role(name, &mtype);

                // --- 1. Graph: Create Module Thing ---
                use thing_models::builtins::ids::*;
                use thing_models::builtins::core_kinds::{BootProgramBody, ModuleBody, FontBody, BitmapBody, ProgramImageBody};
                use thing_models::value::ThingBody;
                use abi::wire::typed::{TypedBytes, TypeId, CodecId};
                
                let role_str = match role_enum {
                    ModuleRole::App => "app", ModuleRole::Driver => "driver",
                    ModuleRole::Debug => "debug", ModuleRole::Asset => "asset",
                    ModuleRole::Ignore => "ignore",
                };
                let mime_str = match &mtype {
                    ModuleType::Elf => "application/x-elf",
                    ModuleType::Psf1 => "font/psf1", ModuleType::Psf2 => "font/psf2",
                    ModuleType::Bmp => "image/bmp", ModuleType::Png => "image/png",
                    ModuleType::Unknown => "application/octet-stream",
                    ModuleType::Other(s) => s.as_str(),
                };

                let mod_body = ModuleBody {
                    path: alloc::string::String::from(name),
                    size_bytes: len as u64,
                    base_phys: base as u64,
                    index: idx as u32,
                    role: alloc::string::String::from(role_str),
                    mime: alloc::string::String::from(mime_str),
                };

                let mod_id_bytes = postcard::to_allocvec(&mod_body).unwrap();
                let mod_tb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_MODULE_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: mod_id_bytes,
                }).unwrap();
                let module_id = k.graph.create_thing(THING_MODULE_KIND, mod_tb);

                // Link Root -> Module (HAS_MODULE)
                {
                    let link = thing_models::link::LinkBody {
                        from: THING_BOOT_ROOT, to: module_id, predicate: THING_HAS_MODULE_KIND,
                    };
                    let lb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(THING_LINK_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                         bytes: postcard::to_allocvec(&link).unwrap() 
                    }).unwrap();
                    k.graph.create_thing(THING_LINK_KIND, lb);
                }

                // --- 2. Graph: Classify & Specific Things ---
                match &mtype {
                    ModuleType::Elf => {
                        let prog_img = ProgramImageBody { format: alloc::string::String::from("elf") };
                        let tb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_PROGRAM_IMAGE_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                            bytes: postcard::to_allocvec(&prog_img).unwrap()
                        }).unwrap();
                        let img_id = k.graph.create_thing(THING_PROGRAM_IMAGE_KIND, tb);
                        
                        // Link Module -> ProgramImage (BINARY_IMAGE)
                         let link = thing_models::link::LinkBody {
                            from: module_id, to: img_id, predicate: THING_BINARY_IMAGE_KIND,
                        };
                         let lb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_LINK_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                             bytes: postcard::to_allocvec(&link).unwrap() 
                        }).unwrap();
                        k.graph.create_thing(THING_LINK_KIND, lb);
                    },
                    ModuleType::Psf1 | ModuleType::Psf2 => {
                         let mut w = 0u16; let mut h = 0u16; let mut count = 0u32;
                         if matches!(mtype, ModuleType::Psf1) {
                             if data.len() >= 4 {
                                 let mode = data[2];
                                 let charsize = data[3];
                                 w = 8;
                                 h = charsize as u16;
                                 count = if (mode & 1) != 0 { 512 } else { 256 };
                             }
                         } else {
                             if data.len() >= 32 {
                                 let read_u32 = |off: usize| -> u32 {
                                     u32::from_le_bytes([data[off], data[off+1], data[off+2], data[off+3]])
                                 };
                                 count = read_u32(16);
                                 h = read_u32(24) as u16;
                                 w = read_u32(28) as u16;
                             }
                         }

                        let font_body = FontBody {
                            name: alloc::string::String::from(name),
                            format: alloc::string::String::from(if matches!(mtype, ModuleType::Psf1) { "psf1" } else { "psf2" }),
                            glyph_width: w, glyph_height: h, glyph_count: count,
                        };
                        let tb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_FONT_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                            bytes: postcard::to_allocvec(&font_body).unwrap()
                        }).unwrap();
                        let font_id = k.graph.create_thing(THING_FONT_KIND, tb);

                        // Link Root -> Font (PROVIDES_FONT)
                        {
                            let link = thing_models::link::LinkBody {
                                from: THING_BOOT_ROOT, to: font_id, predicate: THING_PROVIDES_FONT_KIND,
                            };
                            let lb = ThingBody::from(&TypedBytes {
                                type_id: TypeId(THING_LINK_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                                bytes: postcard::to_allocvec(&link).unwrap() 
                            }).unwrap();
                            k.graph.create_thing(THING_LINK_KIND, lb);
                        }
                        
                         // Link Font -> Module (BACKED_BY)
                        {
                            let link = thing_models::link::LinkBody {
                                from: font_id, to: module_id, predicate: THING_BACKED_BY_KIND,
                            };
                            let lb = ThingBody::from(&TypedBytes {
                                type_id: TypeId(THING_LINK_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                                bytes: postcard::to_allocvec(&link).unwrap() 
                            }).unwrap();
                            k.graph.create_thing(THING_LINK_KIND, lb);
                        }

                        if name.contains("default") || name.contains("unifont") || name.contains("zap-light16") {
                            let link = thing_models::link::LinkBody {
                                from: THING_BOOT_ROOT, to: font_id, predicate: THING_DEFAULT_FONT_KIND,
                            };
                            let lb = ThingBody::from(&TypedBytes {
                                type_id: TypeId(THING_LINK_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                                bytes: postcard::to_allocvec(&link).unwrap() 
                            }).unwrap();
                            k.graph.create_thing(THING_LINK_KIND, lb);
                        }
                    },
                    _ if mime_str.starts_with("image/") => {
                        let bmp_body = BitmapBody {
                             format: alloc::string::String::from(mime_str),
                             width: 0, height: 0,
                        };
                         let tb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_BITMAP_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                            bytes: postcard::to_allocvec(&bmp_body).unwrap()
                        }).unwrap();
                        let bmp_id = k.graph.create_thing(THING_BITMAP_KIND, tb);
                        
                        {
                            let link = thing_models::link::LinkBody {
                                from: module_id, to: bmp_id, predicate: THING_ASSET_KIND,
                            };
                             let lb = ThingBody::from(&TypedBytes {
                                type_id: TypeId(THING_LINK_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                                bytes: postcard::to_allocvec(&link).unwrap() 
                            }).unwrap();
                            k.graph.create_thing(THING_LINK_KIND, lb);
                        }
                    },
                    _ => {}
                }

                // --- 3. Execution (Spawn) ---
                let should_spawn = match role_enum {
                    ModuleRole::App => true,
                    ModuleRole::Driver => {
                         k.bridge.log("Spawning driver '"); k.bridge.log(name); k.bridge.log("'\n");
                         true
                    },
                    ModuleRole::Debug => {
                        let is_target = smoke_target.map(|t| name.contains(t)).unwrap_or(false);
                         if is_target { true } else {
                             k.bridge.log("Skipping debug module '"); k.bridge.log(name); k.bridge.log("'\n");
                             false
                         }
                    },
                    ModuleRole::Ignore | ModuleRole::Asset => false,
                };

                if should_spawn {
                    let current_app_base = app_load_virt_base;
                    app_load_virt_base += 0x1000_0000;
                    
                    let loaded = load_elf(data, current_app_base, |vaddr, segment| {
                        use alloc::format;
                        let target_virt_start = VirtAddr::new(current_app_base + vaddr);
                        let target_virt_end = target_virt_start + segment.len() as u64;
                        
                        let start_page = Page::<Size4KiB>::containing_address(target_virt_start);
                        let end_page = Page::<Size4KiB>::containing_address(target_virt_end - 1u64);
                        
                        for page in Page::range_inclusive(start_page, end_page) {
                            let frame_phys: PhysAddr;
                            let page_start_virt = page.start_address();
    
                            // Check if mapped
                            if let Some(phys) = mapper.translate_addr(page_start_virt) {
                                frame_phys = phys;
                            } else {
                                // Map new frame
                                let frame = frame_allocator.allocate_frame().expect("No frames");
                                frame_phys = frame.start_address();
                                let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
                                unsafe {
                                    if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                                        map_to.flush();
                                    }
                                }
                            }
                            
                            let frame_virt = hhdm_offset + frame_phys.as_u64();
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
                        k.bridge.log("Spawning app: "); k.bridge.log(name); k.bridge.log("\n");

                        k.scheduler.spawn(&k.bridge, name, entry_point, stack_top_virt.as_u64(), 0);

                         let bp = BootProgramBody {
                            name: alloc::string::String::from(name),
                            binary: alloc::string::String::from(name), 
                            priority: 0,
                            entry_point: entry_point as u64,
                        };
                        let tb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_BOOT_PROGRAM_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                            bytes: postcard::to_allocvec(&bp).unwrap()
                        }).unwrap();
                        let prog_id = k.graph.create_thing(THING_BOOT_PROGRAM_KIND, tb);

                        {
                            let link = thing_models::link::LinkBody {
                                from: prog_id, to: module_id, predicate: THING_USES_MODULE_KIND,
                            };
                             let lb = ThingBody::from(&TypedBytes {
                                type_id: TypeId(THING_LINK_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                                bytes: postcard::to_allocvec(&link).unwrap() 
                            }).unwrap();
                            k.graph.create_thing(THING_LINK_KIND, lb);
                        }

                        {
                            let link = thing_models::link::LinkBody {
                                from: THING_BOOT_ROOT, to: prog_id, predicate: THING_LAUNCHES_KIND,
                            };
                             let lb = ThingBody::from(&TypedBytes {
                                type_id: TypeId(THING_LINK_KIND.0 as u128), codec_id: CodecId::POSTCARD,
                                bytes: postcard::to_allocvec(&link).unwrap() 
                            }).unwrap();
                            k.graph.create_thing(THING_LINK_KIND, lb);
                        }
                    }
                }
            }
        }

    }
    

    // Handover to Scheduled Mode
    {
        // 1. Install Hook
        bridge_x86_64::set_tick_hook(scheduler_tick);
        bridge_x86_64::interrupts::syscall::set_syscall_hook(syscall_hook);
        
        // 2. Set Global
        *KERNEL.lock() = Some(k);
        
        // 3. Enable Interrupts
             use hw::HardwareBridge;
             let bridge = Bridge;
             bridge.log("BRIDGE: enabling interrupts...\n");
             x86_64::instructions::interrupts::enable();
    }

    loop {
        use hw::HardwareBridge;
        let bridge = Bridge;
        bridge.log(thing_models::milestones::IDLE_LOOP);
        bridge.log("\n");
        bridge.idle();
    }
}
