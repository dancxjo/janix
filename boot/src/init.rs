use alloc::boxed::Box;

use arch::{Arch, CurrentArch};

pub fn init_machine() {
    // All limine requests must also be referenced in a called function
    assert!(crate::BASE_REVISION.is_supported());

    unsafe {
        // Heap is initialized in kmain
        let start = core::ptr::addr_of_mut!(crate::HEAP_MEMORY) as usize;
        let end = start + crate::HEAP_SIZE;
        // We can't use alloc::format! here if heap is not initialized?
        // But it IS initialized in kmain.
        // However, we don't need to re-init.
        // And we can log.
        let msg = alloc::format!("Kernel heap initialized: [{:#x}, {:#x})", start, end);
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        kernel::log(leaked);
    }

    if let Some(hhdm_response) = crate::boot_model::HHDM_REQUEST.get_response() {
        let offset = hhdm_response.offset();
        kernel::memory::set_hhdm_offset(offset);
        unsafe { arch::user::init_user_stack(offset) };
    }

    // Initialize architecture-specific tables (GDT, etc.)
    // Initialize architecture-specific tables (GDT, etc.)
    // This MUST happen before we try to enter user mode or load segment selectors.
    arch::platform::init_arch_tables();

    kernel::log("Initializing kernel core...");
    kernel::init();

    kernel::register_spawn_program_handler(crate::program::spawn_program);
    {
        let mut sched = kernel::sched::SCHEDULER.lock();
        sched.init_graph_mirror();
    }
    kernel::log("Kernel core initialized.");

    // Seed memory graph early so frame allocator is available for arch init
    // (AArch64 needs this for paging::map_device_region during map_boot_device_regions)
    crate::boot_model::seed_memory_graph_from_limine();

    // Seed DTB frequency (RISC-V)
    #[cfg(target_arch = "riscv64")]
    arch::riscv64::dtb::init();

    if arch::platform::map_boot_device_regions() {
        kernel::log("PCI regions mapped.");
        #[cfg(target_arch = "aarch64")]
        {
            crate::serial::arch::init_pl011(kernel::memory::get_hhdm_offset());
            kernel::log("PL011 initialized.");
        }
    }
    // Now that boot-time device regions are mapped into the kernel page tables,
    // initialize hardware drivers that access MMIO (e.g. PCI/XHCI). Previously
    // this ran earlier during `kernel::init()` and could cause data-abort
    // accesses when drivers tried to dereference `phys + HHDM_OFFSET` before
    // the mappings existed.
    kernel::log("Initializing hardware drivers (MMIO-dependent)");
    kernel::driver_bringup::init();
    crate::graph_reifier::init_graph_subscriptions();

    // Register IRQ controller callback to manage IRQ masking via graph requests
    #[cfg(target_arch = "x86_64")]
    kernel::bridge::io::register_irq_controller(arch::x86_64::pic::set_irq_mask);

    CurrentArch::install_syscall_handler();

    let rtc_epoch = arch::read_boot_rtc_epoch_seconds();
    crate::time_utils::log_rtc_epoch(rtc_epoch);
    kernel::time::init_timekeeping(rtc_epoch);

    init_console();

    kernel::log("ThingOS booting...");
}

pub fn init_world_graph() {
    kernel::create_builtin_things();
    // seed_memory_graph_from_limine moved to init_machine
    crate::boot_model::seed_cpu_graph_from_limine();
    crate::boot_model::seed_display_from_limine();
    crate::boot_model::seed_boot_profile();
    crate::boot_model::seed_font_modules_from_limine();
    crate::boot_model::seed_program_images_from_limine();
    crate::boot_model::seed_raw_modules_from_limine();
    crate::boot_model::seed_boot_programs_from_limine();
    crate::boot_model::seed_time_graph();
    kernel::bridge::io::seed_io_regions();
}

#[cfg(not(feature = "boot-dashboard-only"))]
pub fn init_userland_and_enter_scheduler() -> ! {
    kernel::log("Launching init (PID 1) ...");
    launch_init_process();
    kernel::log("Launching idle thread...");
    launch_idle_thread();
    kernel::log("Handing control to scheduler...");
    arch::user::schedule_next();
}

#[cfg(feature = "boot-dashboard-only")]
pub fn init_userland_and_enter_scheduler() -> ! {
    render_dashboard_and_halt();
}

#[cfg(feature = "boot-dashboard-only")]
pub fn render_dashboard_and_halt() -> ! {
    if init_console() {
        crate::console::with_console(|console| {
            crate::dashboard::render_dashboard(console);
        });
    } else {
        kernel::log("No framebuffer available for dashboard");
    }
    crate::panic_handler::hcf();
}

fn launch_idle_thread() {
   // Attach to PID 1 (init)
   let pid = abi::ProcessId(1);
   let mut sched = kernel::sched::SCHEDULER.lock();
   sched.add_idle_thread(pid);
}

pub fn launch_init_process() {
    kernel::log("launch_init_process: spawning init via ProgramImage");
    if let Err(err) = crate::program::spawn_program_by_identifier("init", "init", 10) {
        kernel::log("launch_init_process: failed to spawn init via ProgramImage");
        kernel::log(err);
        crate::panic_handler::hcf();
    }

    if kernel::model::program_image_exists("debug_input_events") {
        kernel::log("launch_init_process: spawning debug_input_events debug app");
        if let Err(err) =
            crate::program::spawn_program_by_identifier("debug_input_events", "debug_input_events", 1)
        {
            kernel::log("launch_init_process: failed to spawn debug_input_events");
            kernel::log(err);
        }
    }
}

pub fn init_console() -> bool {
    if !crate::console::FRAMEBUFFER_CONSOLE_ENABLED {
        return false;
    }
    if let Some(framebuffer_response) = crate::FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            unsafe { crate::console::init_global(&framebuffer) };
            return true;
        }
    }
    false
}
