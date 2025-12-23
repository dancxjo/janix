extern crate alloc;

use alloc::format;

use abi::ThingId;
use thing_models::PropValue;

use crate::graph;
use crate::graph_kinds;
use crate::symbols;

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

fn spawn_driver_by_name(name: &str) {
    match find_boot_program(name) {
        Some(id) => {
            crate::log::log_message(&format!("Spawning userland driver: {}", name));
            match crate::spawn_program(id) {
                Ok((_pid, _)) => crate::log::log_message("Driver spawned successfully."),
                Err(e) => {
                    crate::log::log_message("Failed to spawn driver:");
                    crate::log::log_message(e);
                }
            }
        }
        None => {
            crate::log::log_message(&format!("Driver BootProgram not found: {}", name));
        }
    }
}

pub fn init() {
    crate::log("Initializing drivers...");

    #[cfg(all(not(test), target_arch = "x86_64"))]
    crate::bridge::ps2::init();

    #[cfg(all(not(test), target_arch = "x86_64"))]
    crate::bridge::ata::init();

    // 1. Initialize USB driver (Userland)
    if let Some(usb_program) = find_boot_program("usb") {
        match crate::spawn_program(usb_program) {
            Ok((pid, _)) => {
                let msg = format!("Spawned USB driver (PID {})", pid.0);
                crate::log::log_message(&msg);
            }
            Err(e) => {
                let msg = format!("Failed to spawn USB driver: {}", e);
                crate::log::log_message(&msg);
            }
        }
    } else {
        crate::log::log_message(
            "USB driver BootProgram not found. Ensure it is added to limine.conf and built.",
        );
    }

    // 2. Initialize PCI driver (Userland)
    crate::log("Spawning userland PCI driver...");
    spawn_driver_by_name("pci");

    crate::log("Drivers initialized (Userland transition).");
}
