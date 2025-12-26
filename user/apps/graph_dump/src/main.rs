#![no_std]
#![no_main] // User's skeleton had pub extern "C" fn main but implied no_std. Usually no_std requires no_main and a #[no_mangle] entry point. Use #![no_main] for safety with #![no_std].

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};

#[derive(serde::Serialize)]
struct DumpReq {
    // v0: empty
}

#[derive(serde::Deserialize)]
struct DumpResp {
    text: alloc::string::String,
}

#[no_mangle]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let g = GraphClient::new();
    let c = StdoutConsole;
    let mut out = [0u8; 4096];

    match g.call::<DumpReq, DumpResp>("graph.dump", &DumpReq {}, &mut out) {
        Ok(resp) => {
            c.write_str(&resp.text);
            c.write_str("\n");
        }
        Err(_) => {
            c.write_str("graph_dump: error\n");
        }
    }
    
    loop {
        // Yield?
        // simple spin for now to avoid exit crash
         core::hint::spin_loop();
    }
}
