#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

extern crate alloc;
extern crate heartbeat;
extern crate hello;
extern crate init as init_app;
extern crate thread_dashboard;

mod arch;
mod boot_model;
mod console;
mod context_switch;
mod dashboard;
mod elf_loader;
mod program;
#[cfg(target_arch = "x86_64")]
mod gdt;
mod graph_reifier;
mod heap;
mod serial;
mod user;

use crate::arch::{Arch, CurrentArch};

use core::arch::asm;

use limine::BaseRevision;
use limine::request::{FramebufferRequest, HhdmRequest, RequestsEndMarker, RequestsStartMarker};

/// Sets the base revision to the latest revision supported by the crate.
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

/// Define the start and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

// 1MB static heap
const HEAP_SIZE: usize = 1024 * 1024;
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

    #[cfg(not(target_arch = "aarch64"))]
    {
        kmain_inner();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain_inner() -> ! {
    kernel_core::log("Entered kmain_inner");
    init_machine();
    init_world_graph();
    init_userland_and_enter_scheduler();
}

fn init_machine() {
    // All limine requests must also be referenced in a called function
    assert!(BASE_REVISION.is_supported());

    unsafe {
        heap::KERNEL_ALLOCATOR.init(core::ptr::addr_of_mut!(HEAP_MEMORY) as usize, HEAP_SIZE);
    }

    kernel_core::log("Initializing kernel core...");
    kernel_core::init();
    kernel_core::register_spawn_program_handler(crate::program::spawn_program);
    {
        let mut sched = kernel_core::sched::SCHEDULER.lock();
        sched.init_graph_mirror();
    }
    kernel_core::log("Kernel core initialized.");
    graph_reifier::init_graph_subscriptions();

    #[cfg(target_arch = "x86_64")]
    {
        gdt::init();
    }

    CurrentArch::install_syscall_handler();

    if let Some(hhdm_response) = boot_model::HHDM_REQUEST.get_response() {
        let offset = hhdm_response.offset();
        unsafe { user::init_user_stack(offset) };
    }

    init_console();
    kernel_core::log("ThingOS booting...");
}

fn init_world_graph() {
    kernel_core::create_builtin_things();
    boot_model::seed_memory_graph_from_limine();
    boot_model::seed_cpu_graph_from_limine();
    boot_model::seed_boot_profile();
    boot_model::seed_program_images_from_limine();
}

#[cfg(not(feature = "boot-dashboard-only"))]
fn init_userland_and_enter_scheduler() -> ! {
    kernel_core::log("Launching init (PID 1) ...");
    launch_init_process();
    kernel_core::log("Handing control to scheduler...");
    user::schedule_next();
}

#[cfg(feature = "boot-dashboard-only")]
fn init_userland_and_enter_scheduler() -> ! {
    render_dashboard_and_halt();
}

#[cfg(feature = "boot-dashboard-only")]
fn render_dashboard_and_halt() -> ! {
    if init_console() {
        console::with_console(|console| {
            dashboard::render_dashboard(console);
        });
    } else {
        kernel_core::log("No framebuffer available for dashboard");
    }
    hcf();
}

fn launch_init_process() {
    kernel_core::log("launch_init_process: allocating stack");
    let stack = user::alloc_user_stack();
    kernel_core::log("launch_init_process: stack allocated, creating process");
    let mut sched = kernel_core::sched::SCHEDULER.lock();
    let pid = sched.add_process("init");
    kernel_core::log("launch_init_process: process created, adding thread");
    let _tid = sched.add_thread(pid, "init", user::user_thread_main, 0, stack, 1);
    kernel_core::log("launch_init_process: init thread added");
}

fn init_console() -> bool {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            unsafe { console::init_global(&framebuffer) };
            return true;
        }
    }
    false
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    kernel_core::log("PANIC!");
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }
}
