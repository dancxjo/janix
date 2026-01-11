#![no_std]
#![no_main]

extern crate alloc;

use stem::kprintln;
use stem::thing::{sys as thingsys, query};

mod devtree;

#[no_mangle]
pub extern "C" fn main(_arg0: usize) {
    kprintln!("SPROUT: v0.2 starting...");
    
    // 1. Initialize Context
    let ctx = match devtree::init() {
        Ok(c) => c,
        Err(_) => {
            kprintln!("SPROUT: Failed to initialize devtree context!");
            stem::syscall::exit(1);
        }
    };
    
    // 2. Build Device Tree
    if let Err(_) = devtree::build(&ctx) {
        kprintln!("SPROUT: Failed to build device tree!");
    } else {
        kprintln!("SPROUT: Device tree build complete.");
    }
    
    // 3. Query Demo
    kprintln!("SPROUT: Running Queries...");
    
    // Query 1: Find all memory ranges
    let mut mems = [stem::thing::ThingId(0); 16];
    if let Ok(count) = query::query_nodes_by_kind("mem.range", 16, &mut mems) {
        kprintln!("Q1: Found {} memory ranges.", count);
        for i in 0..count {
             kprintln!("  - ID {}", mems[i].0);
        }
    }
    
    // Query 2: Outgoing edges from Host
    kprintln!("Q2: Edges from Host (HAS_DEVICE, etc)");
    if let Ok(edges) = query::query_edges(ctx.host, None, 16) {
        for (rel, dst) in edges {
             kprintln!("  - Host --[{}]--> {}", rel, dst.0);
        }
    }
    
    // 4. Dump Entire Graph
    kprintln!("SPROUT: Dumping Root graph (legacy dump)...");
    let mut buf = [0u8; 512];
    for i in 1..64 {
        if let Ok(len) = thingsys::describe_thing(stem::thing::ThingId(i), &mut buf) {
             if len > 0 {
                 if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                      kprintln!("{}", s);
                      if let Ok(elen) = thingsys::dump_edges(stem::thing::ThingId(i), &mut buf) {
                           if let Ok(es) = core::str::from_utf8(&buf[..elen]) {
                               kprintln!("{}", es);
                           }
                      }
                 }
             }
        }
    }
    
    kprintln!("SPROUT: Done.");
    
    loop {
        stem::syscall::yield_now();
    }
}
