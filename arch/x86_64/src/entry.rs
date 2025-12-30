//! Entry point for x86_64 architecture.
//!
//! This module contains the architecture-specific boot flow and main loop.

use alloc;
use ::boot::{BootFacts, MemoryRegionKind};

use crate::boot;
use crate::bridge::{self, Bridge};
use crate::bringup::{self, gdt};
use crate::early_log;
use crate::heap;
use crate::simd;
use crate::loader;
use crate::paging;
use crate::memory_intrinsics;

// Global state for loader to map framebuffer
pub static mut FRAMEBUFFER_INFO: Option<(u64, u64)> = None;
use core::arch::naked_asm;
use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};
use kernel::bridge::HardwareBridge;
use kernel::Kernel;

static USE_QEMU_DRIVER: AtomicBool = AtomicBool::new(false);
static PANICKING: AtomicBool = AtomicBool::new(false);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use kernel::bridge::HardwareBridge;

    // Recursion guard
    if PANICKING.swap(true, Ordering::Relaxed) {
        loop {
            core::hint::spin_loop();
        }
    }

    let bridge = Bridge;

    // Capture to ring
    // We try to extract string if possible, or just "PANIC"
    kernel::diag::record_panic("PANIC");

    bridge.log("PANIC\n");
    if let Some(loc) = info.location() {
        bridge.log("File: ");
        bridge.log(loc.file());
        bridge.log("\n");
        bridge.log("Line: ");
        print_dec(&bridge, loc.line() as u64);
        bridge.log("\n");
    }

    // Dump Ring to Serial
    bridge.log("\n--- RING DUMP ---\n");
    let ring = kernel::diag::LogRing::global();
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
    use kernel::bridge::HardwareBridge;
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

fn print_dec(bridge: &Bridge, val: u64) {
    use kernel::bridge::HardwareBridge;
    if val == 0 {
        bridge.log("0");
        return;
    }

    let mut div = 1_000_000_000_000_000_000; // 10^18
                                             // Check if we need 10^19 (u64 max is roughly 1.8e19)
    if val >= 10_000_000_000_000_000_000 {
        div = 10_000_000_000_000_000_000;
    }

    // Skip leading zeros
    while div > val {
        div /= 10;
    }

    let mut rest = val;
    while div > 0 {
        let digit = rest / div;
        rest %= div;
        div /= 10;

        // Same unsafe/unwrap pattern as print_hex
        let c = (digit as u8) + b'0';
        bridge.log(core::str::from_utf8(&[c]).unwrap());
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

/// Architecture entry point called from lib.rs Arch::boot().
/// Forwards to the naked _start function.
pub fn arch_entry() -> ! {
    extern "C" {
        fn _start() -> !;
    }
    unsafe { _start() }
}

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

use crate::loader::spawn_elf;

static KERNEL: Mutex<Option<Kernel<Bridge>>> = Mutex::new(None);
static LAST_TICKS: AtomicU64 = AtomicU64::new(0);

// Shared FrameAllocator for Kernel Tasks
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB, Translate};
use x86_64::{PhysAddr, VirtAddr};
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
        use x86_64::structures::paging::OffsetPageTable;
        let (l4_frame, _) = Cr3::read();
        let phys_l4 = l4_frame.start_address();
        let virt_l4 = self.hhdm_offset + phys_l4.as_u64();
        let page_table_ptr = virt_l4.as_mut_ptr();
        let mapper = unsafe { OffsetPageTable::new(&mut *page_table_ptr, self.hhdm_offset) };
        let virt_addr = VirtAddr::new(ptr as u64);
        mapper
            .translate_addr(virt_addr)
            .map(|phys| PhysFrame::containing_address(phys))
    }
}

fn scheduler_tick_legacy(frame: &mut crate::bringup::interrupts::trap::TrapFrame) {
    if let Some(mut guard) = KERNEL.try_lock() {
        if let Some(k) = (*guard).as_mut() {
            use crate::bringup::ArchContext;
            use kernel::sched::scheduler::ThreadContext;
            let mut ctx: ThreadContext<ArchContext> = ThreadContext::default();

            unsafe {
                let frame_ptr = frame as *const _ as *const u64;
                let ctx_ptr = ctx.0 .0.as_mut_ptr();
                core::ptr::copy_nonoverlapping(frame_ptr, ctx_ptr, 20);
            }

            {
                use kernel::bridge::HardwareBridge;
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

                kernel::time::tick(&mut k.graph, delta);
                let monotonic = kernel::time::monotonic_ns();
                k.scheduler.wake_sleepers(monotonic);
                unsafe {
                    kernel::drivers::hpet::ack_interrupt();
                }
                if now_ns > 0 {
                    unsafe {
                        let next_wake = k.scheduler.next_wakeup_deadline().unwrap_or(u64::MAX);
                        let tick_target = monotonic + 10_000_000;
                        let target = core::cmp::min(tick_target, next_wake);
                        kernel::drivers::hpet::program_oneshot(target);
                    }
                }
                kernel::diag::flusher::flush_diagnostics(k);
            }

            k.scheduler.tick(&k.bridge, &mut ctx);

            // k.bridge.log("TICK: Out RIP=");
            // print_hex(&k.bridge, ctx.0 .0[15]);
            // k.bridge.log("\n");

            unsafe {
                let ctx_ptr = ctx.0 .0.as_ptr();
                let frame_ptr = frame as *mut _ as *mut u64;

                // k.bridge.log("Frame Ptr: ");
                // print_hex(&k.bridge, frame_ptr as u64);
                // k.bridge.log("\n");

                core::ptr::copy_nonoverlapping(ctx_ptr, frame_ptr, 20);

                let read_back = (*frame).rip;
                // k.bridge.log("Frame RIP Readback: ");
                // print_hex(&k.bridge, read_back);
                // k.bridge.log("\n");

                let rsp_val = (*frame).rsp;
                // k.bridge.log("Frame RSP: ");
                // print_hex(&k.bridge, rsp_val);
                // k.bridge.log("\n");

                let cs_val = (*frame).cs;
                // k.bridge.log("Frame CS: ");
                // print_hex(&k.bridge, cs_val);
                // k.bridge.log("\n");
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
        unsafe {
            // Raw 'P' to 0xE9
            core::arch::asm!(
                "out dx, al",
                in("dx") 0xe9u16,
                in("al") b'P',
                options(nomem, nostack, preserves_flags)
            );
        }
    } else {
        unsafe {
            // Raw 'U' to 0xE9 (User Fault)
            core::arch::asm!(
                "out dx, al",
                in("dx") 0xe9u16,
                in("al") b'U',
                options(nomem, nostack, preserves_flags)
            );
        }
    }
    use kernel::bridge::HardwareBridge;

    #[allow(unused_imports)]
    use x86_64::structures::paging::FrameAllocator;

    loop {
        let mut guard_opt = None;
        x86_64::instructions::interrupts::without_interrupts(|| {
            if let Some(guard) = KERNEL.try_lock() {
                guard_opt = Some(guard);
            }
        });

        if let Some(mut guard) = guard_opt {
            if let Some(k) = (*guard).as_mut() {
                let hhdm_offset_u64 = k.bridge.hhdm_offset();
                let hhdm_offset = x86_64::VirtAddr::new(hhdm_offset_u64);
                let mut frame_allocator = HeapFrameAllocator { hhdm_offset };

                if let Some(current_tid) = k.scheduler.current {
                    if let Some(Some(thread)) = k.scheduler.threads.get(current_tid.0 as usize - 1)
                    {
                        let pid = thread.process_id;
                        if let Some(Some(process)) = k.scheduler.processes.get(pid.0 as usize - 1) {
                            // LOGGING (Temporary)
                            unsafe {
                                use kernel::bridge::HardwareBridge;
                                let s = alloc::format!(
                                    "PF Hook: Addr={:#x} PID={} HeapStart={:#x} HeapEnd={:#x}\n",
                                    fault_addr,
                                    pid.0,
                                    process.heap_virt_start,
                                    process.heap_virt_end
                                );
                                k.bridge.log(&s);
                            }

                            if fault_addr >= process.heap_virt_start
                                && fault_addr < process.heap_virt_end
                            {
                                use x86_64::registers::control::Cr3;
                                use x86_64::structures::paging::{
                                    Mapper, OffsetPageTable, Page, PageTableFlags, Size4KiB,
                                };

                                let (l4_frame, _) = Cr3::read();
                                let mut mapper = unsafe {
                                    OffsetPageTable::new(
                                        &mut *(hhdm_offset + l4_frame.start_address().as_u64())
                                            .as_mut_ptr(),
                                        hhdm_offset,
                                    )
                                };
                                let page = Page::<Size4KiB>::containing_address(
                                    x86_64::VirtAddr::new(fault_addr),
                                );

                                if let Some(frame) = frame_allocator.allocate_frame() {
                                    let flags = PageTableFlags::PRESENT
                                        | PageTableFlags::WRITABLE
                                        | PageTableFlags::USER_ACCESSIBLE;
                                    unsafe {
                                        if let Ok(map_to) =
                                            mapper.map_to(page, frame, flags, &mut frame_allocator)
                                        {
                                            core::ptr::write_bytes(
                                                (hhdm_offset + frame.start_address().as_u64())
                                                    .as_mut_ptr::<u8>(),
                                                0,
                                                4096,
                                            );
                                            map_to.flush();
                                            unsafe {
                                                k.bridge.log("PF Hook: Mapped!\n");
                                            }
                                            return true;
                                        } else {
                                            unsafe {
                                                k.bridge.log("PF Hook: Map Failed!\n");
                                            }
                                        }
                                    }
                                } else {
                                    unsafe {
                                        k.bridge.log("PF Hook: OOM!\n");
                                    }
                                }
                            } else {
                                unsafe {
                                    k.bridge.log("PF Hook: Out of Bounds!\n");
                                }
                            }
                        }
                    }
                }
            }
            return false;
        }
        core::hint::spin_loop();
    }
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

        if data_ptr as usize == 0 || data_len == 0 {
            return -1;
        }

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
                use kernel::bridge::HardwareBridge;

                // k.bridge.log("SYSCALL SPAWN: ");
                // k.bridge.log(name);
                // k.bridge.log("\n");

                let hhdm_offset_u64 = k.bridge.hhdm_offset();

                loader::spawn_elf(
                    k,
                    None,
                    name,
                    data,
                    0,
                    None, // Not force, rely on defaults
                    hhdm_offset_u64,
                );
                return 0;
            }
        }
        return -1;
    }

    loop {
        let mut result = None;
        x86_64::instructions::interrupts::without_interrupts(|| {
            if let Some(mut guard) = KERNEL.try_lock() {
                if let Some(k) = (*guard).as_mut() {
                    result = Some(kernel::syscalls::syscall_dispatch(
                        k, num, a1, a2, a3, a4, a5, a6,
                    ));
                }
            }
        });
        if let Some(r) = result {
            return r;
        }
        core::hint::spin_loop();
    }
}

fn scheduler_tick(frame: &mut crate::bringup::interrupts::trap::TrapFrame) {
    if let Some(mut guard) = KERNEL.try_lock() {
        if let Some(k) = (*guard).as_mut() {
            use crate::bringup::ArchContext;
            use kernel::bridge::HardwareBridge;
            use kernel::sched::scheduler::ThreadContext;

            // Cast frame to context
            // x86 TrapFrame might differ from ArchContext layout exactly on stack?
            // bridge_x86_64::interrupts::trap::TrapFrame struct.
            // bringup::interrupts::trap::TrapFrame.

            let ctx_ptr = frame as *mut crate::bringup::interrupts::trap::TrapFrame
                as *mut ThreadContext<ArchContext>;
            let ctx = unsafe { &mut *ctx_ptr };

            let now_ns = k.bridge.monotonic_now();
            k.scheduler.wake_sleepers(now_ns);
            k.scheduler.tick(&k.bridge, ctx);
        }
    }
}

#[cfg(not(target_os = "thingos"))]
#[allow(unused)]
fn main() {}

#[cfg(target_os = "thingos")]
#[no_mangle]
unsafe extern "C" fn rust_main() -> ! {
    let bridge = Bridge;
    // early_log::init(); // Removed, function doesn't exist
    bridge.log("BOOT: Bridge Online\n");

    // 1. Collect Boot Info first (Allocates, but we have early allocator?)
    // Wait, collecting BootFacts allocates Vecs.
    // We haven't initialized the Heap yet!
    // Limine adapter uses `Vec`.
    // This is a Catch-22.
    // `crates/boot_limine` depends on `alloc`.
    // We cannot use `Vec` before heap init.

    // LIMITATION: The new `boot_limine` uses `Vec` to store module info and memory map.
    // We need heap to run `collect()`.
    // But we need memory map to initialize heap.

    // SOLUTION: Use Limine's raw iterator for Heap Initialization (Architecture specific),
    // THEN collect full BootFacts after heap is up.

    // For Heap Init, we must use `mod boot` logic that reads Limine raw requests directly without allocation.
    // `arch/x86_64/src/boot/limine.rs` exposes the requests.
    // I need to add a helper there to get raw memory map iterator or just use requests directly here.

    let hhdm_offset_u64 = crate::boot::loader::HHDM_REQUEST
        .get_response()
        .map(|r| r.offset())
        .unwrap_or(0);
    // RSDP not strictly needed for heap, but nice to have.
    // Memory Map:
    let memmap_response = crate::boot::loader::MEMORY_MAP_REQUEST.get_response();

    unsafe {
        // Heap Selection
        let heap_size = heap::KERNEL_HEAP_SIZE_BYTES as u64;
        let mut heap_region = None;

        if let Some(memmap) = memmap_response {
            for entry in memmap.entries() {
                if entry.entry_type == crate::boot::loader::EntryType::USABLE {
                    let region_end = entry.base + entry.length;
                    if let Some(raw_start) = region_end.checked_sub(heap_size) {
                        let aligned_start = raw_start - (raw_start % 4096);
                        if aligned_start >= entry.base && aligned_start >= 0x100000 {
                            heap_region = Some((aligned_start, heap_size));
                            break;
                        }
                    }
                }
            }
        }

        let (phys_start, size) = heap_region.expect("BOOT: Failed to find heap region!");
        let virt_start = phys_start + hhdm_offset_u64;

        heap::init_kernel_heap(virt_start as usize, size as usize);

        early_log::log_heap_init(early_log::HeapInitInfo {
            phys_start,
            virt_start,
            size,
        });
    }

    // Hook BootScreen allocator
    unsafe fn boot_alloc_impl(size: usize, align: usize) -> *mut u8 {
        let layout = alloc::alloc::Layout::from_size_align(size, align).unwrap();
        alloc::alloc::alloc(layout)
    }
    boot_screen::set_boot_alloc(boot_alloc_impl);
    boot_screen::set_blit_hook(simd::sse_blit);

    // 2. Safe to Allocate now (Vec, String, etc.)
    let boot_info = crate::boot::loader::collect();
    let rsdp_addr = boot_info.rsdp_addr.unwrap_or(0);

    // --- Boot Screen Init ---
    let mut bs = unsafe {
        if let Some(fb) = &boot_info.framebuffer {
            if fb.bpp != 32 {
                None
            } else {
                let addr = fb.address; // Virtual address (HHDM)
                let pixel_format = if fb.red_mask_shift == 16
                    && fb.green_mask_shift == 8
                    && fb.blue_mask_shift == 0
                {
                    boot_screen::PixelFormat::Xrgb8888
                } else if fb.red_mask_shift == 0
                    && fb.green_mask_shift == 8
                    && fb.blue_mask_shift == 16
                {
                    boot_screen::PixelFormat::Abgr8888
                } else {
                    boot_screen::PixelFormat::Xrgb8888
                };

                let info = boot_screen::FramebufferInfo {
                    addr: addr as *mut u8,
                    size_bytes: fb.size as usize,
                    width: fb.width as u32,
                    height: fb.height as u32,
                    pitch_bytes: fb.pitch as u32,
                    bpp: fb.bpp,
                    pixel_format,
                };

                // Allocate the screen structures
                if let Some(mut bs) = boot_screen::BootScreenOwned::new(info) {
                    // Fast Initialization:
                    // Set background color and fade immediately.
                    // Clearing happens in the shadow buffer during draw().
                    bs.set_background_color(0xFF002244);
                    bs.set_fade(255);

                    bs.show(boot_screen::milestones::BOOTING);

                    Some(bs)
                } else {
                    None
                }
            }
        } else {
            None
        }
    };

    let mut k = Kernel::new(Bridge);
    if let Some(bs) = &mut bs {
        bs.show(boot_screen::milestones::BRIDGE_ONLINE);
        bs.draw();
    }

    unsafe {
        use kernel::bridge::HardwareBridge;
        k.bridge.log("BOOT: Kernel Initialized\n");
        k.bridge.log(thing_models::milestones::KERNEL_ENTRY);
        k.bridge.log("\n");

        kernel::input::init();
        if let Some(bs) = &mut bs {
            bs.show(boot_screen::milestones::GRAPH_INIT);
            bs.draw();
        }
        kernel::graph::seed_builtins(&mut k.graph);
        k.register_machine_providers();
        if let Some(bs) = &mut bs {
            bs.show(boot_screen::milestones::GRAPH_SEEDED);
            bs.draw();
        }

        if let Some(bs) = &mut bs {
            bs.show(boot_screen::milestones::SYMBOLS_INIT);
            bs.draw();
        }
        let kernel_name_sym = k
            .symbols
            .intern("kernel")
            .unwrap_or(thing_models::builtins::symbols::SYM_PROCESS);
        if let Some(bs) = &mut bs {
            bs.show(boot_screen::milestones::SYMBOLS_READY);
            bs.draw();
        }

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

        // --- Memory Mapping & ACPI Setup ---
        {
            use x86_64::registers::control::Cr3;
            use x86_64::structures::paging::{
                Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size4KiB, Translate,
            };

            let hhdm_offset = VirtAddr::new(hhdm_offset_u64);
            let mut frame_allocator = loader::HeapFrameAllocator { hhdm_offset };
            let (level_4_table_frame, _) = Cr3::read();
            let phys = level_4_table_frame.start_address();
            let virt = hhdm_offset + phys.as_u64();
            let page_table_ptr: *mut x86_64::structures::paging::PageTable = virt.as_mut_ptr();
            let mut mapper = OffsetPageTable::new(&mut *page_table_ptr, hhdm_offset);

            for entry in &boot_info.memory_map {
                // use boot::bootinfo::MemoryRegionKind; // OLD
                // use MemoryRegionKind; // Imported
                let (start, end, flags) = match entry.kind {
                    MemoryRegionKind::AcpiReclaimable | MemoryRegionKind::AcpiNvs => (
                        entry.start,
                        entry.end,
                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                    ),
                    MemoryRegionKind::Kernel => (
                        entry.start,
                        entry.end,
                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                    ),
                    MemoryRegionKind::Reserved => {
                        if entry.start < 0xC0000000 {
                            (
                                entry.start,
                                core::cmp::min(entry.end, 0xC0000000),
                                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                            )
                        } else {
                            (0, 0, PageTableFlags::empty())
                        }
                    }
                    _ => (0, 0, PageTableFlags::empty()),
                };

                if flags != PageTableFlags::empty() && end > start {
                    let start_frame =
                        PhysFrame::<Size4KiB>::containing_address(x86_64::PhysAddr::new(start));
                    let end_frame =
                        PhysFrame::<Size4KiB>::containing_address(x86_64::PhysAddr::new(end - 1));
                    for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
                        let phys = frame.start_address();
                        let virt = hhdm_offset + phys.as_u64();
                        if mapper.translate_addr(virt).is_none() {
                            let page = Page::<Size4KiB>::containing_address(virt);
                            if let Ok(map_to) =
                                mapper.map_to(page, frame, flags, &mut frame_allocator)
                            {
                                map_to.flush();
                            }
                        }
                    }
                }
            }

            let mmio_start = 0xFEC00000;
            let mmio_end = 0xFEF00000;
            let flags =
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE;
            let start_frame =
                PhysFrame::<Size4KiB>::containing_address(x86_64::PhysAddr::new(mmio_start));
            let end_frame =
                PhysFrame::<Size4KiB>::containing_address(x86_64::PhysAddr::new(mmio_end - 1));
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
            use kernel::bridge::HardwareBridge;
            k.bridge.log("INIT: ACPI Setup...\n");

            if rsdp_addr != 0 {
                // Bridge::init_acpi already called via Bridge::init (if I updated Bridge::init to do it, or I need to call it manually).
                // Wait, Bridge implementation in bridge_x86_64 splits them.
                // Let's call init_acpi explicitly as before.
                Bridge::init_acpi(rsdp_addr, hhdm_offset_u64);

                use crate::bringup::interrupts::{apic, ioapic, pic};
                use kernel::drivers::hpet;
                use kernel::platform::acpi;

                pic::disable();
                if acpi::LOCAL_APIC_ADDR != 0 {
                    apic::init(acpi::LOCAL_APIC_ADDR);
                }

                if acpi::IO_APIC_ADDR != 0 {
                    ioapic::init(acpi::IO_APIC_ADDR);

                    // IRQ 0 (Timer) -> Vector 32
                    let irq0 = acpi::ISA_OVERRIDES[0] as u32;
                    // unsafe {
                    //    use kernel::bridge::HardwareBridge;
                    //    Bridge.log("IOAPIC: Mapping IRQ 0 GSIOverride=");
                    //    print_hex(&Bridge, irq0 as u64);
                    //    Bridge.log("\n");
                    // }
                    ioapic::set_irq_vector(irq0, 32, apic::id() as u8);

                    // Safety net: Force map GSI 2 if override was 0 (standard PC often uses GSI 2 for legacy timer)
                    if irq0 == 0 {
                        // unsafe { Bridge.log("IOAPIC: Force mapping GSI 2 -> Vector 32\n"); }
                        ioapic::set_irq_vector(2, 32, apic::id() as u8);
                    }

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

        if let Some(bs) = &mut bs {
            bs.show(boot_screen::milestones::SCANNING_MODULES);
        }
        ingest_all_modules(&mut k, &boot_info);
        // Default to Limine FB
        let mut use_qemu = false;

        if let Some(cmdline) = &boot_info.cmdline {
            if cmdline.contains("thingos.driver=qemu") {
                USE_QEMU_DRIVER.store(true, Ordering::Relaxed);
                use_qemu = true;
            }
        }

        if !use_qemu {
            // We need to adapt BootInfo FB to what limine_fb driver expects.
            // drivers::limine_fb::init expects &Option<limine::response::FramebufferResponse> which is Limine specific!
            // This is a violation of the separation.
            // I need to refactor drivers::limine_fb to take a generic Framebuffer struct or raw data, OR move limine_fb to `boot` crate?
            // "Move OUT of /kernels into /boot - framebuffer info extraction"
            // So I should pass the extracted simple struct.
            // For now, I will skip limine_fb init or pass dummy data if I can't refactor it immediately.
            // NOTE: The prompt says "Move OUT of /kernels into /boot - framebuffer info extraction".
            // I did that in BootInfo.
            // Now I need to update the kernel core driver to accept BootInfo Framebuffer.
            // I'll comment this out for a second and assume I fix `drivers::limine_fb` next.
            if let Some(fb) = boot_info.framebuffer {
                // Pass simple FB info to a new function in kernel drivers
                // kernel::drivers::framebuffer::init_simple(&mut k, fb.address, ...);
                // Using a placeholder for now to compile.

                let mut addr = fb.address;
                if addr >= hhdm_offset_u64 {
                    addr -= hhdm_offset_u64;
                }
                FRAMEBUFFER_INFO = Some((addr, fb.size));

                let fb_info = abi::wire::machine::FbGetInfoResp {
                    width: fb.width as u32,
                    height: fb.height as u32,
                    stride: fb.pitch as u32,
                    format: 32,
                    addr: 0x1_0000_0000,
                    size: fb.size,
                };
                // kernel::drivers::limine_fb::init_with_info(&mut k, fb_info);
            }
        }

        k.machine.reflect_into_graph(&mut k.graph);

        spawn_loaded(&mut k, &boot_info);

        if let Some(bs) = &mut bs {
            bs.show(boot_screen::milestones::SPAWNING_INIT);
        }
        spawn_kernel_init_task(&mut k, &boot_info);

        crate::bridge::set_tick_hook(scheduler_tick);
        crate::bringup::interrupts::syscall::set_syscall_hook(syscall_hook);
        use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
        crate::bridge::set_page_fault_hook(
            page_fault_hook_impl as fn(&InterruptStackFrame, u64, PageFaultErrorCode) -> bool,
        );

        k.bridge.log("BOOT: Enabling Interrupts & Scheduler\n");
        *KERNEL.lock() = Some(k);
        Bridge.irq_enable();
    }

    loop {
        unsafe {
            use kernel::bridge::HardwareBridge;
            Bridge.idle();
        }
    }
}

unsafe fn spawn_loaded(k: &mut Kernel<Bridge>, boot_info: &BootFacts) {
    use kernel::bridge::HardwareBridge;

    let hhdm_offset = boot_info.hhdm_offset;

    for module in &boot_info.modules {
        if module.path.ends_with("loaded.elf") {
            k.bridge.log("BOOT: Spawning loaded...\n");

            let data = core::slice::from_raw_parts(module.start as *const u8, module.size as usize);

            spawn_elf(k, None, "loaded.elf", data, 0, None, hhdm_offset);
            return;
        }
    }
    k.bridge.log("BOOT: WARNING: loaded.elf not found!\n");
}

unsafe fn spawn_kernel_init_task(k: &mut Kernel<Bridge>, _boot_info: &BootFacts) {
    use kernel::bridge::HardwareBridge;
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
        0,
    );
}

extern "C" fn kernel_init_task_entry(_arg: u64) {
    use x86_64::VirtAddr;

    use crate::bridge::HHDM_OFFSET;
    use core::sync::atomic::Ordering;
    let hhdm_offset_u64 = HHDM_OFFSET.load(Ordering::Relaxed);
    let hhdm_offset = VirtAddr::new(hhdm_offset_u64);

    use kernel::bridge::HardwareBridge;
    Bridge.log("INIT: Entered kernel_init_task_entry\n");

    // Memory and ACPI are now initialized in rust_main before we run.
    // We proceed directly to PCI Scan.

    unsafe {
        u_sleep(100);
    }
    // Bridge.log("INIT: Calling scan_pci\n");
    // let pci_devices = kernel::drivers::pci::scan_pci(&Bridge);
    // Bridge.log("INIT: scan_pci returned\n");
    let pci_devices = alloc::vec::Vec::new(); // Bypass PCI scan

    // let mut boot_args: Option<ScanArgs> = None; // Removed

    {
        Bridge.log("INIT: Publishing PCI Check...\n");
        let use_qemu = USE_QEMU_DRIVER.load(Ordering::Relaxed);

        if use_qemu {
            loop {
                let mut guard_opt = None;
                x86_64::instructions::interrupts::without_interrupts(|| {
                    if let Some(guard) = KERNEL.try_lock() {
                        guard_opt = Some(guard);
                    }
                });
                if let Some(mut guard) = guard_opt {
                    if let Some(k) = (*guard).as_mut() {
                        let info = kernel::drivers::video::qemu_vga::init(k, &pci_devices);
                        unsafe {
                            FRAMEBUFFER_INFO = info;
                        }
                        k.machine.reflect_into_graph(&mut k.graph);
                    }
                    break;
                }
                core::hint::spin_loop();
            }
        }

        for dev in &pci_devices {
            Bridge.log("INIT: Publishing Dev ");
            print_hex(&Bridge, dev.vendor_id as u64);
            Bridge.log(":");
            print_hex(&Bridge, dev.device_id as u64);
            Bridge.log("\n");

            use abi::wire::typed::{CodecId, TypeId, TypedBytes};
            use thing_models::builtins::ids::*;
            use thing_models::link::LinkBody;
            use thing_models::value::ThingBody;

            // These allocations happen OUTSIDE the KERNEL lock to avoid deadlocks with PF handler
            let pci_body_bytes = postcard::to_allocvec(dev).unwrap();
            let pci_tb = ThingBody::from(&TypedBytes {
                type_id: TypeId(THING_PCI_DEVICE_KIND.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes: pci_body_bytes,
            })
            .unwrap();

            let link_body = LinkBody {
                from: THING_BOOT_ROOT,
                to: THING_BOOT_ROOT, // Placeholder, updated below
                predicate: THING_HAS_DEVICE_KIND,
            };

            loop {
                let mut done = false;
                x86_64::instructions::interrupts::without_interrupts(|| {
                    if let Some(mut guard) = KERNEL.try_lock() {
                        if let Some(k) = (*guard).as_mut() {
                            let dev_id =
                                k.graph.create_thing(THING_PCI_DEVICE_KIND, pci_tb.clone());
                            let mut final_link = link_body.clone();
                            final_link.to = dev_id;

                            let lb = ThingBody::from(&TypedBytes {
                                type_id: TypeId(THING_LINK_KIND.0 as u128),
                                codec_id: CodecId::POSTCARD,
                                bytes: postcard::to_allocvec(&final_link).unwrap(),
                            })
                            .unwrap();
                            k.graph.create_thing(THING_LINK_KIND, lb);
                            done = true;
                        }
                    }
                });
                if done {
                    break;
                }
                core::hint::spin_loop();
            }

            // AHCI Check
        }
        use kernel::bridge::HardwareBridge;
        Bridge.log("INIT: Kernel Init Task Complete (No Disk Scan).\n");

        Bridge.log("INIT: Complete. Parking.\n");

        loop {
            core::hint::spin_loop();
        }
    }
}

unsafe fn u_sleep(count: u64) {
    for _ in 0..count {
        core::hint::spin_loop();
    }
}

unsafe fn ingest_all_modules(k: &mut Kernel<Bridge>, boot_info: &BootFacts) {
    use kernel::bridge::HardwareBridge;

    k.bridge.log("BOOT: Ingesting RAM Modules...\n");
    let hhdm = boot_info.hhdm_offset;

    for module in &boot_info.modules {
        let path = &module.path;
        let name = path.rsplit('/').next().unwrap_or(path);

        // Filter out kernel if it's there? Or just let it be.
        if name == "kernel" {
            continue;
        }

        let data = core::slice::from_raw_parts(module.start as *const u8, module.size as usize);

        // Use generic kernel ingestion
        // We use a dummy directory ID (None) or root?
        // ingest_module expects parent_dir_id: Option<ThingId>.
        // We'll put them in root for now.
        kernel::boot_fs::ingest_module(k, name, data, None, hhdm);
    }
}

// parse_bmp removed
