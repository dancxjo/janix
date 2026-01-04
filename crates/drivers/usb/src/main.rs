#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

mod pci;
mod xhci;

#[no_mangle]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("USB: Driver starting");

    if let Some(addr) = pci::scan() {
        log_info(&alloc::format!("USB: xHCI Found at {}:{}:{} BAR0={:#x}",
            addr.bus, addr.device, addr.function, addr.bar0()));
            
        unsafe {
            if let Ok(mut xhc) = xhci::XhciController::new(addr.bar0() as u64) {
                 xhc.init();
            } else {
                 log_info("xHCI: Failed to map memory");
            }
        }
    } else {
        log_info("USB: xHCI Not Found");
    }

    loop {
        sched_yield();
    }
}
