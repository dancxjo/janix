pub fn rust_panic(info: &core::panic::PanicInfo) -> ! {
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

    let current_thread = kernel_core::sched::SCHEDULER
        .try_lock()
        .map(|sched| sched.current_id())
        .flatten();
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

    kernel_core::log("PANIC! Panic in the streets of Birmingham!");
    if let Some(location) = info.location() {
        kernel_core::println!(
            "Panic location: {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        kernel_core::println!("Panic location: <unknown>");
    }
    hcf();
}

pub fn hcf() -> ! {
    arch::cpu::halt_loop();
}
