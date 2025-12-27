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

use bridge_aarch64::Bridge;
use core::arch::naked_asm;
use kernel_core::Kernel;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use hw::HardwareBridge;
    let bridge = Bridge;
    bridge.log("PANIC\n");
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
        "mov sp, x9",
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
        panic!("Test Panic");
        
        // -1. Init Bridge (Exception Vectors) EARLY
        Bridge::init();

        // TEST: Trigger exception to prove vectors are working
        // core::arch::asm!("brk #0");

        // 0. Init Heap FIRST (needed for paging/alloc)
        let info = limine::heap_init::init_heap_from_limine(heap::KERNEL_HEAP_SIZE_BYTES as u64);

        // 1. Get HHDM offset
        if let Some(resp) = limine::requests::HHDM_REQUEST.get_response() {
            let offset = resp.offset();
            // Init paging with HHDM offset
            paging::init(offset);

            // 2. Update logic UART base (Physical 0x09000000 + Offset)
            bridge_aarch64::set_uart_base(0x09000000 + offset);

            // 3. Map UART (Physical 0x09000000)
            paging::map_device_region(0x09000000, 4096);
            
            bootlog!("UART mapped at HHDM offset 0x{:x}", offset);
        } else {
            // If HHDM fails, we can't print easily unless we assume identity map for UART
            // Try blind write to 0x09000000 as last resort
             core::ptr::write_volatile(0x0900_0000 as *mut u8, 0x46); // 'F'
             loop {}
        }

        use hw::HardwareBridge;
        let bridge = Bridge;
        


        bootlog!("Booting ThingOS (aarch64)...");
        bootlog!("Init finished, jumping to kernel");

        early_log::log_heap_init(info);
    }

    let mut k = Kernel::new(Bridge);
    k.boot(None);
}
