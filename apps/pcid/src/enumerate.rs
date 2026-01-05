use crate::access::ConfigAccess;
use crate::decode::decode_function;
use crate::publish::publish_function;
use models::ThingId;
use thing_std::log_info;
use alloc::format;

const MAX_BUS: u16 = 256;
const MAX_DEV: u8 = 32;
const MAX_FUN: u8 = 8;

pub fn scan_segment(access: &dyn ConfigAccess, seg: u16, bus_start: u8, bus_end: u8, parent: ThingId) {
    log_info(&format!("Scanning PCI segment {} buses {}-{}", seg, bus_start, bus_end));
    
    for bus in bus_start..=bus_end {
        for dev in 0..MAX_DEV {
            // Check usage of function 0 to determine presence and multifunction
            let func0 = decode_function(access, seg, bus, dev, 0);
            
            if func0.is_none() {
                continue;
            }
            
            let f0 = func0.unwrap();
            publish_function(parent, &f0);
            
            let is_multi = (f0.header_type & 0x80) != 0;
            
            if is_multi {
                for fun in 1..MAX_FUN {
                   if let Some(func) = decode_function(access, seg, bus, dev, fun) {
                       publish_function(parent, &func);
                   }
                }
            }
        }
    }
}
