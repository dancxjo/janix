#![no_std]

use hw::HardwareBridge;

pub struct Bridge;

impl HardwareBridge for Bridge {
    fn log(&self, _msg: &str) { /* TODO */
    }
    fn ticks(&self) -> u64 {
        0
    }
    fn idle(&self) { /* TODO: hlt/wfi */
    }
    fn shutdown(&self) -> ! {
        loop { /* TODO */ }
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
}
