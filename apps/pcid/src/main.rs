#![no_std]
#![no_main]

extern crate alloc;
extern crate thing_std;

mod access;
mod decode;
mod enumerate;
mod publish;

use thing_std::{log_info, sys_exit, init};
use thing_std::{thing_create, thing_register_name, relationship_create, thing_find, symbol_intern};
use models::PciEcamV1;
use access::{EcamAccess, LegacyIoAccess};
use models::ThingId;

#[no_mangle]
pub fn main() {
    init(0);
    log_info("pcid: starting PCI enumeration");

    // Ensure we have a place to put devices
    let devices_place = match thing_find("place.pci") {
        Some(d) => d,
        None => {
            // thing_create takes (kind, parent). If we pass ThingId(0), it is root?
            // Wait, graph.rs says: thing_create(kind, parent).
            // We want place.pci to be in place.root.
            let root = thing_find("place.root").unwrap_or(ThingId(0));
            // Ensure root is valid if possible.
            
            let p = thing_create(symbol_intern("kind.Place"), root);
            thing_register_name(p, "place.pci");
            // If parent=0 was passed effectively, or if we need explicit contains relationship:
            if root.0 != 0 {
                relationship_create(symbol_intern("contains"), root, p);
            }
            p
        }
    };

    // Strategy 1: Look for ECAM
    if let Some(_ecam_thing) = thing_find("device.pci.ecam0") {
        log_info("pcid: found ECAM descriptor");
        // TODO: Map ECAM
    }
    
    #[cfg(target_arch = "x86_64")]
    {
        // log_info("pcid: using Legacy IO access"); 
        // Commented out to reduce noise or avoid missing symbol if log_info macro vs func issues
        thing_std::log_info("pcid: using Legacy IO access");
        let access = LegacyIoAccess;
        enumerate::scan_segment(&access, 0, 0, 255, devices_place);
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    {
        // Must find ECAM. If not found, log error.
        if thing_find("device.pci.ecam0").is_none() {
            thing_std::log_info("pcid: NO ECAM found and not x86_64. Aborting.");
        }
    }

    log_info("pcid: enumeration complete");
    sys_exit(0);
}
