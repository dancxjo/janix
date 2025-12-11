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

mod arch;
mod boot_model;
mod console;
mod context_switch;
mod dashboard;
mod elf_loader;
#[cfg(target_arch = "x86_64")]
mod gdt;
mod graph_reifier;
mod heap;
mod program;
mod serial;
mod user;

// Decomposed modules (moved out of this file to reduce size)
mod framebuffer;
mod init;
mod panic_handler;
mod time_utils;

use crate::arch::{Arch, CurrentArch};
use alloc::boxed::Box;

use core::arch::asm;

use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

/// Sets the base revision to the latest revision supported by the crate.
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
pub(crate) static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

/// Define the start and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

// Early boot heap before we reserve pages from the memory map.
const HEAP_SIZE: usize = heap::KERNEL_HEAP_SIZE_BYTES;
static mut HEAP_MEMORY: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

const STACK_SIZE: usize = 16 * 1024; // 16KB
#[repr(align(16))]
struct Stack([u8; STACK_SIZE]);
static mut BOOT_STACK: Stack = Stack([0; STACK_SIZE]);

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // Initialize serial console first (best effort)
    // We use 0 offset initially; Semihosting doesn't need offset.
    serial::arch::init_serial(0);

    kernel_core::log("Serial initialized. Preparing to switch stack...");

    unsafe {
        let heap_addr = core::ptr::addr_of_mut!(HEAP_MEMORY) as usize;
        kernel_core::println!("HEAP_MEMORY address: {:#x}", heap_addr);
        kernel_core::println!("Probing HEAP_MEMORY...");
        // Volatile write to ensure it's not optimized out
        core::ptr::write_volatile(&mut HEAP_MEMORY[0], 0xAA);
        core::ptr::write_volatile(&mut HEAP_MEMORY[HEAP_SIZE - 1], 0xBB);
        kernel_core::println!("HEAP_MEMORY probe successful.");
    }

    #[cfg(target_arch = "aarch64")]
    {
        // Initialize exception vector table early
        arch::aarch64::trap::init();

        let stack_top = core::ptr::addr_of!(BOOT_STACK) as u64 + STACK_SIZE as u64;

        unsafe extern "C" {
            fn kmain_inner_asm() -> !;
        }

        // Switch to SP_EL1 for kernel stack
        unsafe {
            arch::aarch64::trap::jump_to_el1_stack(stack_top, kmain_inner_asm);
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        // Switch away from the Limine-provided stack into a kernel-owned stack in .bss.
        // On some machines the boot stack may live in a reserved/unmapped hole, which
        // causes a page fault once we exhaust the initial stack space.
        let stack_top = core::ptr::addr_of!(BOOT_STACK) as u64 + STACK_SIZE as u64;
        unsafe {
            asm!(
                "mov rsp, {0}",
                "xor rbp, rbp",
                "call {1}",
                in(reg) stack_top,
                sym kmain_inner,
                options(noreturn)
            );
        }
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        kmain_inner();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain_inner() -> ! {
    kernel_core::log("Entered kmain_inner");
    #[cfg(feature = "fill-framebuffer")]
    crate::framebuffer::fill_framebuffer_with_color();

    crate::init::init_machine();

    crate::init::init_world_graph();
    crate::init::init_userland_and_enter_scheduler();
}

fn init_machine() {
    crate::init::init_machine();
}

fn init_world_graph() {
    crate::init::init_world_graph();
}

#[cfg(not(feature = "boot-dashboard-only"))]
fn init_userland_and_enter_scheduler() -> ! {
    crate::init::init_userland_and_enter_scheduler()
}

#[cfg(feature = "boot-dashboard-only")]
fn init_userland_and_enter_scheduler() -> ! {
    crate::init::init_userland_and_enter_scheduler()
}

#[cfg(feature = "boot-dashboard-only")]
fn render_dashboard_and_halt() -> ! {
    crate::init::render_dashboard_and_halt()
}

fn launch_init_process() {
    crate::init::launch_init_process();
}

fn init_console() -> bool {
    crate::init::init_console()
}

#[cfg(feature = "fill-framebuffer")]
fn fill_framebuffer_with_color() {
    crate::framebuffer::fill_framebuffer_with_color()
}

#[cfg(feature = "fill-framebuffer")]
fn virtual_framebuffer_address(guest_addr: u64, hhdm_offset: u64) -> Option<u64> {
    crate::framebuffer::virtual_framebuffer_address(guest_addr, hhdm_offset)
}

#[cfg(feature = "fill-framebuffer")]
fn is_canonical_address(addr: u64) -> bool {
    crate::framebuffer::is_canonical_address(addr)
}

fn log_rtc_epoch(seconds: i64) {
    crate::time_utils::log_rtc_epoch(seconds)
}

fn unix_seconds_to_datetime(seconds: i64) -> (i32, u32, u32, u32, u32, u32) {
    crate::time_utils::unix_seconds_to_datetime(seconds)
}

fn days_in_year(year: i32) -> i32 {
    crate::time_utils::days_in_year(year)
}

fn days_in_month(year: i32, month: i32) -> i32 {
    crate::time_utils::days_in_month(year, month)
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    crate::panic_handler::rust_panic(info)
}

fn hcf() -> ! {
    crate::panic_handler::hcf()
}
