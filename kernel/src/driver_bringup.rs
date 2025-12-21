extern crate alloc;
use crate::graph;
use crate::symbols;
use crate::graph_kinds;
use abi::ThingId;
use thing_models::PropValue;
use alloc::format;

fn find_boot_program(name: &str) -> Option<ThingId> {
    let kind = symbols::intern(graph_kinds::KIND_BOOT_PROGRAM);
    let mut current = ThingId(0);
    while let Some(next) = graph::next_thing_of_kind_sym(kind, current) {
         if let Some(PropValue::Str(s)) = graph::get_prop(next, "name") {
             if s == name {
                 return Some(next);
             }
         }
         current = next;
    }
    None
}

pub fn init() {
    crate::log("Initializing drivers...");

    // 1. Initialize USB driver (Userland)
    if let Some(usb_program) = find_boot_program("usb") {
         match crate::spawn_program(usb_program) {
             Ok((pid, _)) => {
                 let msg = format!("Spawned USB driver (PID {})", pid.0);
                 crate::log::log_message(&msg);
             },
             Err(e) => {
                 let msg = format!("Failed to spawn USB driver: {}", e);
                 crate::log::log_message(&msg);
             },
         }
    } else {
         crate::log::log_message("USB driver BootProgram not found. Ensure it is added to limine.conf and built.");
    }

    // 2. Initialize and run PCI driver (Publish PciDevice)
    // let mut graph_sink = crate::graph::sink::KernelGraphSink;
    // let pci_config = crate::hal_impl::KernelPciConfigAccess;
    // let pci_driver = drivers_pci::PciDriver::new(&pci_config);
    // pci_driver.scan_and_publish(&mut graph_sink);
    // TODO: Spawn userland PCI driver

    crate::log("Drivers initialized (Userland transition).");
}
