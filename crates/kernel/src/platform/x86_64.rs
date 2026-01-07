//! x86_64 Platform layer.
//!
//! This module wires machine facilities to kernel services and publishes
//! platform-specific Things to the graph.

use crate::machine::machine;
use crate::machine::x86_64::{pci, usb};

/// Initialize the x86_64 platform.
pub fn init() {
    // Initialize LAPIC now that heap is ready
    let hhdm = crate::boot::get_boot_ctx().hhdm_offset;
    unsafe {
        crate::machine::x86_64::timer::init_lapic(hhdm);
    }

    crate::seeding::seed_platform_graph();
    setup_xhci();
    crate::log::kprintln("PLATFORM: x86_64 initialized");
}
/// Discover xHCI on PCI and expose it as Things/Bytespaces.
fn setup_xhci() {
    const XHCI_VECTOR: u8 = 45;
    const XHCI_MMIO_LEN: u64 = 0x10000;

    let Some(addr) = pci::scan_xhci() else {
        crate::log::kprintln("PLATFORM: no xHCI controller found");
        return;
    };

    let bar0 = addr.bar0();
    let irq_line = addr.interrupt_line();

    // Route legacy INTx line through IO-APIC to our vector.
    unsafe {
        crate::machine::x86_64::timer::route_irq(irq_line, XHCI_VECTOR);
    }

    let irq_phys = usb::irq_counter_phys();
    let irq_len = usb::irq_bytespace_len();
    crate::seeding::register_xhci(bar0, irq_line, XHCI_VECTOR, irq_phys, irq_len);
    crate::log::kprintln(&alloc::format!(
        "PLATFORM: xHCI bus={} dev={} irq_line={} bar0={:#x}",
        addr.bus,
        addr.device,
        irq_line,
        bar0
    ));
}
