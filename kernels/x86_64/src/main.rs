#![no_std]
#![no_main]
#![cfg_attr(target_os = "thingos", feature(alloc_error_handler))]

extern crate alloc;

#[cfg(target_os = "thingos")]
mod early_log;
#[cfg(target_os = "thingos")]
mod heap;
#[cfg(target_os = "thingos")]
mod limine_local;
mod memory_intrinsics;

use bridge_x86_64::Bridge;
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
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        let c = if digit < 10 {
            digit as u8 + b'0'
        } else {
            digit as u8 - 10 + b'a'
        };
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
use models as thing_models;
use spin::Mutex;
use abi::ThingId;
use thing_models::core::fs::{DirBody, MountBody};

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
                // Use monotonic_now for precision if available
                let now_ns = k.bridge.monotonic_now();

                // If now_ns is > 0, we trust it. If 0, fallback to ticks delta?
                // For now, let's just use it if we have it.
                // But `tick` function expects a delta currently.
                // We planned to change `tick` or just calc delta here.

                // If we have HPET (which we should), now_ns is absolute.
                // We can just diff against last *ns*.
                // But `LAST_TICKS` stores *raw ticks*.
                // Let's change LAST_TICKS to LAST_NS if we switch fully.
                // For this step, let's keep it simple:
                // If now_ns > 0: use it.

                let delta = if now_ns > 0 {
                    let last_ns = LAST_TICKS.swap(now_ns, Ordering::Relaxed);
                    if now_ns >= last_ns && last_ns > 0 {
                        now_ns - last_ns
                    } else {
                        0
                    }
                } else {
                    // Fallback to TSC ticks (uncalibrated)
                    let now_raw = k.bridge.ticks();
                    let last = LAST_TICKS.swap(now_raw, Ordering::Relaxed);
                    if now_raw >= last {
                        now_raw - last
                    } else {
                        0
                    }
                };

                // 1. Advance kernel time (and update Graph)
                kernel_core::time::tick(&mut k.graph, delta);

                // 2. Wake sleepers
                let monotonic = kernel_core::time::monotonic_ns();
                k.scheduler.wake_sleepers(monotonic);

                // 3. Program Next Deadline (HPET)
                // We acknowledge *before* or *after*?
                // Ack should clear the status.
                unsafe {
                    bridge_x86_64::hpet::ack_interrupt();
                }

                if now_ns > 0 {
                    unsafe {
                        let next_wake = k.scheduler.next_wakeup_deadline().unwrap_or(u64::MAX);
                        let tick_target = monotonic + 10_000_000; // 10ms default tick for RR
                        let target = core::cmp::min(tick_target, next_wake);
                        bridge_x86_64::hpet::program_oneshot(target);
                    }
                }

                // Flush Logs to Graph
                kernel_core::diag::flusher::flush_diagnostics(k);
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

fn syscall_hook(
    num: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> isize {
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

        let info =
            limine_local::heap_init::init_heap_from_limine(heap::KERNEL_HEAP_SIZE_BYTES as u64);
        early_log::log_heap_init(info);
    }

    let mut k = Kernel::new(Bridge);

    // Init Bridge (GDT/IDT/PIC) EARLY so selectors are ready for spawn
    // Init Bridge (GDT/IDT/PIC) EARLY so selectors are ready for spawn
    // We need RSDP for ACPI/HPET, but that might be fine later?
    // Actually, we should probably grab HHDM/RSDP early if we can.
    // But HHDM relies on memory map which is available.
    // Let's grab them inside the unsafe block or move init later?
    // Bridge::init now *requires* arguments.
    // Let's move Bridge::init down, OR grab requests early.
    // Limine requests are static, we can access them.

    let hhdm_offset_u64 = limine_local::requests::HHDM_REQUEST
        .get_response()
        .map(|r| r.offset())
        .unwrap_or(0);
    // Limine address() returns usize, cast to u64
    let rsdp_addr = limine_local::requests::RSDP_REQUEST
        .get_response()
        .map(|r| r.address() as u64);

    unsafe {
        use hw::HardwareBridge;
        Bridge.log("HHDM: ");
        print_hex(&Bridge, hhdm_offset_u64);
        Bridge.log("\n");
        Bridge.log("RSDP: ");
        if let Some(r) = rsdp_addr {
            print_hex(&Bridge, r);
        } else {
            Bridge.log("None");
        }
        Bridge.log("\n");

        Bridge::init(rsdp_addr, hhdm_offset_u64);
    }

    #[cfg(target_os = "thingos")]
    {
        let mut boot_fs_device: Option<(u64, usize)> = None;
        use alloc::alloc::{alloc, Layout};
        use alloc::string::ToString;
        use core::slice;
        use hw::HardwareBridge;
        use kernel_core::sched::elf::load_elf;
        use limine_local::requests::MODULE_REQUEST;
        use x86_64::registers::control::Cr3;
        use x86_64::structures::paging::{
            FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags,
            PhysFrame, Size2MiB, Size4KiB, Translate,
        };
        use x86_64::{PhysAddr, VirtAddr};

        // Get HHDM
        let hhdm_offset_u64 = limine_local::requests::HHDM_REQUEST
            .get_response()
            .unwrap()
            .offset();
        let hhdm_offset = VirtAddr::new(hhdm_offset_u64);

        // Frame Allocator wrapping Global Allocator
        struct HeapFrameAllocator {
            hhdm_offset: VirtAddr,
        }

        unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator {
            fn allocate_frame(&mut self) -> Option<PhysFrame> {
                let layout = Layout::from_size_align(4096, 4096).ok()?;
                let ptr = unsafe { alloc(layout) };
                if ptr.is_null() {
                    return None;
                }

                let (l4_frame, _) = Cr3::read();
                let phys_l4 = l4_frame.start_address();
                let virt_l4 = self.hhdm_offset + phys_l4.as_u64();
                let page_table_ptr = virt_l4.as_mut_ptr();
                let mut mapper =
                    unsafe { OffsetPageTable::new(&mut *page_table_ptr, self.hhdm_offset) };

                let virt_addr = VirtAddr::new(ptr as u64);
                mapper
                    .translate_addr(virt_addr)
                    .map(|phys| PhysFrame::containing_address(phys))
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

        // Ensure all physical memory regions are mapped in HHDM
        // Limine is supposed to do this, but we are seeing faults in ACPI regions (Reserved/Reclaim).
        // We will iterate the memory map and map everything to be safe.

        // Helper helper? No, we can't define valid closure here easily with generic types?
        // Let's just inline the logic or define a local function if possible?
        // Rust supports specific nested functions.
        // Or just copy-paste logic for now since I can't easily refactor entire file.
        // Wait, I can define a local closure `map_region`.

        let mut map_region_fn = |start: u64, end: u64, flags: PageTableFlags| {
            let start_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(start));
            let end_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(end - 1));
            for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
                let phys = frame.start_address();
                let virt = hhdm_offset + phys.as_u64();
                if mapper.translate_addr(virt).is_none() {
                    let page = Page::<Size4KiB>::containing_address(virt);
                    unsafe {
                        if let Ok(map_to) = mapper.map_to(page, frame, flags, &mut frame_allocator)
                        {
                            map_to.flush();
                        }
                    }
                }
            }
        };

        // Redo the loop using the closure to avoid code duplication
        if let Some(resp) = limine_local::requests::MEMORY_MAP_REQUEST.get_response() {
            for entry in resp.entries() {
                use limine::memory_map::EntryType;
                match entry.entry_type {
                    EntryType::ACPI_RECLAIMABLE | EntryType::ACPI_NVS => {
                        // Map as cached RAM
                        let start = entry.base;
                        let end = start + entry.length;
                        map_region_fn(
                            start,
                            end,
                            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                        );
                    }
                    EntryType::RESERVED => {
                        // Map reserved memory if it's in likely RAM range (< 3GB)
                        let start = entry.base;
                        let end = start + entry.length;
                        if start < 0xC0000000 {
                            let end_cap = core::cmp::min(end, 0xC0000000);
                            map_region_fn(
                                start,
                                end_cap,
                                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        // Explicitly map MMIO range for HPET/IO-APIC (standard PC locations)
        // 0xFEC00000 (IO-APIC) to 0xFEEFFFFF (Local APIC / HPET usually in between)
        {
            let mmio_start = 0xFEC00000;
            let mmio_end = 0xFEF00000; // 3MB covering standard range
            map_region_fn(
                mmio_start,
                mmio_end,
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE,
            );
        }

        // Now Init ACPI & APIC
        k.bridge.log("ACPI: Pre-Init\n");
        if let Some(r) = rsdp_addr {
            unsafe {
                Bridge::init_acpi(r, hhdm_offset_u64);
                use bridge_x86_64::acpi;
                use bridge_x86_64::hpet;
                use bridge_x86_64::interrupts::{apic, ioapic, pic};

                // 1. Disable PIC
                pic::disable();
                k.bridge.log("PIC: Disabled\n");

                // 2. Init Local APIC
                // We rely on parsed MADT addr.
                if acpi::LOCAL_APIC_ADDR != 0 {
                    apic::init(acpi::LOCAL_APIC_ADDR);
                } else {
                    k.bridge.log("PANIC: No Local APIC found in MADT\n");
                    loop {}
                }

                // 3. Init IOAPIC
                if acpi::IO_APIC_ADDR != 0 {
                    ioapic::init(acpi::IO_APIC_ADDR);

                    // Route Keyboard (IRQ 1) -> Vector 33
                    // Check ISA Overrides
                    let irq1_gsi = acpi::ISA_OVERRIDES[1] as u32;
                    k.bridge.log("IOAPIC: Routing Keyboard IRQ1 -> GSI ");
                    print_hex(&Bridge, irq1_gsi as u64);
                    k.bridge.log(" -> Vector 33\n");
                    ioapic::set_irq_vector(irq1_gsi, 33, apic::id() as u8);

                    // Route Mouse (IRQ 12) -> Vector 44
                    let irq12_gsi = acpi::ISA_OVERRIDES[12] as u32;
                    k.bridge.log("IOAPIC: Routing Mouse IRQ12 -> GSI ");
                    print_hex(&Bridge, irq12_gsi as u64);
                    k.bridge.log(" -> Vector 44\n");
                    ioapic::set_irq_vector(irq12_gsi, 44, apic::id() as u8);
                } else {
                    k.bridge
                        .log("WARNING: No IOAPIC found in MADT (Input will die)\n");
                }

                // 4. Timer Setup (HPET + LAPIC Timer)
                if hpet::read_ticks() != 0 {
                    k.bridge
                        .log("HPET: Alive. Disabling Legacy Replacement (Using APIC timer)\n");
                    // We DO NOT enable legacy replacement mode if we use APIC.
                    // Actually, if we use IOAPIC for IRQ 0 (timer), we need override.
                    // BUT, we want to use Local APIC Timer for scheduling ticks!

                    // Enable Local APIC Timer -> Vector 32
                    apic::enable_timer(32);

                    // We still use HPET for *monotonic time reading* and *sleep deadlines*?
                    // Yes. But we don't need HPET interrupts for periodic ticks if LAPIC does it.
                    // WAIT. The scheduler plan was:
                    // - HPET provides monotonic time.
                    // - SLEEP relies on *some* interrupt to wake up.
                    //   If we use HPET one-shot for sleep deadlines, we need HPET interrupt.
                    //   If we user LAPIC timer for periodic ticks, that wakes us up too.

                    // Plan:
                    // 1. LAPIC Timer provides 10ms heartbeat (for now).
                    // 2. HPET provides time reading.
                    // 3. Future: Use HPET comparators for precise sleep.

                    // For now, let's stick to: LAPIC Timer = Scheduler Tick (Vector 32).
                    // HPET = Time Source.
                }
            }
        } else {
            k.bridge.log("Skipping ACPI init (No RSDP)\n");
        }

        // Boot Initialization
        {
            use abi::wire::typed::{CodecId, TypeId, TypedBytes};
            use hw::HardwareBridge;
            k.bridge.log(thing_models::milestones::KERNEL_ENTRY);
            k.bridge.log("\n");
            k.bridge.log(thing_models::milestones::BRIDGE_ONLINE);
            k.bridge.log("\n");

            k.bridge.log("THINGOS: graph init\n");
            kernel_core::graph::seed_builtins(&mut k.graph);

            // Seed Boot Root
            // We need symbol for "kernel"
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
                k.symbols
                    .seed_builtin(builtin)
                    .expect("Builtin seed failed");
            }
            k.bridge.log("THINGOS: symbols ready\n");

            unsafe { bridge_x86_64::serial::publish_serial_thing(&mut k) };

            // --- PCI ENUMERATION ---
            k.bridge.log("PCI: Scanning...\n");
            let pci_devices = unsafe { bridge_x86_64::pci::scan_pci() };
            k.bridge.log("PCI: Found ");
            print_hex(&Bridge, pci_devices.len() as u64);
            k.bridge.log(" devices\n");

            for dev in pci_devices {
                use abi::wire::typed::{CodecId, TypeId, TypedBytes};
                use thing_models::builtins::ids::*;
                use thing_models::value::ThingBody;

                // Create Thing
                let body_bytes = postcard::to_allocvec(&dev).unwrap();
                let tb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_PCI_DEVICE_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: body_bytes,
                })
                .unwrap();

                let dev_id = k.graph.create_thing(THING_PCI_DEVICE_KIND, tb);

                // Link Root -> Device
                let link = thing_models::link::LinkBody {
                    from: THING_BOOT_ROOT,
                    to: dev_id,
                    predicate: THING_HAS_DEVICE_KIND,
                };
                let lb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_LINK_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: postcard::to_allocvec(&link).unwrap(),
                })
                .unwrap();
                k.graph.create_thing(THING_LINK_KIND, lb);

                k.bridge.log("PCI: Published Device ");
                print_hex(&Bridge, dev.vendor_id as u64);
                k.bridge.log(":");
                print_hex(&Bridge, dev.device_id as u64);
                k.bridge.log("\n");

                // Check for AHCI (Mass Storage (01), SATA (06), AHCI (01))
                if dev.class_id == 0x01 && dev.subclass_id == 0x06 && dev.prog_if == 0x01 {
                    k.bridge
                        .log("PCI: Detected AHCI Controller. Mapping BAR5...\n");

                    let bar5 = dev.bars[5];
                    if bar5 != 0 && (bar5 & 1) == 0 {
                        // Memory BAR
                        let base = (bar5 & 0xFFFFFFF0) as u64;
                        // HbaMem is ~4.5KB. Map 8KB.
                        let size = 8192;

                        k.bridge.log("AHCI: Mapping BAR5 at ");
                        print_hex(&Bridge, base);
                        k.bridge.log("\n");

                        map_region_fn(
                            base,
                            base + size,
                            PageTableFlags::PRESENT
                                | PageTableFlags::WRITABLE
                                | PageTableFlags::NO_CACHE,
                        );

                        unsafe {
                            bridge_x86_64::ahci::init(&dev, &mut k);
                        }

                        // Scan for ISO9660
                        for p in 0..32 {
                            let mut buf = alloc::vec![0u8; 2048];
                            if unsafe {
                                bridge_x86_64::ahci::read_sector_at(
                                    base,
                                    p,
                                    16,
                                    &mut buf,
                                    hhdm_offset_u64,
                                )
                            } {
                                if &buf[1..6] == b"CD001" {
                                    k.bridge.log("bootfs: found iso9660 on port ");
                                    print_hex(&Bridge, p as u64);
                                    k.bridge.log("\n");
                                    boot_fs_device = Some((base, p));
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        // --- CMDLINE PARSING (Rudimentary) ---
        let mut smoke_target = None;
        {
            use limine_local::requests::KERNEL_FILE_REQUEST;
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
        let _ = boot_fs_device;
        // --- FILESYSTEM SCANNING ---
        k.bridge.log("Scanning boot filesystem...\n");
        let mut app_load_virt_base = 0x2000_0000u64;

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
            Other(alloc::string::String),
        }
        fn classify_bytes(data: &[u8]) -> ModuleType {
            if data.len() >= 4
                && data[0] == 0x7F
                && data[1] == b'E'
                && data[2] == b'L'
                && data[3] == b'F'
            {
                return ModuleType::Elf;
            }
            if data.len() >= 4
                && data[0] == 0x00
                && data[1] == 0x01
                && data[2] == 0x00
                && data[3] == 0x00
            {
                return ModuleType::Ttf;
            }
            if data.len() >= 4
                && data[0] == b'O'
                && data[1] == b'T'
                && data[2] == b'T'
                && data[3] == b'O'
            {
                return ModuleType::Otf;
            }
            if data.len() >= 4
                && data[0] == b'w'
                && data[1] == b'O'
                && data[2] == b'F'
                && data[3] == b'F'
            {
                return ModuleType::Woff;
            }
            if data.len() >= 4
                && data[0] == b'w'
                && data[1] == b'O'
                && data[2] == b'F'
                && data[3] == b'2'
            {
                return ModuleType::Woff2;
            }
            if data.len() >= 2 && data[0] == 0x36 && data[1] == 0x04 {
                return ModuleType::Psf1;
            }
            if data.len() >= 4
                && data[0] == 0x72
                && data[1] == 0xB5
                && data[2] == 0x4A
                && data[3] == 0x86
            {
                return ModuleType::Psf2;
            }
            if data.len() >= 2 && data[0] == b'B' && data[1] == b'M' {
                return ModuleType::Bmp;
            }
            if data.len() >= 4
                && data[0] == 0x89
                && data[1] == b'P'
                && data[2] == b'N'
                && data[3] == b'G'
            {
                return ModuleType::Png;
            }

            if let Some(kind) = infer::get(data) {
                match kind.mime_type() {
                    "application/x-executable"
                    | "application/x-elf"
                    | "application/x-sharedlib" => ModuleType::Elf,
                    "image/bmp" => ModuleType::Bmp,
                    "image/png" => ModuleType::Png,
                    other => ModuleType::Other(alloc::string::String::from(other)),
                }
            } else {
                ModuleType::Unknown
            }
        }

        enum ModuleRole {
            App,
            Driver,
            Debug,
            Asset,
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

        let mut process_file = |k: &mut Kernel<Bridge>,
                                parent_dir_id: Option<ThingId>,
                                name: &str,
                                data: &[u8],
                                idx: usize,
                                spawn_override: Option<bool>| {
            let mtype = classify_bytes(data);
            let role_enum = get_module_role(name, &mtype);

            use abi::wire::typed::{CodecId, TypeId, TypedBytes};
            use thing_models::builtins::ids::*;
            use thing_models::core::block::{BlockDeviceBody, BlockDeviceType};
            use thing_models::core::fs::{DirBody, FileBody, MountBody, VolumeBody};
            use thing_models::value::ThingBody;

            let file_id = {
                let file = FileBody {
                    name: alloc::string::String::from(name),
                    size: data.len() as u64,
                    lba: 0,
                    flags: 0,
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
                let link = thing_models::link::LinkBody {
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

            // --- 1. Graph: Create Module Thing ---
            use thing_models::builtins::core_kinds::{
                BitmapBody, BootProgramBody, FontBody, ModuleBody, ProgramImageBody,
            };
            use thing_models::core::process::{ProcessBody, ProcessState};

            let role_str = match role_enum {
                ModuleRole::App => "app",
                ModuleRole::Driver => "driver",
                ModuleRole::Debug => "debug",
                ModuleRole::Asset => "asset",
                ModuleRole::Ignore => "ignore",
            };
            let mime_str = match &mtype {
                ModuleType::Elf => "application/x-elf",
                ModuleType::Psf1 => "font/psf1",
                ModuleType::Psf2 => "font/psf2",
                ModuleType::Ttf => "font/ttf",
                ModuleType::Otf => "font/otf",
                ModuleType::Woff => "font/woff",
                ModuleType::Woff2 => "font/woff2",
                ModuleType::Bmp => "image/bmp",
                ModuleType::Png => "image/png",
                ModuleType::Unknown => "application/octet-stream",
                ModuleType::Other(s) => s.as_str(),
            };

            let kind_str = match &mtype {
                ModuleType::Elf => "elf",
                ModuleType::Psf1
                | ModuleType::Psf2
                | ModuleType::Ttf
                | ModuleType::Otf
                | ModuleType::Woff
                | ModuleType::Woff2 => "font",
                ModuleType::Bmp | ModuleType::Png => "bitmap",
                ModuleType::Unknown => "unknown",
                ModuleType::Other(_) => "other",
            };

            let sniff_val = if data.len() >= 4 {
                u32::from_be_bytes([data[0], data[1], data[2], data[3]])
            } else {
                0
            };

            let is_valid = match role_enum {
                ModuleRole::Ignore => false,
                _ => !matches!(mtype, ModuleType::Unknown),
            };

            let mod_body = ModuleBody {
                path: alloc::string::String::from(name),
                size_bytes: data.len() as u64,
                base_phys: 0,
                index: idx as u32,
                role: alloc::string::String::from(role_str),
                mime: alloc::string::String::from(mime_str),
                kind: alloc::string::String::from(kind_str),
                sniff: sniff_val,
                valid: is_valid,
            };

            let mod_id_bytes = postcard::to_allocvec(&mod_body).unwrap();
            let mod_tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_MODULE_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: mod_id_bytes,
            })
            .unwrap();
            let module_id = k.graph.create_thing(THING_MODULE_KIND, mod_tb);

            {
                let link = thing_models::link::LinkBody {
                    from: THING_BOOT_ROOT,
                    to: module_id,
                    predicate: THING_HAS_MODULE_KIND,
                };
                let lb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_LINK_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: postcard::to_allocvec(&link).unwrap(),
                })
                .unwrap();
                k.graph.create_thing(THING_LINK_KIND, lb);
            }

            {
                let link = thing_models::link::LinkBody {
                    from: module_id,
                    to: file_id,
                    predicate: THING_BACKED_BY_KIND,
                };
                let lb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_LINK_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: postcard::to_allocvec(&link).unwrap(),
                })
                .unwrap();
                k.graph.create_thing(THING_LINK_KIND, lb);
            }

            match &mtype {
                ModuleType::Elf => {
                    let prog_img = ProgramImageBody {
                        format: alloc::string::String::from("elf"),
                    };
                    let tb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(THING_PROGRAM_IMAGE_KIND.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&prog_img).unwrap(),
                    })
                    .unwrap();
                    let img_id = k.graph.create_thing(THING_PROGRAM_IMAGE_KIND, tb);

                    let link = thing_models::link::LinkBody {
                        from: module_id,
                        to: img_id,
                        predicate: THING_BINARY_IMAGE_KIND,
                    };
                    let lb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(THING_LINK_KIND.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&link).unwrap(),
                    })
                    .unwrap();
                    k.graph.create_thing(THING_LINK_KIND, lb);
                }
                ModuleType::Psf1
                | ModuleType::Psf2
                | ModuleType::Ttf
                | ModuleType::Otf
                | ModuleType::Woff
                | ModuleType::Woff2 => {
                    let mut w = 0u16;
                    let mut h = 0u16;
                    let mut count = 0u32;
                    let fmt_str = match mtype {
                        ModuleType::Psf1 => "psf1",
                        ModuleType::Psf2 => "psf2",
                        ModuleType::Ttf => "ttf",
                        ModuleType::Otf => "otf",
                        ModuleType::Woff => "woff",
                        ModuleType::Woff2 => "woff2",
                        _ => "unknown",
                    };

                    if matches!(mtype, ModuleType::Psf1) {
                        if data.len() >= 4 {
                            let mode = data[2];
                            let charsize = data[3];
                            w = 8;
                            h = charsize as u16;
                            count = if (mode & 1) != 0 { 512 } else { 256 };
                        }
                    } else if matches!(mtype, ModuleType::Psf2) {
                        if data.len() >= 32 {
                            let read_u32 = |off: usize| -> u32 {
                                u32::from_le_bytes([
                                    data[off],
                                    data[off + 1],
                                    data[off + 2],
                                    data[off + 3],
                                ])
                            };
                            count = read_u32(16);
                            h = read_u32(24) as u16;
                            w = read_u32(28) as u16;
                        }
                    }

                    let font_body = FontBody {
                        name: alloc::string::String::from(name),
                        format: alloc::string::String::from(fmt_str),
                        glyph_width: w,
                        glyph_height: h,
                        glyph_count: count,
                    };
                    let tb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(THING_FONT_KIND.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&font_body).unwrap(),
                    })
                    .unwrap();
                    let font_id = k.graph.create_thing(THING_FONT_KIND, tb);

                    {
                        let link = thing_models::link::LinkBody {
                            from: THING_BOOT_ROOT,
                            to: font_id,
                            predicate: THING_PROVIDES_FONT_KIND,
                        };
                        let lb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_LINK_KIND.0 as u128),
                            codec_id: CodecId::POSTCARD,
                            bytes: postcard::to_allocvec(&link).unwrap(),
                        })
                        .unwrap();
                        k.graph.create_thing(THING_LINK_KIND, lb);
                    }

                    {
                        let link = thing_models::link::LinkBody {
                            from: font_id,
                            to: module_id,
                            predicate: THING_BACKED_BY_KIND,
                        };
                        let lb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_LINK_KIND.0 as u128),
                            codec_id: CodecId::POSTCARD,
                            bytes: postcard::to_allocvec(&link).unwrap(),
                        })
                        .unwrap();
                        k.graph.create_thing(THING_LINK_KIND, lb);
                    }

                    if name.contains("NotoSans-Regular")
                        || name.contains("default")
                        || name.contains("unifont")
                        || name.contains("zap-light16")
                    {
                        let link = thing_models::link::LinkBody {
                            from: THING_BOOT_ROOT,
                            to: font_id,
                            predicate: THING_DEFAULT_FONT_KIND,
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
                _ if mime_str.starts_with("image/") => {
                    let bmp_body = BitmapBody {
                        format: alloc::string::String::from(mime_str),
                        width: 0,
                        height: 0,
                    };
                    let tb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(THING_BITMAP_KIND.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&bmp_body).unwrap(),
                    })
                    .unwrap();
                    let bmp_id = k.graph.create_thing(THING_BITMAP_KIND, tb);

                    {
                        let link = thing_models::link::LinkBody {
                            from: module_id,
                            to: bmp_id,
                            predicate: THING_ASSET_KIND,
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
                _ => {}
            }

            let should_spawn = if let Some(s) = spawn_override {
                s
            } else {
                match role_enum {
                    ModuleRole::App => true,
                    ModuleRole::Driver => {
                        k.bridge.log("Spawning driver '");
                        k.bridge.log(name);
                        k.bridge.log("'\n");
                        true
                    }
                    ModuleRole::Debug => {
                        let is_target = smoke_target.map(|t| name.contains(t)).unwrap_or(false);
                        if is_target {
                            true
                        } else {
                            k.bridge.log("Skipping debug module '");
                            k.bridge.log(name);
                            k.bridge.log("'\n");
                            false
                        }
                    }
                    ModuleRole::Ignore | ModuleRole::Asset => false,
                }
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

                        if let Some(phys) = mapper.translate_addr(page_start_virt) {
                            frame_phys = phys;
                        } else {
                            let frame = frame_allocator.allocate_frame().expect("No frames");
                            frame_phys = frame.start_address();
                            let flags = PageTableFlags::PRESENT
                                | PageTableFlags::WRITABLE
                                | PageTableFlags::USER_ACCESSIBLE;
                            unsafe {
                                if let Ok(map_to) =
                                    mapper.map_to(page, frame, flags, &mut frame_allocator)
                                {
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
                            let dest_ptr = unsafe {
                                (frame_virt.as_mut_ptr::<u8>()).add(page_offset as usize)
                            };
                            unsafe {
                                core::ptr::copy_nonoverlapping(
                                    src_ptr,
                                    dest_ptr,
                                    copy_len as usize,
                                );
                            }
                        }
                    }
                });

                if let Some(img) = loaded {
                    k.bridge.log("Loaded app entry\n");
                    let stack_bottom_virt = VirtAddr::new(current_app_base + 0x0800_0000);
                    let stack_size = 131072;
                    let stack_top_virt = stack_bottom_virt + stack_size;

                    let start_page = Page::<Size4KiB>::containing_address(stack_bottom_virt);
                    let end_page = Page::<Size4KiB>::containing_address(stack_top_virt - 1u64);

                    for page in Page::range_inclusive(start_page, end_page) {
                        let frame = frame_allocator.allocate_frame().expect("No stack frames");
                        let flags = PageTableFlags::PRESENT
                            | PageTableFlags::WRITABLE
                            | PageTableFlags::USER_ACCESSIBLE;
                        unsafe {
                            if let Ok(map_to) =
                                mapper.map_to(page, frame, flags, &mut frame_allocator)
                            {
                                map_to.flush();
                            }
                        }
                    }

                    let heap_virt_start = current_app_base + 0x0100_0000;
                    let heap_size = 256 * 1024;
                    let heap_virt_end = heap_virt_start + heap_size;

                    let start_page =
                        Page::<Size4KiB>::containing_address(VirtAddr::new(heap_virt_start));
                    let end_page =
                        Page::<Size4KiB>::containing_address(VirtAddr::new(heap_virt_end - 1u64));

                    for page in Page::range_inclusive(start_page, end_page) {
                        let frame = frame_allocator.allocate_frame().expect("No heap frames");
                        let flags = PageTableFlags::PRESENT
                            | PageTableFlags::WRITABLE
                            | PageTableFlags::USER_ACCESSIBLE;
                        unsafe {
                            if let Ok(map_to) =
                                mapper.map_to(page, frame, flags, &mut frame_allocator)
                            {
                                map_to.flush();
                                let phys = frame.start_address();
                                let virt = hhdm_offset + phys.as_u64();
                                core::ptr::write_bytes(virt.as_mut_ptr::<u8>(), 0, 4096);
                            }
                        }
                    }

                    let entry_point = current_app_base + img.entry_point;
                    k.bridge.log("Spawning app: ");
                    k.bridge.log(name);
                    k.bridge.log("\n");
                    let process_pid = (k.scheduler.processes.len() + 1) as u64;
                    k.scheduler.spawn(
                        &k.bridge,
                        name,
                        entry_point,
                        stack_top_virt.as_u64(),
                        heap_virt_start,
                    );

                    let p_body = ProcessBody {
                        pid: process_pid,
                        name: k
                            .symbols
                            .intern(name)
                            .unwrap_or(thing_models::builtins::symbols::SYM_PROCESS),
                        state: ProcessState::Running,
                    };
                    let tb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(THING_PROCESS_KIND.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&p_body).unwrap(),
                    })
                    .unwrap();
                    let process_id = k.graph.create_thing(THING_PROCESS_KIND, tb);

                    {
                        let link = thing_models::link::LinkBody {
                            from: THING_BOOT_ROOT,
                            to: process_id,
                            predicate: THING_SPAWNED_KIND,
                        };
                        let lb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_LINK_KIND.0 as u128),
                            codec_id: CodecId::POSTCARD,
                            bytes: postcard::to_allocvec(&link).unwrap(),
                        })
                        .unwrap();
                        k.graph.create_thing(THING_LINK_KIND, lb);
                    }

                    let bp = BootProgramBody {
                        name: alloc::string::String::from(name),
                        binary: alloc::string::String::from(name),
                        priority: 0,
                        entry_point: entry_point as u64,
                    };
                    let tb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(THING_BOOT_PROGRAM_KIND.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&bp).unwrap(),
                    })
                    .unwrap();
                    let prog_id = k.graph.create_thing(THING_BOOT_PROGRAM_KIND, tb);

                    {
                        let link = thing_models::link::LinkBody {
                            from: prog_id,
                            to: module_id,
                            predicate: THING_USES_MODULE_KIND,
                        };
                        let lb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_LINK_KIND.0 as u128),
                            codec_id: CodecId::POSTCARD,
                            bytes: postcard::to_allocvec(&link).unwrap(),
                        })
                        .unwrap();
                        k.graph.create_thing(THING_LINK_KIND, lb);
                    }

                    {
                        let link = thing_models::link::LinkBody {
                            from: process_id,
                            to: prog_id,
                            predicate: THING_RUNS_KIND,
                        };
                        let lb = ThingBody::from(&TypedBytes {
                            type_id: TypeId(THING_LINK_KIND.0 as u128),
                            codec_id: CodecId::POSTCARD,
                            bytes: postcard::to_allocvec(&link).unwrap(),
                        })
                        .unwrap();
                        k.graph.create_thing(THING_LINK_KIND, lb);
                    }

                    {
                        let link = thing_models::link::LinkBody {
                            from: THING_BOOT_ROOT,
                            to: prog_id,
                            predicate: THING_LAUNCHES_KIND,
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
        };

        if let Some((base, port)) = boot_fs_device {
            use kernel_core::fs::iso9660::Iso9660Reader;
            if let Some(mut iso) = Iso9660Reader::new(|lba, buf: &mut [u8]| unsafe {
                bridge_x86_64::ahci::read_sector_at(base, port, lba, buf, hhdm_offset_u64)
            }) {
                k.bridge.log("bootfs: ISO Reader Ready.\n");

                // Create /boot Mount and Dirs
                let (apps_dir_id, drivers_dir_id, fonts_dir_id) = {
                    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
                    use thing_models::builtins::ids::*;
                    use thing_models::value::ThingBody;
                    use thing_models::core::fs::{MountBody, DirBody};
                    use abi::ThingId;

                    // 1. Mount "/boot"
                    let m_body = MountBody {
                        path: alloc::string::String::from("/boot"),
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
                    let l_root = thing_models::link::LinkBody {
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

                    // 2. Root Dir (of the mount)
                    let d_body = DirBody {
                        name: alloc::string::String::from("/boot"),
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
                    let l_mnt = thing_models::link::LinkBody {
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

                    // Helper to make dir
                    let mut make_dir = |name: &str, parent: ThingId| {
                        let body = DirBody {
                            name: alloc::string::String::from(name),
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
                        let l = thing_models::link::LinkBody {
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

                    (apps, drivers, fonts)
                };

                // 1. Read Policy (init.txt)
                let mut whitelist = None;
                if let Some(h) = iso.open("/boot/init.txt") {
                    let mut data = alloc::vec![0u8; h.size as usize];
                    iso.read(&h, 0, h.size as usize, &mut data);
                    if let Ok(s) = core::str::from_utf8(&data) {
                        let list: alloc::vec::Vec<alloc::string::String> = s
                            .lines()
                            .map(|l| l.trim().to_ascii_lowercase())
                            .filter(|l| !l.is_empty())
                            .collect();
                        k.bridge.log("bootfs: init.txt policy loaded (");
                        use alloc::string::ToString; // Ensure we can print number or just ignore
                        k.bridge.log(" entries)\n");
                        whitelist = Some(list);
                    }
                }

                // Helper to scan and spawn
                // We need to move `iso` into closure? No, we can borrow iso.
                // But `load_elf` inside `process_file` uses `mapper` which is borrowed check...
                // `process_file` borrows `k`.
                // We are inside a big unsafe block or big scope.

                // 2. Scan /boot/apps
                k.bridge.log("bootfs: scanning /boot/apps\n");
                let mut entries = iso.read_dir("/boot/apps").unwrap_or_default();
                entries.sort_by(|a, b| a.name.cmp(&b.name));
                k.bridge.log("bootfs: found ");
                print_hex(&Bridge, entries.len() as u64);
                k.bridge.log(" entries in apps\n");

                for entry in entries {
                    let name = &entry.name;
                    if entry.is_dir {
                        continue;
                    }

                    let should_run = if !name.ends_with(".elf") {
                        k.bridge.log("init: skipped ");
                        k.bridge.log(name);
                        k.bridge.log(" (not .elf)\n");
                        false
                    } else {
                        if let Some(wl) = &whitelist {
                            if wl.contains(name) {
                                true
                            } else {
                                k.bridge.log("init: policy skipped ");
                                k.bridge.log(name);
                                k.bridge.log("\n");
                                false
                            }
                        } else {
                            true
                        }
                    };

                    if !should_run {
                        // We might still want to intern them as 'files' or 'assets'?
                        // For now, let's skip totally if it's just noise, OR load as asset?
                        // User said "Skip with reason otherwise (log it)".
                        // But if I want assets to be available (e.g. README), I should `process_file` with spawn=false.
                        // But `process_file` does heavy lifting.
                        // Let's load everything but spawn only candidates.
                    }

                    let path = alloc::format!("/boot/apps/{}", entry.name); // Using normalized name for path
                    if let Some(handle) = iso.open(&path) {
                        let mut data = alloc::vec![0u8; handle.size as usize];
                        iso.read(&handle, 0, handle.size as usize, &mut data);
                        // We pass should_run to process_file
                        process_file(&mut k, Some(apps_dir_id), &path, &data, 0, Some(should_run));
                    }
                }

                // 3. Scan /boot/drivers (Optional scope)
                k.bridge.log("bootfs: scanning /boot/drivers\n");
                let mut drv_entries = iso.read_dir("/boot/drivers").unwrap_or_default();
                drv_entries.sort_by(|a, b| a.name.cmp(&b.name));

                for entry in drv_entries {
                    if entry.is_dir {
                        continue;
                    }
                    let name = &entry.name;

                    // Drivers: Only spawn if in whitelist!
                    let should_run = if let Some(wl) = &whitelist {
                        if wl.contains(name) {
                            true
                        } else {
                            false
                        }
                    } else {
                        false // Default: Do NOT auto-run drivers unless listed
                    };

                    let path = alloc::format!("/boot/drivers/{}", entry.name);
                    if let Some(handle) = iso.open(&path) {
                        let mut data = alloc::vec![0u8; handle.size as usize];
                        iso.read(&handle, 0, handle.size as usize, &mut data);
                        process_file(
                            &mut k,
                            Some(drivers_dir_id),
                            &path,
                            &data,
                            0,
                            Some(should_run),
                        );
                    }
                }

                // 4. Scan /boot/fonts (Legacy/Asset) - Scan but don't spawn
                let fonts = iso.read_dir("/boot/fonts").unwrap_or_default();
                for entry in fonts {
                    let path = alloc::format!("/boot/fonts/{}", entry.name);
                    if let Some(handle) = iso.open(&path) {
                        let mut data = alloc::vec![0u8; handle.size as usize];
                        iso.read(&handle, 0, handle.size as usize, &mut data);
                        process_file(&mut k, Some(fonts_dir_id), &path, &data, 0, Some(false));
                    }
                }
            }
        } else {
            if let Some(resp) = MODULE_REQUEST.get_response() {
                for (idx, module) in resp.modules().iter().enumerate() {
                    let name = module.path().to_str().unwrap_or("unknown");
                    let base = module.addr();
                    let len = module.size() as usize;
                    let data = unsafe { slice::from_raw_parts(base, len) };
                    process_file(&mut k, None, name, data, idx, None);
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

        // Flush Diagnostics
        if let Some(mut guard) = KERNEL.try_lock() {
            if let Some(k) = (*guard).as_mut() {
                kernel_core::diag::flusher::flush_diagnostics(k);
            }
        }

        bridge.idle();
    }
}
