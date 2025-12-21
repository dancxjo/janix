use crate::graph::sink::KernelGraphSink;
use crate::hal_impl::{KernelMmio, KernelPciConfigAccess};
use alloc::format;
use crate::graph;
use crate::graph_kinds;
use crate::symbols;
use thing_models::PropValue;
use abi::ThingId;

fn spawn_driver_by_name(name: &str) {
    let kind_boot_program = symbols::intern(graph_kinds::KIND_BOOT_PROGRAM);
    let mut current_id = ThingId(0);

    while let Some(id) = graph::next_thing_of_kind(kind_boot_program, current_id) {
        if let Some(PropValue::Str(prog_name)) = graph::get_prop(id, "name") {
             if prog_name == name {
                 crate::log::log_message(&format!("Spawning userland driver: {}", name));
                 match crate::spawn_program(id) {
                     Ok(_) => crate::log::log_message("Driver spawned successfully."),
                     Err(e) => {
                         crate::log::log_message("Failed to spawn driver:");
                         // e is &'static str, so we can pass it directly
                         crate::log::log_message(e);
                     }
                 }
                 return;
             }
        }
        current_id = id;
    }
    crate::log::log_message(&format!("Driver BootProgram not found: {}", name));
}

pub fn init() {
    crate::log("Initializing drivers...");

    let mut graph_sink = KernelGraphSink;

    // 1. Initialize USB driver (Watch PciDevice)
    // We must do this BEFORE scanning PCI so that the watcher catches the creation events.
    // let mmio = KernelMmio;
    // drivers_usb::init(&mmio, &mut graph_sink);
    // TODO: Spawn userland USB driver

    // 2. Initialize and run PCI driver (Publish PciDevice)
    // let pci_config = KernelPciConfigAccess;
    // let pci_driver = drivers_pci::PciDriver::new(&pci_config);
    // pci_driver.scan_and_publish(&mut graph_sink);
    crate::log("Spawning userland PCI driver...");
    spawn_driver_by_name("pci");

    crate::log("Drivers initialized (Userland transition).");
}
