#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use models::*;

#[no_mangle]
pub fn main() {
    thing_std::init(0);
    log_info("--- Thing Check Tool ---");

    // 1. Check Display Device
    if let Some(id) = thing_find("device.display0") {
        match DisplayDevice::read(&SyscallGraphClient, id) {
            Ok(display) => {
                log_info("Found DisplayDevice!");
                log_fmt("  Resolution: ", format_args!("{}x{}", display.width, display.height));
                log_fmt("  Refresh: ", format_args!("{} Hz", display.refresh_hz));
            }
            Err(e) => {
                log_fmt("  [ERROR] DisplayDevice read failed: ", format_args!("{}", e));
            }
        }
    }

    // 2. Check Window bodies
    // Usually windows are discovered via relationships, but for check we might name them
    if let Some(id) = thing_find("window.test") {
        match Window::read(&SyscallGraphClient, id) {
            Ok(window) => {
                log_info("Found Window!");
                log_fmt("  Title symbol: ", format_args!("0x{:X}", window.title.0));
                log_fmt("  Pos: ", format_args!("{},{}", window.x, window.y));
            }
            Err(e) => {
                log_fmt("  [ERROR] Window read failed: ", format_args!("{}", e));
            }
        }
    }

    // 3. Check Clock
    if let Some(id) = thing_find("clock.system") {
        match SystemClock::read(&SyscallGraphClient, id) {
            Ok(clock) => {
                log_info("Found SystemClock!");
                log_fmt("  Unix Time: ", format_args!("{}", clock.unix_epoch_ns));
            }
            Err(e) => {
                log_fmt("  [ERROR] SystemClock read failed: ", format_args!("{}", e));
            }
        }
    }

    log_info("Check complete.");
}

fn log_fmt(prefix: &str, args: core::fmt::Arguments) {
    let mut buf = alloc::string::String::new();
    let _ = core::fmt::write(&mut buf, args);
    log_info(alloc::format!("{}{}", prefix, buf).as_str());
}
