
#![no_std]
#![no_main]

extern crate alloc;

mod chime;

use alloc::vec::Vec;
use stem::syscall::port::{port_create, port_recv, port_send};
use stem::thing::sys as thingsys;
use stem::{info, warn, error};
use abi::schema::kinds;
use stem::thing::ThingId;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("BEEPER: Starting audio demo...");

    // 1. Find VirtIO Sound Device to get its Write Port
    let mut dev_buf = [ThingId::default(); 1];
    let mut dev_id = ThingId::default();
    let mut port_handle = 0;
    
    // Retry loop to wait for driver to start
    for i in 0..50 {
        if let Ok(count) = thingsys::find(kinds::DEV_SOUND, &mut dev_buf) {
            if count > 0 {
                dev_id = dev_buf[0];
                // Read the WRITE_PORT_HANDLE property
                let h = thingsys::prop_get(dev_id, abi::schema::keys::WRITE_PORT_HANDLE).unwrap_or(0);
                if h != 0 {
                    port_handle = h as u32;
                    break;
                }
            }
        }
        stem::time::sleep_ms(100);
        if i % 10 == 0 {
            info!("BEEPER: Waiting for sound driver...");
        }
    }

    if port_handle == 0 {
        error!("BEEPER: Failed to discover sound driver port");
        loop { stem::yield_now(); }
    }
    
    info!("BEEPER: Found driver port {}", port_handle);

    // 2. Generate Audio (Proof-of-Life Chime)
    info!("BEEPER: Generating chime...");
    let chime_data = chime::generate_chime(44100);
    info!("BEEPER: Chime generated ({} bytes)", chime_data.len());
    
    // 3. Play Chime
    // Send in 4KB chunks
    for chunk in chime_data.chunks(4096) {
        // Loop until sent (blocking-ish)
        loop {
            match port_send(port_handle, chunk) {
                Ok(_) => break,
                Err(_) => {
                    // Port full? Yield and retry to keep pressure up.
                    stem::yield_now();
                }
            }
        }
    }
    
    info!("BEEPER: Chime complete. Exiting.");
    
    // Exit (or sleep forever)
    loop { stem::time::sleep_ms(1000); }
}
