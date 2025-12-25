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

    let k = Kernel::new(Bridge);
    k.boot();
}
