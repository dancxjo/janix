use crate::hw::pci::{scan_pci, read_bar};
use crate::memory::phys_to_virt;
use thing_models::usb::UsbController;
use abi::{ThingId, Thing};
use alloc::vec::Vec;
use alloc::format;
use crate::graph;



pub fn init_xhci() {
    let devices = scan_pci();
    for dev in devices {
        // Class 0x0C (Serial Bus), Subclass 0x03 (USB), ProgIF 0x30 (XHCI)
        if dev.class == 0x0C && dev.subclass == 0x03 && dev.prog_if == 0x30 {
            crate::println!("Found XHCI Controller at {:02x}:{:02x}.{:x}", dev.bus, dev.slot, dev.func);
            
            let bar0 = read_bar(dev.bus, dev.slot, dev.func, 0);
            // Mask out flag bits (bit 0-3 usually) to get base address. 
            // TODO: Handle 64-bit BARs properly.
            
            let mmio_base_phys = (bar0 & 0xFFFFFFF0) as u64;
            let mmio_base_virt = phys_to_virt(mmio_base_phys);
            
            crate::println!("XHCI MMIO Base: Phys={:#x}, Virt={:#x}", mmio_base_phys, mmio_base_virt);
            
            // Create UsbController Thing
            let controller = UsbController {
                id: ThingId(0), // Dummy ID, will be assigned by graph
                name: format!("xhci_{:02x}_{:02x}_{:x}", dev.bus, dev.slot, dev.func),
                pci_bus: dev.bus,
                pci_slot: dev.slot,
                pci_func: dev.func,
                mmio_base: mmio_base_virt,
            };
            
            let mut props = Vec::new();
            controller.to_props(&mut props);
            
            // Add to graph
            graph::create_thing(UsbController::KIND, &props);
        }
    }
}

