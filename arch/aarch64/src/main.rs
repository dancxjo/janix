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
    
    // 2. Collect Boot Info (Allocates)
    let boot_info = boot::collect();
    
    unsafe {
        use kernel::bridge::HardwareBridge;
        let bridge = Bridge;
        bridge.log("\n--- BootInfo (AArch64) ---\n");
       
        // Map ACPI / Reserved Regions
        use paging::{map_region, PTE_VALID, PTE_PAGE, PTE_AF, PTE_SH_INNER, PTE_AP_RW_EL1, PTE_UXN, PTE_PXN};
        let normal_flags = PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL1 | PTE_UXN | PTE_PXN;

        for entry in &boot_info.memory_map {
             use boot::bootinfo::MemoryRegionKind;
             match entry.kind {
                 MemoryRegionKind::AcpiReclaimable | MemoryRegionKind::AcpiNvs => {
                      map_region(entry.start, (entry.end - entry.start) as usize, normal_flags);
                 }
                 _ => {}
             }
        }
        
        spawn_loaded_stub(&boot_info);
    }
    
    // Init Kernel Global
    let mut k = Kernel::new(Bridge);
    
    unsafe {
        use kernel::bridge::HardwareBridge;
        Bridge.log("Slice B: Enabling Interrupts...\n");
        
        *KERNEL.lock() = Some(k);
        bridge_aarch64::set_tick_hook(scheduler_tick);
        
        Bridge.irq_enable();
        
        loop {
            Bridge.idle();
        }
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

unsafe fn spawn_loaded_stub(boot_info: &boot::BootInfo) {
    use kernel::bridge::HardwareBridge;
    let bridge = Bridge;
    for module in &boot_info.modules {
        if module.path.ends_with("loaded.elf") {
            bridge.log("Slice A Success: Found loaded.elf at ");
             print_hex(&bridge, module.start);
             bridge.log("\n");
             return;
        }
    }
    bridge.log("WARNING: loaded.elf not found in modules!\n");
}

fn scheduler_tick(_frame: &mut bridge_aarch64::interrupts::trap::TrapFrame) {
    if let Some(mut guard) = KERNEL.try_lock() {
        if let Some(k) = (*guard).as_mut() {
             use bridge_aarch64::ArchContext;
             use kernel::sched::scheduler::ThreadContext;
             use kernel::bridge::HardwareBridge;

             // We need to sync the TrapFrame to the ArchContext if we were in a thread.
             // But for now, just the basic tick logic.
             
             {
                let now_raw = k.bridge.ticks();
                let last = LAST_TICKS.swap(now_raw, Ordering::Relaxed);
                let now_ns = k.bridge.monotonic_now();
             }

             k.bridge.log("TICK\n");
        }
    }
}
