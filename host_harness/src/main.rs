use userland_rt::HostedSys;
use kernel_core::model::dashboard_snapshot;

/// ThingOS Host Harness
///
/// This binary simulates the kernel environment for userland applications.
/// IMPORTANT: This harness must maintain strict feature parity with the actual kernel.
/// Any syscall or capability available in `boot::sys_kernel::KernelSys` must be mirrored here.
fn main() {
    println!("=== ThingOS Host Harness ===");
    println!();

    // Initialize kernel core (simulated)
    kernel_core::init();
    kernel_core::create_builtin_things();

    // Seed a fake memory graph
    kernel_core::model::create_frame_pool(0x1000, 0x9000, 4096);
    kernel_core::model::create_cpu_core(0);

    // Print dashboard snapshot
    let snap = dashboard_snapshot();

    println!("Host Dashboard Snapshot:");
    println!("  Memory: total={} used={} free={}",
        snap.memory.total_frames,
        snap.memory.used_frames,
        snap.memory.free_frames,
    );
    println!("  Scheduler: processes={} threads={} runnable={}",
        snap.scheduler.process_count,
        snap.scheduler.thread_count,
        snap.scheduler.runnable_threads,
    );
    println!("  Things:");
    println!("    total       = {}", snap.counts.total_things);
    println!("    processes   = {}", snap.counts.processes);
    println!("    threads     = {}", snap.counts.threads);
    println!("    PhysFrame   = {}", snap.counts.phys_frames);
    println!("    VirtRegion  = {}", snap.counts.virt_regions);
    println!("    FramePool   = {}", snap.counts.frame_pools);
    println!("    AddressSpace= {}", snap.counts.address_spaces);
    println!("    CpuCore     = {}", snap.counts.cpu_cores);

    let sys = HostedSys;

    // Test MemorySummary
    if let Some(mem) = userland_std::memory_summary() {
        println!(
            "Memory summary: total={} used={} free={}",
            mem.total_frames, mem.used_frames, mem.free_frames
        );
    } else {
        println!("Memory summary: unavailable");
    }

    if let Some(sched) = userland_std::scheduler_summary() {
        println!(
            "Scheduler summary: processes={} threads={} runnable={}",
            sched.process_count, sched.thread_count, sched.runnable_threads
        );
    } else {
        println!("Scheduler summary: unavailable");
    }

    println!("Running user_app_hello with HostedSys...");
    user_app_hello::run(&sys);
    println!("Finished user_app_hello.");
}
