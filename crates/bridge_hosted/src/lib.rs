#![cfg_attr(not(feature = "std"), no_std)]

use hw::HardwareBridge;

pub mod symbol_store;
pub use symbol_store::FileSymbolStore;

pub struct HostedBridge;

impl HardwareBridge for HostedBridge {
    fn log(&self, msg: &str) {
        #[cfg(feature = "std")]
        print!("{msg}");
    }
    fn ticks(&self) -> u64 {
        #[cfg(feature = "std")]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
        }
        #[cfg(not(feature = "std"))]
        0
    }
    fn idle(&self) {
        #[cfg(feature = "std")]
        std::thread::yield_now();
    }
    fn shutdown(&self) -> ! {
        #[cfg(feature = "std")]
        std::process::exit(0);
        #[cfg(not(feature = "std"))]
        loop {}
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
}
