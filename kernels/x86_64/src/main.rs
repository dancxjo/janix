#![no_std]
#![no_main]
#![cfg_attr(target_os = "thingos", feature(alloc_error_handler))]

extern crate alloc;

#[cfg(target_os = "thingos")]
mod early_log;
pub mod font;

#[cfg(target_os = "thingos")]
mod heap;
#[cfg(target_os = "thingos")]
mod memory_intrinsics;

#[cfg(target_os = "thingos")]
pub(crate) mod limine_local;

use bridge_x86_64::Bridge;

// Global state for loader to map framebuffer
pub static mut FRAMEBUFFER_INFO: Option<(u64, u64)> = None;
use core::arch::naked_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use kernel_core::Kernel;

static PANICKING: AtomicBool = AtomicBool::new(false);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use hw::HardwareBridge;

    // Recursion guard
    if PANICKING.swap(true, Ordering::Relaxed) {
        loop {
            core::hint::spin_loop();
        }
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
        bridge.log("Line: ");
        print_hex(&bridge, loc.line() as u64);
        bridge.log("\n");
    }

    // Dump Ring to Serial
    bridge.log("\n--- RING DUMP ---\n");
    let ring = kernel_core::diag::LogRing::global();
    ring.drain(|entry| {
        // Simple formatter
        let level_char = match entry.level {
            0 => 'T',
            1 => 'D',
            2 => 'I',
            3 => 'W',
            4 => 'E',
            5 => 'F',
            6 => '?',
            _ => '?',
        };
        // Print level
        bridge.log(core::str::from_utf8(&[level_char as u8]).unwrap());
        bridge.log(": ");

        let len = entry.msg_len as usize;
        if len > 0 && len <= 256 {
            // MSG_MAX
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
    let mut printed = false;
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        if digit != 0 || printed || i == 0 {
            let c = if digit < 10 {
                digit as u8 + b'0'
            } else {
                digit as u8 - 10 + b'a'
            };
            bridge.log(core::str::from_utf8(&[c]).unwrap());
            printed = true;
        }
    }
}

pub const HEAP_SIZE: usize = 128 * 1024 * 1024;

const BOOT_STACK_SIZE: usize = 65536;

#[repr(align(16))]
struct AlignedStack([u8; BOOT_STACK_SIZE]);

#[cfg(target_os = "thingos")]
#[used]
#[unsafe(link_section = ".bss")]
static mut BOOT_STACK: AlignedStack = AlignedStack([0; BOOT_STACK_SIZE]);

#[cfg(target_os = "thingos")]
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

use models as thing_models;
use spin::Mutex;

pub mod loader;
use loader::{process_file, scan_boot_fs_task, ScanArgs};

static KERNEL: Mutex<Option<Kernel<Bridge>>> = Mutex::new(None);
use core::sync::atomic::AtomicU64;
static LAST_TICKS: AtomicU64 = AtomicU64::new(0);

// Shared FrameAllocator for Kernel Tasks
use x86_64::{VirtAddr, PhysAddr};
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB, Translate};
struct HeapFrameAllocator {
    hhdm_offset: VirtAddr,
}

unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        use alloc::alloc::{alloc, Layout};
        let layout = Layout::from_size_align(4096, 4096).ok()?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return None;
        }
        use x86_64::registers::control::Cr3;
        use x86_64::structures::paging::{OffsetPageTable};
        let (l4_frame, _) = Cr3::read();
        let phys_l4 = l4_frame.start_address();
        let virt_l4 = self.hhdm_offset + phys_l4.as_u64();
        let page_table_ptr = virt_l4.as_mut_ptr();
        let mut mapper = unsafe { OffsetPageTable::new(&mut *page_table_ptr, self.hhdm_offset) };
        let virt_addr = VirtAddr::new(ptr as u64);
        mapper
            .translate_addr(virt_addr)
            .map(|phys| PhysFrame::containing_address(phys))
    }
}


fn scheduler_tick(frame: &mut bridge_x86_64::interrupts::trap::TrapFrame) {
    if let Some(mut guard) = KERNEL.try_lock() {
        if let Some(k) = (*guard).as_mut() {
            use kernel_core::sched::scheduler::ThreadContext;
            let mut ctx = ThreadContext::default();

            unsafe {
                let frame_ptr = frame as *const _ as *const u64;
                let ctx_ptr = ctx.0.as_mut_ptr();
                core::ptr::copy_nonoverlapping(frame_ptr, ctx_ptr, 20);
            }

            {
                use hw::HardwareBridge;
                let now_raw = k.bridge.ticks();
                let last = LAST_TICKS.swap(now_raw, Ordering::Relaxed);
                let now_ns = k.bridge.monotonic_now();

                let delta = if now_ns > 0 {
                    let last_ns = LAST_TICKS.swap(now_ns, Ordering::Relaxed);
                    if now_ns >= last_ns && last_ns > 0 {
                        now_ns - last_ns
                    } else {
                        0
                    }
                } else {
                    if now_raw >= last {
                        now_raw - last
                    } else {
                        0
                    }
                };

                kernel_core::time::tick(&mut k.graph, delta);
                let monotonic = kernel_core::time::monotonic_ns();
                k.scheduler.wake_sleepers(monotonic);
                unsafe {
                    bridge_x86_64::hpet::ack_interrupt();
                }
                if now_ns > 0 {
                    unsafe {
                        let next_wake = k.scheduler.next_wakeup_deadline().unwrap_or(u64::MAX);
                        let tick_target = monotonic + 10_000_000;
                        let target = core::cmp::min(tick_target, next_wake);
                        bridge_x86_64::hpet::program_oneshot(target);
                    }
                }
                kernel_core::diag::flusher::flush_diagnostics(k);
            }

            k.scheduler.tick(&k.bridge, &mut ctx);

            unsafe {
                let ctx_ptr = ctx.0.as_ptr();
                let frame_ptr = frame as *mut _ as *mut u64;
                core::ptr::copy_nonoverlapping(ctx_ptr, frame_ptr, 20);
            }
        }
    }
}

fn page_fault_hook_impl(
    _stack_frame: &x86_64::structures::idt::InterruptStackFrame,
    fault_addr: u64,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) -> bool {
    use x86_64::structures::idt::PageFaultErrorCode;
    if !error_code.contains(PageFaultErrorCode::USER_MODE) {
        return false;
    }
    
    use hw::HardwareBridge; 

    #[allow(unused_imports)]
    use x86_64::structures::paging::FrameAllocator;
    
    if let Some(mut guard) = KERNEL.try_lock() {
        if let Some(k) = (*guard).as_mut() {
            use limine_local::requests::HHDM_REQUEST;
            let hhdm_offset_u64 = HHDM_REQUEST.get_response().unwrap().offset();
            let hhdm_offset = x86_64::VirtAddr::new(hhdm_offset_u64);

            // HeapFrameAllocator is now defined at module level
            let mut frame_allocator = HeapFrameAllocator { hhdm_offset };

            if let Some(current_tid) = k.scheduler.current {
                 if let Some(Some(thread)) = k.scheduler.threads.get(current_tid.0 as usize - 1) {
                     let pid = thread.process_id;
                     if let Some(Some(process)) = k.scheduler.processes.get(pid.0 as usize - 1) {
                         if fault_addr >= process.heap_virt_start
                             && fault_addr < process.heap_virt_end
                         {
                             use x86_64::structures::paging::{
                                 Mapper, OffsetPageTable, Page, PageTableFlags, Size4KiB,
                             };
                             use x86_64::registers::control::Cr3;

                             let (l4_frame, _) = Cr3::read();
                             let phys_l4 = l4_frame.start_address();
                             let virt_l4 = hhdm_offset + phys_l4.as_u64();
                             let page_table_ptr = virt_l4.as_mut_ptr();
                             let mut mapper = unsafe { OffsetPageTable::new(&mut *page_table_ptr, hhdm_offset) };

                             let page = Page::<Size4KiB>::containing_address(x86_64::VirtAddr::new(fault_addr));
                             if let Some(frame) = frame_allocator.allocate_frame() {
                                  let flags = PageTableFlags::PRESENT
                                     | PageTableFlags::WRITABLE
                                     | PageTableFlags::USER_ACCESSIBLE;
                                 unsafe {
                                     if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                                         {
                                              let phys = frame.start_address();
                                              let virt = hhdm_offset + phys.as_u64();
                                              core::ptr::write_bytes(virt.as_mut_ptr::<u8>(), 0, 4096);
                                         }
                                         map_to.flush();
                                        //  k.bridge.log("PF: Demand Alloc ");
                                        //  print_hex(&k.bridge, fault_addr);
                                        //  k.bridge.log("\n");
                                         return true; 
                                     }
                                 }
                             } else {
                                 k.bridge.log("PF: OOM in Demand Alloc\n");
                             }
                         }
                     }
                 }
            }
        }
    }

    false 
}

fn syscall_hook(
    num: usize,
    a1: usize, // data_ptr
    a2: usize, // data_len
    a3: usize, // name_ptr
    a4: usize, // name_len
    a5: usize,
    a6: usize,
) -> isize {
    use abi::syscall_defs::SYSCALL_SPAWN;

    // Intercept SYSCALL_SPAWN
    if num == SYSCALL_SPAWN {
         let data_ptr = a1 as *const u8;
         let data_len = a2;
         let name_ptr = a3 as *const u8;
         let name_len = a4;
         
         if data_ptr as usize == 0 || data_len == 0 { return -1; }
         
         // Safety: We assume user passed valid mapped pointers.
         // Since we are in the same address space (kernel high/user low with user pages accessible),
         // we can just read them. 
         // TODO: Validate user pointers against user memory range.
         
         let data = unsafe { core::slice::from_raw_parts(data_ptr, data_len) };
         
         let name = if name_ptr as usize != 0 && name_len > 0 {
             let name_bytes = unsafe { core::slice::from_raw_parts(name_ptr, name_len) };
             core::str::from_utf8(name_bytes).unwrap_or("unknown")
         } else {
             "unknown"
         };
         
         if let Some(mut guard) = KERNEL.try_lock() {
             if let Some(k) = (*guard).as_mut() {
                 use limine_local::requests::HHDM_REQUEST;
                 use hw::HardwareBridge;

                 // k.bridge.log("SYSCALL SPAWN: ");
                 // k.bridge.log(name);
                 // k.bridge.log("\n");

                 let hhdm_offset_u64 = HHDM_REQUEST.get_response().unwrap().offset();
                 
                 unsafe {
                     process_file(
                         k, 
                         None, 
                         name,
                         data,
                         0, 
                         None, // Not force, rely on defaults
                         hhdm_offset_u64
                     );
                 }
                 return 0;
             }
         }
         return -1;
    }

    loop {
        if let Some(mut guard) = KERNEL.try_lock() {
            if let Some(k) = (*guard).as_mut() {
                return kernel_core::syscalls::syscall_dispatch(k, num, a1, a2, a3, a4, a5, a6);
            }
        }
        core::hint::spin_loop();
    }
}

#[cfg(not(target_os = "thingos"))]
fn main() {}

#[cfg(target_os = "thingos")]
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let hhdm_offset_u64 = limine_local::requests::HHDM_REQUEST
        .get_response()
        .map(|r: &limine::response::HhdmResponse| r.offset())
        .unwrap_or(0);
    let rsdp_addr = limine_local::requests::RSDP_REQUEST
        .get_response()
        .map(|r: &limine::response::RsdpResponse| r.address() as u64);

    unsafe {
        use hw::HardwareBridge;
        Bridge::init(rsdp_addr, hhdm_offset_u64);
        Bridge.log("BOOT: Bridge Online\n");

        let info =
            limine_local::heap_init::init_heap_from_limine(heap::KERNEL_HEAP_SIZE_BYTES as u64);
        early_log::log_heap_init(info);
    }

    let mut k = Kernel::new(Bridge);

    unsafe {
        use hw::HardwareBridge;
        k.bridge.log("BOOT: Kernel Initialized\n");
        k.bridge.log(thing_models::milestones::KERNEL_ENTRY);
        k.bridge.log("\n");

        kernel_core::graph::seed_builtins(&mut k.graph);
        
        let kernel_name_sym = k
            .symbols
            .intern("kernel")
            .unwrap_or(thing_models::builtins::symbols::SYM_PROCESS);

        let boot_root_body = thing_models::core::process::ProcessBody {
            pid: 0,
            name: kernel_name_sym,
            state: thing_models::core::process::ProcessState::Running,
        };
        let root_bytes = postcard::to_allocvec(&boot_root_body).unwrap();
        
        use abi::wire::typed::{CodecId, TypeId, TypedBytes};
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

        // --- Memory Mapping & ACPI Setup (Moved from kernel_init_task) ---
        {
            use limine_local::requests::{MEMORY_MAP_REQUEST};
            use x86_64::structures::paging::{PageTableFlags, PhysFrame, Size4KiB, Page, OffsetPageTable, Translate, Mapper};
            use x86_64::registers::control::Cr3;
            
            let hhdm_offset = VirtAddr::new(hhdm_offset_u64);
            let mut frame_allocator = HeapFrameAllocator { hhdm_offset };
            let (level_4_table_frame, _) = Cr3::read();
            let phys = level_4_table_frame.start_address();
            let virt = hhdm_offset + phys.as_u64();
            let page_table_ptr: *mut x86_64::structures::paging::PageTable = virt.as_mut_ptr();
            let mut mapper = OffsetPageTable::new(&mut *page_table_ptr, hhdm_offset);

            if let Some(resp) = MEMORY_MAP_REQUEST.get_response() {
                for entry in resp.entries() {
                    use limine::memory_map::EntryType;
                     let (start, end, flags) = match entry.entry_type {
                         EntryType::ACPI_RECLAIMABLE | EntryType::ACPI_NVS => {
                             (entry.base, entry.base + entry.length, PageTableFlags::PRESENT | PageTableFlags::WRITABLE)
                         },
                         EntryType::RESERVED => {
                             if entry.base < 0xC0000000 {
                                 (entry.base, core::cmp::min(entry.base + entry.length, 0xC0000000), PageTableFlags::PRESENT | PageTableFlags::WRITABLE)
                             } else {
                                 (0,0, PageTableFlags::empty())
                             }
                         },
                         _ => (0,0, PageTableFlags::empty())
                     };
                     
                     if flags != PageTableFlags::empty() && end > start {
                         let start_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(start));
                         let end_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(end - 1));
                         for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
                             let phys = frame.start_address();
                             let virt = hhdm_offset + phys.as_u64();
                             if mapper.translate_addr(virt).is_none() {
                                 let page = Page::<Size4KiB>::containing_address(virt);
                                 if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                                     map_to.flush();
                                 }
                             }
                         }
                     }
                }
            }
            
            let mmio_start = 0xFEC00000;
            let mmio_end = 0xFEF00000;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE;
            let start_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(mmio_start));
            let end_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(mmio_end - 1));
            for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
                 let phys = frame.start_address();
                 let virt = hhdm_offset + phys.as_u64();
                 if mapper.translate_addr(virt).is_none() {
                     let page = Page::<Size4KiB>::containing_address(virt);
                     if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                         map_to.flush();
                     }
                 }
            }
        }
        
        {
            use hw::HardwareBridge;
            use limine_local::requests::{RSDP_REQUEST};
            k.bridge.log("INIT: ACPI Setup...\n");
            let rsdp_addr = RSDP_REQUEST.get_response().map(|r| r.address() as u64).unwrap_or(0);
            
            if rsdp_addr != 0 {
                Bridge::init_acpi(rsdp_addr, hhdm_offset_u64);
                use bridge_x86_64::{acpi, hpet, interrupts::{apic, ioapic, pic}};
                
                pic::disable();
                if acpi::LOCAL_APIC_ADDR != 0 { apic::init(acpi::LOCAL_APIC_ADDR); }
                
                if acpi::IO_APIC_ADDR != 0 {
                    ioapic::init(acpi::IO_APIC_ADDR);
                    let irq1 = acpi::ISA_OVERRIDES[1] as u32;
                    ioapic::set_irq_vector(irq1, 33, apic::id() as u8);
                    let irq12 = acpi::ISA_OVERRIDES[12] as u32;
                    ioapic::set_irq_vector(irq12, 44, apic::id() as u8);
                }
                
                if hpet::read_ticks() != 0 {
                    apic::enable_timer(32);
                }
            }
        }
        // -----------------------------------------------------------------


        ingest_bitmaps(&mut k);
        // Default to Limine FB
        let mut use_qemu = false;

        if let Some(file) = limine_local::requests::KERNEL_FILE_REQUEST.get_response().map(|r| r.file()) {
             let cmdline_bytes = file.cmdline();
             if let Ok(cmdline) = core::str::from_utf8(cmdline_bytes) {
                 if cmdline.contains("thingos.driver=qemu") {
                     use_qemu = true;
                 }
             }
        }

        if !use_qemu {
            // Pass the Framebuffer Response to the generic driver
            let fb_response = limine_local::requests::FRAMEBUFFER_REQUEST.get_response();
            kernel_core::drivers::limine_fb::init(&mut k, fb_response);

            // Capture info for loader
            if let Some(resp) = fb_response {
                if let Some(fb) = resp.framebuffers().next() {
                    let mut addr = fb.addr() as u64;
                    if addr >= hhdm_offset_u64 {
                        addr -= hhdm_offset_u64;
                    }
                    FRAMEBUFFER_INFO = Some((addr, (fb.pitch() as u64) * (fb.height() as u64)));
                }
            }
        }

        spawn_loaded(&mut k);
        spawn_kernel_init_task(&mut k);

        bridge_x86_64::set_tick_hook(scheduler_tick);
        bridge_x86_64::interrupts::syscall::set_syscall_hook(syscall_hook);
        use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
        bridge_x86_64::set_page_fault_hook(page_fault_hook_impl as fn(&InterruptStackFrame, u64, PageFaultErrorCode) -> bool);
        
        k.bridge.log("BOOT: Enabling Interrupts & Scheduler\n");
        *KERNEL.lock() = Some(k);
        Bridge.irq_enable();
    }

    loop {
        unsafe { use hw::HardwareBridge; Bridge.idle(); }
    }
}


unsafe fn spawn_loaded(k: &mut Kernel<Bridge>) {
    use limine_local::requests::MODULE_REQUEST;
    use hw::HardwareBridge;
    
    if let Some(resp) = MODULE_REQUEST.get_response() {
        let resp: &limine::response::ModuleResponse = resp;
        for module in resp.modules() {
             let path = module.path().to_str().unwrap_or("?");
             if path.ends_with("loaded.elf") {
                 k.bridge.log("BOOT: Spawning loaded...\n");
                 
                 use limine_local::requests::HHDM_REQUEST;
                 let hhdm_offset = HHDM_REQUEST.get_response().unwrap().offset();
                 
                 let data = core::slice::from_raw_parts(module.addr() as *const u8, module.size() as usize);
                 
                 process_file(
                     k, 
                     None, 
                     "loaded.elf",
                     data,
                     0, 
                     None,
                     hhdm_offset
                 );
                 return;
             }
        }
    }
    k.bridge.log("BOOT: WARNING: loaded.elf not found!\n");
}

unsafe fn spawn_kernel_init_task(k: &mut Kernel<Bridge>) {
    use hw::HardwareBridge;
    k.bridge.log("BOOT: Spawning Kernel Init Task...\n");
    
    let stack_layout = alloc::alloc::Layout::from_size_align(65536, 16).unwrap();
    let stack_ptr = alloc::alloc::alloc(stack_layout);
    let stack_top = stack_ptr.add(65536) as u64;
    let entry = kernel_init_task_entry as *const () as u64;

    k.scheduler.spawn(
        &k.bridge,
        "kernel_init",
        entry,
        stack_top - 8, // Adjust for ABI alignment (RSP % 16 == 8 at entry)
        0, 
        0, 
        0  
    );
}

extern "C" fn kernel_init_task_entry(_arg: u64) {
    use limine_local::requests::{HHDM_REQUEST};
    use x86_64::VirtAddr;

    let hhdm_offset_u64 = HHDM_REQUEST.get_response().unwrap().offset();
    let hhdm_offset = VirtAddr::new(hhdm_offset_u64);
    
    // Memory and ACPI are now initialized in rust_main before we run.
    // We proceed directly to PCI Scan.
    
    unsafe { u_sleep(100); }
    let pci_devices = unsafe { bridge_x86_64::pci::scan_pci() };
    
    let mut boot_args: Option<ScanArgs> = None;
    
    {
        loop {
            if let Some(mut guard) = KERNEL.try_lock() {
                if let Some(k) = (*guard).as_mut() {
                     use hw::HardwareBridge;
                     k.bridge.log("INIT: Publishing PCI Check...\n");

                     let mut use_qemu = false;
                     if let Some(file) = limine_local::requests::KERNEL_FILE_REQUEST.get_response().map(|r: &limine::response::ExecutableFileResponse| r.file()) {
                         let file: &limine::file::File = file;
                         let cmdline_bytes = file.cmdline();
                         if let Ok(cmdline) = core::str::from_utf8(cmdline_bytes) {
                             if cmdline.contains("thingos.driver=qemu") {
                                 use_qemu = true;
                             }
                         }
                     }

                     if use_qemu {
                         let info = unsafe { bridge_x86_64::drivers::qemu_vga::init(k, &pci_devices) };
                         unsafe { FRAMEBUFFER_INFO = info; }
                     }
                     
                     for dev in &pci_devices {
                         use abi::wire::typed::{CodecId, TypeId, TypedBytes};
                         use thing_models::builtins::ids::*;
                         use thing_models::value::ThingBody;
                         use thing_models::link::LinkBody;
                         
                        let body = postcard::to_allocvec(dev).unwrap();
                        let tb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_PCI_DEVICE_KIND.0 as u128),
                            codec_id: CodecId::POSTCARD,
                            bytes: body,
                        }).unwrap();
                        let dev_id = k.graph.create_thing(THING_PCI_DEVICE_KIND, tb);
                        
                        let link = LinkBody { from: THING_BOOT_ROOT, to: dev_id, predicate: THING_HAS_DEVICE_KIND };
                        let lb = ThingBody::from(&TypedBytes { type_id: TypeId(THING_LINK_KIND.0 as u128), codec_id: CodecId::POSTCARD, bytes: postcard::to_allocvec(&link).unwrap()}).unwrap();
                        k.graph.create_thing(THING_LINK_KIND, lb);
                        
                        // AHCI Check
                        if dev.class_id == 0x01 && dev.subclass_id == 0x06 && dev.prog_if == 0x01 {
                            k.bridge.log("INIT: AHCI Found\n");
                            let bar5 = dev.bars[5];
                            if bar5 != 0 && (bar5 & 1) == 0 {
                                let base = (bar5 & 0xFFFFFFF0) as u64;
                                let size = 8192;
                                
                                unsafe {
                                     // Quick Map
                                    use x86_64::structures::paging::{PageTableFlags, PhysFrame, Size4KiB, Page, OffsetPageTable, Translate, Mapper};
                                    use x86_64::{VirtAddr, PhysAddr};
                                    use x86_64::registers::control::Cr3;

                                    // HeapFrameAllocator is now defined at module level
                                    let mut frame_allocator = HeapFrameAllocator { hhdm_offset: VirtAddr::new(hhdm_offset_u64) };
                                    let (l4_frame, _) = Cr3::read();
                                    let phys_l4 = l4_frame.start_address();
                                    let virt_l4 = hhdm_offset + phys_l4.as_u64();
                                    let page_table_ptr = virt_l4.as_mut_ptr();
                                    let mut mapper = OffsetPageTable::new(&mut *page_table_ptr, hhdm_offset);
                                    
                                    let start_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(base));
                                    let end_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(base + size - 1));
                                    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE;
                                    for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
                                         let phys = frame.start_address();
                                         let virt = hhdm_offset + phys.as_u64();
                                         if mapper.translate_addr(virt).is_none() {
                                             let page = Page::<Size4KiB>::containing_address(virt);
                                             if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                                                 map_to.flush();
                                             }
                                         }
                                    }
                                }
                                
                                // Init and Check Ports
                                // We use bridge::ahci::init which now returns u32 active ports.
                                let ports = unsafe { bridge_x86_64::ahci::init(dev, k) };
                                if ports > 0 {
                                     let _virt_base = hhdm_offset_u64 + base;
                                     // Find first
                                    for p in 0..32 {
                                        if (ports & (1 << p)) != 0 {
                                            k.bridge.log("INIT: Booting from Port ");
                                            print_hex(&Bridge, p as u64);
                                            k.bridge.log("\n");
                                            boot_args = Some(ScanArgs {
                                                base, // Physical Address (AHCI driver adds HHDM internally)
                                                port: p,
                                                hhdm: hhdm_offset_u64,
                                            });
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                     }
                }
                break;
            }
            unsafe { core::hint::spin_loop(); } 
        }
    }
    
    if let Some(args) = boot_args {
         unsafe { u_sleep(100); }
         loop {
             if let Some(mut guard) = KERNEL.try_lock() {
                 if let Some(k) = (*guard).as_mut() {
                     use hw::HardwareBridge;

                     k.bridge.log("INIT: BootFS Scan...\n");
                 }
                 break; 
             }
             unsafe { core::hint::spin_loop(); }
         }
         
         scan_boot_fs_task(alloc::boxed::Box::into_raw(alloc::boxed::Box::new(args)) as u64);
    } else {
        unsafe {
             use hw::HardwareBridge;
             Bridge.log("INIT: No AHCI boot device found.\n");
        }
    }
    
    unsafe {
         use hw::HardwareBridge;
         Bridge.log("INIT: Complete. Parking.\n");
    }
    
    loop {
         unsafe { core::hint::spin_loop(); }
    }
}

unsafe fn u_sleep(count: u64) {
    for _ in 0..count {
        core::hint::spin_loop();
    }
}

unsafe fn ingest_bitmaps(k: &mut Kernel<Bridge>) {
    use limine_local::requests::MODULE_REQUEST;
    use hw::HardwareBridge;
    use thing_models::builtins::ids::{THING_BITMAP_KIND, THING_BOOT_ROOT, THING_HAS_DEVICE_KIND, THING_LINK_KIND};

    use thing_models::value::ThingBody;
    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
    use thing_models::link::LinkBody;

    if let Some(resp) = MODULE_REQUEST.get_response() {
        let resp: &limine::response::ModuleResponse = resp;
        for module in resp.modules() {
             let path = module.path().to_str().unwrap_or("?");
             if path.ends_with(".bmp") {
                 k.bridge.log("BOOT: Ingesting Bitmap ");
                 k.bridge.log(path);
                 k.bridge.log("\n");

                 let data = core::slice::from_raw_parts(module.addr() as *const u8, module.size() as usize);

                 if let Some(bitmap) = parse_bmp(data) {
                     let bytes = postcard::to_allocvec(&bitmap).unwrap();
                     let tb = ThingBody::from(&TypedBytes {
                         type_id: TypeId(THING_BITMAP_KIND.0 as u128),
                         codec_id: CodecId::POSTCARD,
                         bytes,
                     }).unwrap();

                     let id = k.graph.create_thing(THING_BITMAP_KIND, tb);

                     let link = LinkBody {
                         from: THING_BOOT_ROOT,
                         to: id,
                         predicate: THING_HAS_DEVICE_KIND
                     };

                     let lb = ThingBody::from(&TypedBytes {
                         type_id: TypeId(THING_LINK_KIND.0 as u128),
                         codec_id: CodecId::POSTCARD,
                         bytes: postcard::to_allocvec(&link).unwrap()
                     }).unwrap();
                     k.graph.create_thing(THING_LINK_KIND, lb);
                 } else {
                     k.bridge.log("BOOT: Failed to parse BMP\n");
                 }
             }
        }
    }
}

fn parse_bmp(data: &[u8]) -> Option<thing_models::schema::bitmap::BitmapBody> {
    if data.len() < 54 || &data[0..2] != b"BM" { return None; }
    let pixel_offset = u32::from_le_bytes(data[10..14].try_into().ok()?) as usize;
    let width = i32::from_le_bytes(data[18..22].try_into().ok()?) as u32;
    let height = i32::from_le_bytes(data[22..26].try_into().ok()?);
    let bpp = u16::from_le_bytes(data[28..30].try_into().ok()?);

    if bpp != 32 { return None; }

    let height_abs = height.abs() as u32;
    let row_size = (width * 4) as usize;
    let pixels_len = (width * height_abs * 4) as usize;

    if data.len() < pixel_offset + pixels_len { return None; }

    let mut pixels = alloc::vec![0u8; pixels_len];
    let src_pixels = &data[pixel_offset..];

    for y in 0..height_abs {
        let src_row_idx = if height > 0 {
            (height_abs - 1 - y) as usize
        } else {
            y as usize
        };

        let src_start = src_row_idx * row_size;
        let dst_start = (y as usize) * row_size;

        if src_start + row_size <= src_pixels.len() && dst_start + row_size <= pixels.len() {
             pixels[dst_start..dst_start+row_size].copy_from_slice(&src_pixels[src_start..src_start+row_size]);
        }
    }

    Some(thing_models::schema::bitmap::BitmapBody {
        width,
        height: height_abs,
        format: 0,
        pixels
    })
}
