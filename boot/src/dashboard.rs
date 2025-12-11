use crate::console::Console;
use core::fmt::Write;
use kernel::log;
use kernel::model::{DashboardSnapshot, dashboard_snapshot};

pub fn render_dashboard(console: &mut Console) {
    let snapshot: DashboardSnapshot = dashboard_snapshot();

    console.clear();

    let _ = writeln!(console, "ThingOS Boot Dashboard");
    let _ = writeln!(console, "======================");
    let _ = writeln!(console, "");

    // Memory
    let _ = writeln!(console, "Memory:");
    let _ = writeln!(
        console,
        "  frames: total={} used={} free={}",
        snapshot.memory.total_frames, snapshot.memory.used_frames, snapshot.memory.free_frames,
    );
    let _ = writeln!(console, "");

    // Scheduler
    let _ = writeln!(console, "Scheduler:");
    let _ = writeln!(
        console,
        "  processes={} threads={} runnable={}",
        snapshot.scheduler.process_count,
        snapshot.scheduler.thread_count,
        snapshot.scheduler.runnable_threads,
    );
    let _ = writeln!(console, "");

    // Thing counts
    let _ = writeln!(console, "Thing counts:");
    let _ = writeln!(console, "  total       = {}", snapshot.counts.total_things);
    let _ = writeln!(console, "  processes   = {}", snapshot.counts.processes);
    let _ = writeln!(console, "  threads     = {}", snapshot.counts.threads);
    let _ = writeln!(console, "  PhysFrame   = {}", snapshot.counts.phys_frames);
    let _ = writeln!(console, "  VirtRegion  = {}", snapshot.counts.virt_regions);
    let _ = writeln!(console, "  FramePool   = {}", snapshot.counts.frame_pools);
    let _ = writeln!(
        console,
        "  AddressSpace= {}",
        snapshot.counts.address_spaces
    );
    let _ = writeln!(console, "  CpuCore     = {}", snapshot.counts.cpu_cores);
    let _ = writeln!(console, "");

    // Optional: tail of kernel logs
    let _ = writeln!(console, "Kernel log (tail):");
    let logs = kernel::log::get_logs();
    let start = logs.len().saturating_sub(5);
    for entry in &logs[start..] {
        if let Some(msg) = entry {
            let _ = writeln!(console, "  {}", msg);
        }
    }

    log::log_message("Rendered boot dashboard");
}
