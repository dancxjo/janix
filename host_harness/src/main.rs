use abi::PropValue;

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

    // Create some demo Things
    println!("Creating kernel Things...");
    
    let system_props: &'static [(abi::PropKey, PropValue)] = &[
        ("version", PropValue::U64(1)),
        ("enabled", PropValue::Bool(true)),
    ];
    
    let counter_props: &'static [(abi::PropKey, PropValue)] = &[
        ("count", PropValue::U64(42)),
        ("max", PropValue::U64(100)),
    ];
    
    if let Some(system_id) = kernel_core::graph::create_thing("System", system_props) {
        kernel_core::log("Created System Thing");
        println!("  System Thing ID: {:?}", system_id);
        
        // Retrieve and display it
        if let Some((kind, props)) = kernel_core::graph::get_thing(system_id) {
            println!("  Retrieved: kind={}, props:", kind);
            for prop in props {
                if let Some((k, v)) = prop {
                    println!("    {} = {:?}", k, v);
                }
            }
        }
    }
    
    if let Some(counter_id) = kernel_core::graph::create_thing("Counter", counter_props) {
        kernel_core::log("Created Counter Thing");
        println!("  Counter Thing ID: {:?}", counter_id);
        
        // Retrieve and display it
        if let Some((kind, props)) = kernel_core::graph::get_thing(counter_id) {
            println!("  Retrieved: kind={}, props:", kind);
            for prop in props {
                if let Some((k, v)) = prop {
                    println!("    {} = {:?}", k, v);
                }
            }
        }
    }

    kernel_core::log("ThingOS started successfully");

    println!();
    println!("Kernel Logs:");
    for (i, entry) in kernel_core::get_logs().iter().enumerate() {
        if let Some(msg) = entry {
            println!("  [Entry {:04}] {}", i, msg);
        }
    }
    println!();
    println!("Host harness exercises the same kernel_core logic as the boot path.");
}
