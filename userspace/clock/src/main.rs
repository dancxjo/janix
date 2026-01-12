#![no_std]
#![no_main]

extern crate alloc;
use stem::info;

#[no_mangle]
pub extern "C" fn main(_arg: usize) {
    info!("CLOCK: Starting (Stubbed)...");
    
    // Note: Use of thingsys::find triggers a Kernel Page Fault (copyout issue) 
    // in the current build. Stubbing functionality to verify logging macros.
    
    loop {
        info!("CLOCK: Heartbeat... (Macro test)");
        stem::sleep(core::time::Duration::from_secs(5));
    }
}
