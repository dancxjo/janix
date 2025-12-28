#![no_std]
#![no_main]
#![cfg_attr(target_os = "thingos", feature(alloc_error_handler))]

extern crate alloc;

#[cfg(target_os = "thingos")]
mod early_log;
pub mod font;
pub mod framebuffer;
#[cfg(target_os = "thingos")]
mod heap;
#[cfg(target_os = "thingos")]
mod memory_intrinsics;

#[cfg(target_os = "thingos")]
pub(crate) mod limine_local;

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
use abi::ThingId;
use models as thing_models;
use spin::Mutex;

pub mod loader;
use loader::{process_file, scan_boot_fs_task, ScanArgs};

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

#[cfg(not(target_os = "thingos"))]
fn main() {}

#[cfg(target_os = "thingos")]
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    #[cfg(target_os = "thingos")]
    unsafe {
        use hw::HardwareBridge;
        let bridge = Bridge;
        framebuffer::fill_framebuffer_progress(
            framebuffer::BOOT_DOMINANT_COLOR,
            0.1,
            Some("Booting ThingOS..."),
        );
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
        .map(|r: &limine::response::HhdmResponse| r.offset())
        .unwrap_or(0);
    // Limine address() returns usize, cast to u64
    let rsdp_addr = limine_local::requests::RSDP_REQUEST
        .get_response()
        .map(|r: &limine::response::RsdpResponse| r.address() as u64);

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
        framebuffer::fill_framebuffer_progress(
            framebuffer::BOOT_DOMINANT_COLOR,
            0.25,
            Some("Heap & Bridge Init"),
        );
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
            framebuffer::fill_framebuffer_progress(
                framebuffer::BOOT_DOMINANT_COLOR,
                0.4,
                Some("Graph Seeded"),
            );

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

                framebuffer::fill_framebuffer_progress(
                    framebuffer::BOOT_DOMINANT_COLOR,
                    0.6,
                    Some("PCI Scanned"),
                );

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

        // --- FRAMEBUFFER PUBLICATION ---
        {
            use limine_local::requests::FRAMEBUFFER_REQUEST;
            if let Some(resp) = FRAMEBUFFER_REQUEST.get_response() {
                if let Some(fb) = resp.framebuffers().next() {
                    k.bridge
                        .log("FRAMEBUFFER: Publishing DisplayFramebuffer...\n");

                    // Schema: DisplayFramebuffer { width, height, pitch, format, address }
                    // We need to define the struct body or use a generic Map/Struct if schema allows.
                    // The Types are not defined in `models` yet?
                    // We need to verify `models/src/lib.rs` or define a local struct.
                    // For now, let's assume we can define a struct here or use a dynamic one?
                    // Postcard requires a defined struct.

                    use thing_models::builtins::core_kinds::DisplayFramebufferBody;

                    let kind_id = thing_models::builtins::ids::THING_DISPLAY_FRAMEBUFFER_KIND;

                    let fb_virt_raw = fb.addr() as u64;
                    let phys_addr = mapper
                        .translate_addr(VirtAddr::new(fb_virt_raw))
                        .expect("Framebuffer not mapped in kernel")
                        .as_u64();

                    let virt_addr = 0x1_0000_0000; // 4GB Base
                    let size = (fb.pitch() * fb.height()) as u64;

                    k.bridge.log("FRAMEBUFFER: Mapping Phys ");
                    print_hex(&Bridge, phys_addr);
                    k.bridge.log(" -> Virt ");
                    print_hex(&Bridge, virt_addr);
                    k.bridge.log("\n");

                    // Manual mapping to handle Virtual Address != HHDM
                    {
                        use x86_64::structures::paging::PageTableFlags as Flags;
                        let start_frame =
                            PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(phys_addr));
                        let end_frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(
                            phys_addr + size - 1,
                        ));

                        let start_page =
                            Page::<Size4KiB>::containing_address(VirtAddr::new(virt_addr));

                        let mut page_iter = Page::range_inclusive(
                            start_page,
                            start_page + (end_frame - start_frame),
                        );

                        for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
                            let page = page_iter.next().unwrap();
                            unsafe {
                                if let Ok(map_to) = mapper.map_to(
                                    page,
                                    frame,
                                    Flags::PRESENT
                                        | Flags::WRITABLE
                                        | Flags::USER_ACCESSIBLE
                                        | Flags::NO_CACHE,
                                    &mut frame_allocator,
                                ) {
                                    map_to.flush();
                                }
                            }
                        }
                    }

                    let body = DisplayFramebufferBody {
                        width: fb.width(),
                        height: fb.height(),
                        pitch: fb.pitch(),
                        format: 32, // BGRA typically
                        address: virt_addr,
                    };

                    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
                    use thing_models::value::ThingBody;

                    let tb = ThingBody::from(&TypedBytes {
                        type_id: TypeId(kind_id.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&body).unwrap(),
                    })
                    .unwrap();

                    let fb_id = k.graph.create_thing(kind_id, tb);

                    // Link Root -> Framebuffer? Or Display -> Framebuffer?
                    // Root -> DisplayFramebuffer
                    let l = thing_models::link::LinkBody {
                        from: thing_models::builtins::ids::THING_BOOT_ROOT,
                        to: fb_id,
                        predicate: thing_models::builtins::ids::THING_HAS_DEVICE_KIND,
                    };
                    let lbs = ThingBody::from(&TypedBytes {
                        type_id: TypeId(thing_models::builtins::ids::THING_LINK_KIND.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&l).unwrap(),
                    })
                    .unwrap();
                    k.graph
                        .create_thing(thing_models::builtins::ids::THING_LINK_KIND, lbs);

                    k.bridge.log("FRAMEBUFFER: Published.\n");
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
        // --- THREADED BOOT LAUNCH ---

        // 1. Publish Kernel & Enable Interrupts Early
        // This allows tasks to be spawned and run immediately.
        {
            bridge_x86_64::set_tick_hook(scheduler_tick);
            bridge_x86_64::interrupts::syscall::set_syscall_hook(syscall_hook);
            *KERNEL.lock() = Some(k);

            use hw::HardwareBridge;
            let bridge = Bridge;
            // bridge.log("BRIDGE: enabling interrupts (threaded-boot)...\n");
            // x86_64::instructions::interrupts::enable(); // Moved to end
        }

        // 2. Spawn Boot Scanner (Async)
        // DEFERRED: Bootfs scanning is disabled in favor of direct module loading.
        /*
        if let Some((base, port)) = boot_fs_device {
             use alloc::boxed::Box;
             let args = ScanArgs {
                base, // HBA Base
                port, // Port Index
                hhdm: hhdm_offset_u64
             };

             let args_box = Box::new(args);
             let args_ptr = Box::into_raw(args_box) as u64;

             // Allocate Aligned Stack for Boot Scanner (64KB)
             let layout = Layout::from_size_align(64 * 1024, 16).unwrap();
             let stack_ptr = unsafe { alloc(layout) };
             // Subtract 8 to satisfy System V ABI (stack = 8 mod 16 at entry)
             let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 } - 8;
             // We intentionally leak this memory (no counterpart dealloc yet)

             let mut k_lock = KERNEL.lock();
             if let Some(k) = k_lock.as_mut() {
                  k.scheduler.spawn(&k.bridge, "boot_scanner", scan_boot_fs_task as u64 as u64, stack_top, args_ptr);
             }
        }
        */

        // 3. Load Ramdisk Modules (Synchronous fallback)
        {
            use limine_local::requests::MODULE_REQUEST;
            if let Some(resp) = MODULE_REQUEST.get_response() {
                let mut guard = KERNEL.lock();
                if let Some(k) = guard.as_mut() {
                    // --- Synthesize Directory Structure ---
                    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
                    use thing_models::builtins::ids::*;
                    use thing_models::core::fs::{DirBody, MountBody};
                    use thing_models::value::ThingBody;

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

                    // 2. Root Dir "/boot"
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

                    let apps_dir_id = make_dir("apps", root_id);
                    let drivers_dir_id = make_dir("drivers", root_id);
                    let fonts_dir_id = make_dir("fonts", root_id);

                    for (idx, module) in resp.modules().iter().enumerate() {
                        let full_path = module.path().to_str().unwrap_or("unknown");
                        let name = full_path.rsplit('/').next().unwrap_or(full_path);
                        let base = module.addr();
                        let len = module.size() as usize;
                        let data = unsafe { slice::from_raw_parts(base, len) };

                        // Determine Parent
                        let parent_id = if name.ends_with(".ttf") {
                            Some(fonts_dir_id)
                        } else if name.contains("driver")
                            || name.contains("ps2_")
                            || name.contains("rtc_")
                        {
                            Some(drivers_dir_id)
                        } else if name.ends_with(".elf") {
                            Some(apps_dir_id)
                        } else {
                            Some(root_id)
                        };

                        process_file(k, parent_id, name, data, idx, None, hhdm_offset_u64);
                    }
                }
            }
            framebuffer::fill_framebuffer_progress(
                framebuffer::BOOT_DOMINANT_COLOR,
                0.8,
                Some("Modules Loaded"),
            );
        }

        // 4. Enable Interrupts (Start Scheduler)
        {
            use hw::HardwareBridge;
            framebuffer::fill_framebuffer_progress(
                framebuffer::BOOT_DOMINANT_COLOR,
                1.0,
                Some("System Ready"),
            );
            Bridge.log("BRIDGE: enabling interrupts (threaded-boot)...\n");
            x86_64::instructions::interrupts::enable();
        }
    } // End cfg(target_os="thingos") block

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
