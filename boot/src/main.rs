#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

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
        let start = core::ptr::addr_of_mut!(HEAP_MEMORY) as usize;
        heap::KERNEL_ALLOCATOR.init(start, HEAP_SIZE);
        let end = start + HEAP_SIZE;
        let msg = alloc::format!("Kernel heap initialized: [{:#x}, {:#x})", start, end);
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        kernel_core::log(leaked);
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

    let rtc_epoch = crate::arch::read_boot_rtc_epoch_seconds();
    log_rtc_epoch(rtc_epoch);
    kernel_core::time::init_timekeeping(rtc_epoch);

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
    boot_model::seed_boot_programs_from_limine();
    boot_model::seed_time_graph();
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
    kernel_core::log("launch_init_process: spawning init via ProgramImage");
    if let Err(err) = program::spawn_program_by_identifier("init", "init", 1) {
        kernel_core::log("launch_init_process: failed to spawn init via ProgramImage");
        kernel_core::log(err);
        hcf();
    }
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

fn log_rtc_epoch(seconds: i64) {
    let (year, month, day, hour, minute, second) = unix_seconds_to_datetime(seconds);
    let msg = alloc::format!(
        "RTC epoch (raw): {:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC ({}s)",
        year,
        month,
        day,
        hour,
        minute,
        second,
        seconds
    );
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    kernel_core::log(leaked);
}

fn unix_seconds_to_datetime(seconds: i64) -> (i32, u32, u32, u32, u32, u32) {
    let mut days = seconds.div_euclid(86_400);
    let mut secs_of_day = seconds.rem_euclid(86_400);
    if secs_of_day < 0 {
        secs_of_day += 86_400;
        days -= 1;
    }

    let mut year = 1970;
    while days >= days_in_year(year) as i64 {
        days -= days_in_year(year) as i64;
        year += 1;
    }

    let mut month = 1;
    while days >= days_in_month(year, month) as i64 {
        days -= days_in_month(year, month) as i64;
        month += 1;
    }

    let day = days as u32 + 1;
    let mut remaining = secs_of_day;
    let hour = (remaining / 3_600) as u32;
    remaining %= 3_600;
    let minute = (remaining / 60) as u32;
    let second = (remaining % 60) as u32;

    (year, month as u32, day, hour, minute, second)
}

fn days_in_year(year: i32) -> i32 {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        366
    } else {
        365
    }
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if days_in_year(year) == 366 {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    kernel_core::log("PANIC!");
    kernel_core::println!("========== KERNEL PANIC ==========");
    let panic_message = info.message();
    kernel_core::println!("Message: {}", panic_message);
    if let Some(location) = info.location() {
        kernel_core::println!(
            "Location: {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        kernel_core::println!("Location: <unknown>");
    }

    let ticks = kernel_core::time::ticks_since_boot();
    let monotonic_ns = kernel_core::time::monotonic_now_ns();
    let (unix_seconds, unix_nanos) = kernel_core::time::now_unix_from_rtc();
    kernel_core::println!(
        "Time: ticks={} monotonic_ns={} unix_seconds={} unix_nanos={}",
        ticks,
        monotonic_ns,
        unix_seconds,
        unix_nanos
    );

    let snapshot = kernel_core::model::dashboard_snapshot();
    kernel_core::println!(
        "Memory: total_frames={} used_frames={} free_frames={}",
        snapshot.memory.total_frames,
        snapshot.memory.used_frames,
        snapshot.memory.free_frames
    );
    kernel_core::println!(
        "Scheduler: processes={} threads={} runnable={}",
        snapshot.scheduler.process_count,
        snapshot.scheduler.thread_count,
        snapshot.scheduler.runnable_threads
    );
    kernel_core::println!(
        "ThingCounts: total={} processes={} threads={} phys_frames={} virt_regions={} frame_pools={} address_spaces={} cpu_cores={}",
        snapshot.counts.total_things,
        snapshot.counts.processes,
        snapshot.counts.threads,
        snapshot.counts.phys_frames,
        snapshot.counts.virt_regions,
        snapshot.counts.frame_pools,
        snapshot.counts.address_spaces,
        snapshot.counts.cpu_cores
    );

    let current_thread = {
        let sched = kernel_core::sched::SCHEDULER.lock();
        sched.current_id()
    };
    match current_thread {
        Some(tid) => kernel_core::println!("Current thread: {}", tid.0),
        None => kernel_core::println!("Current thread: <none>"),
    }

    const LOG_TAIL: usize = 10;
    let logs = kernel_core::get_logs();
    kernel_core::println!("Recent kernel log entries (last {}):", LOG_TAIL);
    let start = logs.len().saturating_sub(LOG_TAIL);
    for entry in &logs[start..] {
        if let Some(msg) = entry {
            kernel_core::println!("  {}", msg);
        }
    }

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
