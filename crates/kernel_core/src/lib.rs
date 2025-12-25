#![no_std]

use hw::HardwareBridge;

pub struct Kernel<B: HardwareBridge> {
bridge: B,
}

impl<B: HardwareBridge> Kernel<B> {
    pub fn new(bridge: B) -> Self {
        Self { bridge }
    }

    pub fn boot(&self) -> ! {
        self.bridge.log(models::milestones::KERNEL_ENTRY);
        self.bridge.log("\n");
        self.bridge.log(models::milestones::BRIDGE_ONLINE);
        self.bridge.log("\n");

        loop {
            self.bridge.log(models::milestones::IDLE_LOOP);
            self.bridge.log("\n");
            self.bridge.idle();
        }
    }
}
