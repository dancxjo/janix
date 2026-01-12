//! PS/2 Keyboard Driver (Interrupt-driven)
//! 
//! Subscribes to IRQ1 via IOAPIC, reads scancodes on interrupt, sends to Bristle.
//! Ignores mouse data (aux port) - that's handled by ps2_mouse.

#![no_std]
#![no_main]

use stem::info;
use stem::syscall::{ioport_read, irq_subscribe, irq_wait, port_send, PortHandle};

/// PS/2 controller status register
const PS2_STATUS: usize = 0x64;
/// PS/2 controller data register  
const PS2_DATA: usize = 0x60;

/// Status: output buffer full
const STATUS_OUTPUT_FULL: usize = 0x01;
/// Status: data from aux port (mouse) - skip
const STATUS_AUX_DATA: usize = 0x20;

/// IRQ1 vector (keyboard) - legacy IRQ1 maps to vector 0x21 after IOAPIC remap
const KBD_VECTOR: u8 = 0x21;

#[stem::main]
fn main(raw_write_handle: usize) -> ! {
    let handle = raw_write_handle as PortHandle;
    
    info!("ps2_kbd: online (handle={})", handle);
    
    // Subscribe to keyboard interrupt
    match irq_subscribe(KBD_VECTOR) {
        Ok(()) => info!("ps2_kbd: subscribed to IRQ1 (vector 0x{:02x})", KBD_VECTOR),
        Err(e) => {
            info!("ps2_kbd: IRQ subscribe failed ({:?}), falling back to polling", e);
            polling_loop(handle);
        }
    }
    
    info!("ps2_kbd: entering interrupt-driven loop");
    
    loop {
        // Wait for keyboard interrupt
        match irq_wait(KBD_VECTOR) {
            Ok(_count) => {
                // Drain all available keyboard data
                drain_keyboard_data(handle);
            }
            Err(_) => {
                // Should not happen, but fallback to yield
                stem::yield_now();
            }
        }
    }
}

/// Drain all pending keyboard data from the controller
fn drain_keyboard_data(handle: PortHandle) {
    // Read while data is available (handle burst of scancodes)
    for _ in 0..16 {
        let status = ioport_read(PS2_STATUS, 1);
        
        if status & STATUS_OUTPUT_FULL == 0 {
            break; // No more data
        }
        
        if status & STATUS_AUX_DATA == 0 {
            // Keyboard data - read and send
            let scancode = ioport_read(PS2_DATA, 1) as u8;
            let _ = port_send(handle, &[scancode]);
        }
        // If aux data (mouse), don't read - let ps2_mouse handle it
    }
}

/// Fallback polling loop (if IRQ subscribe fails)
fn polling_loop(handle: PortHandle) -> ! {
    info!("ps2_kbd: using polling mode");
    loop {
        let status = ioport_read(PS2_STATUS, 1);
        
        if status & STATUS_OUTPUT_FULL != 0 {
            if status & STATUS_AUX_DATA == 0 {
                let scancode = ioport_read(PS2_DATA, 1) as u8;
                let _ = port_send(handle, &[scancode]);
            }
        } else {
            stem::yield_now();
        }
    }
}
