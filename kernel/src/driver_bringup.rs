use crate::graph::sink::KernelGraphSink;
use crate::hal_impl::{KernelMmio, KernelPciConfigAccess};

pub fn init() {
    crate::log("Initializing drivers...");

    let mut graph_sink = KernelGraphSink;

    // 1. Initialize USB driver (Watch PciDevice)
    // We must do this BEFORE scanning PCI so that the watcher catches the creation events.
    let mmio = KernelMmio;
    drivers_usb::init(&mmio, &mut graph_sink);

    // 2. Initialize and run PCI driver (Publish PciDevice)
    let pci_config = KernelPciConfigAccess;
    let pci_driver = drivers_pci::PciDriver::new(&pci_config);
    pci_driver.scan_and_publish(&mut graph_sink);

    crate::log("Drivers initialized.");
}
