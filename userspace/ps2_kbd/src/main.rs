//! PS/2 Keyboard Driver
//! 
//! Reads scancodes from i8042 controller (first port) and sends them to a port.
//! Ignores mouse data (aux port) - that's handled by ps2_mouse.

#![no_std]
#![no_main]

use stem::info;
use stem::syscall::{ioport_read, port_send, PortHandle};

/// PS/2 controller status register (read-only)
const PS2_STATUS: usize = 0x64;
/// PS/2 controller data register (read/write)
const PS2_DATA: usize = 0x60;

/// Status bit: output buffer full (data ready to read)
const STATUS_OUTPUT_FULL: usize = 0x01;
/// Status bit: data is from aux port (mouse) - skip this
const STATUS_AUX_DATA: usize = 0x20;

#[stem::main]
fn main(raw_write_handle: usize) -> ! {
    let handle = raw_write_handle as PortHandle;
    
    info!("ps2_kbd: online (handle={})", handle);
    info!("ps2_kbd: using ports 0x60/0x64 (keyboard only)");
    
    loop {
        // Poll status register for data availability
        let status = ioport_read(PS2_STATUS, 1);
        
        if status & STATUS_OUTPUT_FULL != 0 {
            // Check if this is keyboard data (NOT from aux/mouse port)
            if status & STATUS_AUX_DATA == 0 {
                // Keyboard data - read and send
                let scancode = ioport_read(PS2_DATA, 1) as u8;
                let _ = port_send(handle, &[scancode]);
            }
            // If it's mouse data, DON'T read it - let ps2_mouse handle it
        } else {
            // No data available, yield
            stem::yield_now();
        }
    }
}
