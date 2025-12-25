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
    fn system_now(&self) -> u64 {
        #[cfg(feature = "std")]
        {
             use std::time::{SystemTime, UNIX_EPOCH};
             SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
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
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> [u64; 20] {
        [0; 20] // Hosted doesn't support user threads yet
    }
    fn resume_user_mode(&self, _context: &[u64]) -> ! {
        loop {}
    }
}
