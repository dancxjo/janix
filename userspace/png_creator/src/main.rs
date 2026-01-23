#![no_std]
#![no_main]

extern crate alloc;
use stem::syscall;
use abi::symbols::{SYMBOL_REF_TAG_STR, SymbolRefWire};

#[stem::main]
fn main(_arg: usize) -> ! {
    syscall::log_write("PNG_CREATOR: Starting...", 1).unwrap();

    // Create a node (Bytespace)
    // 4KB size, flags=0, format=0
    let node_id = match syscall::root_bytespace_create(4096, 0, 0) {
        Ok(id) => id,
        Err(e) => {
            use alloc::format;
            let msg = format!("PNG_CREATOR: Failed to create bytespace: {:?}", e);
            syscall::log_write(&msg, 1).unwrap();
            loop { syscall::sleep_ms(1000); }
        }
    };

    // Write PNG header
    // \x89PNG\r\n\x1a\n
    let png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    if let Err(e) = syscall::root_bytespace_write(node_id as usize, 0, &png_header) {
        let msg = format!("PNG_CREATOR: Failed to write bytespace: {:?}", e);
        syscall::log_write(&msg, 1).unwrap();
        loop { syscall::sleep_ms(1000); }
    }

    syscall::log_write("PNG_CREATOR: Wrote PNG header", 1).unwrap();

    loop {
        syscall::sleep_ms(1000);
    }
}
