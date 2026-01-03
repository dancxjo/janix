#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    init(0);
    log_info("GRAPH SMOKE: Starting...");
    log_info("GRAPH SMOKE: Done.");
    sys_exit(0);
}
