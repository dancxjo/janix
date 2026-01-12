#![no_std]
#![no_main]

extern crate alloc;
use stem::{info, error};

mod devtree;
mod registry;
mod supervisor;

#[no_mangle]
pub extern "C" fn main(_arg0: usize) {
    info!("SPROUT: v0.4 starting (Supervisor Mode)...");
    
    match devtree::init() {
        Ok(ctx) => {
             if let Err(_) = devtree::build(&ctx) {
                 error!("SPROUT: Failed to build device tree!");
             }
        },
        Err(_) => {
            error!("SPROUT: Failed to initialize devtree context! (continuing)");
        }
    }

    let mut sup = supervisor::Supervisor::new();
    sup.run_forever();
}
