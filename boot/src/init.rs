use alloc::boxed::Box;

use arch::{Arch, CurrentArch};

pub fn init_machine() {
    // All limine requests must also be referenced in a called function
    assert!(crate::BASE_REVISION.is_supported());

    // Initialize console early to show boot progress
    init_console();

    unsafe {
        // Heap is initialized in kmain
        let start = core::ptr::addr_of_mut!(crate::HEAP_MEMORY) as usize;
        let end = start + crate::HEAP_SIZE;
        // We can't use alloc::format! here if heap is not initialized?
        // But it IS initialized in kmain.
        // However, we don't need to re-init.
        // And we can log.
        let msg = alloc::format!("Acervus nuclei initus: [{:#x}, {:#x})", start, end);
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        // kernel::log(leaked); // Replaced by step below
        crate::boot_screen::step(leaked);
    }

    if let Some(hhdm_response) = crate::boot_model::HHDM_REQUEST.get_response() {
        let offset = hhdm_response.offset();
        kernel::memory::set_hhdm_offset(offset);
        unsafe { arch::user::init_user_stack(offset) };
    }

    // Initialize architecture-specific tables (GDT, etc.)
    // This MUST happen before we try to enter user mode or load segment selectors.
    crate::boot_screen::step("Tabulas architecturae instruit...");
    let t = kernel::time::boot_span_start("init_arch_tables");
    arch::platform::init_arch_tables();
    kernel::time::boot_span_end("init_arch_tables", t);

    crate::boot_screen::step("Cor nuclei instruitur...");
    let t = kernel::time::boot_span_start("kernel::init");
    kernel::init();
    kernel::time::boot_span_end("kernel::init", t);

    kernel::register_spawn_program_handler(crate::program::spawn_program);
    {
        let mut sched = kernel::sched::SCHEDULER.lock();
        sched.init_graph_mirror();
    }
    crate::boot_screen::step("Cor nuclei initum est.");

    // Seed memory graph early so frame allocator is available for arch init
    // (AArch64 needs this for paging::map_device_region during map_boot_device_regions)
    let t = kernel::time::boot_span_start("seed_memory_graph");
    crate::boot_model::seed_memory_graph_from_limine();
    kernel::time::boot_span_end("seed_memory_graph", t);
    crate::boot_screen::step("Graphis memoriae seminatus est.");

    // Seed DTB frequency (RISC-V)
    #[cfg(target_arch = "riscv64")]
    {
        let freq_override =
            crate::boot_model::get_kernel_arg("timer_freq=").and_then(|s| s.parse::<u64>().ok());
        arch::riscv64::dtb::init(freq_override);
    }

    let t = kernel::time::boot_span_start("map_boot_device_regions");
    if arch::platform::map_boot_device_regions() {
        crate::boot_screen::step("Regiones PCI mappatae.");
        #[cfg(target_arch = "aarch64")]
        {
            crate::serial::arch::init_serial(kernel::memory::get_hhdm_offset());
            kernel::log("PL011 initum est.");
        }
        #[cfg(target_arch = "riscv64")]
        {
            crate::serial::arch::init_serial(kernel::memory::get_hhdm_offset());
            kernel::log("NS16550 initum est.");
        }
    }
    kernel::time::boot_span_end("map_boot_device_regions", t);

    crate::boot_screen::step("Subnotationes graphidis instruuntur...");
    crate::graph_reifier::init_graph_subscriptions();

    // Register IRQ controller callback to manage IRQ masking via graph requests
    #[cfg(target_arch = "x86_64")]
    kernel::bridge::io::register_irq_controller(arch::x86_64::pic::set_irq_mask);

    crate::boot_screen::step("Curator vocationis systematis inseritur...");
    CurrentArch::install_syscall_handler();

    let rtc_epoch = arch::read_boot_rtc_epoch_seconds();
    crate::time_utils::log_rtc_epoch(rtc_epoch);
    crate::boot_screen::step("Chronologia instruitur...");
    kernel::time::init_timekeeping(rtc_epoch);

    // init_console(); // Moved to top

    crate::boot_screen::step("ThingOS incipit...");
}

pub fn init_world_graph() {
    let t_all = kernel::time::boot_span_start("init_world_graph");

    crate::boot_screen::step("Res intrinsecae creantur...");
    kernel::create_builtin_things();

    // seed_memory_graph_from_limine moved to init_machine
    crate::boot_model::seed_cpu_graph_from_limine();
    crate::boot_model::seed_display_from_limine();
    crate::boot_model::seed_boot_profile();
    crate::boot_model::seed_font_modules_from_limine();

    let t = kernel::time::boot_span_start("seed_images");
    crate::boot_model::seed_program_images_from_limine();
    kernel::time::boot_span_end("seed_images", t);

    crate::boot_model::seed_raw_modules_from_limine();
    crate::boot_model::seed_boot_programs_from_limine();

    crate::boot_screen::step("Moderatores instrumentorum instruuntur...");
    let t = kernel::time::boot_span_start("driver_bringup");
    kernel::driver_bringup::init();
    kernel::time::boot_span_end("driver_bringup", t);

    crate::boot_model::seed_time_graph();
    kernel::bridge::io::seed_io_regions();

    kernel::time::boot_span_end("init_world_graph", t_all);
}

#[cfg(not(feature = "boot-dashboard-only"))]
pub fn init_userland_and_enter_scheduler() -> ! {
    crate::boot_screen::step("Init (PID 1) exsolvitur...");
    launch_init_process();
    crate::boot_screen::step("Fila otiosa mittitur...");
    launch_idle_thread();
    crate::boot_screen::step("Imperium schedulatori traditur...");
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
        kernel::log("Nulla tabula imaginis ad tabulam praesto");
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
    let is_debug_profile = crate::boot_model::get_kernel_arg("profile=")
        .map(|s| s == "debug")
        .unwrap_or(false);

    let (binary, app_id) = if is_debug_profile {
        ("init_debug", 1)
    } else {
        ("init", 1)
    };

    kernel::log("launch_init_process: PID 1 progeneratur");
    // Ensure clock is seeded if we use it. The prior boot_model logic ensures it.

    if let Err(err) = crate::program::spawn_program_by_identifier(binary, binary, app_id) {
        kernel::log("launch_init_process: defecit in generando PID 1");
        kernel::log(err);
        crate::panic_handler::hcf();
    }

    if !is_debug_profile && kernel::model::program_image_exists("debug_input_events") {
        kernel::log("launch_init_process: generans applicationem diagnosticam debug_input_events");
        if let Err(err) = crate::program::spawn_program_by_identifier(
            "debug_input_events",
            "debug_input_events",
            100,
        ) {
            kernel::log("launch_init_process: defecit in generando debug_input_events");
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
