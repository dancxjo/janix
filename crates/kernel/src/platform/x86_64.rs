//! x86_64 Platform layer.
//!
//! This module wires machine facilities to kernel services and publishes
//! platform-specific Things to the graph.

use crate::machine::machine;
use graph::store;
use graph::symbols::{self, sym};

/// Initialize the x86_64 platform.
pub fn init() {
    seed_platform_graph();
    crate::log::kprintln("PLATFORM: x86_64 initialized");
}

/// Seed CPU, Timer, and InterruptController Things into the graph.
fn seed_platform_graph() {
    let apic_id = machine().local_cpu_id();
    let timer_freq = machine().timer_frequency_hz();
    let resolution_ns = if timer_freq > 0 { 1_000_000_000u64 / timer_freq as u64 } else { 0 };

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
        "PLATFORM: CPU apic_id={}, timer freq={}Hz", apic_id, timer_freq
    ));
}
