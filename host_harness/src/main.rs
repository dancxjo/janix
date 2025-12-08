fn main() {
    println!("=== ThingOS Host Harness ===");
    println!();

    // Initialize kernel_core in a "host" simulation
    kernel_core::init();
    kernel_core::log("ThingOS booting (host)");
    kernel_core::log("Initializing kernel core");
    kernel_core::log("Graph subsystem initialized");
    kernel_core::log("Transaction subsystem initialized");
    kernel_core::log("Log subsystem initialized");
    kernel_core::log("ThingOS started successfully");

    println!("Kernel Logs:");
    for (i, entry) in kernel_core::get_logs().iter().enumerate() {
        if let Some(msg) = entry {
            println!("  [Entry {:04}] {}", i, msg);
        }
    }
    println!();
    println!("Host harness exercises the same kernel_core logic as the boot path.");
}
