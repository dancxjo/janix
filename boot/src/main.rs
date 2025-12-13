#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]
#![allow(
    unused_unsafe,
    unsafe_op_in_unsafe_fn,
    unreachable_code,
    dead_code,
    unused_variables,
    function_casts_as_integer,
    unused_mut
)]

extern crate alloc;

mod boot_model;
mod console;
mod context_switch;
mod dashboard;
mod elf_loader;
mod graph_reifier;
mod heap;
mod program;
mod serial;

// Decomposed modules
mod framebuffer;
mod init;
mod panic_handler;
mod time_utils;

use limine::BaseRevision;
use limine::request::{
    FramebufferRequest, KernelAddressRequest, RequestsEndMarker, RequestsStartMarker,
};

/// Sets the base revision to the latest revision supported by the crate.
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
pub(crate) static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub(crate) static KERNEL_ADDRESS_REQUEST: KernelAddressRequest = KernelAddressRequest::new();

/// Limine request markers
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

const HEAP_SIZE: usize = heap::KERNEL_HEAP_SIZE_BYTES;
static mut HEAP_MEMORY: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

const STACK_SIZE: usize = 128 * 1024; // 128KB
#[repr(align(16))]
struct Stack([u8; STACK_SIZE]);

static mut BOOT_STACK: Stack = Stack([0; STACK_SIZE]);
static mut STACK_GUARD: [u8; 4096] = [0; 4096];

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // Best-effort early serial
    serial::arch::init_serial(0);
    kernel::println!("boot: serial ready");

    // Initialize heap *before* anything allocation-hungry.
    unsafe {
        let heap_addr = core::ptr::addr_of_mut!(HEAP_MEMORY) as usize;

        // Tiny probe (kept, but not chatty)
        core::ptr::write_volatile(&mut HEAP_MEMORY[0], 0xAA);
        core::ptr::write_volatile(&mut HEAP_MEMORY[HEAP_SIZE - 1], 0xBB);

        heap::init_kernel_heap(heap_addr, HEAP_SIZE);
    }

    kernel::println!("boot: heap ready; switching stack");

    // Stack switch into kmain_inner
    let stack_base = core::ptr::addr_of!(BOOT_STACK) as u64;
    arch::boot::enter_kernel_stack(stack_base, STACK_SIZE as u64);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain_inner() -> ! {
    kernel::println!("boot: entered kmain_inner");

    #[cfg(feature = "fill-framebuffer")]
    crate::framebuffer::fill_framebuffer_with_color(0x00_33_33_33);

    crate::init::init_machine();

    #[cfg(feature = "fill-framebuffer")]
    crate::framebuffer::fill_framebuffer_with_color(0x00_80_80_80);

    crate::init::init_world_graph();

    #[cfg(feature = "fill-framebuffer")]
    crate::framebuffer::fill_framebuffer_with_color(0x00_CC_CC_CC);

    crate::init::init_userland_and_enter_scheduler();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    crate::panic_handler::rust_panic(info)
}
