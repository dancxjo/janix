use kernel_core::model::dashboard_snapshot;

mod frame_pool;
use frame_pool::{init_host_frame_pool, allocate_frame, free_frame, frame_stats};
use abi::{KernelRequest, KernelResponse, FrameInfo, FrameId, MemorySummary};
use userland_rt::Sys;

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
                    KernelResponse::Error { message: "Host out of frames" }
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
                if let KernelRequest::Log { message } = &request {
                    println!("{}", message);
                }
                kernel_core::handle_request(request)
            }
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
    kernel_core::create_builtin_things();

    // Seed a fake memory graph
    kernel_core::model::create_frame_pool(0x1000, 0x9000, 4096);
    kernel_core::model::create_cpu_core(0);

    init_host_frame_pool(128);
    let sys = HarnessSys;

    println!("Running user_app_hello with HarnessSys...");
    user_app_hello::run(&sys);
    println!("Finished user_app_hello.");

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
