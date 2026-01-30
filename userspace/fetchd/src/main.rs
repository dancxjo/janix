#![no_std]
#![no_main]

//! # Fetch Service (fetchd)
//!
//! A demo application that fetches URLs and stores the results in the graph.
//! Uses netd's socket API for network access.
//!
//! NOTE: This is a stub implementation - full HTTP client functionality will
//! be added once netd exposes a socket API.

extern crate alloc;

use stem::{info, warn};

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("FETCHD: Starting fetch service demo...");
    
    // For now, just wait indefinitely
    // In the future, this will:
    // 1. Wait for netd to be ready
    // 2. Connect to netd's socket API
    // 3. Use TCP sockets to perform HTTP requests
    // 4. Parse and store results in the graph
    
    info!("FETCHD: Waiting for netd socket API to be available...");
    
    // TODO: Implement socket client to connect to netd
    // TODO: Move HTTP client code here
    // TODO: Move graph_sink code here
    
    loop {
        stem::time::sleep_ms(10000);
        info!("FETCHD: Still waiting for socket API...");
    }
}
