#![no_std]
#![no_main]

extern crate alloc;
use stem::println;

mod devtree;
mod registry;
mod supervisor;

#[no_mangle]
pub extern "C" fn main(_arg0: usize) {
    println!("SPROUT: v0.4 starting (Supervisor Mode)...");
    
    match devtree::init() {
        Ok(ctx) => {
             if let Err(_) = devtree::build(&ctx) {
                 println!("SPROUT: Failed to build device tree!");
             }
        },
        Err(_) => {
            println!("SPROUT: Failed to initialize devtree context! (continuing)");
        }
    }

    let mut sup = supervisor::Supervisor::new();
    sup.run_forever();
}
