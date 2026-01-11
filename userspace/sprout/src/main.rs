#![no_std]
#![no_main]

extern crate alloc;

use stem::kprintln;
use stem::thing::sys as thingsys;

mod devtree;

#[no_mangle]
pub extern "C" fn main(_arg0: usize) {
    kprintln!("SPROUT: v0.2 starting (Unified Device Graph Mode)...");
    
    // 1. Initialize Context
    match devtree::init() {
        Ok(ctx) => {
             // 2. Build Device Tree (Enrichment)
             if let Err(_) = devtree::build(&ctx) {
                 kprintln!("SPROUT: Failed to build device tree!");
             }
        },
        Err(_) => {
            kprintln!("SPROUT: Failed to initialize devtree context! (continuing to dump)");
        }
    }

    // 3. Dump Entire Graph (Deterministic)
    kprintln!("SPROUT: Dumping Root graph...");
    let _ = thingsys::dump_graph(4096);
    
    kprintln!("SPROUT: Done.");
    stem::syscall::exit(0);
}
