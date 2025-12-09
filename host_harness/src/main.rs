use kernel_core::model::dashboard_snapshot;
use kernel_core::console::{ConsoleSink, register_sink};

mod frame_pool;
use abi::{FrameId, FrameInfo, KernelRequest, KernelResponse, MemorySummary};
use frame_pool::{allocate_frame, frame_stats, free_frame, init_host_frame_pool};
use userland_rt::Sys;

struct HostConsole;
unsafe impl Sync for HostConsole {}
unsafe impl Send for HostConsole {}

impl ConsoleSink for HostConsole {
    fn write_str(&self, s: &str) {
        print!("{}", s);
    }
}

static HOST_CONSOLE: HostConsole = HostConsole;

struct HarnessSys;

impl Sys for HarnessSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        match request {
            KernelRequest::AllocFrame { .. } => {
                if let Some(frame) = allocate_frame() {
                    let frame_info = FrameInfo {
                        id: FrameId(frame.id),
                        base: frame.id,
                        size: 4096,
                    };
                    KernelResponse::FrameAllocated { frame: frame_info }
                } else {
                    KernelResponse::Error {
                        message: "Host out of frames",
                    }
                }
            }
            KernelRequest::FreeFrame { frame_id } => {
                free_frame(frame_id.0);
                KernelResponse::FrameFreed { frame_id }
            }
            KernelRequest::GetMemorySummary => {
                let (total, used, free) = frame_stats();
                let summary = MemorySummary {
                    total_frames: total,
                    used_frames: used,
                    free_frames: free,
                };
                KernelResponse::MemorySummary { summary }
            }
            _ => {
                kernel_core::handle_request(request)
            }
        }
    }

    fn time_now_ns(&mut self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn time_system_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn sleep_until_ns(&mut self, deadline_ns: u64) {
        use std::thread;
        use std::time::{Duration, SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        if deadline_ns > now {
            thread::sleep(Duration::from_nanos(deadline_ns - now));
        }
    }
}

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
    register_sink(&HOST_CONSOLE);
    kernel_core::create_builtin_things();

    // Seed a fake memory graph
    kernel_core::model::create_frame_pool(0x1000, 0x9000, 4096);
    kernel_core::model::create_cpu_core(0);

    init_host_frame_pool(128);
    let mut sys = HarnessSys;

    println!("Running user_app_hello with HarnessSys...");
    user_app_hello::run(&mut sys);

    println!("Running user_app_heartbeat with HarnessSys...");
    user_app_heartbeat::run(&mut sys);

    println!("Starting scheduler loop...");
    for _ in 0..20 {
        if let Some(thread) = userland_std::scheduler_tick(&sys) {
            match thread.tid {
                101 => user_app_hello::tick(&sys),
                201 => user_app_heartbeat::tick(&mut sys),
                _ => println!("Unknown thread: {}", thread.tid),
            }
        } else {
            println!("No runnable threads");
        }
    }
    println!("Finished scheduler loop.");

    // Print dashboard snapshot
    let snap = dashboard_snapshot();
    let (mem_total, mem_used, mem_free) = frame_stats();

    println!("Host Dashboard Snapshot:");
    println!(
        "  Memory: total={} used={} free={}",
        mem_total, mem_used, mem_free,
    );
    println!(
        "  Scheduler: processes={} threads={} runnable={}",
        snap.scheduler.process_count, snap.scheduler.thread_count, snap.scheduler.runnable_threads,
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
}
