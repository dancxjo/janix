#![no_std]
#![no_main]

extern crate alloc;
use stem::info;

#[no_mangle]
pub extern "C" fn main(_arg: usize) {
    info!("CLOCK: Starting...");

    loop {
        let now = stem::time::now_unix_seconds();
        info!("CLOCK: unix={}", now);
        stem::sleep(core::time::Duration::from_secs(5));
    }
}
