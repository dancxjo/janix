//! x86_64 Platform layer.
//!
//! This module wires machine facilities to kernel services and publishes
//! platform-specific Things to the graph.

use crate::machine::machine;
use crate::machine::x86_64::{pci, usb};
use abi::bodies::BYTESPACE_FLAG_HAS_PHYS_BASE;
use graph::store;
use graph::symbols::{self, sym};
use models::BytespaceBody;
use models::Thing;
use alloc::vec::Vec;

/// Initialize the x86_64 platform.
pub fn init() {
    // Initialize LAPIC now that heap is ready
    let hhdm = crate::boot::get_boot_ctx().hhdm_offset;
    unsafe {
        crate::machine::x86_64::timer::init_lapic(hhdm);
    }

    seed_platform_graph();
    setup_xhci();
    crate::log::kprintln("PLATFORM: x86_64 initialized");
}

/// Seed CPU, Timer, and InterruptController Things into the graph.
fn seed_platform_graph() {
    let apic_id = machine().local_cpu_id();
    let timer_freq = machine().timer_frequency_hz();
    let resolution_ns = if timer_freq > 0 {
        1_000_000_000u64 / timer_freq as u64
    } else {
        0
    };

    // Create place.platform
    let platform_place = if let Some(p) = store::find_thing_by_name(sym::PLACE_PLATFORM) {
        p
    } else {
        let p = store::thing_create(sym::KIND_PLACE);
        store::thing_register_name(p, sym::PLACE_PLATFORM);
        if let Some(root) = store::find_thing_by_name(sym::PLACE_ROOT) {
            store::relationship_create(sym::PRED_CONTAINS, root, p);
        }
        p
    };

    // Create CPU Thing (cpu.0)
    let cpu_thing = store::thing_create(sym::KIND_CPU);
    let cpu_name = symbols::intern(b"cpu.0");
    store::thing_register_name(cpu_thing, cpu_name);
    store::relationship_create(sym::PRED_CONTAINS, platform_place, cpu_thing);

    let mut cpu_payload = alloc::vec::Vec::new();
    cpu_payload.extend_from_slice(&apic_id.to_le_bytes());
    cpu_payload.push(1u8); // is_bsp
    store::thing_set_inline_payload(cpu_thing, &cpu_payload);

    // Create Timer Thing
    let timer_thing = store::thing_create(sym::KIND_TIMER);
    let timer_name = symbols::intern(b"lapic_timer.0");
    store::thing_register_name(timer_thing, timer_name);
    store::relationship_create(sym::PRED_CONTAINS, platform_place, timer_thing);

    let mut timer_payload = alloc::vec::Vec::new();
    timer_payload.extend_from_slice(b"lapic\0\0\0");
    timer_payload.extend_from_slice(&timer_freq.to_le_bytes());
    timer_payload.push(1u8); // periodic
    timer_payload.extend_from_slice(&resolution_ns.to_le_bytes());
    store::thing_set_inline_payload(timer_thing, &timer_payload);

    store::relationship_create(sym::PRED_HAS_TIMER, cpu_thing, timer_thing);

    // Create InterruptController Thing
    let ic_thing = store::thing_create(sym::KIND_INTERRUPT_CONTROLLER);
    let ic_name = symbols::intern(b"lapic.0");
    store::thing_register_name(ic_thing, ic_name);
    store::relationship_create(sym::PRED_CONTAINS, platform_place, ic_thing);

    let mut ic_payload = alloc::vec::Vec::new();
    ic_payload.extend_from_slice(b"lapic\0\0\0");
    ic_payload.extend_from_slice(b"local\0\0\0");
    store::thing_set_inline_payload(ic_thing, &ic_payload);

    crate::log::kprintln(&alloc::format!(
        "PLATFORM: CPU apic_id={}, timer freq={}Hz",
        apic_id,
        timer_freq
    ));
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

    // Graph plumbing
    let devices_place = store::find_thing_by_name(sym::PLACE_DEVICES)
        .unwrap_or_else(|| store::thing_create(sym::KIND_PLACE));

    // MMIO bytespace
    let mmio_bs = store::thing_create(sym::KIND_BYTE_SPACE);
    store::thing_register_name(mmio_bs, symbols::intern(b"bytespace.usb.xhci0.mmio"));
    store::relationship_create(sym::PRED_CONTAINS, devices_place, mmio_bs);

    let mmio_payload = BytespaceBody {
        len: XHCI_MMIO_LEN,
        flags: BYTESPACE_FLAG_HAS_PHYS_BASE,
        _pad: 0,
        phys_base: bar0,
    };
    store::thing_set_inline_payload(mmio_bs, &mmio_payload.encode());

    let mmio_phys = store::thing_create(sym::KIND_PLACE);
    store::thing_set_inline_payload(mmio_phys, &bar0.to_le_bytes());
    store::relationship_create(sym::PRED_BASE_PHYS, mmio_bs, mmio_phys);

    let mmio_size = store::thing_create(sym::KIND_PLACE);
    store::thing_set_inline_payload(mmio_size, &XHCI_MMIO_LEN.to_le_bytes());
    store::relationship_create(sym::PRED_SIZE, mmio_bs, mmio_size);

    // IRQ bytespace (simple counter)
    let irq_bs = store::thing_create(sym::KIND_BYTE_SPACE);
    store::thing_register_name(irq_bs, symbols::intern(b"bytespace.irq.usb.xhci0"));
    store::relationship_create(sym::PRED_CONTAINS, devices_place, irq_bs);

    let irq_phys = usb::irq_counter_phys();
    let irq_len = usb::irq_bytespace_len();
    let irq_payload = BytespaceBody {
        len: irq_len,
        flags: BYTESPACE_FLAG_HAS_PHYS_BASE,
        _pad: 0,
        phys_base: irq_phys,
    };
    store::thing_set_inline_payload(irq_bs, &irq_payload.encode());

    let irq_phys_thing = store::thing_create(sym::KIND_PLACE);
    store::thing_set_inline_payload(irq_phys_thing, &irq_phys.to_le_bytes());
    store::relationship_create(sym::PRED_BASE_PHYS, irq_bs, irq_phys_thing);

    let irq_size_thing = store::thing_create(sym::KIND_PLACE);
    store::thing_set_inline_payload(irq_size_thing, &irq_len.to_le_bytes());
    store::relationship_create(sym::PRED_SIZE, irq_bs, irq_size_thing);

    // Controller Thing
    let ctrl = store::thing_create(sym::KIND_XHCI_CONTROLLER);
    store::thing_register_name(ctrl, symbols::intern(b"device.usb.controller0"));
    store::relationship_create(sym::PRED_CONTAINS, devices_place, ctrl);
    store::relationship_create(sym::PRED_MMIO, ctrl, mmio_bs);
    store::relationship_create(sym::PRED_IRQ, ctrl, irq_bs);

    // Inline payload: bar0 | irq line | vector
    let mut ctrl_payload = Vec::new();
    ctrl_payload.extend_from_slice(&bar0.to_le_bytes());
    ctrl_payload.push(irq_line);
    ctrl_payload.push(XHCI_VECTOR);
    store::thing_set_inline_payload(ctrl, &ctrl_payload);

    crate::log::kprintln(&alloc::format!(
        "PLATFORM: xHCI bus={} dev={} irq_line={} bar0={:#x}",
        addr.bus,
        addr.device,
        irq_line,
        bar0
    ));
}
