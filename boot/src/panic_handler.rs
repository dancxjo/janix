pub fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    kernel::println!("========== KERNEL PANIC ==========");
    let panic_message = info.message();
    kernel::println!("Message: {}", panic_message);
    if let Some(location) = info.location() {
        kernel::println!(
            "Location: {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        kernel::println!("Location: <unknown>");
    }

    let ticks = kernel::time::ticks_since_boot();
    let monotonic_ns = kernel::time::monotonic_now_ns();
    let (unix_seconds, unix_nanos) = kernel::time::now_unix_from_rtc();
    kernel::println!(
        "Time: ticks={} monotonic_ns={} unix_seconds={} unix_nanos={}",
        ticks,
        monotonic_ns,
        unix_seconds,
        unix_nanos
    );

    let snapshot = kernel::model::dashboard_snapshot();
    kernel::println!(
        "Memory: total_frames={} used_frames={} free_frames={}",
        snapshot.memory.total_frames,
        snapshot.memory.used_frames,
        snapshot.memory.free_frames
    );
    kernel::println!(
        "Scheduler: processes={} threads={} runnable={}",
        snapshot.scheduler.process_count,
        snapshot.scheduler.thread_count,
        snapshot.scheduler.runnable_threads
    );
    kernel::println!(
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

    let current_thread = kernel::sched::SCHEDULER
        .try_lock()
        .map(|sched| sched.current_id())
        .flatten();
    match current_thread {
        Some(tid) => kernel::println!("Current thread: {}", tid.0),
        None => kernel::println!("Current thread: <none>"),
    }

    const LOG_TAIL: usize = 10;
    let logs = kernel::get_logs();
    kernel::println!("Recent kernel log entries (last {}):", LOG_TAIL);
    let start = logs.len().saturating_sub(LOG_TAIL);
    for entry in &logs[start..] {
        if let Some(msg) = entry {
            kernel::println!("  {}", msg);
        }
    }

    kernel::log("PANIC! Panic in the streets of Birmingham!");
    if let Some(location) = info.location() {
        kernel::println!(
            "Panic location: {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        kernel::println!("Panic location: <unknown>");
    }
    hcf();
}

pub fn hcf() -> ! {
    arch::cpu::halt_loop();
}
