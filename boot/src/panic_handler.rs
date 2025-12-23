pub fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    kernel::console::print("\n========== PAVOR NUCLEI ==========\n");
    let panic_message = info.message();
    kernel::console::print("Nuntius: ");
    kernel::println!("{}", panic_message);
    
    if let Some(location) = info.location() {
         kernel::console::print("Locus: ");
         kernel::println!("{}:{}:{}", location.file(), location.line(), location.column());
    } else {
         kernel::console::print("Locus: <incertus>\n");
    }

    let ticks = kernel::time::ticks_since_boot();
    let monotonic_ns = kernel::time::monotonic_now_ns();
    let (unix_seconds, unix_nanos) = kernel::time::now_unix_from_rtc();
    kernel::println!(
        "Tempus: tictus={} monotonic_ns={} unix_secundae={} unix_nani={}",
        ticks,
        monotonic_ns,
        unix_seconds,
        unix_nanos
    );

    let snapshot = kernel::model::dashboard_snapshot();
    kernel::println!(
        "Memoria: tabulae_totales={} tabulae_usitatae={} tabulae_liberae={}",
        snapshot.memory.total_frames,
        snapshot.memory.used_frames,
        snapshot.memory.free_frames
    );
    kernel::println!(
        "Ordonnator: processus={} fila={} curribilia={}",
        snapshot.scheduler.process_count,
        snapshot.scheduler.thread_count,
        snapshot.scheduler.runnable_threads
    );
    kernel::println!(
        "Numeri Rerum: summa={} processus={} fila={} tabulae_physicae={} regiones_virtuosae={} piscinae_tabularum={} spatia_adressuum={} nuclei_cpu={}",
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
        Some(tid) => kernel::println!("Fila praesens: {}", tid.0),
        None => kernel::println!("Fila praesens: <nulla>"),
    }

    const LOG_TAIL: usize = 10;
    let logs = kernel::get_logs();
    kernel::println!("Commentarii nuclei recentes (postrema {}):", LOG_TAIL);
    let start = logs.len().saturating_sub(LOG_TAIL);
    for entry in &logs[start..] {
        if let Some(msg) = entry {
            kernel::println!("  {}", msg);
        }
    }

    kernel::log("PAVOR! Pavor per vias Birminghamienses!");
    if let Some(location) = info.location() {
        kernel::println!(
            "Locus pavoris: {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        kernel::println!("Locus pavoris: <incertus>");
    }
    hcf();
}

pub fn hcf() -> ! {
    arch::cpu::halt_loop();
}
