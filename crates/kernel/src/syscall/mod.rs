pub mod cap;
pub mod dispatch;
pub mod graph;
pub mod memory;
pub mod watch;
pub mod surface;
pub mod input;
pub mod log;
pub mod display; // Added
pub use dispatch::dispatch;
pub use abi::syscall::{nr, err};

pub fn init() {
    crate::log::klog(crate::log::Level::Info, "SYSCALL", "init");
}
