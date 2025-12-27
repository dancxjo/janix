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
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> [u64; 34] {
        [0; 34] // Hosted doesn't support user threads yet
    }
    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample) {
        #[cfg(feature = "std")]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            // This is rough approximation for hosted
            if let Ok(dur) = SystemTime::now().duration_since(UNIX_EPOCH) {
                let secs = dur.as_secs();
                // We'd need a real datetime crate to convert to YMD, but for now just leave 0
                // or put simple fake values.
                out.year = 2024;
                out.mon = 1;
                out.day = 1;
                out.hour = ((secs / 3600) % 24) as u8;
                out.min = ((secs / 60) % 60) as u8;
                out.sec = (secs % 60) as u8;
            }
        }
    }
    fn resume_user_mode(&self, _context: &[u64]) -> ! {
        loop {}
    }
    fn set_kernel_stack(&self, _stack_top: u64) {}
}
