#![no_std]
#![no_main]

use stem::syscall;
use abi::kinds::{KIND_HOST, REL_HAS_MODULE};

#[stem::main]
fn main() -> ! {
    let _ = syscall::log_write("PNG_CREATOR: Starting...", 1);

    // Find Host
    let mut host_ids = [0u64; 1];
    let host_id = match syscall::graph::find(KIND_HOST, &mut host_ids) {
        Ok(n) if n > 0 => host_ids[0],
        _ => {
            let _ = syscall::log_write("PNG_CREATOR: Failed to find Host", 1);
            loop { syscall::sleep_ms(1000); }
        }
    };

    // Create a bytespace (node) to store the PNG
    match syscall::root_bytespace_create(1024, 0, 0) {
        Ok(bs_id) => {
            // PNG signature
            let png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

            match syscall::root_bytespace_write(bs_id as usize, 0, &png_header) {
                Ok(_) => {
                    let _ = syscall::log_write("PNG_CREATOR: Wrote PNG header", 1);

                    // Link to Host so ingestd can see it (if it watches connected graph)
                    // Note: root_link expects usize
                    match syscall::root_link(host_id as usize, REL_HAS_MODULE as usize, bs_id as usize) {
                        Ok(_) => {
                            let _ = syscall::log_write("PNG_CREATOR: Linked to Host", 1);
                        }
                        Err(_) => {
                            let _ = syscall::log_write("PNG_CREATOR: Failed to link to Host", 1);
                        }
                    }
                }
                Err(_) => {
                    let _ = syscall::log_write("PNG_CREATOR: Failed to write PNG header", 1);
                }
            }
        }
        Err(_) => {
            let _ = syscall::log_write("PNG_CREATOR: Failed to create bytespace", 1);
        }
    }

    // Sleep forever
    loop {
        syscall::sleep_ms(1000);
    }
}
