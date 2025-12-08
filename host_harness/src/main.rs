use userland_rt::HostedSys;
use abi::KernelRequest;

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

    let sys = HostedSys;

    // Test MemorySummary
    let resp = kernel_core::handle_request(KernelRequest::MemorySummary);
    println!("Memory summary (host): {resp:?}");

    println!("Running user_app_hello with HostedSys...");
    user_app_hello::run(&sys);
    println!("Finished user_app_hello.");
}
