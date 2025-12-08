use userland_rt::HostedSys;

/// ThingOS Host Harness
///
/// This binary simulates the kernel environment for userland applications.
/// IMPORTANT: This harness must maintain strict feature parity with the actual kernel.
/// Any syscall or capability available in `boot::sys_kernel::KernelSys` must be mirrored here.
fn main() {
    println!("=== ThingOS Host Harness ===");
    println!();

    let sys = HostedSys;

    println!("Running user_app_hello with HostedSys...");
    user_app_hello::run(&sys);
    println!("Finished user_app_hello.");
}
