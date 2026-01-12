//! PS/2 Keyboard Driver
//! 
//! Reads scancodes from i8042 controller and sends them to a port.
//! This is a "leaf" driver with no dependencies on higher-level services.

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

#[stem::main]
fn main(raw_write_handle: usize) -> ! {
    let handle = raw_write_handle as PortHandle;
    
    info!("ps2_kbd: online (handle={})", handle);
    info!("ps2_kbd: using ports 0x60/0x64");
    
    loop {
        // Poll status register for data availability
        let status = ioport_read(PS2_STATUS, 1);
        
        if status & STATUS_OUTPUT_FULL != 0 {
            // Read scancode from data port
            let scancode = ioport_read(PS2_DATA, 1) as u8;
            
            // Send to the raw port
            let _ = port_send(handle, &[scancode]);
        } else {
            // No data available, yield to avoid burning CPU
            stem::yield_now();
        }
    }
}
