use models::PciFunctionV1;
use models::{ThingId, SymbolId};
use thing_std::{thing_create, thing_register_name, relationship_create, thing_find, symbol_intern, thing_set_body};
use thing_std::log_info;
use alloc::format;
use postcard::to_allocvec;

pub fn publish_function(parent: ThingId, func: &PciFunctionV1) {
    let name_str = format!("device.pci.{:04x}:{:02x}:{:02x}.{}", func.seg, func.bus, func.dev, func.fun);
    let name_sym = symbol_intern(&name_str);

    // Idempotent check
    // thing_find takes &str, not SymbolId.
    // But we already interned name_sym.
    // If we want to find by name, we should pass the str.
    // store::find_thing_by_name uses SymbolId in my previous code assumption.
    // thing_std::thing_find(name: &str) -> Option<ThingId>.
    
    let thing = if let Some(existing) = thing_find(&name_str) {
        existing
    } else {
        let t = thing_create(symbol_intern("kind.PciFunction"), ThingId(0));
        thing_register_name(t, &name_str);
        relationship_create(symbol_intern("contains"), parent, t);
        t
    };

    // Serialize payload
    match to_allocvec(func) {
        Ok(payload) => {
            let _ = thing_set_body(thing, &payload);
        }
        Err(_) => {
            log_info("Failed to serialize PCI function payload");
        }
    }
}
