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

// Decomposed modules (moved out of this file to reduce size)
mod framebuffer;
mod init;
mod panic_handler;
mod time_utils;

use core::alloc::Layout;
use linked_list_allocator::LockedHeap;

use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

/// Sets the base revision to the latest revision supported by the crate.
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
pub(crate) static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub(crate) static KERNEL_ADDRESS_REQUEST: limine::request::KernelAddressRequest =
    limine::request::KernelAddressRequest::new();

/// Define the start and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[global_allocator]
static KERNEL_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn get_heap_stats() -> (usize, usize) {
    let heap = KERNEL_ALLOCATOR.lock();
    (heap.used(), heap.size())
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    kernel::println!(
        "alloc_error_handler: KERNEL_ALLOCATOR address: {:p}",
        &KERNEL_ALLOCATOR
    );
    let (used, size) = get_heap_stats();
    kernel::println!("Heap stats: used={} size={}", used, size);
    panic!("allocation error: {:?}", layout);
}

const HEAP_SIZE: usize = heap::KERNEL_HEAP_SIZE_BYTES;
static mut HEAP_MEMORY: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

const STACK_SIZE: usize = 128 * 1024; // 128KB
#[repr(align(16))]
struct Stack([u8; STACK_SIZE]);
static mut BOOT_STACK: Stack = Stack([0; STACK_SIZE]);
static mut STACK_GUARD: [u8; 4096] = [0; 4096];

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // Initialize serial console first (best effort)
    // We use 0 offset initially; Semihosting doesn't need offset.
    serial::arch::init_serial(0);

    kernel::println!("Serial initialized. Preparing to switch stack...");

    unsafe {
        let heap_addr = core::ptr::addr_of_mut!(HEAP_MEMORY) as usize;
        kernel::println!("HEAP_MEMORY address: {:#x}", heap_addr);
        kernel::println!("Probing HEAP_MEMORY...");
        // Volatile write to ensure it's not optimized out
        core::ptr::write_volatile(&mut HEAP_MEMORY[0], 0xAA);
        core::ptr::write_volatile(&mut HEAP_MEMORY[HEAP_SIZE - 1], 0xBB);
        kernel::println!("HEAP_MEMORY probe successful.");

        kernel::println!("KERNEL_ALLOCATOR address: {:p}", &KERNEL_ALLOCATOR);
        KERNEL_ALLOCATOR
            .lock()
            .init(heap_addr as *mut u8, HEAP_SIZE);
        kernel::println!(
            "Kernel heap initialized: [{:#x}, {:#x})",
            heap_addr,
            heap_addr + HEAP_SIZE
        );
    }

    let stack_base = core::ptr::addr_of!(BOOT_STACK) as u64;
    unsafe {
        arch::boot::enter_kernel_stack(stack_base, STACK_SIZE as u64);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain_inner() -> ! {
    kernel::println!("Entered kmain_inner");
    crate::init::init_machine();

    #[cfg(feature = "fill-framebuffer")]
    crate::framebuffer::fill_framebuffer_with_color();

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
