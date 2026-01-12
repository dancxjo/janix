#![no_std]
#![no_main]

extern crate alloc;
use stem::{error, info};

mod devtree;
mod registry;
mod supervisor;

#[stem::main]
fn main(_arg0: usize) -> ! {
    let cpu = stem::arch::whoami();
    info!(
        "[sprout] whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x} rflags=0x{:x}",
        cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip, cpu.rflags
    );

    info!("SPROUT: v0.4 starting (Supervisor Mode)...");

    match devtree::init() {
        Ok(ctx) => {
            if let Err(_) = devtree::build(&ctx) {
                error!("SPROUT: Failed to build device tree!");
            }
        }
        Err(_) => {
            error!("SPROUT: Failed to initialize devtree context! (continuing)");
        }
    }

    stem::info!("SPROUT: About to create Supervisor...");
    let mut sup = supervisor::Supervisor::new();
    stem::info!("SPROUT: Supervisor created, calling run_forever...");
    sup.run_forever()
}
