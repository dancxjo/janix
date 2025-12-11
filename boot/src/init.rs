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
        kernel_core::log(leaked);
    }

    if let Some(hhdm_response) = crate::boot_model::HHDM_REQUEST.get_response() {
        let offset = hhdm_response.offset();
        kernel_core::memory::set_hhdm_offset(offset);
        unsafe { arch::user::init_user_stack(offset) };
    }

    if arch::platform::map_boot_device_regions() {
        kernel_core::log("PCI regions mapped.");
    }

    kernel_core::log("Initializing kernel core...");
    kernel_core::init();

    kernel_core::register_spawn_program_handler(crate::program::spawn_program);
    {
        let mut sched = kernel_core::sched::SCHEDULER.lock();
        sched.init_graph_mirror();
    }
    kernel_core::log("Kernel core initialized.");
    crate::graph_reifier::init_graph_subscriptions();

    arch::platform::init_arch_tables();

    CurrentArch::install_syscall_handler();

    let rtc_epoch = arch::read_boot_rtc_epoch_seconds();
    crate::time_utils::log_rtc_epoch(rtc_epoch);
    kernel_core::time::init_timekeeping(rtc_epoch);

    init_console();

    kernel_core::log("ThingOS booting...");
}

pub fn init_world_graph() {
    kernel_core::create_builtin_things();
    crate::boot_model::seed_memory_graph_from_limine();
    crate::boot_model::seed_cpu_graph_from_limine();
    crate::boot_model::seed_display_from_limine();
    crate::boot_model::seed_boot_profile();
    crate::boot_model::seed_font_modules_from_limine();
    crate::boot_model::seed_program_images_from_limine();
    crate::boot_model::seed_raw_modules_from_limine();
    crate::boot_model::seed_boot_programs_from_limine();
    crate::boot_model::seed_time_graph();
    kernel_core::hw::io::seed_io_regions();
}

#[cfg(not(feature = "boot-dashboard-only"))]
pub fn init_userland_and_enter_scheduler() -> ! {
    kernel_core::log("Launching init (PID 1) ...");
    launch_init_process();
    kernel_core::log("Handing control to scheduler...");
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
        kernel_core::log("No framebuffer available for dashboard");
    }
    crate::panic_handler::hcf();
}

pub fn launch_init_process() {
    kernel_core::log("launch_init_process: spawning init via ProgramImage");
    if let Err(err) = crate::program::spawn_program_by_identifier("init", "init", 1) {
        kernel_core::log("launch_init_process: failed to spawn init via ProgramImage");
        kernel_core::log(err);
        crate::panic_handler::hcf();
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
