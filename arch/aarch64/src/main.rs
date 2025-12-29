#![no_std]
#![no_main]
#![cfg_attr(target_os = "thingos", feature(alloc_error_handler))]
#![allow(unused)]

extern crate alloc;

#[cfg(target_os = "thingos")]
mod early_log;
#[cfg(target_os = "thingos")]
mod heap;
#[cfg(target_os = "thingos")]
mod limine;

use boot;
use bridge_aarch64::Bridge;
use kernel::bridge::HardwareBridge;
use core::arch::naked_asm;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use kernel::Kernel;
use spin::Mutex;

static PANICKING: AtomicBool = AtomicBool::new(false);
static KERNEL: Mutex<Option<Kernel<Bridge>>> = Mutex::new(None);
static LAST_TICKS: AtomicU64 = AtomicU64::new(0);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use kernel::bridge::HardwareBridge;
    use kernel::diag::LogRing;

    // Recursion guard
    if PANICKING.swap(true, Ordering::Relaxed) {
        loop {
            core::hint::spin_loop();
        }
    }

    let bridge = Bridge;
    bridge.log("PANIC\n");
    kernel::diag::record_panic("PANIC");

    if let Some(payload) = info.payload().downcast_ref::<&str>() {
        bridge.log("Message: ");
        bridge.log(payload);
        bridge.log("\n");
    }
    if let Some(loc) = info.location() {
        bridge.log("File: ");
        bridge.log(loc.file());
        bridge.log("\n");
        bridge.log("Line: ");
        print_dec(&bridge, loc.line() as u64);
        bridge.log("\n");
    }

    // Dump ring buffer for recent logs/faults
    bridge.log("\n--- RING DUMP ---\n");
    let ring = LogRing::global();
    ring.drain(|entry| {
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
        bridge.log(core::str::from_utf8(&[level_char as u8]).unwrap());
        bridge.log(": ");

        let len = entry.msg_len as usize;
        if len > 0 && len <= 256 {
            if let Ok(s) = core::str::from_utf8(&entry.msg_bytes[..len]) {
                bridge.log(s);
            } else {
                bridge.log("<utf8 error>");
            }
        }
        bridge.log("\n");

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

const BOOT_STACK_SIZE: usize = 16384;
#[repr(align(16))]
struct BootStack([u8; BOOT_STACK_SIZE]);

#[used]
#[unsafe(link_section = ".bss")]
static mut BOOT_STACK: BootStack = BootStack([0; BOOT_STACK_SIZE]);

mod paging;
mod loader;

#[no_mangle]
#[no_mangle]
#[unsafe(naked)]
pub extern "C" fn _start() -> ! {
    naked_asm!(
        "adrp x9, {2}",
        "add x9, x9, :lo12:{2}",
        "add x9, x9, {0}",
        "msr spsel, #1",
        "mov sp, x9",
        "msr daifset, #0xf",
        "mov x0, #(3 << 20)",
        "msr cpacr_el1, x0",
        "isb",
        "bl {1}",
        "1: wfi",
        "b 1b",
        const BOOT_STACK_SIZE,
        sym rust_main,
        sym BOOT_STACK
    )
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    #[cfg(target_os = "thingos")]
    unsafe {
        use kernel::bridge::HardwareBridge;
        // -1. Init Bridge (Exception Vectors) EARLY

        // 1. Get HHDM offset (Before Heap!)
        // Limine maps this as Normal memory. We will remap as Device later.
        if let Some(resp) = boot::limine::HHDM_REQUEST.get_response() {
            let offset = resp.offset();
            // 1. Init Bridge (and set HHDM)
            Bridge::init(offset);
            
            // Force mask interrupts (Limit/UEFI might have left them on, and init might unmask)
            Bridge.irq_disable();

            // 2. Init Heap (Needed for paging)
            let info =
                limine::heap_init::init_heap_from_limine(heap::KERNEL_HEAP_SIZE_BYTES as u64);

            // 3. Init Paging & Remap UART (Mapped as Device Memory)
            paging::init(offset);
            
            // Map UART (0x0900_0000)
            paging::map_device_region(0x09000000, 4096);
            
            // Map GIC Distributor (0x0800_0000) & CPU Interface (0x0801_0000)
            paging::map_device_region(0x08000000, 4096); // Dist
            paging::map_device_region(0x08010000, 4096); // CPU

            // Now safe to init Platform (GIC + Timer)
            Bridge::init_platform(offset);

            // 4. Update logic UART base (Physical 0x09000000 + Offset)
            bridge_aarch64::set_uart_base(0x09000000 + offset);

            // Should now be able to print to Device-mapped UART
            bootlog!("Booting ThingOS (aarch64)...");
            bootlog!("UART mapped at HHDM offset 0x{:x} (Device)", offset);

            early_log::log_heap_init(info);
        } else {
            // Fallback: Blind write to Phys
            core::ptr::write_volatile(0x0900_0000 as *mut u8, 0x46); // 'F'
            loop {}
        }
    }
    
    // Hook BootScreen allocator
    unsafe fn boot_alloc_impl(size: usize, align: usize) -> *mut u8 {
        let layout = alloc::alloc::Layout::from_size_align(size, align).unwrap();
        alloc::alloc::alloc(layout)
    }
    boot_screen::set_boot_alloc(boot_alloc_impl);

    // 2. Collect Boot Info (Allocates)
    let boot_info = boot::collect();

    // --- Boot Screen Init ---
    let mut bs = unsafe {
        if let Some(fb) = &boot_info.framebuffer {
             if fb.bpp != 32 {
                 use kernel::bridge::HardwareBridge;
                 Bridge.log("BOOTSCREEN: Unsupported BPP (needs 32)\n");
                 None
             } else {
                 let addr = fb.address; // Virtual address

                 // Pixel Format Detection
                 let pixel_format = if fb.red_mask_shift == 16 && fb.green_mask_shift == 8 && fb.blue_mask_shift == 0 {
                      boot_screen::PixelFormat::Xrgb8888
                 } else if fb.red_mask_shift == 0 && fb.green_mask_shift == 8 && fb.blue_mask_shift == 16 {
                      boot_screen::PixelFormat::Bgra8888
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

                 if let Some(mut bs) = boot_screen::BootScreen::new(info) {
                     bs.show("Booting ThingOS");

                     // Fade in
                     for a in (0..=255).step_by(8) {
                         bs.set_fade(a as u8);
                         bs.draw();
                         boot_delay(100_000);
                     }
                     Some(bs)
                 } else {
                     use kernel::bridge::HardwareBridge;
                     Bridge.log("BOOTSCREEN: Init Failed (Alloc/Size)\n");
                     None
                 }
             }
        } else {
             use kernel::bridge::HardwareBridge;
             Bridge.log("BOOTSCREEN: No Framebuffer\n");
             None
        }
    };

    if let Some(bs) = &mut bs { bs.show("Bridge online"); }
    
    let k = Kernel::new(Bridge);
    *KERNEL.lock() = Some(k);

    if let Some(bs) = &mut bs { bs.show("Graph init"); }
    if let Some(bs) = &mut bs { bs.show("Graph seeded"); }
    if let Some(bs) = &mut bs { bs.show("Symbols init"); }
    if let Some(bs) = &mut bs { bs.show("Symbols ready"); }
    if let Some(bs) = &mut bs { bs.show("Scanning modules"); }

    if let Some(bs) = &mut bs { bs.show("Spawning init tasks"); }
    Bridge.log("Starting Loader Task...\n");
    if let Some(mut guard) = KERNEL.try_lock() {
        if let Some(k) = guard.as_mut() {
             use alloc::alloc::{alloc, Layout};
             let layout = Layout::from_size_align(64 * 1024, 16).unwrap();
                 let stack_ptr = unsafe { alloc(layout) };
                 let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 };
             
             let fb_phys = if let Some(fb) = boot_info.framebuffer.as_ref() { fb.address } else { 0 };
             let fb_size = if let Some(fb) = boot_info.framebuffer.as_ref() { (fb.pitch * fb.height) as usize } else { 0 };

             let args = alloc::boxed::Box::new(loader::ScanArgs {
                 bb_info: None,
                 hhdm: boot_info.hhdm_offset,
                 fb_phys,
                 fb_size,
                 modules: boot_info.modules.clone(),
             });
             let args_ptr = alloc::boxed::Box::into_raw(args) as u64;
             
             let entry = loader::scan_boot_fs_task as *const () as usize as u64;
             Bridge.log("Spawning loader at: ");
             print_hex(&Bridge, entry);
             Bridge.log("\n");

             k.scheduler.spawn(
                 &k.bridge,
                 "scan_boot_fs",
                 entry,
                 stack_top,
                 args_ptr,
                 0, 0
             );
             Bridge.log("Spawned loader task.\n");
        }
    }
    unsafe {
        bridge_aarch64::set_tick_hook(scheduler_tick);
        bridge_aarch64::interrupts::syscall::set_syscall_hook(syscall_hook);
        bridge_aarch64::set_page_fault_hook(page_fault_hook);
        
        Bridge.irq_enable();
        
        loop {
            Bridge.idle();
        }
    }
}

#[inline(always)]
fn boot_delay(count: u64) {
    for _ in 0..count {
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
    let mut buf = [0u8; 20]; // enough for u64
    let mut n = val;
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    bridge.log(core::str::from_utf8(&buf[i..]).unwrap());
}




fn page_fault_hook(
    frame: &mut bridge_aarch64::interrupts::trap::TrapFrame,
    fault_addr: u64,
    esr: u64
) {
     unsafe {
        use kernel::bridge::HardwareBridge;
        Bridge.log("PAGE FAULT\n");
        Bridge.log("FAR: ");
        print_hex(&Bridge, fault_addr);
        Bridge.log(" ESR: ");
        print_hex(&Bridge, esr);
        Bridge.log("\n");
        
        if let Some(mut guard) = KERNEL.try_lock() {
            if let Some(k) = (*guard).as_mut() {
                 // TODO: Route to kernel demand paging
                 // k.handle_page_fault(fault_addr, ...);
                 
                 // For now, if it's user mode (EC & 0b11 == 0), kill task?
                 // Current scheduler doesn't expose easy kill yet from here?
                 // Just loop to prevent "return to faulting instruction"
                 Bridge.log("Spinning...\n");
                 loop { core::hint::spin_loop(); }
            }
        }
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
    unsafe {
        use kernel::bridge::HardwareBridge;
        // Bridge.log("SYSCALL: ");
        // print_hex(&Bridge, num as u64);
        // Bridge.log("\n");
        0
    }
}


fn scheduler_tick(frame: &mut bridge_aarch64::interrupts::trap::TrapFrame) {
    if let Some(mut guard) = KERNEL.try_lock() {
        if let Some(k) = (*guard).as_mut() {
             use bridge_aarch64::ArchContext;
             use kernel::sched::scheduler::ThreadContext;
             use kernel::bridge::HardwareBridge;

             let ctx_ptr = frame as *mut bridge_aarch64::interrupts::trap::TrapFrame as *mut ThreadContext<ArchContext>;
             let ctx = unsafe { &mut *ctx_ptr };
             
             let now_ns = k.bridge.monotonic_now();
             
             k.scheduler.wake_sleepers(now_ns);
             k.scheduler.tick(&k.bridge, ctx);
        }
    }
}
